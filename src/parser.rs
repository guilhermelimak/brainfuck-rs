use std::vec::IntoIter;

use crate::lexer::{Token, TokenType};

#[derive(Clone, Copy, Debug)]
enum Value {
    Inc,
    Dec,
    In,
    Out,
}

#[derive(Clone, Copy, Debug)]
enum StatementKind {
    Loop,
    Ptr,
    Io,
    Math,
}

#[derive(Clone, Debug)]
pub struct Statement {
    kind: StatementKind,
    children: Vec<Statement>,
    value: Option<Value>,
}

pub struct Parser {}

impl Parser {
    pub fn parse(tokens: &mut IntoIter<Token>) -> Vec<Statement> {
        let mut ast: Vec<Statement> = vec![];

        while let Some(token) = tokens.next() {
            match token.token_type {
                TokenType::LoopStart => {
                    ast.push(Statement {
                        kind: StatementKind::Loop,
                        value: None,
                        children: Parser::parse(tokens),
                    });
                }
                TokenType::LoopEnd => {}
                TokenType::PtrLeft | TokenType::PtrRight => {
                    let statement = Statement {
                        value: Some(match token.token_type {
                            TokenType::PtrLeft => Value::Dec,
                            TokenType::PtrRight => Value::Inc,
                            _ => panic!("Invalid value"),
                        }),
                        kind: StatementKind::Ptr,
                        children: vec![],
                    };

                    ast.push(statement);
                }
                TokenType::Inc | TokenType::Dec => {
                    let statement = Statement {
                        value: Some(match token.token_type {
                            TokenType::Dec => Value::Dec,
                            TokenType::Inc => Value::Inc,
                            _ => panic!("Invalid value"),
                        }),
                        kind: StatementKind::Math,
                        children: vec![],
                    };

                    ast.push(statement);
                }
                TokenType::Read | TokenType::Write => {
                    let statement = Statement {
                        value: Some(match token.token_type {
                            TokenType::Read => Value::In,
                            TokenType::Inc => Value::Out,
                            _ => panic!("Invalid value"),
                        }),
                        kind: StatementKind::Io,
                        children: vec![],
                    };

                    ast.push(statement);
                }
                TokenType::Illegal => {}
                TokenType::Eof => break,
            };
        }

        println!("{:#?}", ast);
        ast
    }
}
#[cfg(test)]
mod parser_tests {}
