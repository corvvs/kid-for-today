#![no_std]
#![no_main]
// カーネル本体

use core::panic::PanicInfo;

// マングリングを無効化して、ASMから "kmain" という名前で呼べるようにする
#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {

    // ここにVGAバッファへの書き込み処理を書く
    // メモリ番地 0xb8000 に書き込むと画面に出る

    let mut vga = VGAScreen::new();
    
    vga.change_pen(2);
    vga.write_byte(b'2');
    vga.change_pen(3);
    vga.write_byte(b'4');
    vga.change_pen(4);
    vga.write_byte(b'3');
    vga.change_pen(5);
    vga.write_byte(b'1');
    vga.change_pen(6);
    vga.write_byte(b'5');
    vga.change_pen(7);
    vga.write_byte(b'7');

    loop {}
}

struct VGAScreen {
    buffer: *mut u8,
    pen: u8,
    cursor_pos: usize,
}

impl VGAScreen {
    const WIDTH: usize = 80;
    const HEIGHT: usize = 25;

    fn new() -> Self {
        VGAScreen {
            buffer: 0xb8000 as *mut u8,
            pen: 0x0f, // 白文字・黒背景
            cursor_pos: 0,
        }
    }

    fn change_pen(&mut self, pen: u8) {
        self.pen = pen;
    }

    fn write_byte(&mut self, byte: u8) {
        unsafe {
            *self.buffer.add(self.cursor_pos * 2) = byte;
            *self.buffer.add(self.cursor_pos * 2 + 1) = self.pen;
        }
        self.cursor_pos += 1;
        if self.cursor_pos >= Self::WIDTH * Self::HEIGHT {
            self.scroll();
        }
    }

    fn scroll(&mut self) {
        unsafe {
            for row in 1..Self::HEIGHT {
                for col in 0..Self::WIDTH {
                    let from = (row * Self::WIDTH + col) * 2;
                    let to = ((row - 1) * Self::WIDTH + col) * 2;
                    *self.buffer.add(to) = *self.buffer.add(from);
                    *self.buffer.add(to + 1) = *self.buffer.add(from + 1);
                }
            }
            for col in 0..Self::WIDTH {
                let last_row = (Self::HEIGHT - 1) * Self::WIDTH + col;
                *self.buffer.add(last_row * 2) = b' ';
                *self.buffer.add(last_row * 2 + 1) = self.pen;
            }
        }
        self.cursor_pos = (Self::HEIGHT - 1) * Self::WIDTH;
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
