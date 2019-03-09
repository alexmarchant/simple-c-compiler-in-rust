use parser::expression;
use parser::expression::Expression;
use lexer::Token;

#[derive(Debug)]
pub enum Statement {
    Return(Expression),
}

pub fn parse(tokens: &[Token]) -> Result<(Statement, &[Token]), String> {
    let statement: Statement;

    match tokens[0] {
        Token::KeywordReturn => {},
        _ => return Err("Expecting 'return'".to_string()),
    }

    let leftover_tokens: &[Token];
    match expression::parse(&tokens[1..]) {
        Ok((expression, tokens)) => {
            statement = Statement::Return(expression);
            leftover_tokens = tokens;
        },
        Err(err) => return Err(err),
    }

    match leftover_tokens[0] {
        Token::Semicolon => {},
        _ => return Err("Expecting ';'".to_string()),
    }

    return Ok((statement, &leftover_tokens[1..]));
}
