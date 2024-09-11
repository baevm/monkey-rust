use super::{expression, statement};

pub trait Node {
    fn token_literal(&self) -> String;
}

#[derive(Debug)]
pub enum Statement {
    LetStatement(statement::LetStatement),
    ReturnStatement(statement::ReturnStatement),
}

impl Statement {
    pub fn token_literal(&self) -> String {
        match self {
            Statement::LetStatement(v) => v.token_literal(),
            Statement::ReturnStatement(v) => v.token_literal(),
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Identifier(expression::Identifier),
}

impl Expression {
    pub fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(v) => v.token_literal(),
        }
    }
}

/// Root node of every AST
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    fn token_literal(&self) -> String {
        if !self.statements.is_empty() {
            self.statements.first().unwrap().token_literal()
        } else {
            "".to_string()
        }
    }
}
