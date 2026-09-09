use core::ptr::{read_volatile, write_volatile};

pub fn read_byte(address : usize) -> u8 {
    let byte: u8;
    unsafe {
        byte = read_volatile(address as *const u8);
    }
    return byte;
}
pub fn write_byte(address : usize, value: u8) {
    unsafe {
        write_volatile(address as *mut u8, value);
    }
}