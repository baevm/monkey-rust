use crate::token::Token;

/* name of variable */
#[derive(Default)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Identifier {
    pub fn expression_node(&self) {}

    pub fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }
}
