use core::arch::asm;

// On indique à Rust qu'une fonction nommée _trap_vector existe quelque part
unsafe extern "C" {
    fn _trap_vector();
}

pub fn init() {
    unsafe {
        asm!("csrw mtvec, {}", in(reg) _trap_vector as *const() as usize);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_trap_handler() {
    let mut cause: usize;
    unsafe {
        asm!("csrr {}, mcause", out(reg) cause);
    }

    let is_interrupt = (cause >> 31) == 1;

    let exception_code = cause & 0x7FFF_FFFF;

    if is_interrupt {
        if exception_code == 7 {
            crate::time::timer::TICKS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
            crate::time::timer::set_next_timeout(1000);
        }
        else {
            crate::info!("Interruption materielle ! Code : {}", exception_code);
        }
    } else {
        crate::error!("EXCEPTION FATALE DETECTEE !");

        let mut epc: usize;
        let mut tval: usize;
        unsafe {
            asm!("csrr {}, mepc", out(reg) epc);
            asm!("csrr {}, mtval", out(reg) tval);
        }

        crate::error!("Instruction fautive (mepc) : 0x{:X}", epc);
        crate::error!("Valeur/Adresse associee (mtval) : 0x{:X}", tval);

        match exception_code {
            0  => crate::error!("-> Code 0 : Adresse d'instruction non alignee"),
            1  => crate::error!("-> Code 1 : Defaut d'acces a l'instruction"),
            2  => crate::error!("-> Code 2 : Instruction illegale"),
            3  => crate::error!("-> Code 3 : Point d'arret (Breakpoint)"),
            4  => crate::error!("-> Code 4 : Adresse de lecture non alignee"),
            5  => crate::error!("-> Code 5 : Defaut d'acces en lecture (Load access fault)"),
            6  => crate::error!("-> Code 6 : Adresse d'ecriture non alignee"),
            7  => crate::error!("-> Code 7 : Defaut d'acces en ecriture (Store access fault)"),
            8  => crate::error!("-> Code 8 : Appel systeme depuis le mode U (User)"),
            9  => crate::error!("-> Code 9 : Appel systeme depuis le mode S (Supervisor)"),
            11 => crate::error!("-> Code 11 : Appel systeme depuis le mode M (Machine)"),
            12 => crate::error!("-> Code 12 : Defaut de page de l'instruction"),
            13 => crate::error!("-> Code 13 : Defaut de page en lecture"),
            15 => crate::error!("-> Code 15 : Defaut de page en ecriture"),
            _  => crate::error!("-> Code inconnu : {}", exception_code),
        }

        loop {}
    }
}