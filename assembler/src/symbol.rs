use std::collections::HashMap;

use thiserror::Error;

use crate::instruction::{Comp, Dest, Instruction, Jump};

#[derive(Debug, PartialEq, Eq)]
pub enum SymbolInstruction {
    AImmediate {
        value: u16,
    },
    ASymbol {
        symbol: String,
    },
    C {
        comp: Comp,
        dest: Option<Dest>,
        jump: Option<Jump>,
    },
}

#[derive(Debug, Error)]
pub enum SymbolError {}
pub type Result<T> = std::result::Result<T, SymbolError>;

pub struct SymbolTable {
    table: HashMap<String, u16>,
    next_address: u16,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut table = HashMap::new();
        table.insert("SP".to_string(), 0);
        table.insert("LCL".to_string(), 1);
        table.insert("ARG".to_string(), 2);
        table.insert("THIS".to_string(), 3);
        table.insert("THAT".to_string(), 4);
        table.insert("R0".to_string(), 0);
        table.insert("R1".to_string(), 1);
        table.insert("R2".to_string(), 2);
        table.insert("R3".to_string(), 3);
        table.insert("R4".to_string(), 4);
        table.insert("R5".to_string(), 5);
        table.insert("R6".to_string(), 6);
        table.insert("R7".to_string(), 7);
        table.insert("R8".to_string(), 8);
        table.insert("R9".to_string(), 9);
        table.insert("R10".to_string(), 10);
        table.insert("R11".to_string(), 11);
        table.insert("R12".to_string(), 12);
        table.insert("R13".to_string(), 13);
        table.insert("R14".to_string(), 14);
        table.insert("R15".to_string(), 15);
        table.insert("SCREEN".to_string(), 0x4000);
        table.insert("KBD".to_string(), 0x6000);
        Self {
            table,
            next_address: 0x0010,
        }
    }

    pub fn insert_variable(&mut self, name: &str) {
        if !self.table.contains_key(name) {
            self.table.insert(name.to_string(), self.next_address);
            self.next_address += 1;
        }
    }

    pub fn insert_label(&mut self, name: &str, value: u16) {
        *self.table.entry(name.to_string()).or_insert(value) = value;
    }

    pub fn resolve_symbols(&self, instructions: &[SymbolInstruction]) -> Result<Vec<Instruction>> {
        instructions
            .iter()
            .map(|instruction| match instruction {
                SymbolInstruction::AImmediate { value } => Ok(Instruction::A { value: *value }),
                SymbolInstruction::ASymbol { symbol } => Ok(Instruction::A {
                    value: self.table[symbol],
                }),
                SymbolInstruction::C { comp, dest, jump } => Ok(Instruction::C {
                    comp: comp.clone(),
                    dest: dest.clone(),
                    jump: jump.clone(),
                }),
            })
            .collect()
    }
}
