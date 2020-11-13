use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
};

use clap::{app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg};

mod instruction;
mod parser;
use parser::parse;
mod symbol;
use symbol::SymbolTable;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = app_from_crate!()
        .arg(
            Arg::with_name("file")
                .help("The assembly file")
                .required(true),
        )
        .get_matches();
    let file_name = Path::new(args.value_of("file").unwrap());
    let reader = BufReader::new(File::open(file_name)?);
    let lines = reader.lines().collect::<Result<Vec<_>, _>>()?;
    let mut symbols = SymbolTable::new();
    let symbol_instructions = parse(&lines, &mut symbols)?;
    let instructions = symbols.resolve_symbols(&symbol_instructions);
    let binary = instructions
        .iter()
        .map(|instruction| {
            instruction
                .as_binary()
                .iter()
                .map(|b| if *b { "1" } else { "0" })
                .collect::<String>()
                + "\n"
        })
        .collect::<String>();
    let file_name = file_name.with_extension("hack");
    let mut file = BufWriter::new(File::create(file_name)?);
    file.write_all(binary.as_bytes())?;

    Ok(())
}
