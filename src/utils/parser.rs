use crate::{print,println,info,warn,error};

pub struct Command<'a> {
    pub name: &'a str,
    pub args: [&'a str; 4],
    pub argc: usize,
}

pub fn parse(input: &str) -> Command<'_> {
    let mut parts = input.split_whitespace();

    let name = parts.next().unwrap_or("");
    let mut args = [""; 4];
    let mut argc = 0;

    for (i, p) in parts.take(4).enumerate() {
        args[i] = p;
        argc += 1;
    }

    Command { name, args, argc }
}

pub fn run(cmd: Command) {
    match cmd.name {
        "help" => help(),
        "echo" => echo(&cmd),
        "uptime" => uptime(),
        "test" => test(),
        _ => println!("Unknown command"),
    }
}

fn help() {
    println!("help echo uptime test");
}

fn echo(cmd: &Command) {
    for i in 0..cmd.argc {
        print!("{} ", cmd.args[i]);
    }
    println!();
}

fn uptime() {
    let ticks = crate::time::timer::TICKS.load(core::sync::atomic::Ordering::Relaxed);
    println!("Uptime: {} secondes", ticks);
}

fn test() {
    println!("[OK] Test affichage");
    info!("Test Info");
    warn!("Test Warning");
    error!("Test Error");
}

