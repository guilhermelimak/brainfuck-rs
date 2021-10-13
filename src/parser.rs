use std::vec::IntoIter;

use crate::lexer::{Token, TokenType};

#[derive(Clone, Copy, Debug)]
enum Value {
    Inc,
    Dec,
}

#[derive(Clone, Copy, Debug)]
enum Kind {
    Loop,
    Ptr,
    Io,
    Math,
}

#[derive(Clone, Debug)]
pub struct Statement {
    token: Token,
    start: usize,
    kind: Kind,
    end: usize,
    children: Vec<Statement>,
    value: Option<Value>,
}

pub struct Parser {
    pub tokens: IntoIter<Token>,
    pub current_token: Option<Token>,
    pub peek_token: Option<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens.into_iter(),
            current_token: None,
            peek_token: None,
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        self.current_token = self.peek_token;
        self.peek_token = self.tokens.next();
        self.peek_token
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut depth = 0;
        let mut ast: Vec<Statement> = vec![];
        let mut current_statement: Option<Statement> = None;

        while let Some(token) = self.next_token() {
            println!("Some(token){:#?}", token);
            match token.token_type {
                TokenType::LoopStart => {
                    depth += 1;
                    current_statement = Some(Statement {
                        token,
                        kind: Kind::Loop,
                        start: token.position,
                        value: None,
                        end: 0,
                        children: vec![],
                    })
                }
                TokenType::LoopEnd => match current_statement.clone() {
                    Some(mut st) => {
                        depth -= 1;

                        if depth == 0 {
                            st.end = token.position;
                            ast.push(st);
                        };
                    }
                    None => panic!("Error, finishing unstarted loop at {}", token.position),
                },
                TokenType::PtrLeft | TokenType::PtrRight => {
                    let statement = Statement {
                        value: Some(match token.token_type {
                            TokenType::PtrLeft => Value::Dec,
                            TokenType::PtrRight => Value::Inc,
                            _ => panic!("Invalid value"),
                        }),
                        kind: Kind::Ptr,
                        children: vec![],
                        token,
                        start: token.position,
                        end: token.position,
                    };

                    match current_statement.clone() {
                        Some(mut st) => {
                            st.children.push(statement);
                            current_statement = Some(st);
                        }
                        None => ast.push(statement),
                    }
                }
                TokenType::Inc | TokenType::Dec => {
                    let statement = Statement {
                        value: Some(match token.token_type {
                            TokenType::Dec => Value::Dec,
                            TokenType::Inc => Value::Inc,
                            _ => panic!("Invalid value"),
                        }),
                        kind: Kind::Math,
                        children: vec![],
                        token,
                        start: token.position,
                        end: token.position,
                    };

                    match current_statement.clone() {
                        Some(mut st) => {
                            st.children.push(statement);
                            current_statement = Some(st)
                        }
                        None => ast.push(statement),
                    }
                }
                TokenType::Read => todo!(),
                TokenType::Write => todo!(),
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
