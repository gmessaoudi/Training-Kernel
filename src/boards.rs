/// Adresse de base de l'UART (Composant NS16550A) sur la machine QEMU virt
pub const UART_BASE: usize = 0x1000_0000;

pub const MTIMECMP : usize = 0x0200_4000;

pub const MTIME : usize = 0x0200_BFF8;

pub const CLOCK_FREQ : u64 = 10_000_000;

///Taille de la Heap (1Mo)
pub const HEAP_SIZE : usize = 0x100000;