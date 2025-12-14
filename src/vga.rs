use core::fmt;

pub struct VGAScreen {
    pub pen: u8,
    pub cursor_pos: usize,
}

impl VGAScreen {
    const WIDTH: usize = 80;
    const HEIGHT: usize = 25;

    pub fn new() -> Self {
        VGAScreen {
            pen: 0x0f, // 白文字・黒背景
            cursor_pos: 0,
        }
    }

    pub fn change_pen(&mut self, pen: u8) {
        self.pen = pen;
    }

    pub fn write_byte(&mut self, byte: u8) {
        unsafe {
            let buffer = 0xb8000 as *mut u8;
            *buffer.add(self.cursor_pos * 2) = byte;
            *buffer.add(self.cursor_pos * 2 + 1) = self.pen;
        }
        self.cursor_pos += 1;
        if self.cursor_pos >= Self::WIDTH * Self::HEIGHT {
            self.scroll();
        }
    }

    pub fn newline(&mut self) {
        self.cursor_pos += Self::WIDTH - (self.cursor_pos % Self::WIDTH);
        if self.cursor_pos >= Self::WIDTH * Self::HEIGHT {
            self.scroll();
        }
    }

    pub fn scroll(&mut self) {
        unsafe {
            let buffer = 0xb8000 as *mut u8;
            for row in 1..Self::HEIGHT {
                for col in 0..Self::WIDTH {
                    let from = (row * Self::WIDTH + col) * 2;
                    let to = ((row - 1) * Self::WIDTH + col) * 2;
                    *buffer.add(to) = *buffer.add(from);
                    *buffer.add(to + 1) = *buffer.add(from + 1);
                }
            }
            for col in 0..Self::WIDTH {
                let last_row = (Self::HEIGHT - 1) * Self::WIDTH + col;
                *buffer.add(last_row * 2) = b' ';
                *buffer.add(last_row * 2 + 1) = self.pen;
            }
        }
        self.cursor_pos = (Self::HEIGHT - 1) * Self::WIDTH;
    }
}

impl fmt::Write for VGAScreen {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            match byte {
                b'\n' => self.newline(),
                byte => self.write_byte(byte),
            }
        }
        Ok(())
    }
}