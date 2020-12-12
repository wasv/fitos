ENTRY(_start)

SECTIONS {
    .boot :
    {
        /* ensure that the multiboot header is at the beginning */
        KEEP(*(.multiboot_header))
        KEEP(*(.bootloader))
    }

    . = 1M;
    .kernel : ALIGN(4)
    {
        KEEP(*(.kernel))
    }

    _stack_start = .;
    . += 0x7c00;
    _stack_end = .;
}
