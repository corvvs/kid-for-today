use crate::byte_io::{inb, outb};

const CRTC_INDEX: u16 = 0x3D4;
const CRTC_DATA: u16 = 0x3D5;

const ROWS: u16 = 25;
const COLS: u16 = 80;

/// row/col → pos（80x25前提）
pub fn vga_set_cursor_pos(row: u16, col: u16) {
    let pos = row * COLS + col;

    unsafe {
        outb(CRTC_INDEX, 0x0F);
        outb(CRTC_DATA, (pos & 0xFF) as u8);
        outb(CRTC_INDEX, 0x0E);
        outb(CRTC_DATA, (pos >> 8) as u8);
    }
}

pub fn vga_get_cursor_pos() -> (u16, u16) {
    unsafe {
        outb(CRTC_INDEX, 0x0F);
        let lo = inb(CRTC_DATA) as u16;
        outb(CRTC_INDEX, 0x0E);
        let hi = inb(CRTC_DATA) as u16;
        let pos = (hi << 8) | lo;
        (pos / COLS, pos % COLS)
    }
}
