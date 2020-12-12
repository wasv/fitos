ENTRY(start)

SECTIONS {
    . = 1M;

    .boot :
    {
        /* ensure that the multiboot header is at the beginning */
        KEEP(*(.multiboot_header))
        KEEP(*(.bootloader))
        . = ALIGN(512);
    }

    .kernel : ALIGN(4)
    {
        KEEP(*(.text.*))
    }

    _stack_start = .;
    . += 0x7c00;
    _stack_end = .;
}
