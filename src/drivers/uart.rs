use crate::hal::mmio::{read_byte, write_byte};
use crate::boards::UART_BASE;
use core::fmt::{Write, Result};
use alloc::string::String;

pub struct UART;

impl UART{
    pub fn new() -> UART {return UART;}

    pub fn put_char(&self, character: u8) {
        let mut status : u8 = 0;
        while status == 0 {
            status = read_byte(UART_BASE + 5) & 0x20;
        }
        write_byte(UART_BASE, character);
    }

    pub fn put_str(&self, string: &str) {
        for char in string.bytes() {
            self.put_char(char);
        }
    }

    pub fn put_str_ln(&self, string: &str) {
        self.put_str(string);
        self.put_char(b'\n');
    }

    pub fn try_get_char(&self) -> Option<u8> {
        let status : u8 = read_byte(UART_BASE+5);
        if status & 0x01 == 0x01 {
            return Some(read_byte(UART_BASE));
        }
        else {
            return None;
        }
    }

    pub fn read_line(&self) -> String {
        let mut read_value = String::new();

        loop {
            if let Some(c) = self.try_get_char() {
                match c {
                    b'\r' | b'\n' => {
                        crate::println!();
                        break;
                    }
                    127 | 8 => {
                        if !read_value.is_empty() {
                            read_value.pop();
                            crate::print!("\x08 \x08");
                        }
                    }
                    _ => {
                        read_value.push(c as char);
                        crate::print!("{}", c as char);
                    }
                }
            }
        }

        read_value
    }
}

impl Write for UART {
    fn write_str(&mut self, s: &str) -> Result {
        self.put_str(s);
        return Ok(());
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        let mut uart = $crate::drivers::uart::UART::new();
        let _ = core::fmt::Write::write_fmt(&mut uart, core::format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => (
        $crate::println!("\x1b[31m[ERROR] {}\x1b[0m", format_args!($($arg)*))
    );
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => (
        $crate::println!("\x1b[33m[WARNING] {}\x1b[0m", format_args!($($arg)*))
    );
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => (
        $crate::println!("\x1b[36m[INFO] {}\x1b[0m", format_args!($($arg)*))
    );
}

#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => (
        $crate::println!("\x1b[32m[SUCCESS] {}\x1b[0m", format_args!($($arg)*))
    );
}

#[macro_export]
macro_rules! input {
    () => ({
        let uart = $crate::drivers::uart::UART::new();
        uart.read_line()
    });
}