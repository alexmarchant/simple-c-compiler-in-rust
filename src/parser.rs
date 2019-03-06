use lexer::Token;

#[derive(Debug)]
pub struct Program {
    pub function: Function,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub statement: Statement,
}

#[derive(Debug)]
pub enum Statement {
    Return(Expression),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Constant(i64),
    UnaryOperation(Box<UnaryOperation>),
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Negation,
    BitwiseComplement,
    LogicalNegation,
}

#[derive(Debug, Clone)]
pub struct UnaryOperation {
    pub operator: UnaryOperator,
    pub expression: Expression,
}

#[derive(Debug)]
pub enum Error {
    FunctionInvalid,
    FunctionMissingReturnType,
    FunctionMissingName,
    FunctionMissingOpeningParen,
    FunctionMissingClosingParen,
    FunctionMissingOpeningBrace,
    FunctionMissingClosingBrace,
    StatementInvalid,
    StatementMissingSemicolon,
    ExpressionInvalid,
}

pub fn parse_program(tokens: &Vec<Token>) -> Result<Program, Error> {
    let mut p_tokens = tokens.clone();
    match parse_function(&mut p_tokens) {
        Ok(function) => {
            return Ok(Program { function: function });
        },
        Err(error) => return Err(error),
    }
}

pub fn parse_function(tokens: &mut Vec<Token>) -> Result<Function, Error> {
    let f_name: String;
    let f_statement: Statement;

    match shift(tokens) {
        Some(token) => {
            match token {
                Token::KeywordInt => {},
                _ => {
                    println!("{:?}", token);
                    return Err(Error::FunctionMissingReturnType)
                },
            }
        },
        None => return Err(Error::FunctionInvalid),
    }

    match shift(tokens) {
        Some(token) => {
            match token {
                Token::Identifier(name) => f_name = name,
                _ => return Err(Error::FunctionMissingName),
            }
        },
        None => return Err(Error::FunctionInvalid),
    }

    match shift(tokens) {
        Some(token) => {
            match token {
                Token::OpenParen => {},
                _ => return Err(Error::FunctionMissingOpeningParen),
            }
        },
        None => return Err(Error::FunctionInvalid),
    }

    match shift(tokens) {
        Some(token) => {
            match token {
                Token::CloseParen => {},
                _ => return Err(Error::FunctionMissingClosingParen),
            }
        },
        None => return Err(Error::FunctionInvalid),
    }

    match shift(tokens) {
        Some(token) => {
            match token {
                Token::OpenBrace => {},
                _ => return Err(Error::FunctionMissingOpeningBrace),
            }
        },
        None => return Err(Error::FunctionInvalid),
    }

    match parse_statement(tokens) {
        Ok(statement) => f_statement = statement,
        Err(error) => return Err(error),
    }

    match shift(tokens) {
        Some(token) => {
            match token {
                Token::CloseBrace => {},
                _ => return Err(Error::FunctionMissingClosingBrace),
            }
        },
        None => return Err(Error::FunctionInvalid),
    }

    return Ok(Function {
        name: f_name,
        statement: f_statement,
    });
}

pub fn parse_statement(tokens: &mut Vec<Token>) -> Result<Statement, Error> {
    let s_expression: Expression;

    match shift(tokens) {
        Some(token) => {
            match token {
                Token::KeywordReturn => {},
                _ => return Err(Error::StatementInvalid),
            }
        },
        None => return Err(Error::StatementInvalid),
    }

    match parse_expression(tokens) {
        Ok(expression) => s_expression = expression,
        Err(error) => return Err(error),
    }

    match shift(tokens) {
        Some(token) => {
            match token {
                Token::Semicolon => {},
                _ => return Err(Error::StatementMissingSemicolon),
            }
        },
        None => return Err(Error::StatementInvalid),
    }

    return Ok(Statement::Return(s_expression));
}

pub fn parse_expression(tokens: &mut Vec<Token>) -> Result<Expression, Error> {
    match shift(tokens) {
        Some(token) => {
            match token {
                Token::Negation | Token::BitwiseComplement | Token::LogicalNegation => {
                    let operator = operator_for_token(token).expect("Error converting token to operator");
                    let nested_expression: Expression;
                    match parse_expression(tokens) {
                        Ok(expression) => nested_expression = expression,
                        Err(error) => return Err(error),
                    }

                    let unary_operation = UnaryOperation {
                        operator: operator,
                        expression: nested_expression,
                    };

                    return Ok(Expression::UnaryOperation(Box::new(unary_operation)));
                }
                Token::IntegerLiteral(value) => return Ok(Expression::Constant(value)),
                _ => return Err(Error::ExpressionInvalid),
            }
        },
        None => return Err(Error::ExpressionInvalid),
    }
}

fn shift(tokens: &mut Vec<Token>) -> Option<Token> {
    if tokens.len() > 0 {
        return Some(tokens.remove(0));
    } else {
        return None;
    }
}

fn operator_for_token(token: Token) -> Option<UnaryOperator> {
    match token {
        Token::Negation => return Some(UnaryOperator::Negation),
        Token::BitwiseComplement => return Some(UnaryOperator::BitwiseComplement),
        Token::LogicalNegation => return Some(UnaryOperator::LogicalNegation),
        _ => return None,
    }
}
