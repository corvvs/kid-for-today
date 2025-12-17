#[macro_export]
macro_rules! printk {
    // TODO: 割り込みを実装するとデッドロックする可能性があるので注意
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut w = $crate::vga::writer().lock();
        write!(w, $($arg)*); // discard Result
    });
}
