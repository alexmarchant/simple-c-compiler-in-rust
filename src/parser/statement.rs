use parser::expression;
use parser::expression::Expression;
use lexer::Token;

#[derive(Debug)]
pub enum Statement {
    Return(Expression),
}

pub fn parse(tokens: Vec<Token>) -> Result<(Statement, Vec<Token>), &'static str> {
    let statement: Statement;

    match tokens[0] {
        Token::KeywordReturn => {},
        _ => return Err("Expecting 'return'"),
    }

    let leftover_tokens: Vec<Token>;
    match expression::parse(tokens[1..].to_vec()) {
        Ok((expression, tokens)) => {
            statement = Statement::Return(expression);
            leftover_tokens = tokens;
        },
        Err(err) => return Err(err),
    }

    match leftover_tokens[0] {
        Token::Semicolon => {},
        _ => return Err("Expecting ';'"),
    }

    return Ok((statement, leftover_tokens[1..].to_vec()));
}
