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
macro_rules! printkd {
    // TODO: 割り込みを実装するとデッドロックする可能性があるので注意
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut w = $crate::vga::writer().lock();
        let mut ww = w.get_error_screen().lock();
        ww.change_pen(0x7);
        write!(ww, $($arg)*); // discard Result
        ww.change_pen(0xf);
    });
}

#[macro_export]
macro_rules! printki {
    // TODO: 割り込みを実装するとデッドロックする可能性があるので注意
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut w = $crate::vga::writer().lock();
        let mut ww = w.get_error_screen().lock();
        write!(ww, $($arg)*); // discard Result
    });
}

#[macro_export]
macro_rules! printkw {
    // TODO: 割り込みを実装するとデッドロックする可能性があるので注意
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut w = $crate::vga::writer().lock();
        let mut ww = w.get_error_screen().lock();
        ww.change_pen(0x6);
        write!(ww, $($arg)*); // discard Result
        ww.change_pen(0xf);
    });
}

#[macro_export]
macro_rules! printke {
    // TODO: 割り込みを実装するとデッドロックする可能性があるので注意
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut w = $crate::vga::writer().lock();
        let mut ww = w.get_error_screen().lock();
        ww.change_pen(0x4);
        write!(ww, $($arg)*); // discard Result
        ww.change_pen(0xf);
    });
}

#[macro_export]
macro_rules! printkg {
    // TODO: 割り込みを実装するとデッドロックする可能性があるので注意
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut w = $crate::vga::writer().lock();
        let mut ww = w.get_error_screen().lock();
        ww.change_pen(0xa);
        write!(ww, $($arg)*); // discard Result
        ww.change_pen(0xf);
    });
}
