use crate::token::Token;

use super::ast::Node;

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    PrefixExpression(PrefixExpression),
    InfixExpression(InfixExpression),
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(v) => v.token_literal(),
            Expression::IntegerLiteral(v) => v.token_literal(),
            Expression::PrefixExpression(v) => v.token_literal(),
            Expression::InfixExpression(v) => v.token_literal(),
        }
    }

    fn to_str(&self) -> String {
        match self {
            Expression::Identifier(v) => v.to_str(),
            Expression::IntegerLiteral(v) => v.to_str(),
            Expression::PrefixExpression(v) => v.to_str(),
            Expression::InfixExpression(v) => v.to_str(),
        }
    }
}

// Identifier struct
// Example: foo;
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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

// Prefix expression struct
// Example: !foo, -10;
#[derive(Debug, Clone)]
pub struct PrefixExpression {
    pub token: Token, // prefix token: token.Minus, token.Bang
    pub operator: String,
    pub right: Option<Box<Expression>>,
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_str(&self) -> String {
        let mut sb = String::new();

        sb.push_str("(");
        sb.push_str(&self.operator);
        sb.push_str(&self.right.as_ref().unwrap().to_str());
        sb.push_str(")");

        sb
    }
}

// Infix expression struct
// Example: 5 + 5, 5 != 5;
#[derive(Debug, Clone)]
pub struct InfixExpression {
    pub token: Token, // prefix token: token.Minus, token.Bang
    pub left: Option<Box<Expression>>,
    pub operator: String,
    pub right: Option<Box<Expression>>,
}

impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_str(&self) -> String {
        let mut sb = String::new();

        sb.push_str("(");
        sb.push_str(&self.left.as_ref().unwrap().to_str());
        sb.push_str(" ");
        sb.push_str(&self.operator);
        sb.push_str(" ");
        sb.push_str(&self.right.as_ref().unwrap().to_str());
        sb.push_str(")");

        sb
    }
}
