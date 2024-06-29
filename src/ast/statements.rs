use crate::token::Token;

use super::{
    ast::{Expression, Statement},
    expressions::Identifier,
};

/* let ... */
struct LetStatement {
    token: Token,
    name: Identifier,
    value: Box<dyn Expression>,
}

impl Statement for LetStatement {
    fn statement_node(&self) {}

    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }
}
