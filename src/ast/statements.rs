use crate::token::Token;

use super::{
    ast::{Expression, Statement},
    expressions::Identifier,
};

/* let ... */
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Option<Box<dyn Expression>>,
}

impl Statement for LetStatement {
    fn statement_node(&self) {}

    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Default for LetStatement {
    fn default() -> Self {
        Self {
            token: Default::default(),
            name: Default::default(),
            value: Default::default(),
        }
    }
}
