use core::ptr::{read_volatile, write_volatile};
use core::arch::asm;
use core::sync::atomic::{AtomicU32};
use crate::boards::{MTIME, MTIMECMP, CLOCK_FREQ};

pub static TICKS: AtomicU32 = AtomicU32::new(0);

pub fn init(){
    set_next_timeout(1000);

    unsafe {
        asm!("csrs mie, {}", in(reg) 1 << 7);
        asm!("csrs mstatus, {}", in(reg) 1 << 3);
    }
}

pub fn set_next_timeout(milliseconds : u64){
    let mut mtime_value : u64;

    unsafe {mtime_value = read_volatile(MTIME as * const u64);}
    mtime_value = mtime_value + (milliseconds * CLOCK_FREQ) / 1000;

    unsafe {write_volatile(MTIMECMP as * mut u64, mtime_value);}
}