use crate::token::{token, Kind, Token};

pub struct Lexer {
    input: String,
    position: usize,      // current character pos in input
    read_position: usize, // next character pos in input
    ch: char,             // current character
}

impl Lexer {
    const EMPTY_CHAR: char = '\0';

    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            ch: Self::EMPTY_CHAR,
        };
        lexer.read_char();

        return lexer;
    }

    pub fn next_token(&mut self) -> Token {
        let mut token = Token::default();

        self.skip_whitespace();

        match self.ch {
            ';' => {
                token = self.new_token(Kind::Semicolon, self.ch);
            }
            '(' => {
                token = self.new_token(Kind::Lparen, self.ch);
            }
            ')' => {
                token = self.new_token(Kind::Rparen, self.ch);
            }
            ',' => {
                token = self.new_token(Kind::Comma, self.ch);
            }
            '+' => {
                token = self.new_token(Kind::Plus, self.ch);
            }
            '{' => {
                token = self.new_token(Kind::Lbrace, self.ch);
            }
            '}' => {
                token = self.new_token(Kind::Rbrace, self.ch);
            }
            '-' => {
                token = self.new_token(Kind::Minus, self.ch);
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    token = Token {
                        kind: Kind::Neq,
                        literal: "!=".to_string(),
                    }
                } else {
                    token = self.new_token(Kind::Bang, self.ch);
                }
            }
            '*' => {
                token = self.new_token(Kind::Asterisk, self.ch);
            }
            '/' => {
                token = self.new_token(Kind::Slash, self.ch);
            }
            '<' => {
                token = self.new_token(Kind::Lt, self.ch);
            }
            '>' => {
                token = self.new_token(Kind::Gt, self.ch);
            }
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    token = Token {
                        kind: Kind::Eq,
                        literal: "==".to_string(),
                    };
                } else {
                    token = self.new_token(Kind::Assign, self.ch);
                }
            }
            Self::EMPTY_CHAR => {
                token.literal = "".to_string();
                token.kind = Kind::Eof;
            }
            _ => {
                if self.is_letter(self.ch) {
                    token.literal = self.read_identifier();
                    token.kind = Kind::lookup_ident(&token.literal);
                    return token;
                } else if self.is_digit(self.ch) {
                    token.literal = self.read_number();
                    token.kind = Kind::Number;
                    return token;
                } else {
                    token = self.new_token(Kind::Illegal, self.ch);
                }
            }
        }

        self.read_char();

        return token;
    }

    fn new_token(&self, kind: token::Kind, ch: char) -> Token {
        Token {
            kind: kind,
            literal: ch.to_string(),
        }
    }

    /// Reads current character in input
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = Self::EMPTY_CHAR;
        } else {
            self.ch = self
                .input
                .chars()
                .nth(self.read_position)
                .expect("read_char failed read_position");
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    /// Peeks next character in input
    fn peek_char(&mut self) -> char {
        if self.read_position >= self.input.len() {
            return Self::EMPTY_CHAR;
        } else {
            return self
                .input
                .chars()
                .nth(self.read_position)
                .expect("peek_char failed read_position");
        }
    }

    /// Checks if char is valid letter. Only ASCII and _
    fn is_letter(&self, ch: char) -> bool {
        return ch.is_ascii_alphabetic() || ch == '_';
    }

    /// Checks if char is valid number.
    fn is_digit(&self, ch: char) -> bool {
        return ch.is_ascii_digit();
    }

    /// Reads identifier
    fn read_identifier(&mut self) -> String {
        let position = self.position;

        while self.is_letter(self.ch) {
            self.read_char();
        }

        return self.input[position..self.position].to_string();
    }

    /// Skips whitespaces
    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    /// Reads number
    fn read_number(&mut self) -> String {
        let position = self.position;

        while self.is_digit(self.ch) {
            self.read_char();
        }

        return self.input[position..self.position].to_string();
    }
}

mod test {
    use crate::token;

    use super::Lexer;

    #[test]
    fn test_next_token() {
        let input = "
            let five = 5;
            let ten = 10;

            let add = function(x, y) {
                return x + y;
            };

            let result = add(five, ten);

            !-/*5;
            5 < 10 > 5;

            if (5 < 10) {
                return true;
            } else {
                return false;
            }

            10 == 10;
            10 != 9;
        ";

        let tests = Vec::from([
            (token::Kind::Let, "let"),
            (token::Kind::Ident, "five"),
            (token::Kind::Assign, "="),
            (token::Kind::Number, "5"),
            (token::Kind::Semicolon, ";"),
            (token::Kind::Let, "let"),
            (token::Kind::Ident, "ten"),
            (token::Kind::Assign, "="),
            (token::Kind::Number, "10"),
            (token::Kind::Semicolon, ";"),
            (token::Kind::Let, "let"),
            (token::Kind::Ident, "add"),
            (token::Kind::Assign, "="),
            (token::Kind::Function, "function"),
            (token::Kind::Lparen, "("),
            (token::Kind::Ident, "x"),
            (token::Kind::Comma, ","),
            (token::Kind::Ident, "y"),
            (token::Kind::Rparen, ")"),
            (token::Kind::Lbrace, "{"),
            (token::Kind::Return, "return"),
            (token::Kind::Ident, "x"),
            (token::Kind::Plus, "+"),
            (token::Kind::Ident, "y"),
            (token::Kind::Semicolon, ";"),
            (token::Kind::Rbrace, "}"),
            (token::Kind::Semicolon, ";"),
            (token::Kind::Let, "let"),
            (token::Kind::Ident, "result"),
            (token::Kind::Assign, "="),
            (token::Kind::Ident, "add"),
            (token::Kind::Lparen, "("),
            (token::Kind::Ident, "five"),
            (token::Kind::Comma, ","),
            (token::Kind::Ident, "ten"),
            (token::Kind::Rparen, ")"),
            (token::Kind::Semicolon, ";"),
            (token::Kind::Bang, "!"),
            (token::Kind::Minus, "-"),
            (token::Kind::Slash, "/"),
            (token::Kind::Asterisk, "*"),
            (token::Kind::Number, "5"),
            (token::Kind::Semicolon, ";"),
            (token::Kind::Number, "5"),
            (token::Kind::Lt, "<"),
            (token::Kind::Number, "10"),
            (token::Kind::Gt, ">"),
            (token::Kind::Number, "5"),
            (token::Kind::Semicolon, ";"),
            (token::Kind::If, "if"),
            (token::Kind::Lparen, "("),
            (token::Kind::Number, "5"),
            (token::Kind::Lt, "<"),
            (token::Kind::Number, "10"),
            (token::Kind::Rparen, ")"),
            (token::Kind::Lbrace, "{"),
            (token::Kind::Return, "return"),
            (token::Kind::True, "true"),
            (token::Kind::Semicolon, ";"),
            (token::Kind::Rbrace, "}"),
            (token::Kind::Else, "else"),
            (token::Kind::Lbrace, "{"),
            (token::Kind::Return, "return"),
            (token::Kind::False, "false"),
            (token::Kind::Semicolon, ";"),
            (token::Kind::Rbrace, "}"),
            (token::Kind::Number, "10"),
            (token::Kind::Eq, "=="),
            (token::Kind::Number, "10"),
            (token::Kind::Semicolon, ";"),
            (token::Kind::Number, "10"),
            (token::Kind::Neq, "!="),
            (token::Kind::Number, "9"),
            (token::Kind::Semicolon, ";"),
            (token::Kind::Eof, ""),
        ]);

        let mut lexer = Lexer::new(input);

        for (id, test) in tests.iter().enumerate() {
            let token = lexer.next_token();

            assert_eq!(
                test.0, token.kind,
                "Expected Kind: {:?}. Got: {:?}. id: {}",
                test.0, token.kind, id
            );

            assert_eq!(
                test.1, token.literal,
                "Expected Literal: {:?}. Got: {:?}. id: {}",
                test.1, token.literal, id
            );
        }
    }
}
