use regex::Regex;

use thiserror::Error;

use crate::{
    instruction::{Comp, Dest, Jump},
    symbol::{SymbolInstruction, SymbolTable},
};

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("{0} is an invalid symbol")]
    InvalidSymbol(String),

    #[error("Unknown comp ({0})")]
    UnknownComp(String),

    #[error("Syntax error: {0}")]
    InvalidSyntax(String),
}
pub type Result<T> = std::result::Result<T, ParseError>;

pub fn parse<S: AsRef<str>>(
    lines: &[S],
    symbols: &mut SymbolTable,
) -> Result<Vec<SymbolInstruction>> {
    let mut line_number = 0;
    lines.iter().filter_map(|line| {
        let line = line.as_ref().trim().split("//").next().unwrap();
        if line.is_empty() {
            return None;
        }
        let c_instruction = Regex::new(r"(?:(?P<dest>M|D|DM|MD|A|AM|MA|AD|DA|AMD|ADM|DAM|DMA|MAD|MDA)\s*=)?\s*(?P<comp>[^;]+)(?:;\s*(?P<jump>JGT|JEQ|JGE|JLT|JNE|JLE|JMP))?\s*").unwrap();
        if line.starts_with("@") {
            line_number += 1;
            let line = line.strip_prefix("@").unwrap();
            line.split_whitespace().next().map_or(
                Some(Err(ParseError::InvalidSyntax(line.to_string()))),
                |value| {
                    if value.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.' || c == '$' || c == ':') {
                        if let Ok(value) = value.parse() {
                            Some(Ok(SymbolInstruction::AImmediate { value }))
                        } else if !value.starts_with(|c: char| c.is_digit(10)) {
                            symbols.insert_variable(value);
                            Some(Ok(SymbolInstruction::ASymbol { symbol: value.to_string() }))
                        } else {
                            Some(Err(ParseError::InvalidSymbol(value.to_string())))
                        }
                    } else {
                            Some(Err(ParseError::InvalidSymbol(value.to_string())))
                    }
                })
        } else if line.starts_with("(") {
            line.strip_prefix("(").unwrap().strip_suffix(")").map_or(
                Some(Err(ParseError::InvalidSyntax(line.to_string()))),
                |label| {
                    let label = label.trim();
                    if label.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.' || c == '$' || c == ':') {
                        if !label.starts_with(|c: char| c.is_digit(10)) {
                            symbols.insert_label(label, line_number);
                            None
                        } else {
                            Some(Err(ParseError::InvalidSymbol(label.to_string())))
                        }
                    } else {
                        Some(Err(ParseError::InvalidSymbol(label.to_string())))
                    }
                })
        } else if let Some(captures) = c_instruction.captures(line) {
            line_number += 1;
            let dest = if let Some(dest) = captures.name("dest") {
                let dest = dest.as_str();
                let m = dest.contains("M");
                let d = dest.contains("D");
                let a = dest.contains("A");
                if m && d && a {
                    Some(Dest::ADM)
                } else if m && d {
                    Some(Dest::DM)
                } else if m && a {
                    Some(Dest::AM)
                } else if d && a {
                    Some(Dest::AD)
                } else if m {
                    Some(Dest::M)
                } else if d {
                    Some(Dest::D)
                } else if a {
                    Some(Dest::A)
                } else {
                    unreachable!()
                }
            } else {
                None
            };
            let comp = captures["comp"].split_whitespace().collect::<String>();
            let comp = match comp.as_str() {
                "0" => Comp::Zero,
                "1" => Comp::One,
                "-1" => Comp::MinusOne,
                "D" => Comp::D,
                "A" => Comp::A,
                "M" => Comp::M,
                "!D" => Comp::NotD,
                "!A" => Comp::NotA,
                "!M" => Comp::NotM,
                "-D" => Comp::MinusD,
                "-A" => Comp::MinusA,
                "-M" => Comp::MinusM,
                "D+1" | "1+D" => Comp::DPlusOne,
                "A+1" | "1+A" => Comp::APlusOne,
                "M+1" | "1+M" => Comp::MPlusOne,
                "D-1" => Comp::DMinusOne,
                "A-1" => Comp::AMinusOne,
                "M-1" => Comp::MMinusOne,
                "D+A" | "A+D" => Comp::DPlusA,
                "D+M" | "M+D" => Comp::DPlusM,
                "D-A" => Comp::DMinusA,
                "A-D" => Comp::AMinusD,
                "D-M" => Comp::DMinusM,
                "M-D" => Comp::MMinusD,
                "D&A" | "A&D" => Comp::DAndA,
                "D&M" | "M&D" => Comp::DAndM,
                "D|A" | "A|D" => Comp::DOrA,
                "D|M" | "M|D" => Comp::DOrM,
                _ => return Some(Err(ParseError::UnknownComp(comp.to_string()))),
            };
            let jump = if let Some(jump) = captures.name("jump") {
                let jump = jump.as_str();
                match jump {
                    "JGT" => Some(Jump::JGT),
                    "JEQ" => Some(Jump::JEQ),
                    "JGE" => Some(Jump::JGE),
                    "JLT" => Some(Jump::JLT),
                    "JNE" => Some(Jump::JNE),
                    "JLE" => Some(Jump::JLE),
                    "JMP" => Some(Jump::JMP),
                    _ => unreachable!(),
                }
            } else {
                None
            };
            Some(Ok(SymbolInstruction::C { dest, comp, jump }))
        } else {
            Some(Err(ParseError::InvalidSyntax(line.to_string())))
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_accepts_non_negative_immediate_after_a_instruction() {
        let lines = ["    @13 // A = 13\n"];
        let mut symbols = SymbolTable::new();
        let result = parse(&lines, &mut symbols);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            [SymbolInstruction::AImmediate { value: 13 }]
        );
    }

    #[test]
    fn parser_denies_negative_immediate_after_a_instruction() {
        let lines = [" @-1\n"];
        let mut symbols = SymbolTable::new();
        let result = parse(&lines, &mut symbols);
        assert!(result.is_err());
    }

    #[test]
    fn parser_accepts_minus_one() {
        let lines = ["   -1 // Just negative"];
        let mut symbols = SymbolTable::new();
        let result = parse(&lines, &mut symbols);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            [SymbolInstruction::C {
                comp: Comp::MinusOne,
                dest: None,
                jump: None
            }]
        );
    }

    #[test]
    fn parser_accepts_c_instruction_without_dest_and_jump() {
        let lines = ["     0"];
        let mut symbols = SymbolTable::new();
        let result = parse(&lines, &mut symbols);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            [SymbolInstruction::C {
                comp: Comp::Zero,
                dest: None,
                jump: None
            }]
        );
    }

    #[test]
    fn parser_accepts_c_instruction_with_dest() {
        let lines = ["  D =     1 // ; D=4 \n"];
        let mut symbols = SymbolTable::new();
        let result = parse(&lines, &mut symbols);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            [SymbolInstruction::C {
                comp: Comp::One,
                dest: Some(Dest::D),
                jump: None
            }]
        );
    }

    #[test]
    fn parser_accepts_c_instruction_with_jump() {
        let lines = ["1; JMP\n"];
        let mut symbols = SymbolTable::new();
        let result = parse(&lines, &mut symbols);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            [SymbolInstruction::C {
                comp: Comp::One,
                dest: None,
                jump: Some(Jump::JMP)
            }]
        );
    }

    #[test]
    fn parser_accepts_c_instruction_with_dest_and_jump() {
        let lines = ["M=1;JMP"];
        let mut symbols = SymbolTable::new();
        let result = parse(&lines, &mut symbols);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            [SymbolInstruction::C {
                comp: Comp::One,
                dest: Some(Dest::M),
                jump: Some(Jump::JMP)
            }]
        );
    }

    #[test]
    fn parser_accepts_a_instruction_with_symbol() {
        let lines = ["@i"];
        let mut symbols = SymbolTable::new();
        let result = parse(&lines, &mut symbols);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            [SymbolInstruction::ASymbol {
                symbol: "i".to_string(),
            }]
        );
    }

    #[test]
    fn parser_accepts_a_label_and_jump_to_there() {
        let lines = ["0", "  (LABEL) ", "@LABEL", "0; JMP"];
        let mut symbols = SymbolTable::new();
        let result = parse(&lines, &mut symbols);
        assert!(result.is_ok(), "{:?}", result);
        assert_eq!(
            result.unwrap(),
            [
                SymbolInstruction::C {
                    comp: Comp::Zero,
                    dest: None,
                    jump: None
                },
                SymbolInstruction::ASymbol {
                    symbol: "LABEL".to_string()
                },
                SymbolInstruction::C {
                    comp: Comp::Zero,
                    dest: None,
                    jump: Some(Jump::JMP)
                }
            ]
        );
    }
}
