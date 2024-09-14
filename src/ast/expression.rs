use crate::token::Token;

use super::ast::{self, Node};

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

// Expression statement struct
// Example: `
// let x = 5;
// x + 10; <- ExpressionStatement
//`
#[derive(Debug)]
pub struct ExpressionStatement {
    pub token: Token, // first token of expression
    pub expression: Option<Expression>,
}

impl ast::Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }

    fn to_str(&self) -> String {
        let mut sb = String::new();

        if let Some(expr) = &self.expression {
            sb.push_str(&expr.to_str());
        }

        sb
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub token: Token, // token.Ident
    pub value: String,
}

impl ast::Node for Identifier {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }

    fn to_str(&self) -> String {
        self.value.clone()
    }
}
