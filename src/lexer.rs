#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TokenType {
    PtrLeft,
    PtrRight,
    Inc,
    Dec,
    Read,
    Write,
    LoopStart,
    LoopEnd,
    Illegal,
    Eof,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: char,
    pub position: usize,
}

pub struct Lexer {
    pub input: String,
    pub position: usize,
    pub read_position: usize,
    pub ch: char,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        if input == "" {
            panic!("Can't parse empty input");
        }

        let mut lexer = Lexer {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            ch: input.chars().next().unwrap(),
        };

        lexer.read_char();

        lexer
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.chars().count() {
            self.ch = '0';
        } else {
            self.ch = self
                .input
                .chars()
                .nth(self.read_position)
                .expect("read_char read after end of input.chars() buffer")
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let token_type = match self.ch {
            '>' => TokenType::PtrRight,
            '<' => TokenType::PtrLeft,
            '+' => TokenType::Inc,
            '-' => TokenType::Dec,
            '.' => TokenType::Write,
            ',' => TokenType::Read,
            '[' => TokenType::LoopStart,
            ']' => TokenType::LoopEnd,
            '0' => TokenType::Eof,
            _ => TokenType::Illegal,
        };

        let literal = self.ch;
        let position = self.position;

        self.read_char();

        Token {
            position,
            token_type,
            literal,
        }
    }
}

#[cfg(test)]
mod lexer_tests {
    use crate::lexer::{Lexer, Token, TokenType};

    #[test]
    fn next_token_recognize_all_tokens() {
        let mut l = Lexer::new("+<>-,.[]");

        assert_eq!(
            l.next_token(),
            Token {
                position: 0,
                token_type: TokenType::Inc,
                literal: '+'
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                position: 1,
                token_type: TokenType::PtrLeft,
                literal: '<'
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                position: 2,
                token_type: TokenType::PtrRight,
                literal: '>'
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                position: 3,
                token_type: TokenType::Dec,
                literal: '-'
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                token_type: TokenType::Read,
                position: 4,
                literal: ','
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                token_type: TokenType::Write,
                literal: '.',
                position: 5,
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                token_type: TokenType::LoopStart,
                position: 6,
                literal: '['
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                token_type: TokenType::LoopEnd,
                literal: ']',
                position: 7,
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                position: 8,
                token_type: TokenType::Eof,
                literal: '0'
            }
        );
    }

    #[test]
    fn next_token_recognize_illegal_token() {
        let mut l = Lexer::new("ab");
        assert_eq!(
            l.next_token(),
            Token {
                token_type: TokenType::Illegal,
                position: 0,
                literal: 'a'
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                token_type: TokenType::Illegal,
                position: 1,
                literal: 'b'
            }
        );
        assert_eq!(
            l.next_token(),
            Token {
                position: 2,
                token_type: TokenType::Eof,
                literal: '0'
            }
        )
    }
}
