use crate::token::Token;

/* name of variable */
pub struct Identifier {
    token: Token,
    value: String,
}

impl Identifier {
    fn expression_node(&self) {}

    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }
}
