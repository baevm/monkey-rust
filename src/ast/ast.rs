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
        match self {
            Expression::Identifier(v) => v.to_str(),
        }
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

mod test {
    use crate::{
        ast::{ast::Node, Identifier, LetStatement},
        token::{Kind, Token},
    };

    use super::{Expression, Program, Statement};

    #[test]
    fn test_to_str() {
        // "let foo = bar;"
        let stmt = Statement::LetStatement(LetStatement {
            token: Token {
                kind: Kind::Let,
                literal: "let".to_string(),
            },
            name: Identifier {
                token: Token {
                    kind: Kind::Ident,
                    literal: "foo".to_string(),
                },
                value: "foo".to_string(),
            },
            value: Some(Expression::Identifier(Identifier {
                token: Token {
                    kind: Kind::Ident,
                    literal: "bar".to_string(),
                },
                value: "bar".to_string(),
            })),
        });

        let program = Program {
            statements: vec![stmt],
        };

        assert_eq!(program.to_str(), "let foo = bar;")
    }
}
