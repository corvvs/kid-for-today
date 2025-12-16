#![no_std]
#![no_main]
// カーネル本体

mod byte_io;
mod vga;
mod printk;
mod key;
mod cursor;

use core::{panic::PanicInfo};
use key::{KbdState, scan_key, kbd_has_scancode};

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
        }

        printk!("{}", c as char);
    }
}



#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
