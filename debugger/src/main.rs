use std::io::Write;

use clap::{app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg};

use computer::{
    keyboard::DummyKeyboard as Keyboard, rom::Rom, screen::DummyScreen as Screen, Computer,
};

fn print_help() {
    println!(
        r#"commands:
    help: Show this help
    show: Show the status
    next: Next step
    load: Load the ROM file
    exit: Exit"#
    );
}

fn main() {
    let args = app_from_crate!()
        .arg(
            Arg::with_name("rom")
                .long("rom")
                .short("r")
                .takes_value(true)
                .help("Path to a ROM file"),
        )
        .get_matches();

    let rom = args
        .value_of("rom")
        .and_then(|path| match Rom::from_binary(path) {
            Ok(rom) => Some(rom),
            Err(e) => {
                eprintln!("Couldn't read the ROM file (error: {})", e);
                None
            }
        })
        .unwrap_or(Rom::new());

    let mut computer = Computer::<Screen, Keyboard>::new();
    computer.set_rom(rom);
    computer.tick(true);

    let mut line = String::new();
    loop {
        line.clear();
        print!(" PC = {} > ", computer.pc().as_raw());
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut line).unwrap();
        match line.trim() {
            "help" => print_help(),
            "show" => {
                println!(
                    "A: {}, D: {}, M: {}",
                    computer.a().as_raw(),
                    computer.d().as_raw(),
                    computer.m().as_raw(),
                );
            }
            "next" => {
                computer.tick(false);
            }
            "load" => {
                let mut path = String::new();
                print!("Path to a ROM file > ");
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut path).unwrap();
                match Rom::from_binary(path.trim()) {
                    Ok(rom) => {
                        computer.set_rom(rom);
                        computer.tick(true);
                    }
                    Err(e) => {
                        eprintln!("Couldn't read the ROM file (error: {})", e);
                    }
                }
            }
            "exit" => break,
            _ => {
                if !line.trim().is_empty() {
                    eprintln!("Unknown command: \"{}\"", line.trim());
                }
            }
        }
    }
}
