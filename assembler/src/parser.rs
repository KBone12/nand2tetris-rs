use std::collections::VecDeque;

use thiserror::Error;

use crate::{instruction::Instruction, token::Token};

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("A-instruction's value should not be negative (value: {0})")]
    NegativeAInstruction(i16),

    #[error("A-instruction's value is not found ({0:?})")]
    InvalidAInstruction(Token),

    #[error("'=' is not found after 'dest'")]
    NoEqualAfterDest,

    #[error("'jump' is not found after ';'")]
    NoJumpAfterSemicolon,

    #[error("C-instruction's comp is not found ({0:?})")]
    InvalidCInstruction(Token),

    #[error("Syntax Error: {0:?}")]
    InvalidSyntax(Token),
}
pub type Result<T> = std::result::Result<T, ParseError>;

pub struct Parser;

impl Parser {
    pub fn parse(tokens: &[Token]) -> Result<Vec<Instruction>> {
        let mut tokens: VecDeque<_> = tokens.iter().collect();
        let mut instructions = Vec::new();
        while let Some(token) = tokens.pop_front() {
            let instruction = match token {
                Token::AInst => {
                    if let Some(token) = tokens.pop_front() {
                        match token {
                            Token::Immediate(value) => {
                                if (value >> 15) & 1 == 1 {
                                    return Err(ParseError::NegativeAInstruction(*value as i16));
                                }
                                Instruction::A { value: *value }
                            }
                            _ => {
                                return Err(ParseError::InvalidAInstruction(token.clone()));
                            }
                        }
                    } else {
                        return Err(ParseError::InvalidAInstruction(token.clone()));
                    }
                }
                Token::Comp(comp) => {
                    if let Some(token) = tokens.pop_front() {
                        match token {
                            Token::Semicolon => {
                                if let Some(token) = tokens.pop_front() {
                                    match token {
                                        Token::Jump(jump) => Instruction::C {
                                            comp: comp.clone(),
                                            dest: None,
                                            jump: Some(jump.clone()),
                                        },
                                        _ => {
                                            return Err(ParseError::NoJumpAfterSemicolon);
                                        }
                                    }
                                } else {
                                    return Err(ParseError::NoJumpAfterSemicolon);
                                }
                            }
                            _ => {
                                tokens.push_front(token);
                                Instruction::C {
                                    comp: comp.clone(),
                                    dest: None,
                                    jump: None,
                                }
                            }
                        }
                    } else {
                        Instruction::C {
                            comp: comp.clone(),
                            dest: None,
                            jump: None,
                        }
                    }
                }
                Token::Dest(dest) => {
                    let token = tokens.pop_front();
                    if matches!(token, Some(Token::Equal)) {
                        if let Some(token) = tokens.pop_front() {
                            match token {
                                Token::Comp(comp) => {
                                    if let Some(token) = tokens.pop_front() {
                                        match token {
                                            Token::Semicolon => {
                                                if let Some(token) = tokens.pop_front() {
                                                    match token {
                                                        Token::Jump(jump) => Instruction::C {
                                                            comp: comp.clone(),
                                                            dest: Some(dest.clone()),
                                                            jump: Some(jump.clone()),
                                                        },
                                                        _ => {
                                                            return Err(
                                                                ParseError::NoJumpAfterSemicolon,
                                                            );
                                                        }
                                                    }
                                                } else {
                                                    return Err(ParseError::NoJumpAfterSemicolon);
                                                }
                                            }
                                            _ => {
                                                tokens.push_front(token);
                                                Instruction::C {
                                                    comp: comp.clone(),
                                                    dest: Some(dest.clone()),
                                                    jump: None,
                                                }
                                            }
                                        }
                                    } else {
                                        Instruction::C {
                                            comp: comp.clone(),
                                            dest: Some(dest.clone()),
                                            jump: None,
                                        }
                                    }
                                }
                                _ => {
                                    return Err(ParseError::InvalidCInstruction(token.clone()));
                                }
                            }
                        } else {
                            return Err(ParseError::InvalidCInstruction(token.unwrap().clone()));
                        }
                    } else {
                        return Err(ParseError::NoEqualAfterDest);
                    }
                }
                _ => {
                    return Err(ParseError::InvalidSyntax(token.clone()));
                }
            };
            instructions.push(instruction);
        }
        Ok(instructions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{Comp, Dest, Jump};

    #[test]
    fn parser_accepts_non_negative_immediate_after_a_instruction() {
        let tokens = [Token::AInst, Token::Immediate(13)];
        let result = Parser::parse(&tokens);
        assert!(result.is_ok());
    }

    #[test]
    fn parser_denies_negative_immediate_after_a_instruction() {
        let tokens = [Token::AInst, Token::Immediate(-1i16 as u16)];
        let result = Parser::parse(&tokens);
        assert!(result.is_err());
    }

    #[test]
    fn parser_accepts_c_instruction_without_dest_and_jump() {
        let tokens = [Token::Comp(Comp::Zero)];
        let result = Parser::parse(&tokens);
        assert!(result.is_ok());
    }

    #[test]
    fn parser_accepts_c_instruction_with_dest() {
        let tokens = [Token::Dest(Dest::D), Token::Equal, Token::Comp(Comp::One)];
        let result = Parser::parse(&tokens);
        assert!(result.is_ok());
    }

    #[test]
    fn parser_accepts_c_instruction_with_jump() {
        let tokens = [
            Token::Comp(Comp::One),
            Token::Semicolon,
            Token::Jump(Jump::JMP),
        ];
        let result = Parser::parse(&tokens);
        assert!(result.is_ok());
    }

    #[test]
    fn parser_accepts_c_instruction_with_dest_and_jump() {
        let tokens = [
            Token::Dest(Dest::M),
            Token::Equal,
            Token::Comp(Comp::One),
            Token::Semicolon,
            Token::Jump(Jump::JMP),
        ];
        let result = Parser::parse(&tokens);
        assert!(result.is_ok());
    }
}
