use crate::{
    ast::{ast::Program, expression, statement, Expression, ExpressionStatement, Statement},
    lexer::Lexer,
    token::{Kind, Token},
};

pub struct Parser {
    lexer: Lexer,

    curr_token: Token,
    peek_token: Token,

    errors: Vec<String>,
}

/// Operator precedence
#[derive(PartialEq, Eq, PartialOrd, Debug)]
enum Precedence {
    Lowest,
    Equals,      // ==
    LessGreater, // < or >
    Sum,         // +
    Product,     // *
    Prefix,      // -foo or !foo
    Call,        // foo_function()
}

impl Precedence {
    fn get_precedence_for_token(kind: Kind) -> Precedence {
        match kind {
            Kind::Eq => Precedence::Equals,
            Kind::Neq => Precedence::Equals,
            Kind::Lt => Precedence::LessGreater,
            Kind::Gt => Precedence::LessGreater,
            Kind::Plus => Precedence::Sum,
            Kind::Minus => Precedence::Sum,
            Kind::Slash => Precedence::Product,
            Kind::Asterisk => Precedence::Product,
            _ => Precedence::Lowest,
        }
    }
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
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

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program { statements: vec![] };

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

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.curr_token.kind {
            Kind::Let => self.parse_let_statement(),
            Kind::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        let left_expr = self.get_parsed_prefix_expression(self.curr_token.kind);

        if left_expr.is_none() {
            let message = format!(
                "no prefix parse function for {:?} found",
                self.curr_token.kind
            );
            self.errors.push(message);
            return None;
        }

        let mut left_expr = left_expr.unwrap();

        while !self.is_peek_token(Kind::Semicolon) && precedence < self.peek_precedence() {
            let infix = self.get_parsed_infix_expression(self.peek_token.kind, left_expr.clone());

            if infix.is_none() {
                return Some(left_expr);
            }

            left_expr = infix.unwrap();
        }

        Some(left_expr)
    }

    fn get_parsed_prefix_expression(&mut self, kind: Kind) -> Option<Expression> {
        match kind {
            Kind::Ident => Some(Expression::Identifier(expression::Identifier {
                token: self.curr_token.clone(),
                value: self.curr_token.literal.clone(),
            })),
            Kind::Number => self.parse_integer_literal(),
            Kind::Bang | Kind::Minus => self.parse_prefix_expression(),
            _ => None,
        }
    }

    fn get_parsed_infix_expression(&mut self, kind: Kind, left: Expression) -> Option<Expression> {
        match kind {
            Kind::Plus
            | Kind::Minus
            | Kind::Slash
            | Kind::Asterisk
            | Kind::Eq
            | Kind::Neq
            | Kind::Lt
            | Kind::Gt => self.parse_infix_expression(left),
            _ => None,
        }
    }

    /// Parses let statements: "let foo = 5;"
    fn parse_let_statement(&mut self) -> Option<Statement> {
        let kind = self.curr_token.clone();

        if !self.expect_peek(Kind::Ident) {
            return None;
        }

        let identifier = expression::Identifier {
            token: self.curr_token.clone(),
            value: self.curr_token.literal.clone(),
        };

        if !self.expect_peek(Kind::Assign) {
            return None;
        }

        let let_stmt = statement::LetStatement {
            token: kind,
            name: identifier,
            value: Some(Expression::Identifier(expression::Identifier {
                token: Token {
                    kind: Kind::Ident,
                    literal: "".to_string(),
                },
                value: "".to_string(),
            })),
        };

        while !self.is_curr_token(Kind::Semicolon) {
            self.next_token();
        }

        Some(Statement::LetStatement(let_stmt))
    }

    /// Parses return statements: "return foo;"
    fn parse_return_statement(&mut self) -> Option<Statement> {
        let token = self.curr_token.clone();

        self.next_token();

        while !self.is_curr_token(Kind::Semicolon) {
            self.next_token();
        }

        let return_stmt = statement::ReturnStatement {
            token,
            return_value: None,
        };

        Some(Statement::ReturnStatement(return_stmt))
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let token = self.curr_token.clone();
        let expression = self.parse_expression(Precedence::Lowest);

        let stmt = ExpressionStatement { token, expression };

        if self.is_peek_token(Kind::Semicolon) {
            self.next_token()
        }

        Some(Statement::ExpressionStatement(stmt))
    }

    // Parses integer literals: 1, 5, 100, etc.
    fn parse_integer_literal(&mut self) -> Option<Expression> {
        let token = self.curr_token.clone();
        let value = self.curr_token.literal.clone().parse::<i64>();

        if value.is_err() {
            let message = format!("could not parse {} as integer", self.curr_token.literal);
            self.errors.push(message);
            return None;
        }

        let literal = expression::IntegerLiteral {
            token,
            value: value.unwrap(),
        };

        Some(Expression::IntegerLiteral(literal))
    }

    // Parsed prefix expressions: -100, !foo, etc.
    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let mut expr = expression::PrefixExpression {
            token: self.curr_token.clone(),
            operator: self.curr_token.literal.clone(),
            right: None,
        };

        // Move to "right" side
        self.next_token();

        match self.parse_expression(Precedence::Prefix) {
            Some(val) => expr.right = Some(Box::new(val)),
            None => self
                .errors
                .push("No value found in prefix right side.".to_string()),
        }

        Some(Expression::PrefixExpression(expr))
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        self.next_token();

        let mut expr = expression::InfixExpression {
            token: self.curr_token.clone(),
            operator: self.curr_token.literal.clone(),
            left: Some(Box::new(left)),
            right: None,
        };

        let precedence = self.curr_precedence();
        self.next_token();

        if let Some(right_val) = self.parse_expression(precedence) {
            expr.right = Some(Box::new(right_val));
        }

        Some(Expression::InfixExpression(expr))
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

    fn peek_precedence(&self) -> Precedence {
        let precedence = Precedence::get_precedence_for_token(self.peek_token.kind);

        precedence
    }

    fn curr_precedence(&self) -> Precedence {
        let precedence = Precedence::get_precedence_for_token(self.curr_token.kind);

        precedence
    }
}

#[cfg(test)]
mod test {
    use crate::{
        ast::{ast::Node, Expression, Statement},
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

    #[test]
    fn test_return_statement() {
        let input = "
            return 5;
        ";

        let lexer = lexer::Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert_eq!(parser.errors().len(), 0, "errors should be zero");

        for stmt in program.statements {
            match stmt {
                Statement::ReturnStatement(_) => assert_eq!(stmt.token_literal(), "return"),
                Statement::ExpressionStatement(_) => assert_eq!(stmt.token_literal(), "5"),
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";

        let lexer = lexer::Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert_eq!(parser.errors().len(), 0, "errors should be zero");

        assert_eq!(
            program.statements.len(),
            1,
            "program has not enough statements"
        );

        let stmt = program.statements.first().unwrap();

        let stmt = match stmt {
            Statement::ExpressionStatement(v) => v,
            _ => panic!("statement not ExpressionStatement"),
        };

        let expr = stmt.expression.as_ref().expect("expression not Some");

        let ident = match expr {
            Expression::Identifier(v) => v,
            _ => panic!("expression not Identifier"),
        };

        assert_eq!(ident.value, "foobar");
        assert_eq!(ident.token_literal(), "foobar");
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;";

        let lexer = lexer::Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert_eq!(parser.errors().len(), 0, "errors should be zero");

        assert_eq!(
            program.statements.len(),
            1,
            "program has not enough statements"
        );

        let stmt = program.statements.first().unwrap();

        let stmt = match stmt {
            Statement::ExpressionStatement(v) => v,
            _ => panic!("statement not ExpressionStatement"),
        };

        let expr = stmt.expression.as_ref().expect("expression not Some");

        let literal = match expr {
            Expression::IntegerLiteral(v) => v,
            _ => panic!("expression not IntegerLiteral"),
        };

        assert_eq!(literal.value, 5);
        assert_eq!(literal.token_literal(), "5");
    }

    #[test]
    fn test_prefix_expression() {
        let tests = vec![("!5", "!", 5), ("-10", "-", 10)];

        for (input, expected_prefix, expected_right) in tests {
            let lexer = lexer::Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();

            for error in parser.errors() {
                println!("ERROR: {}", error);
            }

            assert_eq!(parser.errors().len(), 0, "errors should be zero");

            assert_eq!(
                program.statements.len(),
                1,
                "program has not enough statements"
            );

            let stmt = program.statements.first().unwrap();

            let stmt = match stmt {
                Statement::ExpressionStatement(v) => v,
                _ => panic!("statement not ExpressionStatement"),
            };

            let expr = stmt.expression.as_ref().expect("expression not Some");

            let prefix_expr = match expr {
                Expression::PrefixExpression(v) => v,
                _ => panic!("expression not PrefixExpression"),
            };

            assert_eq!(prefix_expr.operator, expected_prefix);

            // test integer literal
            if let Some(right_expr) = prefix_expr.right.as_deref() {
                let int_literal = match right_expr {
                    Expression::IntegerLiteral(t) => t,
                    _ => panic!("not integer literal"),
                };

                assert_eq!(int_literal.value, expected_right);
            }
        }
    }

    #[test]
    fn test_infix_expression() {
        let tests = vec![
            ("5 + 5;", 5, "+", 5),
            ("5 - 10;", 5, "-", 10),
            ("5 * 5;", 5, "*", 5),
            ("5 / 5;", 5, "/", 5),
            ("5 > 5;", 5, ">", 5),
            ("5 < 5;", 5, "<", 5),
            ("5 == 5;", 5, "==", 5),
            ("5 != 5;", 5, "!=", 5),
        ];

        for (input, expected_left, expected_op, expected_right) in tests {
            let lexer = lexer::Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();

            for error in parser.errors() {
                println!("ERROR: {}", error);
            }

            assert_eq!(parser.errors().len(), 0, "errors should be zero");

            assert_eq!(
                program.statements.len(),
                1,
                "program has not enough statements"
            );

            let stmt = program.statements.first().unwrap();

            let stmt = match stmt {
                Statement::ExpressionStatement(v) => v,
                _ => panic!("statement not ExpressionStatement"),
            };

            let expr = stmt.expression.as_ref().expect("expression not Some");

            let infix_expr = match expr {
                Expression::InfixExpression(v) => v,
                _ => panic!("expression not InfixExpression"),
            };

            // test integer literal
            if let Some(left_expr) = infix_expr.left.as_deref() {
                let int_literal = match left_expr {
                    Expression::IntegerLiteral(t) => t,
                    _ => panic!("not integer literal"),
                };

                assert_eq!(int_literal.value, expected_left);
            }

            assert_eq!(infix_expr.operator, expected_op);

            // test integer literal
            if let Some(right_expr) = infix_expr.right.as_deref() {
                let int_literal = match right_expr {
                    Expression::IntegerLiteral(t) => t,
                    _ => panic!("not integer literal"),
                };

                assert_eq!(int_literal.value, expected_right);
            }
        }
    }

    #[test]
    fn test_operator_precedence() {
        let tests = vec![
            ("-a * b", "((-a) * b)"),
            ("!-a", "(!(-a))"),
            ("a + b + c", "((a + b) + c)"),
            ("a + b - c", "((a + b) - c)"),
            ("a * b * c", "((a * b) * c)"),
            ("a * b / c", "((a * b) / c)"),
            ("a + b / c", "(a + (b / c))"),
            ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
            ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
            ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
            ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
        ];

        for (input, expected) in tests {
            let lexer = lexer::Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();

            for error in parser.errors() {
                println!("ERROR: {}", error);
            }

            assert_eq!(parser.errors().len(), 0, "errors should be zero");

            assert_eq!(program.to_str(), expected);
        }
    }
}
