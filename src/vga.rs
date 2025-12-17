use crate::cursor::vga_set_cursor_pos;
use core::fmt;

use spin::{Mutex, Once};

pub static WRITER: Once<Mutex<VGAVirtualScreen>> = Once::new();

pub fn writer() -> &'static Mutex<VGAVirtualScreen> {
    WRITER.call_once(|| Mutex::new(VGAVirtualScreen::new()))
}

pub fn switch_writer(screen_index: usize) {
    let mut w = writer().lock();
    w.switch_screen(screen_index);
}

const WIDTH: usize = 80;
const HEIGHT: usize = 25;
const CELLS: usize = WIDTH * HEIGHT;

pub struct VGAVirtualScreen {
    screens: usize,
    active_screen: usize,
}

impl VGAVirtualScreen {
    pub fn new() -> Self {
        VGAVirtualScreen {
            screens: 2,
            active_screen: 0,
        }
    }

    pub fn switch_screen(&mut self, screen_index: usize) {
        if screen_index == self.active_screen {
            return;
        }
        if screen_index >= self.screens {
            return;
        }
        self.active_screen = screen_index;
        unsafe {
            let screen = self.get_current_screen();
            (*screen).paint();
        }
    }

    fn get_current_screen(&mut self) -> *mut VGAScreen {
        match self.active_screen {
            1 => &raw mut SCREEN1,
            _ => &raw mut SCREEN0,
        }
    }
}

static mut SCREEN0: VGAScreen = VGAScreen {
    pen: 0x0f,
    cursor_pos: 0,
    local_buffer: [0x0720; CELLS],
};

static mut SCREEN1: VGAScreen = VGAScreen {
    pen: 0x0f,
    cursor_pos: 0,
    local_buffer: [0x0720; CELLS],
};

unsafe impl Sync for VGAScreen {}

impl fmt::Write for VGAVirtualScreen {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let active_screen = self.get_current_screen();
        unsafe {
            for byte in s.bytes() {
                match byte {
                    b'\n' => (*active_screen).newline(),
                    byte => (*active_screen).write_byte(byte),
                }
            }
        }
        Ok(())
    }
}

pub struct VGAScreen {
    pub pen: u8,
    pub cursor_pos: usize,
    local_buffer: [u16; CELLS],
}

impl VGAScreen {
    pub fn paint(&mut self) {
        for pos in 0..CELLS {
            let pixel = self.local_buffer[pos];
            self.write_byte_raw(pos, pixel);
        }
        self.write_cursor_pos();
    }

    pub fn change_pen(&mut self, pen: u8) {
        self.pen = pen;
    }

    pub fn write_byte(&mut self, byte: u8) {
        let pixel = (self.pen as u16) << 8 | (byte as u16);
        self.write_byte_raw(self.cursor_pos, pixel);
        self.cursor_pos += 1;
        if self.cursor_pos >= WIDTH * HEIGHT {
            self.scroll();
        }
        self.write_cursor_pos();
    }

    pub fn newline(&mut self) {
        self.cursor_pos += WIDTH - (self.cursor_pos % WIDTH);
        if self.cursor_pos >= WIDTH * HEIGHT {
            self.scroll();
        }
        self.write_cursor_pos();
    }

    fn write_cursor_pos(&self) {
        let row = (self.cursor_pos / WIDTH) as u16;
        let col = (self.cursor_pos % WIDTH) as u16;
        vga_set_cursor_pos(row, col);
    }

    fn scroll(&mut self) {
        for row in 1..HEIGHT {
            for col in 0..WIDTH {
                let from = row * WIDTH + col;
                let to = (row - 1) * WIDTH + col;
                let cell = self.local_buffer[from];
                self.write_byte_raw(to, cell);
            }
        }
        for col in 0..WIDTH {
            let last_row = (HEIGHT - 1) * WIDTH + col;
            self.write_byte_raw(last_row, (self.pen as u16) << 8 | (b' ' as u16));
        }
        self.cursor_pos = (HEIGHT - 1) * WIDTH;
    }

    // VGAバッファへの書き込みアクセスはすべてここで行う
    fn write_byte_raw(&mut self, pos: usize, pixel: u16) {
        unsafe {
            let buffer = 0xb8000 as *mut u8;
            *buffer.add(pos * 2) = (pixel & 0xFF) as u8;
            *buffer.add(pos * 2 + 1) = (pixel >> 8) as u8;
        }
        self.local_buffer[pos] = pixel;
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
