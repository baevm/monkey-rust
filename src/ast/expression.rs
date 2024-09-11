use crate::token::Token;

use super::ast;

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
