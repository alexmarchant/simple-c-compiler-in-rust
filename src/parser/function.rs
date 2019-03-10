use lexer::Token;
use parser::statement;
use parser::StackFrame;
use parser::statement::Statement;

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub statements: Vec<Statement>,
    pub stack_frame: StackFrame,
}

pub fn parse(tokens: Vec<Token>) -> Result<Function, String> {
    let name: String;
    let mut statements: Vec<Statement> = Vec::new();
    let mut leftover_tokens: Vec<Token> = tokens.clone();
    let mut stack_frame: StackFrame = Default::default();

    match leftover_tokens.get(0) {
        Some(Token::KeywordInt) => (),
        _ => return Err("Expecting function return type".to_string()),
    }

    match leftover_tokens.get(1) {
        Some(Token::Identifier(ref matched_name)) => name = matched_name.clone(),
        _ => return Err("Expecting identifier".to_string()),
    }

    match leftover_tokens.get(2) {
        Some(Token::OpenParen) => (),
        _ => return Err("Expecting '('".to_string()),
    }

    match leftover_tokens.get(3) {
        Some(Token::CloseParen) => (),
        _ => return Err("Expecting ')'".to_string()),
    }

    match leftover_tokens.get(4) {
        Some(Token::OpenBrace) => {},
        _ => return Err("Expecting '{'".to_string()),
    }

    leftover_tokens = leftover_tokens[5..].to_vec();
    loop {
        match leftover_tokens.get(0) {
            Some(Token::CloseBrace) => break,
            _ => (),
        }

        let (statement, tokens) = statement::parse(
            leftover_tokens.clone(),
            &mut stack_frame,
        )?;

        let mut is_return: bool = false;
        match statement {
            Statement::Return(_) => is_return = true,
            _ => (),
        }

        statements.push(statement);
        leftover_tokens = tokens;

        if is_return {
            break;
        }
    }

    return Ok(Function {
        name: name,
        statements: statements,
        stack_frame: stack_frame,
    });
}
