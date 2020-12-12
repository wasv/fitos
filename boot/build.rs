use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Assemble nasm files.
    let asm_files = &["src/boot.asm", "src/multiboot.asm"];
    let mut nasm = nasm_rs::Build::new();
    nasm.flag("-felf");
    for asm_file in asm_files {
        nasm.file(asm_file);
        println!("cargo:rerun-if-changed={}", asm_file);
    }
    nasm.compile("boot").unwrap();

    // link kernel
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=boot");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../linker.ld");
}
