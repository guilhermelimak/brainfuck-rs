use rustyline::error::ReadlineError;
use rustyline::Editor;
mod lexer;
mod parser;

use crate::{
    lexer::{Lexer, TokenType},
    parser::Parser,
};

fn main() {
    let mut rl = Editor::<()>::new();

    let mut print_tokens = true;
    let mut print_ast = true;

    loop {
        let readline = rl.readline(" >> ");
        match readline {
            Ok(line) => {
                match line.as_str() {
                    "_tokens" => {
                        print_tokens = !print_tokens;
                        println!(
                            "Tokens {}",
                            if print_tokens { "enabled" } else { "disabled" }
                        );
                        continue;
                    }
                    "_ast" => {
                        print_ast = !print_ast;
                        println!("AST {}", if print_ast { "enabled" } else { "disabled" });
                        continue;
                    }
                    "" => continue,
                    _ => {
                        if !print_ast && !print_tokens {
                            println!("Line: {}", line)
                        }
                    }
                }

                let mut l = Lexer::new(&line);
                let mut tokens = vec![];

                if print_tokens {
                    println!("Tokens for: {}", line);
                }

                loop {
                    let token = l.next_token();
                    if token.token_type == TokenType::Eof {
                        if print_tokens {
                            println!("{:#?}", tokens);
                        }
                        break;
                    }
                    tokens.push(token);
                }

                if print_ast {
                    println!("AST for: {}", line);
                    Parser::parse(&mut tokens.into_iter());
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
