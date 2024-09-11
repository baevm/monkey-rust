use super::{expression, statement};

pub trait Node {
    fn token_literal(&self) -> String;
    fn to_str(&self) -> String;
}

#[derive(Debug)]
pub enum Statement {
    LetStatement(statement::LetStatement),
    ReturnStatement(statement::ReturnStatement),
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::LetStatement(v) => v.token_literal(),
            Statement::ReturnStatement(v) => v.token_literal(),
        }
    }

    fn to_str(&self) -> String {
        match self {
            Statement::LetStatement(v) => v.to_str(),
            Statement::ReturnStatement(v) => v.to_str(),
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Identifier(expression::Identifier),
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(v) => v.token_literal(),
        }
    }

    fn to_str(&self) -> String {
        todo!()
    }
}

/// Root node of every AST
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if !self.statements.is_empty() {
            self.statements.first().unwrap().token_literal()
        } else {
            "".to_string()
        }
    }

    fn to_str(&self) -> String {
        let mut sb = String::new();

        for statement in &self.statements {
            sb.push_str(&statement.to_str());
        }

        sb
    }
}
