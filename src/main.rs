#![no_std]
#![no_main]
// カーネル本体

mod byte_io;
mod cursor;
mod key;
mod printk;
mod vga;

use core::panic::PanicInfo;
use key::{KbdState, kbd_has_scancode, scan_key};
use vga::switch_writer;

// マングリングを無効化して、ASMから "kmain" という名前で呼べるようにする
#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    printk!("42\n");

    // キーボードループ
    let mut state = KbdState::new();

    loop {
        if !kbd_has_scancode() {
            continue;
        }

        let c = scan_key(&mut state);
        if c == 0 {
            continue;
        } else {
            if state.is_alt() {
                match state.get_key() {
                    // スクリーンを切り替える
                    b'0' => {
                        switch_writer(0);
                    }
                    b'1' => {
                        switch_writer(1);
                    }
                    b'2' => {
                        switch_writer(2);
                    }
                    _ => {}
                }
                continue;
            }

            printk!("{}", c as char);
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
