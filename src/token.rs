#[derive(Default, PartialEq, Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, ch: char) -> Self {
        return Self {
            token_type: token_type,
            literal: ch.to_string(),
        };
    }
}

#[derive(Default, PartialEq, Debug, Clone)]
pub enum TokenType {
    ILLEGAL, // illegal token/character
    EOF,     // end of file

    // Identifiers + literals
    IDENT, // identifier: x, y, foo, etc.
    INT,   // integer: 123

    // Operators
    ASSIGN,   // =
    PLUS,     // +
    MINUS,    // -
    BANG,     // !
    ASTERISK, // *
    SLASH,    // /

    LT,     // <
    GT,     // >
    EQ,     // ==
    NOT_EQ, // !=

    // Delimeters
    COMMA,     // ","
    SEMICOLON, // ";"

    LPAREN, // "("
    RPAREN, // ")"
    LBRACE, // "{"
    RBRACE, // "}"

    // Keywords
    FUNCTION,
    LET,
    RETURN,
    TRUE,
    FALSE,
    IF,
    ELSE,

    // empty token for Default trait
    #[default]
    None,
}

impl TokenType {
    pub fn lookup_identifier(word: &str) -> Self {
        match word {
            "function" => return TokenType::FUNCTION,
            "let" => return TokenType::LET,
            "return" => return TokenType::RETURN,
            "true" => return TokenType::TRUE,
            "false" => return TokenType::FALSE,
            "if" => return TokenType::IF,
            "else" => return TokenType::ELSE,
            _ => return TokenType::IDENT,
        };
    }
}
