OUTPUT_FORMAT(elf64-x86-64)
ENTRY(_start)

/* Explicitly define the Program Headers and their permissions */
/* FLAGS: 4 = Read, 5 = Read+Execute, 6 = Read+Write */
PHDRS
{
    text    PT_LOAD    FLAGS((1 << 0) | (1 << 2)); /* Execute + Read */
    rodata  PT_LOAD    FLAGS((1 << 2));            /* Read only */
    data    PT_LOAD    FLAGS((1 << 1) | (1 << 2)); /* Write + Read */
}

KERNEL_OFFSET = 0xffffffff80000000;

SECTIONS {
    . = KERNEL_OFFSET + 0x100000;

    . = ALIGN(0x1000);
    .text : {
        *(.text .text.*)
    } :text /* Force this into the 'text' PHDR */

    . = ALIGN(0x1000);
    .rodata : {
        *(.rodata .rodata.*)
    } :rodata /* Force this into the 'rodata' PHDR */

    . = ALIGN(0x1000);
    .data : {
        *(.data .data.*)
    } :data /* Force this into the 'data' PHDR */

    . = ALIGN(0x1000);
    .bss : {
        *(.bss .bss.*)
        *(COMMON)
    } :data /* BSS goes into data since it also needs Read/Write */

    /DISCARD/ : {
        *(.eh_frame)
        *(.note .note.*)
    }
}
