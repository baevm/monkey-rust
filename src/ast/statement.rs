use crate::token::Token;

use super::{
    ast::{self, Expression},
    Identifier,
};

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
pub struct ReturnStatement {
    pub kind: Token, // token.Return
    pub return_value: Option<Expression>,
}

impl ast::Node for ReturnStatement {
    fn token_literal(&self) -> String {
        return self.kind.literal.clone();
    }
}
