use super::ast;
use super::lexer;
use super::lexer::TokenType;

pub struct Parser {
    tokens: Vec<lexer::Token>,
}

impl Parser {
    pub fn new() -> Self {
        Parser { tokens: Vec::new() }
    }

    fn not_eof(&mut self) -> bool {
        match self.tokens[0].token_type {
            lexer::TokenType::EOF => false,
            _ => true,
        }
    }

    fn at(&mut self) -> lexer::Token {
        self.tokens[0].clone()
    }

    fn eat(&mut self) -> lexer::Token {
        self.tokens.remove(0)
    }

    fn expect(&mut self, token_type: TokenType) -> Result<lexer::Token, lexer::TokenType> {
        let token = self.eat();
        if token.token_type == token_type {
            return Ok(token);
        }
        Err(token.token_type)
    }

    // fn expect_many(&mut self, types: Vec<TokenType>) -> Result<lexer::Token, lexer::TokenType> {
    //     let token = self.eat();
    //     if types.contains(&token.token_type) {
    //         return Ok(token);
    //     }
    //     Err(token.token_type)
    // }

    pub fn produce_ast(&mut self, source_code: &str) -> Result<ast::Statement, String> {
        match lexer::tokenise(source_code) {
            Ok(tokens) => self.tokens = tokens,
            Err(m) => return Err(m),
        }
        let mut program = ast::ProgramStatement::create();

        while self.not_eof() == true {
            match self.parse_statement() {
                Ok(statement) => program.body.push(statement),
                Err(m) => return Err(m),
            }
        }

        Ok(ast::Statement::Program(program))
    }

    fn parse_statement(&mut self) -> Result<ast::Statement, String> {
        match self.at().token_type {
            TokenType::Let | TokenType::Const => match self.parse_variable_declaration() {
                Ok(statement) => return Ok(statement),
                Err(m) => return Err(m),
            },
            _ => (),
        }
        match self.parse_expression() {
            Ok(expression) => Ok(ast::Statement::Expression(expression)),
            Err(m) => Err(m),
        }
    }

    fn parse_variable_declaration(&mut self) -> Result<ast::Statement, String> {
        let is_constant = self.eat().token_type == TokenType::Const;
        let identifier = match self.expect(TokenType::Identifier) {
            Ok(token) => token.value,
            Err(token_type) => {
                return Err(format!(
                    "Expected identifier name following the let | const keywords, got: {:#?}",
                    token_type
                ))
            }
        };

        let mut value_type: Option<ast::IdentifierExpression> = None;

        match self.at().token_type {
            TokenType::SemiColon => {
                self.eat();
                if is_constant {
                    return Err("Must assign a value to a constant expression".to_string());
                }

                return Ok(ast::Statement::VariableDeclaration(
                    ast::VariableDeclarationStatement::create(false, identifier, None, None),
                ));
            }
            TokenType::Colon => {
                self.eat();

                value_type = match self.expect(TokenType::Identifier) {
                    Ok(token) => match token.value.as_str() {
                        "i8" | "i16" | "i32" | "i64" | "i128" | "u8" | "u16" | "u32" | "u64" | "u128" | "f32" | "f64" | "char" | "str" | "bool" => Some(ast::IdentifierExpression::create(token.value)),
                        _ => return Err(format!("Unexpected value type found during variable declaration parsing, got: {}", token.value))
                    },
                    Err(token_type) => {
                        return Err(format!("Expected identifier name following the Semicolon (:) token, got: {:#?}", token_type));
                    }
                };
            }
            _ => (),
        };

        match self.expect(TokenType::Equals) {
            Ok(_) => (),
            Err(token_type) => {
                return Err(format!(
                "Expected equals token following identifier in variable declaration, got: {:#?}",
                token_type
            ))
            }
        };

        let value = match self.parse_expression() {
            Ok(expression) => expression,
            Err(m) => return Err(m),
        };

        match self.expect(TokenType::SemiColon) {
            Ok(_) => (),
            Err(token_type) => {
                return Err(format!(
                    "Variable declaration statements must end with a Semicolon, got: {:#?}",
                    token_type
                ))
            }
        };

        Ok(ast::Statement::VariableDeclaration(
            ast::VariableDeclarationStatement::create(
                is_constant,
                identifier,
                Some(value),
                value_type,
            ),
        ))
    }

    fn parse_expression(&mut self) -> Result<ast::Expression, String> {
        match self.parse_assignment_expression() {
            Ok(expression) => Ok(expression),
            Err(m) => return Err(m),
        }
    }

    fn parse_assignment_expression(&mut self) -> Result<ast::Expression, String> {
        let left = match self.parse_additive_expression() {
            Ok(expression) => expression,
            Err(m) => return Err(m),
        };

        if self.at().token_type == TokenType::Equals {
            self.eat();
            let value = match self.parse_additive_expression() {
                Ok(expression) => expression,
                Err(m) => return Err(m),
            };
            match self.expect(TokenType::SemiColon) {
                Ok(_) => (),
                Err(token_type) => {
                    return Err(format!(
                        "Assignment expressions must end with a Semicolon, got: {:#?}",
                        token_type
                    )
                    .to_string())
                }
            }

            return Ok(ast::Expression::VariableAssignment(Box::new(
                ast::VariableAssignmentExpression::create(left, value),
            )));
        }

        Ok(left)
    }

    fn parse_additive_expression(&mut self) -> Result<ast::Expression, String> {
        let mut left = match self.parse_multiplicative_expression() {
            Ok(expression) => expression,
            Err(m) => return Err(m),
        };

        while self.at().value == "+" || self.at().value == "-" {
            let operator = self.eat().value;
            let right = match self.parse_multiplicative_expression() {
                Ok(expression) => expression,
                Err(m) => return Err(m),
            };
            left = ast::Expression::Binary(Box::new(ast::BinaryExpression::create(
                left, right, operator,
            )))
        }

        Ok(left)
    }

    fn parse_multiplicative_expression(&mut self) -> Result<ast::Expression, String> {
        let mut left = match self.parse_primary_expression() {
            Ok(expression) => expression,
            Err(m) => return Err(m),
        };

        while self.at().value == "*" || self.at().value == "/" || self.at().value == "%" {
            let operator = self.eat().value;
            let right = match self.parse_primary_expression() {
                Ok(expression) => expression,
                Err(m) => return Err(m),
            };
            left = ast::Expression::Binary(Box::new(ast::BinaryExpression::create(
                left, right, operator,
            )))
        }

        Ok(left)
    }

    fn parse_primary_expression(&mut self) -> Result<ast::Expression, String> {
        let token_type = self.at().token_type;

        match token_type {
            lexer::TokenType::Identifier => Ok(ast::Expression::Identifier(
                ast::IdentifierExpression::create(self.eat().value),
            )),
            lexer::TokenType::Number => {
                let token = self.eat();
                if token.value.contains('.') {
                    return Ok(ast::Expression::Float(ast::FloatLiteral::create(
                        token.value,
                    )));
                }
                Ok(ast::Expression::Integer(ast::IntegerLiteral::create(
                    token.value,
                )))
            }
            lexer::TokenType::OpenParen => {
                self.eat();
                let value = match self.parse_expression() {
                    Ok(expression) => expression,
                    Err(m) => return Err(m),
                };
                match self.expect(TokenType::ClosedParen) {
                    Ok(_) => Ok(value),
                    Err(token_type) => Err(format!(
                        "Expected closing parenthesis, got: {:#?}",
                        token_type
                    )),
                }
            }
            _ => Err(format!(
                "Unexpected token found during parsing: {:#?}",
                self.at()
            )),
        }
    }
}
