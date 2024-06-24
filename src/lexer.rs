use crate::token::{self, Token, TokenType};

#[derive(Default)]
struct Lexer {
    input: String,
    position: i64,      // current position in input (current char)
    read_position: i64, // current reading position in input (next char)
    ch: char,           // current char
}

impl Lexer {
    fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: input.to_string(),
            ..Default::default()
        };

        lexer.read_char();

        return lexer;
    }

    fn read_char(&mut self) {
        let read_position =
            usize::try_from(self.read_position).expect("failed to get read_position");

        // check if we reached end of input
        if read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.as_bytes()[read_position] as char;
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        let mut token = Token {
            ..Default::default()
        };

        match self.ch {
            '=' => {
                token = new_token(TokenType::ASSIGN, self.ch);
            }
            ';' => {
                token = new_token(TokenType::SEMICOLON, self.ch);
            }
            ',' => {
                token = new_token(TokenType::COMMA, self.ch);
            }
            '(' => {
                token = new_token(TokenType::LPAREN, self.ch);
            }
            ')' => {
                token = new_token(TokenType::RPAREN, self.ch);
            }
            '{' => {
                token = new_token(TokenType::LBRACE, self.ch);
            }
            '}' => {
                token = new_token(TokenType::RBRACE, self.ch);
            }
            '+' => {
                token = new_token(TokenType::PLUS, self.ch);
            }
            '\0' => {
                token.literal = "".to_string();
                token.token_type = TokenType::EOF
            }
            _ => {
                panic!("Token not implemented")
            }
        }

        self.read_char();

        return token;
    }
}

fn new_token(token_type: TokenType, ch: char) -> Token {
    return Token {
        token_type: token_type,
        literal: ch.to_string(),
    };
}

#[cfg(test)]
mod tests {
    use crate::token::{self, TokenType};

    use super::*;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";

        let tests = [
            (TokenType::ASSIGN, "="),
            (TokenType::PLUS, "+"),
            (TokenType::LPAREN, "("),
            (TokenType::RPAREN, ")"),
            (TokenType::LBRACE, "{"),
            (TokenType::RBRACE, "}"),
            (TokenType::COMMA, ","),
            (TokenType::SEMICOLON, ";"),
            (TokenType::EOF, ""),
        ];

        let mut lexer = Lexer::new(input);

        for (expected_token_type, expected_literal) in tests {
            let token = lexer.next_token();

            assert_eq!(token.token_type, expected_token_type);
            assert_eq!(token.literal, expected_literal);
        }
    }
}
