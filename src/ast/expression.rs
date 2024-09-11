use crate::token::Token;

use super::ast::{self, Expression};

// Expression statement struct
// Example: `
// let x = 5;
// x + 10; <- ExpressionStatement
//`
#[derive(Debug)]
pub struct ExpressionStatement {
    pub kind: Token, // first token of expression
    pub expression: Expression,
}

impl ast::Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        return self.kind.literal.clone();
    }

    fn to_str(&self) -> String {
        todo!()
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

    fn to_str(&self) -> String {
        todo!()
    }
}
