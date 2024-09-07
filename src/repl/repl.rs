use std::io::{self, BufRead, BufReader, Write};

use crate::{lexer::Lexer, token::token};

const PROMPT: &str = "> ";

pub fn start() {
    println!("Welcome to Monkey programming language v0.0.1.");

    let input = BufReader::new(io::stdin());
    let mut output = io::stdout().lock();

    let mut scanner = input.lines();

    loop {
        write!(output, "{}", PROMPT).unwrap();
        output.flush().unwrap();

        if let Some(line) = scanner.next() {
            match line {
                Ok(line) => {
                    let mut lexer = Lexer::new(&line);

                    let mut tok = lexer.next_token();

                    while tok.kind != token::Kind::Eof {
                        writeln!(output, "{:?}", tok).unwrap();
                        tok = lexer.next_token();
                    }
                }
                Err(_) => return,
            }
        } else {
            return;
        }
    }
}
