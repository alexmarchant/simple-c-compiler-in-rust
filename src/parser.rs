use lexer;

#[derive(Debug)]
pub struct Program {
    function: Function,
}

#[derive(Debug)]
pub struct Function {
    name: String,
    statement: Statement,
}

#[derive(Debug)]
pub enum Statement {
    Return(Expression),
}

#[derive(Debug)]
pub enum Expression {
    Constant(i64),
}

#[derive(Debug)]
pub enum Error {
    ParseError,
}

pub fn parse_program(tokens: &mut Vec<lexer::Token>) -> Result<Program, Error> {
    match parse_function(tokens) {
        Ok(function) => {
            return Ok(Program { function: function });
        },
        Err(error) => return Err(error),
    }
}

pub fn parse_function(tokens: &mut Vec<lexer::Token>) -> Result<Function, Error> {
    let f_name: String;
    let f_statement: Statement;

    match tokens.pop() {
        Some(token) => {
            match token {
                lexer::Token::KeywordInt => {},
                _ => return Err(Error::ParseError),
            }
        },
        None => return Err(Error::ParseError),
    }

    match tokens.pop() {
        Some(token) => {
            match token {
                lexer::Token::Identifier(name) => f_name = name,
                _ => return Err(Error::ParseError),
            }
        },
        None => return Err(Error::ParseError),
    }

    match tokens.pop() {
        Some(token) => {
            match token {
                lexer::Token::OpenParen => {},
                _ => return Err(Error::ParseError),
            }
        },
        None => return Err(Error::ParseError),
    }

    match tokens.pop() {
        Some(token) => {
            match token {
                lexer::Token::CloseParen => {},
                _ => return Err(Error::ParseError),
            }
        },
        None => return Err(Error::ParseError),
    }

    match tokens.pop() {
        Some(token) => {
            match token {
                lexer::Token::OpenBrace => {},
                _ => return Err(Error::ParseError),
            }
        },
        None => return Err(Error::ParseError),
    }

    match parse_statement(tokens) {
        Ok(statement) => f_statement = statement,
        Err(error) => return Err(error),
    }

    match tokens.pop() {
        Some(token) => {
            match token {
                lexer::Token::CloseBrace => {},
                _ => return Err(Error::ParseError),
            }
        },
        None => return Err(Error::ParseError),
    }

    return Ok(Function {
        name: f_name,
        statement: f_statement,
    });
}

pub fn parse_statement(tokens: &mut Vec<lexer::Token>) -> Result<Statement, Error> {
    let s_expression: Expression;

    match tokens.pop() {
        Some(token) => {
            match token {
                lexer::Token::KeywordReturn => {},
                _ => return Err(Error::ParseError),
            }
        },
        None => return Err(Error::ParseError),
    }

    match parse_expression(tokens) {
        Ok(expression) => s_expression = expression,
        Err(error) => return Err(error),
    }

    match tokens.pop() {
        Some(token) => {
            match token {
                lexer::Token::Semicolon => {},
                _ => return Err(Error::ParseError),
            }
        },
        None => return Err(Error::ParseError),
    }

    return Ok(Statement::Return(s_expression));
}

pub fn parse_expression(tokens: &mut Vec<lexer::Token>) -> Result<Expression, Error> {
    match tokens.pop() {
        Some(token) => {
            match token {
                lexer::Token::IntegerLiteral(value) => return Ok(Expression::Constant(value)),
                _ => return Err(Error::ParseError),
            }
        },
        None => return Err(Error::ParseError),
    }
}

