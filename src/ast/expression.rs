use crate::token::Token;

use super::ast::Node;

#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(v) => v.token_literal(),
            Expression::IntegerLiteral(v) => v.token_literal(),
        }
    }

    fn to_str(&self) -> String {
        match self {
            Expression::Identifier(v) => v.to_str(),
            Expression::IntegerLiteral(v) => v.to_str(),
        }
    }
}

// Identifier struct
// Example: foo;
#[derive(Debug)]
pub struct Identifier {
    pub token: Token, // token.Ident
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_str(&self) -> String {
        self.value.clone()
    }
}

// Integer literal struct
// Example: 5;
#[derive(Debug)]
pub struct IntegerLiteral {
    pub token: Token, // token.Number
    pub value: i64,
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_str(&self) -> String {
        self.token.literal.clone()
    }
}
