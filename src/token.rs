use std::collections::HashMap;

#[derive(Default, PartialEq, Debug)]
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

#[derive(Default, PartialEq, Debug)]
pub enum TokenType {
    ILLEGAL, // illegal token/character
    EOF,     // end of file

    // Identifiers + literals
    IDENT, // identifier: x, y, foo, etc.
    INT,   // integer: 123

    // Operators
    ASSIGN, // =
    PLUS,   // +

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
            _ => return TokenType::IDENT,
        };
    }
}
