#[derive(Default, PartialEq, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
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

    // empty token for Default trait
    #[default]
    None,
}
