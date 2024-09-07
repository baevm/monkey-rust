#[derive(Default)]
pub struct Token {
    pub kind: Kind,
    pub literal: String,
}

#[derive(PartialEq, Debug)]
pub enum Kind {
    Eof,
    Illegal,

    // identifiers, literals
    Ident,
    Number,

    // operators
    Assign,
    Plus,

    // delimiters
    Comma,
    Semicolon,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    // keywords
    Function,
    Let,
    Return,
}

impl Default for Kind {
    fn default() -> Self {
        Kind::Eof
    }
}

impl Kind {
    pub fn to_str(&self) -> &'static str {
        match self {
            Kind::Eof => "EOF",
            Kind::Illegal => "Illegal",
            Kind::Ident => "Identifier",
            Kind::Number => "number",
            Kind::Assign => "=",
            Kind::Plus => "+",
            Kind::Comma => ",",
            Kind::Semicolon => ";",
            Kind::Lparen => "(",
            Kind::Rparen => ")",
            Kind::Lbrace => "{",
            Kind::Rbrace => "}",
            Kind::Function => "function",
            Kind::Let => "let",
            Kind::Return => "return",
        }
    }

    /// Checks if given identifier is keyword
    pub fn lookup_ident(ident: &str) -> Self {
        match ident {
            "function" => Kind::Function,
            "let" => Kind::Let,
            "return" => Kind::Return,
            _ => Kind::Ident,
        }
    }
}
