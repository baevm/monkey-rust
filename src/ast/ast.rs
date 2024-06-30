use std::any::Any;

pub trait Statement {
    fn token_literal(&self) -> String;

    fn statement_node(&self);
    fn as_any(&self) -> &dyn Any;
}

pub trait Expression {
    fn token_literal(&self) -> String;

    fn expression_node(&self);
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Program {
    pub fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            return self.statements[0].token_literal();
        } else {
            return String::new();
        }
    }
}
