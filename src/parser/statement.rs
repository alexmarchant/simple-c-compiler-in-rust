use lexer::Token;
use parser::expression;
use parser::StackFrame;
use parser::expression::Var;
use parser::expression::Expression;

#[derive(Debug)]
pub enum Statement {
    Return(Expression),
    Expression(Expression),
    VariableDeclaration(VariableDeclaration),
}

#[derive(Debug)]
pub struct VariableDeclaration {
    pub var: Var,
    pub expression: Option<Expression>,
}

pub fn parse(
    tokens: Vec<Token>,
    stack_frame: &mut StackFrame,
) -> Result<(Statement, Vec<Token>), String> {

    match parse_return(tokens.clone(), stack_frame) {
        Ok((expression, leftover_tokens)) => {
            return Ok((Statement::Return(expression), leftover_tokens))
        },
        Err(_) => (),
    }

    match parse_expression(tokens.clone(), stack_frame) {
        Ok((expression, leftover_tokens)) => {
            return Ok((Statement::Expression(expression), leftover_tokens))
        },
        Err(_) => (),
    }

    match parse_variable_declaration(tokens.clone(), stack_frame) {
        Ok((declaration, leftover_tokens)) => {
            match stack_frame.vars.get(&declaration.var.name) {
                Some(_) => {
                    return Err(format!(
                        "Variable '{}' has already been declared",
                        &declaration.var.name,
                    ));
                },
                None => {
                    stack_frame.add_var(declaration.var.name.clone());
                },
            }
            return Ok((Statement::VariableDeclaration(declaration), leftover_tokens))
        },
        Err(_) => (),
    }

    return Err("Invalid statement".to_string());
}

fn parse_return(
    tokens: Vec<Token>,
    stack_frame: &StackFrame,
) -> Result<(Expression, Vec<Token>), String> {
    match tokens.get(0) {
        Some(Token::KeywordReturn) => (),
        _ => return Err("Expecting 'return'".to_string()),
    }

    let (expression, leftover_tokens) = expression::parse(
        tokens[1..].to_vec(),
        stack_frame,
    )?;

    match leftover_tokens.get(0) {
        Some(Token::Semicolon) => {},
        _ => return Err("Expecting ';'".to_string()),
    }

    return Ok((expression, leftover_tokens[1..].to_vec()));
}

fn parse_expression(tokens: Vec<Token>, stack_frame: &StackFrame) -> Result<(Expression, Vec<Token>), String> {
    let (expression, leftover_tokens) = expression::parse(tokens, stack_frame)?;

    match leftover_tokens.get(0) {
        Some(Token::Semicolon) => {},
        _ => return Err("Expecting ';'".to_string()),
    }

    return Ok((expression, leftover_tokens[1..].to_vec()));
}

fn parse_variable_declaration(tokens: Vec<Token>, stack_frame: &StackFrame) -> Result<(VariableDeclaration, Vec<Token>), String> {
    let var: Var;
    let mut expression: Option<Expression> = None;

    match tokens.get(0) {
        Some(Token::KeywordInt) => {},
        _ => return Err("Expecting 'int'".to_string()),
    }

    match tokens.get(1) {
        Some(Token::Identifier(ref name)) => {
            var = Var { name: name.clone() }
        },
        _ => return Err("Expecting identifier".to_string()),
    }

    let leftover_tokens: Vec<Token>;
    match tokens.get(2) {
        Some(Token::Assignment) => {
            let (parsed_expression, tokens) = expression::parse(tokens[3..].to_vec(), stack_frame)?;
            expression = Some(parsed_expression);
            leftover_tokens = tokens;
        },
        _ => leftover_tokens = tokens[2..].to_vec(),
    }

    match leftover_tokens.get(0) {
        Some(Token::Semicolon) => {},
        _ => return Err("Expecting ';'".to_string()),
    }

    return Ok((
        VariableDeclaration { var: var, expression: expression },
        leftover_tokens[1..].to_vec(),
    ));
}
