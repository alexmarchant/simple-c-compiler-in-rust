use parser::statement;
use parser::statement::Statement;
use lexer::Token;

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub statement: Statement,
}

pub fn parse(tokens: Vec<Token>) -> Result<Function, &'static str> {
    let name: String;
    let statement: Statement;

    let mut token = tokens.get(0).expect("Expecting function return type");
    match token {
        Token::KeywordInt => {},
        _ => return Err("Expecting function return type"),
    }

    token = tokens.get(1).expect("Expecting identifier");
    match token {
        Token::Identifier(ref matched_name) => name = matched_name.clone(),
        _ => return Err("Expecting identifier"),
    }

    token = tokens.get(2).expect("Expecting '('");
    match token {
        Token::OpenParen => {},
        _ => return Err("Expecting '('"),
    }

    token = tokens.get(3).expect("Expecting ')'");
    match token {
        Token::CloseParen => {},
        _ => return Err("Expecting ')'"),
    }

    token = tokens.get(4).expect("Expecting '{'");
    match token {
        Token::OpenBrace => {},
        _ => return Err("Expecting '{'"),
    }

    let leftover_tokens: Vec<Token>;
    match statement::parse(tokens[5..].to_vec()) {
        Ok((matched_statement, tokens)) => {
            statement = matched_statement;
            leftover_tokens = tokens;
        },
        Err(err) => return Err(err),
    }

    match leftover_tokens[0] {
        Token::CloseBrace => {},
        _ => return Err("Expecting '}'"),
    }

    return Ok(Function {
        name: name,
        statement: statement,
    });
}

