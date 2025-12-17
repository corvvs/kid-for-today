#[macro_export]
macro_rules! printk {
    // TODO: 割り込みを実装するとデッドロックする可能性があるので注意
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut w = $crate::vga::writer().lock();
        if !w.is_active_error_screen() {
            // エラースクリーンでなければ出力
            write!(w, $($arg)*); // discard Result
        }
    });
}

#[macro_export]
macro_rules! printke {
    // TODO: 割り込みを実装するとデッドロックする可能性があるので注意
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut w = $crate::vga::writer().lock();
        write!(w.get_error_screen().lock(), $($arg)*); // discard Result
    });
}
