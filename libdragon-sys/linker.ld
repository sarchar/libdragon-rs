ENTRY(_start)

__kseg0_start = 0x80000000;

/* Start address of code is 1K up in cached, unmapped RAM (KSEG0). We have
* to be at least this far up in order to not interfere with the cart
* boot code which is copying it down from the cart
*/
__libdragon_text_start  = __kseg0_start + 0x400;

MEMORY
{
    mem : ORIGIN = __kseg0_start, LENGTH = 4M
}

SECTIONS {
    . = 0x80000400;
    __boot_start = .;

    .boot __libdragon_text_start : {
        *(.boot)
        . = ALIGN(16);
    } > mem

    .text : {
        __text_start = .;
        *(.text)
        *(.text.*)
        *(.init)
        *(.fini)
        *(.gnu.linkonce.t.*)
        . = ALIGN(16);
    } > mem

    .eh_frame_hdr : { *(.eh_frame_hdr) } > mem
    .eh_frame : { 
     	__EH_FRAME_BEGIN__ = .; /* Define symbol for accessing eh_frame section */
     	KEEP (*(.eh_frame)) 
     } > mem
    .gcc_except_table : { *(.gcc_except_table*) } > mem
    .jcr : { KEEP (*(.jcr)) } > mem

    .rodata : {
        *(.rdata)
        *(.rodata)
        *(.rodata.*)
        *(.gnu.linkonce.r.*)
        . = ALIGN(8);
    } > mem

    /* This is important to keep __CTOR_END__ consistent */
    . = ALIGN(4);

    .ctors : {
        __CTOR_LIST__ = .;
        /* Actually we should place the count here like;

        LONG((__CTOR_END__ - __CTOR_LIST__) / 4 - 2)

        but g++ seem to already do this (though incorrectly placing an 0xFFFFFFFF)
        and that breaks things if we also add it. Ideally this should be parsed
        by the code but we already know the end as well.

        The new build system links everything via g++ by default and should use
        __wrap___do_global_ctors and enables it via the --wrap linker option.
        When linking with ld, we should use __do_global_ctors, which is the default
        if you don't provide the --wrap option. */
        KEEP(*(.ctors))
         /* Similarly we should have a;

        LONG(0)

        here, and g++ seem to insert it at __CTOR_END__ */
        __CTOR_END__ = .;
    } > mem

    . = ALIGN(8);

    .data : {
        *(.data)
        *(.data.*)
        *(.gnu.linkonce.d.*)
        . = ALIGN(8);
	    __data_end = .;
    } > mem

    /* Small data START */

    .sdata : {
        _gp = . + 0x8000;
        *(.sdata)
        *(.sdata.*)
        *(.gnu.linkonce.s.*)
        . = ALIGN(8);
    } > mem

    .lit8 : {
        *(.lit8)
        . = ALIGN(8);
    } > mem
    .lit4 : {
        *(.lit4)
        . = ALIGN(8);
    } > mem

    . = ALIGN(8);
    __data_end = .;

    /* Here the ROM is finished. The rest is just in RAM */
    . = ALIGN(8);
    __rom_end = .;

    .sbss (NOLOAD) : {
         __bss_start = .;
        *(.sbss)
        *(.sbss.*)
        *(.gnu.linkonce.sb.*)
        *(.scommon)
        *(.scommon.*)
    } > mem

    /* Small data END */

    .bss : {
        __bss_start = .;
        *(.bss)
        *(.bss*)
        *(.gnu.linkonce.b.*)
        *(COMMON)
        . = ALIGN(8);
        __bss_end = .;
    } > mem

    . = ALIGN(8);

    /DISCARD/ : {
        *(.MIPS.*)
        *(.comment)
        *(.mdebug.*)
        *(.pdr)
        *(.reginfo)
		
        /*
         * We may need the global offset table some day.
         * Our target is currently set with a static relocation-model, so this
         * might not be needed after all.
         */
        *(.got)
    }
}
