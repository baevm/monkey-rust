use crate::token::Token;

use super::ast::Node;

#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier),
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(v) => v.token_literal(),
        }
    }

    fn to_str(&self) -> String {
        match self {
            Expression::Identifier(v) => v.to_str(),
        }
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub token: Token, // token.Ident
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }

    fn to_str(&self) -> String {
        self.value.clone()
    }
}
