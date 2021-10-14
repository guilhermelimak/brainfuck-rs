use std::vec::IntoIter;

use crate::lexer::{Token, TokenType};

#[derive(Clone, Copy, Debug)]
pub enum Value {
    Inc,
    Dec,
}

#[derive(Clone, Copy, Debug)]
pub enum IoMode {
    In,
    Out,
}

#[derive(Clone, Copy, Debug)]
pub enum StatementKind {
    Loop,
    Ptr(Value),
    Math(Value),
    Io(IoMode),
}

#[derive(Clone, Debug)]
pub struct Statement {
    pub kind: StatementKind,
    pub children: Vec<Statement>,
}

pub struct Parser {}

fn get_value(token_type: TokenType) -> Value {
    match token_type {
        TokenType::PtrLeft | TokenType::Dec => Value::Dec,
        TokenType::PtrRight | TokenType::Inc => Value::Inc,
        _ => panic!("The token type {:?} doesn't have a value", token_type),
    }
}

impl Parser {
    pub fn parse(tokens: &mut IntoIter<Token>) -> Vec<Statement> {
        let mut ast: Vec<Statement> = vec![];

        while let Some(token) = tokens.next() {
            match token.token_type {
                TokenType::LoopStart => {
                    ast.push(Statement {
                        kind: StatementKind::Loop,
                        children: Parser::parse(tokens),
                    });
                }
                TokenType::PtrLeft | TokenType::PtrRight => {
                    ast.push(Statement {
                        kind: StatementKind::Ptr(get_value(token.token_type)),
                        children: vec![],
                    });
                }
                TokenType::Inc | TokenType::Dec => {
                    ast.push(Statement {
                        kind: StatementKind::Math(get_value(token.token_type)),
                        children: vec![],
                    });
                }
                TokenType::Read | TokenType::Write => {
                    let value = match token.token_type {
                        TokenType::Read => IoMode::In,
                        TokenType::Write => IoMode::Out,
                        _ => panic!("No value for token type"),
                    };
                    ast.push(Statement {
                        kind: StatementKind::Io(value),
                        children: vec![],
                    });
                }
                TokenType::Eof => break,
                TokenType::Illegal | _ => {}
            };
        }

        ast
    }
}
#[cfg(test)]
mod parser_tests {}
