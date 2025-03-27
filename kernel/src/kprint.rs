use core::arch::asm;
use core::fmt;

static COM1_PORT: i16 = 0x3f8;

struct SerialWriter;

// we want:  out    dx,al
pub fn outb(port: i16, val: i8) {
    unsafe {
        asm!(
            "mov dx, {0:x}",
            "mov al, {1}",
            "out dx, al",
            in(reg_abcd) port,
            in(reg_byte) val,
        );
    }
}

pub unsafe fn inb(port: i16) -> i8 {
    let mut ret: i8;
    asm!(
        "mov dx, {0:x}",
        "in al, dx",
        in(reg_abcd) port,
        out("al") ret
    );

    return ret;
}

// Wait a very small amount of time (1 to 4 microseconds, generally). Useful
// for implementing a small delay for PIC remapping on old hardware or
// generally as a simple but imprecise wait.
// You can do an IO operation on any unused port: the Linux kernel by default
// uses port 0x80, which is often used during POST to log information on the
// motherboard's hex display but almost always unused after boot.
pub fn io_wait() {
    outb(0x80, 0);
}

impl fmt::Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            outb(COM1_PORT, c as i8);
        }
        Ok(())
    }
}

pub fn kprint_internal(args: fmt::Arguments) {
    let mut writer = SerialWriter;

    fmt::write(&mut writer, args).unwrap();
}

#[macro_export]
macro_rules! kprintln {
    ($($arg:tt)*) => {{
        use crate::kprint::kprint_internal;
        kprint_internal(format_args_nl!($($arg)*));
    }};
}
