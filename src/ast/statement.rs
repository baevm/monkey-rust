use crate::token::Token;

use super::ast::{self, Expression};

#[derive(Debug)]
pub struct LetStatement {
    pub kind: Token, // token.Let
    pub name: Identifier,
    pub value: Expression,
}

impl ast::Node for LetStatement {
    fn token_literal(&self) -> String {
        return self.kind.literal.clone();
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub kind: Token, // token.Ident
    pub value: String,
}

impl ast::Node for Identifier {
    fn token_literal(&self) -> String {
        return self.kind.literal.clone();
    }
}
