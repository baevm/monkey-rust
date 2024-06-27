use std::io::{self, Write};
use std::str;

use crate::lexer::Lexer;
use crate::token::TokenType;

const PROMPT: &str = "> ";

pub fn start() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        let _ = stdout.write(PROMPT.as_bytes());
        let _ = stdout.flush();

        let mut buf = String::new();

        let _ = stdin.read_line(&mut buf);

        let mut lexer = Lexer::new(&buf);

        let header = format!(
            "{0: <10} | {1: <10}\n-------------------\n",
            "LITERAL", "TYPE"
        );
        let _ = stdout.write(header.as_bytes());
        let _ = stdout.flush();

        while let token = lexer.next_token() {
            if token.token_type == TokenType::EOF {
                break;
            }

            let msg = format!("{0: <10} | {1:?}\n", token.literal, token.token_type);

            let _ = stdout.write(msg.as_bytes());
            let _ = stdout.flush();
        }

        let _ = stdout.write("\n".as_bytes());
    }
}
