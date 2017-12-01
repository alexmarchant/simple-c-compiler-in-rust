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

pub fn parse_program(tokens: &Vec<lexer::Token>) -> Result<Program, &str> {
    let mut p_tokens = tokens.clone();
    match parse_function(&mut p_tokens) {
        Ok(function) => {
            return Ok(Program { function: function });
        },
        Err(error) => return Err(error),
    }
}

pub fn parse_function(tokens: &mut Vec<lexer::Token>) -> Result<Function, &str> {
    let f_name: String;
    let f_statement: Statement;

    match tokens.pop() {
        Some(token) => {
            match token {
                lexer::Token::KeywordInt => {},
                _ => return Err("Function missing return type"),
            }
        },
        None => return Err("Function is invalid"),
    }

    match tokens.pop() {
        Some(token) => {
            match token {
                lexer::Token::Identifier(name) => f_name = name,
                _ => return Err("Missing function name"),
            }
        },
        None => return Err("Function is invalid"),
    }

    match tokens.pop() {
        Some(token) => {
            match token {
                lexer::Token::OpenParen => {},
                _ => return Err("Function missing opening paren"),
            }
        },
        None => return Err("Function is invalid"),
    }

    match tokens.pop() {
        Some(token) => {
            match token {
                lexer::Token::CloseParen => {},
                _ => return Err("Function missing closing paren"),
            }
        },
        None => return Err("Function is invalid"),
    }

    match tokens.pop() {
        Some(token) => {
            match token {
                lexer::Token::OpenBrace => {},
                _ => return Err("Function missing opening brace"),
            }
        },
        None => return Err("Function is invalid"),
    }

    match parse_statement(&mut tokens) {
        Ok(statement) => f_statement = statement,
        Err(error) => return Err(error),
    }

    match tokens.pop() {
        Some(token) => {
            match token {
                lexer::Token::CloseBrace => {},
                _ => return Err("Function missing closing brace"),
            }
        },
        None => return Err("Function is invalid"),
    }

    return Ok(Function {
        name: f_name,
        statement: f_statement,
    });
}

pub fn parse_statement<'a>(tokens: &'a mut Vec<lexer::Token>) -> Result<Statement, &str> {
    let s_expression: Expression;

    match tokens.pop() {
        Some(token) => {
            match token {
                lexer::Token::KeywordReturn => {},
                _ => return Err("Statement is invalid"),
            }
        },
        None => return Err("Statement is invalid"),
    }

    match parse_expression(&mut tokens) {
        Ok(expression) => s_expression = expression,
        Err(error) => return Err(error),
    }

    match tokens.pop() {
        Some(token) => {
            match token {
                lexer::Token::Semicolon => {},
                _ => return Err("Statement missing semicolon"),
            }
        },
        None => return Err("Statement is invalid"),
    }

    return Ok(Statement::Return(s_expression));
}

pub fn parse_expression(tokens: &mut Vec<lexer::Token>) -> Result<Expression, &str> {
    match tokens.pop() {
        Some(token) => {
            match token {
                lexer::Token::IntegerLiteral(value) => return Ok(Expression::Constant(value)),
                _ => return Err("Expression is invalid"),
            }
        },
        None => return Err("Expression is invalid"),
    }
}

