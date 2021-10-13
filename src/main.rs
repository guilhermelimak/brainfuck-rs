use rustyline::error::ReadlineError;
use rustyline::Editor;
mod lexer;
mod parser;

use crate::{lexer::Lexer, parser::Parser};
struct Repl {
    print_tokens: bool,
    print_ast: bool,
}

impl Repl {
    pub fn toggle_tokens(&mut self) {
        self.print_tokens = !self.print_tokens;
        println!(
            "Tokens {}",
            if self.print_tokens {
                "enabled"
            } else {
                "disabled"
            }
        );
    }

    pub fn toggle_ast(&mut self) {
        self.print_ast = !self.print_ast;
        println!(
            "Ast {}",
            if self.print_ast {
                "enabled"
            } else {
                "disabled"
            }
        );
    }
}

fn main() {
    let mut rl = Editor::<()>::new();

    let mut repl = Repl {
        print_tokens: true,
        print_ast: true,
    };

    loop {
        let readline = rl.readline(" >> ");
        match readline {
            Ok(line) => {
                match line.as_str() {
                    "_tokens" => {
                        repl.toggle_tokens();
                        continue;
                    }
                    "_ast" => {
                        repl.toggle_ast();
                        continue;
                    }
                    "" => continue,
                    _ => {}
                }

                let tokens = Lexer::new(&line).lex();

                if repl.print_tokens {
                    println!("{:#?}", tokens);
                }

                let ast = Parser::parse(&mut tokens.into_iter());

                if repl.print_ast {
                    println!("AST: {:#?}", ast)
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
