ENTRY(_start)

SECTIONS
{
    . = 0x80000;     /* RPi 64-bit kernel boot address */
    
    .text : {
        KEEP(*(.text.boot))
        *(.text .text.*)
    }
    
    .rodata : {
        *(.rodata .rodata.*)
    }
    
    .data : {
        *(.data .data.*)
    }
    
    .bss (NOLOAD) : {
        . = ALIGN(16);
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        __bss_end = .;
        
        /* FIX: Put this calculation INSIDE the block */
        __bss_size = (__bss_end - __bss_start) >> 3;
    }
}

