use clap::{app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg};

use computer::{
    keyboard::DummyKeyboard as Keyboard, rom::Rom, screen::DummyScreen as Screen, Computer,
};

fn main() {
    let args = app_from_crate!()
        .arg(
            Arg::with_name("rom")
                .long("rom")
                .short("r")
                .takes_value(true)
                .help("Path to ROM binary file"),
        )
        .get_matches();

    let rom = args
        .value_of("rom")
        .and_then(|path| match Rom::from_binary(path) {
            Ok(rom) => Some(rom),
            Err(e) => {
                eprintln!("Couldn't open the ROM file (error: {})", e);
                None
            }
        })
        .unwrap_or(Rom::new());

    let mut computer = Computer::<Screen, Keyboard>::new();
    computer.set_rom(rom);
}
