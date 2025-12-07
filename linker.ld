/* リンカスクリプト */
ENTRY(start)

SECTIONS {
    . = 1M; /* . = 0x00100000 と同じ; カーネルを物理アドレス 1MiB に置く */

    .boot :
    {
        /* すべての入力ファイルの .multiboot_header セクションを .boot セクションに集める; 未使用セクション削除から保護 */
        KEEP(*(.multiboot_header)) 
        /* ブートローダに .multiboot_header を確実に読ませるため, .boot セクションに .text と別に置いている */
    }

    .text :
    {
        *(.text*)
    }

    .rodata :
    {
        *(.rodata*)
    }

    .data :
    {
        *(.data*)
    }

    .bss :
    {
        *(.bss*)
    }
}