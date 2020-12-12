use std::env;
use std::process::{self, Command};
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let kernel = env::var("KERNEL").unwrap();

    // Assemble nasm files.
    let asm_files = &["src/boot.asm", "src/multiboot.asm"];
    let mut nasm = nasm_rs::Build::new();
    nasm.flag("-felf");
    for asm_file in asm_files {
        nasm.file(asm_file);
        println!("cargo:rerun-if-changed={}", asm_file);
    }
    nasm.compile("boot").unwrap();

    let kernel: PathBuf = PathBuf::from(kernel);
    let kernel_file_name = kernel.file_name()
        .unwrap()
        .to_str()
        .unwrap();

     // get access to llvm tools shipped in the llvm-tools-preview rustup component
    let llvm_tools = match llvm_tools::LlvmTools::new() {
        Ok(tools) => tools,
        Err(llvm_tools::Error::NotFound) => {
            eprintln!("Error: llvm-tools not found");
            eprintln!("Maybe the rustup component `llvm-tools-preview` is missing?");
            eprintln!("  Install it through: `rustup component add llvm-tools-preview`");
            process::exit(1);
        }
        Err(err) => {
            eprintln!("Failed to retrieve llvm-tools component: {:?}", err);
            process::exit(1);
        }
    };
    // strip debug symbols from kernel for faster loading
    let stripped_kernel_file_name = format!("kernel_stripped-{}", kernel_file_name);
    let stripped_kernel = out_dir.join(&stripped_kernel_file_name);
    let objcopy = llvm_tools
        .tool(&llvm_tools::exe("llvm-objcopy"))
        .expect("llvm-objcopy not found in llvm-tools");
    let mut cmd = Command::new(&objcopy);
    cmd.arg("--strip-debug");
    cmd.arg(&kernel);
    cmd.arg(&stripped_kernel);
    let exit_status = cmd
        .status()
        .expect("failed to run objcopy to strip debug symbols");
    if !exit_status.success() {
        eprintln!("Error: Stripping debug symbols failed");
        process::exit(1);
    }
    // wrap the kernel executable as binary in a new ELF file
    let stripped_kernel_file_name_replaced = stripped_kernel_file_name
        .replace('-', "_")
        .replace('.', "_");
    let kernel_bin = out_dir.join(format!("kernel_bin-{}.o", kernel_file_name));
    let kernel_archive = out_dir.join(format!("libkernel_bin-{}.a", kernel_file_name));
    let mut cmd = Command::new(&objcopy);
    cmd.arg("-I").arg("binary");
    cmd.arg("-O").arg("elf32-i386");
    cmd.arg("--binary-architecture=i386:x86");
    cmd.arg("--rename-section").arg(".data=.kernel");
    cmd.arg("--redefine-sym").arg(format!(
        "_binary_{}_start=_kernel_start_addr",
        stripped_kernel_file_name_replaced
    ));
    cmd.arg("--redefine-sym").arg(format!(
        "_binary_{}_end=_kernel_end_addr",
        stripped_kernel_file_name_replaced
    ));
    cmd.arg("--redefine-sym").arg(format!(
        "_binary_{}_size=_kernel_size",
        stripped_kernel_file_name_replaced
    ));
    cmd.current_dir(&out_dir);
    cmd.arg(&stripped_kernel_file_name);
    cmd.arg(&kernel_bin);
    let exit_status = cmd.status().expect("failed to run objcopy");
    if !exit_status.success() {
        eprintln!("Error: Running objcopy failed");
        process::exit(1);
    }
    // create an archive for linking
    let ar = llvm_tools
        .tool(&llvm_tools::exe("llvm-ar"))
        .unwrap_or_else(|| {
            eprintln!("Failed to retrieve llvm-ar component");
            eprint!("This component is available since nightly-2019-03-29,");
            eprintln!("so try updating your toolchain if you're using an older nightly");
            process::exit(1);
        });
    let mut cmd = Command::new(ar);
    cmd.arg("crs");
    cmd.arg(&kernel_archive);
    cmd.arg(&kernel_bin);
    let exit_status = cmd.status().expect("failed to run ar");
    if !exit_status.success() {
        eprintln!("Error: Running ar failed");
        process::exit(1);
    }


    // link kernel
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=kernel_bin-{}", kernel_file_name);
    println!("cargo:rustc-link-lib=static=boot");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../linker.ld");
}
