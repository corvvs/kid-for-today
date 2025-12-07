#![no_std]
#![no_main]
// カーネル本体

use core::panic::PanicInfo;

// マングリングを無効化して、ASMから "kmain" という名前で呼べるようにする
#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {

    // ここにVGAバッファへの書き込み処理を書く
    // メモリ番地 0xb8000 に書き込むと画面に出る

    let vga_buffer = 0xb8000 as *mut u8;
    
    // "42"を表示する例 [cite: 381]
    unsafe {
        *vga_buffer.offset(0) = b'4';
        *vga_buffer.offset(1) = 0x0f; // 白文字・黒背景
        *vga_buffer.offset(2) = b'2';
        *vga_buffer.offset(3) = 0x0f;
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
