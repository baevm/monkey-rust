use crate::token::Token;

use super::{
    ast::{self, Expression},
    Identifier,
};

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token, // token.Let
    pub name: Identifier,
    pub value: Option<Expression>,
}

impl ast::Node for LetStatement {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }

    fn to_str(&self) -> String {
        let mut sb = String::new();

        sb.push_str(&self.token_literal());
        sb.push_str(" ");
        sb.push_str(&self.name.to_str());
        sb.push_str(" = ");

        if let Some(value) = &self.value {
            sb.push_str(&value.to_str())
        }

        sb.push_str(";");

        sb
    }
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: Token, // token.Return
    pub return_value: Option<Expression>,
}

impl ast::Node for ReturnStatement {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }

    fn to_str(&self) -> String {
        let mut sb = String::new();

        sb.push_str(&self.token_literal());
        sb.push_str(" ");

        if let Some(return_value) = &self.return_value {
            sb.push_str(&return_value.to_str());
        }

        sb.push_str(";");

        sb
    }
}
