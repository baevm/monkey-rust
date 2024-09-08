use crate::{
    ast::{ast, statement},
    lexer::lexer,
    token::{Kind, Token},
};

pub struct Parser {
    lexer: lexer::Lexer,

    curr_token: Token,
    peek_token: Token,

    errors: Vec<String>,
}

impl Parser {
    pub fn new(lexer: lexer::Lexer) -> Self {
        let mut parser = Parser {
            lexer: lexer,
            curr_token: Token::default(),
            peek_token: Token::default(),
            errors: vec![],
        };

        // read 2 next tokens to set both curr_token and peek_token
        parser.next_token();
        parser.next_token();

        parser
    }

    pub fn parse_program(&mut self) -> ast::Program {
        let mut program = ast::Program { statements: vec![] };

        while !self.is_curr_token(Kind::Eof) {
            let stmt = self.parse_statement();

            if let Some(stmt_val) = stmt {
                program.statements.push(stmt_val)
            }

            self.next_token();
        }

        program
    }

    fn next_token(&mut self) {
        self.curr_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_statement(&mut self) -> Option<ast::Statement> {
        match self.curr_token.kind {
            Kind::Let => self.parse_let_statement(),
            _ => None,
        }
    }

    /// Parses let statements: "let foo = 5;"
    fn parse_let_statement(&mut self) -> Option<ast::Statement> {
        let kind = self.curr_token.clone();

        if !self.expect_peek(Kind::Ident) {
            return None;
        }

        let identifier = statement::Identifier {
            kind: self.curr_token.clone(),
            value: self.curr_token.literal.clone(),
        };

        if !self.expect_peek(Kind::Assign) {
            return None;
        }

        let let_stmt = statement::LetStatement {
            kind: kind,
            name: identifier,
            value: ast::Expression::Identifier(statement::Identifier {
                kind: Token {
                    kind: Kind::Ident,
                    literal: "".to_string(),
                },
                value: "".to_string(),
            }),
        };

        while !self.is_curr_token(Kind::Semicolon) {
            self.next_token();
        }

        Some(ast::Statement::LetStatement(let_stmt))
    }

    fn expect_peek(&mut self, expected: Kind) -> bool {
        if self.is_peek_token(expected) {
            self.next_token();
            true
        } else {
            self.peek_error(expected);
            false
        }
    }

    fn is_curr_token(&self, expected: Kind) -> bool {
        self.curr_token.kind == expected
    }

    fn is_peek_token(&self, expected: Kind) -> bool {
        self.peek_token.kind == expected
    }

    fn peek_error(&mut self, token: Kind) {
        let msg = format!(
            "expected next token: {:?}. Got token: {:?}",
            token, self.peek_token.kind
        );
        self.errors.push(msg);
    }

    fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }
}

mod test {
    use crate::{
        ast::ast::{Node, Statement},
        lexer,
    };

    use super::Parser;

    #[test]
    fn test_let_statements() {
        let input = "
        let x = 5;
        let y = 10;
        let foo = 999;
        ";

        let lexer = lexer::Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert_eq!(parser.errors().len(), 0, "errors should be zero");

        let tests = Vec::from(["x".to_string(), "y".to_string(), "foo".to_string()]);

        for (idx, expected_name) in tests.iter().enumerate() {
            let stmt = program.statements.get(idx).unwrap();

            assert_eq!(stmt.token_literal(), "let");

            let stmt = match stmt {
                Statement::LetStatement(v) => v,
                _ => unreachable!(),
            };

            assert_eq!(&stmt.name.value, expected_name);
            assert_eq!(&stmt.name.token_literal(), expected_name);
        }
    }
}
