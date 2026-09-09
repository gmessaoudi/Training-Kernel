#![no_std]
#![no_main]

extern crate alloc;

use core::arch::global_asm;

pub mod arch;
pub mod hal;
pub mod drivers;
pub mod memory;
pub mod time;
pub mod utils;
pub mod boards;

use alloc::string::String;

global_asm!(include_str!("arch/entry.S"));

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    arch::interrupts::init();
    memory::linked_list::init_heap();

    time::timer::init();

    loop {
        print!("> ");
        let line : String = input!();
        let cmd = utils::parser::parse(&line);
        utils::parser::run(cmd);
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("PANIC SYSTEME !");
    println!("{}", _info);
    loop {}
}