use crate::token::{self, Token, TokenType};

#[derive(Default)]
pub struct Lexer {
    input: String,
    position: i64,      // current position in input (current char)
    read_position: i64, // current reading position in input (next char)
    ch: char,           // current char
}

impl Lexer {
    pub fn new(input: &str) -> Self {
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

    /* Function to peek ahead and get char */
    fn peek_char(&mut self) -> char {
        let read_position =
            usize::try_from(self.read_position).expect("failed to get read_position");

        if read_position >= self.input.len() {
            return '\0';
        } else {
            return self.input.as_bytes()[read_position] as char;
        }
    }

    pub fn next_token(&mut self) -> Token {
        let mut token = Token::default();

        self.skip_whitespaces();

        match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    token.literal = "==".to_string();
                    token.token_type = TokenType::EQ;
                } else {
                    token = Token::new(TokenType::ASSIGN, self.ch);
                }
            }
            '+' => {
                token = Token::new(TokenType::PLUS, self.ch);
            }
            '-' => {
                token = Token::new(TokenType::MINUS, self.ch);
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    token.literal = "!=".to_string();
                    token.token_type = TokenType::NOT_EQ;
                } else {
                    token = Token::new(TokenType::BANG, self.ch);
                }
            }
            '*' => {
                token = Token::new(TokenType::ASTERISK, self.ch);
            }
            '/' => {
                token = Token::new(TokenType::SLASH, self.ch);
            }
            '<' => {
                token = Token::new(TokenType::LT, self.ch);
            }
            '>' => {
                token = Token::new(TokenType::GT, self.ch);
            }
            ';' => {
                token = Token::new(TokenType::SEMICOLON, self.ch);
            }
            ',' => {
                token = Token::new(TokenType::COMMA, self.ch);
            }
            '(' => {
                token = Token::new(TokenType::LPAREN, self.ch);
            }
            ')' => {
                token = Token::new(TokenType::RPAREN, self.ch);
            }
            '{' => {
                token = Token::new(TokenType::LBRACE, self.ch);
            }
            '}' => {
                token = Token::new(TokenType::RBRACE, self.ch);
            }
            '\0' => {
                token.literal = "".to_string();
                token.token_type = TokenType::EOF
            }
            ch => {
                if is_letter(ch) {
                    token.literal = self.read_identifier();
                    token.token_type = TokenType::lookup_identifier(&token.literal);
                    return token;
                } else if is_digit(ch) {
                    token.literal = self.read_number();
                    token.token_type = TokenType::INT;
                    return token;
                } else {
                    token = Token::new(TokenType::ILLEGAL, ch)
                }
            }
        }

        self.read_char();

        return token;
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;

        while is_letter(self.ch) {
            self.read_char();
        }

        return self
            .input
            .chars()
            .into_iter()
            .skip(start.try_into().unwrap())
            .take((self.position - start).try_into().unwrap())
            .collect();
    }

    /*
     * Reads number value for variable. Allowed: integers
     */
    fn read_number(&mut self) -> String {
        let start = self.position;

        while is_digit(self.ch) {
            self.read_char();
        }

        return self
            .input
            .chars()
            .into_iter()
            .skip(start.try_into().unwrap())
            .take((self.position - start).try_into().unwrap())
            .collect();
    }

    fn skip_whitespaces(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }
}

/* Allowed characters in identifiers: a-z, A-Z, _ */
fn is_letter(ch: char) -> bool {
    return ch.is_ascii_alphabetic() || ch == '_';
}

fn is_digit(ch: char) -> bool {
    return ch.is_ascii_digit();
}

#[cfg(test)]
mod tests {
    use crate::token::{self, TokenType};

    use super::*;

    #[test]
    fn test_next_token() {
        let input = "
        let foo = 5;
        let bar = 10;

        let add = function(x, y) {
            return x + y;
        }

        let result = add(foo, bar);
        !-/*5;
        10 < 100 > 10;

        if (10 < 100) {
            return true;
        } else {
            return false;
        }

        100 == 100;
        10 != 100;
        ";

        let tests = [
            (TokenType::LET, "let"),
            (TokenType::IDENT, "foo"),
            (TokenType::ASSIGN, "="),
            (TokenType::INT, "5"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::LET, "let"),
            (TokenType::IDENT, "bar"),
            (TokenType::ASSIGN, "="),
            (TokenType::INT, "10"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::LET, "let"),
            (TokenType::IDENT, "add"),
            (TokenType::ASSIGN, "="),
            (TokenType::FUNCTION, "function"),
            (TokenType::LPAREN, "("),
            (TokenType::IDENT, "x"),
            (TokenType::COMMA, ","),
            (TokenType::IDENT, "y"),
            (TokenType::RPAREN, ")"),
            (TokenType::LBRACE, "{"),
            (TokenType::RETURN, "return"),
            (TokenType::IDENT, "x"),
            (TokenType::PLUS, "+"),
            (TokenType::IDENT, "y"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::RBRACE, "}"),
            (TokenType::LET, "let"),
            (TokenType::IDENT, "result"),
            (TokenType::ASSIGN, "="),
            (TokenType::IDENT, "add"),
            (TokenType::LPAREN, "("),
            (TokenType::IDENT, "foo"),
            (TokenType::COMMA, ","),
            (TokenType::IDENT, "bar"),
            (TokenType::RPAREN, ")"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::BANG, "!"),
            (TokenType::MINUS, "-"),
            (TokenType::SLASH, "/"),
            (TokenType::ASTERISK, "*"),
            (TokenType::INT, "5"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::INT, "10"),
            (TokenType::LT, "<"),
            (TokenType::INT, "100"),
            (TokenType::GT, ">"),
            (TokenType::INT, "10"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::IF, "if"),
            (TokenType::LPAREN, "("),
            (TokenType::INT, "10"),
            (TokenType::LT, "<"),
            (TokenType::INT, "100"),
            (TokenType::RPAREN, ")"),
            (TokenType::LBRACE, "{"),
            (TokenType::RETURN, "return"),
            (TokenType::TRUE, "true"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::RBRACE, "}"),
            (TokenType::ELSE, "else"),
            (TokenType::LBRACE, "{"),
            (TokenType::RETURN, "return"),
            (TokenType::FALSE, "false"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::RBRACE, "}"),
            (TokenType::INT, "100"),
            (TokenType::EQ, "=="),
            (TokenType::INT, "100"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::INT, "10"),
            (TokenType::NOT_EQ, "!="),
            (TokenType::INT, "100"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::EOF, ""),
        ];

        let mut lexer = Lexer::new(input);

        for (expected_token_type, expected_literal) in tests {
            let token = lexer.next_token();

            assert_eq!(
                token.token_type, expected_token_type,
                "Current token: {:?}, Expected token {:?}. Literal: {}, Expected literal: {}",
                token.token_type, expected_token_type, token.literal, expected_literal
            );

            assert_eq!(
                token.literal, expected_literal,
                "Current token: {:?}, Expected token {:?}. Literal: {}, Expected literal: {}",
                token.token_type, expected_token_type, token.literal, expected_literal
            );
        }
    }
}
