use parser::statement;
use parser::statement::Statement;
use lexer::Token;

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub statement: Statement,
}

pub fn parse(tokens: &[Token]) -> Result<Function, String> {
    let name: String;
    let statement: Statement;

    match tokens[0] {
        Token::KeywordInt => {},
        _ => return Err("Expecting function return type".to_string()),
    }

    match tokens[1] {
        Token::Identifier(ref matched_name) => name = matched_name.clone(),
        _ => return Err("Expecting identifier".to_string()),
    }

    match tokens[2] {
        Token::OpenParen => {},
        _ => return Err("Expecting '('".to_string()),
    }

    match tokens[3] {
        Token::CloseParen => {},
        _ => return Err("Expecting ')'".to_string()),
    }

    match tokens[4] {
        Token::OpenBrace => {},
        _ => return Err("Expecting '{'".to_string()),
    }

    let leftover_tokens: &[Token];
    match statement::parse(&tokens[5..]) {
        Ok((matched_statement, tokens)) => {
            statement = matched_statement;
            leftover_tokens = tokens;
        },
        Err(err) => return Err(err),
    }

    match leftover_tokens[0] {
        Token::CloseBrace => {},
        _ => return Err("Expecting '}'".to_string()),
    }

    return Ok(Function {
        name: name,
        statement: statement,
    });
}

