use std::fmt::format;

use crate::{
    ast::{ast, expressions, statements},
    lexer::Lexer,
    token::{Token, TokenType},
};

struct Parser {
    lexer: Lexer,

    curr_token: Token, // current token
    peek_token: Token, // next token

    errors: Vec<String>,
}

impl Parser {
    fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            curr_token: Token::default(),
            peek_token: Token::default(),
            errors: vec![],
        };

        /* Read 2 tokens to set both curr_token and peek_token */
        parser.next_token();
        parser.next_token();

        return parser;
    }

    fn next_token(&mut self) {
        self.curr_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    /* Entry point */
    fn parse_program(&mut self) -> ast::Program {
        let mut program = ast::Program {
            statements: Vec::new(),
        };

        while self.curr_token.token_type != TokenType::EOF {
            let statement = self.parse_statement();

            match statement {
                Some(stmt) => {
                    program.statements.push(stmt);
                }
                None => {}
            }

            self.next_token();
        }

        return program;
    }

    fn parse_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        match self.curr_token.token_type {
            TokenType::LET => {
                return self.parse_let_statement();
            }
            _ => {
                return None;
            }
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        let mut let_statement = statements::LetStatement {
            token: self.curr_token.clone(),
            ..Default::default()
        };

        if !self.expect_peek(TokenType::IDENT) {
            return None;
        }

        let_statement.name = expressions::Identifier {
            token: self.curr_token.clone(),
            value: self.curr_token.literal.clone(),
        };

        if !self.expect_peek(TokenType::ASSIGN) {
            return None;
        }

        /* Skipping expressions until we find semicolon */
        while !self.curr_token_eq(TokenType::SEMICOLON) {
            self.next_token();
        }

        return Some(Box::new(let_statement));
    }

    fn curr_token_eq(&self, token: TokenType) -> bool {
        return self.curr_token.token_type == token;
    }

    fn peek_token_eq(&self, token: TokenType) -> bool {
        return self.peek_token.token_type == token;
    }

    fn expect_peek(&mut self, token: TokenType) -> bool {
        if self.peek_token_eq(token.clone()) {
            self.next_token();
            return true;
        } else {
            self.peek_error(token);
            return false;
        }
    }

    fn peek_error(&mut self, token: TokenType) {
        let message = format!(
            "TOKEN ERROR: expected next token to be {:?}. got {:?} instead",
            token, self.peek_token.token_type,
        );

        self.errors.push(message);
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::statements::LetStatement;

    use super::*;

    #[test]
    fn test_let_statement() {
        let input = "
        let x = 5;
        let y = 10;
        let foobar = 838383;
        ";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        check_parser_errors(parser);

        assert_eq!(program.statements.len(), 3);

        let tests = vec!["x", "y", "foobar"];

        for (i, expected_identifier) in tests.into_iter().enumerate() {
            let statement = &program.statements[i];

            assert_eq!(statement.token_literal(), "let");

            let let_statement = statement
                .as_any()
                .downcast_ref::<LetStatement>()
                .expect("failed to convert to LetStatement");

            assert_eq!(let_statement.name.value, expected_identifier);
            assert_eq!(let_statement.name.token_literal(), expected_identifier);
        }
    }

    fn check_parser_errors(parser: Parser) {
        let errors = parser.errors;

        if errors.len() == 0 {
            return;
        }

        for error in &errors {
            eprintln!("{}", error);
        }

        assert_eq!(errors.len(), 0)
    }
}
