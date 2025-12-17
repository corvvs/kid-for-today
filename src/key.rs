use crate::byte_io::{inb};

#[inline(always)]
pub fn kbd_has_scancode() -> bool {
    unsafe { (inb(0x64) & 0x01) != 0 }
}

/// 1バイト読み（スキャンコード Set1 前提）
#[inline(always)]
pub fn kbd_read_scancode() -> u8 {
    unsafe { inb(0x60) }
}

pub enum DirKey {
    Neutral,
    Up,
    Down,
    Left,
    Right,
}

pub struct KbdState {
    shift: bool,
    control: bool,
    alt: bool,
    dir: DirKey,
    key: u8,
}

impl KbdState {
    pub fn new() -> Self {
        KbdState {
            shift: false,
            control: false,
            alt: false,
            dir: DirKey::Neutral,
            key: 0,
        }
    }
    
    pub fn is_alt(&self) -> bool {
        self.alt
    }

    pub fn get_key(&self) -> u8 {
        self.key
    }
}

pub fn scan_key(state: &mut KbdState) -> u8 {
    let scancode = kbd_read_scancode();
    // printk!("scancode: {}\n", scancode);
    let c = match scancode {
        // dir
        0x48 => { state.dir = DirKey::Up; 0 },    // Up 押下
        0x4b => { state.dir = DirKey::Left; 0 },  // Left 押下
        0x4d => { state.dir = DirKey::Right; 0 }, // Right 押下
        0x50 => { state.dir = DirKey::Down; 0 },  // Down 押下
        0xc8 | 0xcb | 0xcd | 0xd0 => { state.dir = DirKey::Neutral; 0 }, // 解放
        
        // shift
        0x2a => { state.shift = true; 0 },   // Left Shift 押下
        0x36 => { state.shift = true; 0 },   // Right Shift 押下
        0xaa => { state.shift = false; 0 },  // Left Shift 解放
        0xb6 => { state.shift = false; 0 },  // Right Shift 解放

        // control
        0x1d => { state.control = true; 0 },    // Left Control 押下
        0x9d => { state.control = false; 0 },   // Left Control 解放

        // alt
        0x38 => { state.alt = true; 0 },    // Left Alt 押下
        0xb8 => { state.alt = false; 0 },   // Left Alt 解放

        // キー
        0x02 => if state.shift { b'!' } else { b'1' },
        0x03 => if state.shift { b'"' } else { b'2' },
        0x04 => if state.shift { b'#' } else { b'3' },
        0x05 => if state.shift { b'$' } else { b'4' },
        0x06 => if state.shift { b'%' } else { b'5' },
        0x07 => if state.shift { b'&' } else { b'6' },
        0x08 => if state.shift { b'\'' } else { b'7' },
        0x09 => if state.shift { b'(' } else { b'8' },
        0x0a => if state.shift { b')' } else { b'9' },
        0x0b => b'0',
        0x0c => if state.shift { b'=' } else { b'-' },
        0x0d => if state.shift { b'~' } else { b'^' }, // 日本語キーボード
        0x7d => if state.shift { b'|' } else { b'\\' }, // 日本語キーボード

        0x10 => if state.shift { b'Q' } else { b'q' },
        0x11 => if state.shift { b'W' } else { b'w' },
        0x12 => if state.shift { b'E' } else { b'e' },
        0x13 => if state.shift { b'R' } else { b'r' },
        0x14 => if state.shift { b'T' } else { b't' },
        0x15 => if state.shift { b'Y' } else { b'y' },
        0x16 => if state.shift { b'U' } else { b'u' },
        0x17 => if state.shift { b'I' } else { b'i' },
        0x18 => if state.shift { b'O' } else { b'o' },
        0x19 => if state.shift { b'P' } else { b'p' },
        0x1a => if state.shift { b'`' } else { b'@' }, // 日本語キーボード
        0x1b => if state.shift { b'{' } else { b'[' }, // 日本語キーボード

        0x1e => if state.shift { b'A' } else { b'a' },
        0x1f => if state.shift { b'S' } else { b's' },
        0x20 => if state.shift { b'D' } else { b'd' },
        0x21 => if state.shift { b'F' } else { b'f' },
        0x22 => if state.shift { b'G' } else { b'g' },
        0x23 => if state.shift { b'H' } else { b'h' },
        0x24 => if state.shift { b'J' } else { b'j' },
        0x25 => if state.shift { b'K' } else { b'k' },
        0x26 => if state.shift { b'L' } else { b'l' },
        0x27 => if state.shift { b'+' } else { b';' },
        0x28 => if state.shift { b'*' } else { b':' },

        0x2c => if state.shift { b'Z' } else { b'z' },
        0x2d => if state.shift { b'X' } else { b'x' },
        0x2e => if state.shift { b'C' } else { b'c' },
        0x2f => if state.shift { b'V' } else { b'v' },
        0x30 => if state.shift { b'B' } else { b'b' },
        0x31 => if state.shift { b'N' } else { b'n' },
        0x32 => if state.shift { b'M' } else { b'm' },
        0x33 => if state.shift { b'<' } else { b',' },
        0x34 => if state.shift { b'>' } else { b'.' },
        0x35 => if state.shift { b'?' } else { b'/' },
        0x39 => b' ',
        0x73 => b'_', // 日本語キーボード

        0x1c => b'\n', // Enter
        0x0f => b'\t', // Tab
        0x0e => 0x08,  // Backspace
        0x01 => 0x1b,  // Escape

        _ => {
            // printk!("unknown scancode: {:x}\n", scancode);
            0
        },
    };
    if c != 0 {
        state.key = c;
    }
    c
}
