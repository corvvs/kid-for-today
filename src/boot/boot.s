; --- Multiboot Header ---
; GRUBに「これはカーネルだよ」と認識させるためのマジックナンバー
section .multiboot_header    ; -> .boot に配置
align 4
    dd 0x1BADB002            ; Magic number
    dd 0x00                  ; Flags
    dd - (0x1BADB002 + 0x00) ; Checksum (Magic + Flags + Checksum = 0)

; --- Stack ---
; Rustを動かすにはスタック領域が必須なので確保する
section .bss                 ; -> .bss に配置
align 16
stack_bottom:
    resb 16384               ; 16KiBのスタックを予約; resb = reserve bytes(擬似命令)
stack_top:                   ; stack_bottom と stack_top の間は16KiBあくことになる

; --- Entry Point ---
section .text                ; -> .text に配置
global start
extern kmain                 ; Rust側の関数を呼ぶ宣言

start:
    ; スタックポインタ(ESP)を設定
    mov esp, stack_top       ; esp に stack_top を入れる

    call kmain               ; -> fn kmain @ src/main.rs

    ; もしkmainから戻ってきたら、CPUを停止させる
    cli                      ; Clear Interrupt Flag
.hang:
    hlt                      ; Haltし続ける
    jmp .hang