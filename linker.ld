ENTRY(_start);

SECTIONS {
    . = 0x80000000;

    .text : {
        *(.text.init)    /* S'assure que _start est bien au début */
        *(.text .text.*)
    }

    .rodata : {
        *(.rodata .rodata.*)
    }

    .data : {
        /* __global_pointer$ est souvent placé au milieu des données
           pour maximiser la portée de l'offset relatif de 2KB */
        __global_pointer$ = . + 0x800;
        *(.data .data.*)
    }

    .bss : {
        _bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        _bss_end = .;
    }

    /* Définition de la pile en fin de bss ou dans une section dédiée */
    .stack (NOLOAD) : {
        . = ALIGN(16);
        _stack_bottom = .;
        . += 0x4000;      /* 16 KB */
        _stack_top = .;
    }

    . = ALIGN(4096);
    _heap_start = .;
}