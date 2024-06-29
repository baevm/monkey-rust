use crate::{ast::ast, lexer::Lexer, token::Token};

struct Parser {
    lexer: Lexer,

    curr_token: Token, // current token
    peek_token: Token, // next token
}

impl Parser {
    fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            curr_token: Token::default(),
            peek_token: Token::default(),
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
    fn parse_program(&self) -> ast::Program {
        let program = ast::Program {
            statements: Vec::new(),
        };

        return program;
    }
}
