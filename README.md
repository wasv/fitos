# FitOS

An x86 rust kernel that is bootable using multiboot2 and grub.

## Dpendencies

- Rust nightly later than 2020-12-11 with the following components:
  - `llvm-tools-preview`
  - `rust-src`
- [nasm](https://www.nasm.us/) for assembling the startup code.
- `grub-mkrescue` command (or equivalent multiboot2 compatible bootloader)
- `qemu-system-i386` command for testing (optional).

## Creating an Image on Linux

``` bash
# Build kernel
cargo build

# Create bootdisk
mkdir -p target/bootdisk/boot/grub
cp res/grub.cfg target/bootdisk/boot/grub/grub.cfg
cp target/i586-bootloader/debug/fitos target/bootdisk/boot/fitos
grub-mkrescue -o target/fitos.iso target/bootdisk/

# Test image
qemu-system-i386 -cdrom target/fitos.iso
```

## About the License

This project is licensed under the hippocratic license, which works similarly to the MIT license, with the exception that the license terminates if you are in violation of the international declaration of human rights. for more info, see here: https://firstdonoharm.dev/

Some of this code is based on tutorial code from [phil-opp/blog-os](https://github.com/phil-opp/blog_os), which is licensed under the MIT license.
