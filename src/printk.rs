use crate::vga::VGAScreen;
use spin::{Mutex, Once};

pub static WRITER: Once<Mutex<VGAScreen>> = Once::new();

pub fn writer() -> &'static Mutex<VGAScreen> {
    WRITER.call_once(|| Mutex::new(VGAScreen::new()))
}

#[macro_export]
macro_rules! printk {
    // TODO: 割り込みを実装するとデッドロックする可能性があるので注意
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut w = $crate::printk::writer().lock();
        write!(w, $($arg)*); // discard Result
    });
}
