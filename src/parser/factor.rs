use lexer::Token;
use parser::expression;
use parser::expression::Expression;
use parser::factor;

#[derive(Debug, Clone)]
pub enum Factor {
    Expression(Box<Expression>),
    UnaryOperation(Box<UnaryOperation>),
    Constant(i64),
    Identifier(String),
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
    pub factor: Factor,
}

#[derive(Debug, Clone)]
pub enum BinaryFactorOperator {
    Multiplication,
    Division,
}

#[derive(Debug, Clone)]
pub struct BinaryFactor {
    pub operator: BinaryFactorOperator,
    pub right_factor: Factor,
}

pub fn parse(tokens: Vec<Token>) -> Result<(Factor, Vec<Token>), &'static str> {
    match expression::parse_with_parens(tokens.clone()) {
        Ok((expression, leftover_tokens)) => {
            return Ok((Factor::Expression(Box::new(expression)), leftover_tokens));
        },
        Err(_) => (),
    }

    match parse_unary_operation(tokens.clone()) {
        Ok((operation, leftover_tokens)) => {
            return Ok((Factor::UnaryOperation(Box::new(operation)), leftover_tokens))
        },
        Err(_) => (),
    }

    match parse_integer_literal(tokens.clone()) {
        Ok((integer, leftover_tokens)) => {
            return Ok((Factor::Constant(integer), leftover_tokens))
        },
        Err(_) => (),
    }

    match parse_identifier(tokens.clone()) {
        Ok((name, leftover_tokens)) => {
            return Ok((Factor::Identifier(name), leftover_tokens))
        },
        Err(_) => (),
    }

    return Err("Invalid factor")
}

pub fn binary_factor_operator_for_token(token: &Token) -> Option<BinaryFactorOperator> {
    match token {
        Token::MultiplicationSign => return Some(BinaryFactorOperator::Multiplication),
        Token::DivisionSign => return Some(BinaryFactorOperator::Division),
        _ => return None,
    }
}

fn parse_unary_operation(tokens: Vec<Token>) -> Result<(UnaryOperation, Vec<Token>), &'static str> {
    let operator: UnaryOperator;
    let factor: Factor;

    match unary_operator_for_token(&tokens[0]) {
        Some(matched_operator) => operator = matched_operator,
        None => return Err("Expecting ~ or ! or -"),
    }

    let leftover_tokens: Vec<Token>;
    match factor::parse(tokens[1..].to_vec()) {
        Ok((matched_factor, tokens)) => {
            factor = matched_factor;
            leftover_tokens = tokens; },
        Err(err) => return Err(err),
    }

    return Ok((
        UnaryOperation { operator: operator, factor: factor },
        leftover_tokens
    ));
}

fn parse_integer_literal(tokens: Vec<Token>) -> Result<(i64, Vec<Token>), &'static str> {
    match tokens[0] {
        Token::IntegerLiteral(value) => return Ok((value, tokens[1..].to_vec())),
        _ => return Err("Expecting integer literal"),
    }
}

fn parse_identifier(tokens: Vec<Token>) -> Result<(String, Vec<Token>), &'static str> {
    match tokens[0] {
        Token::Identifier(ref name) => return Ok((name.clone(), tokens[1..].to_vec())),
        _ => return Err("Expecting identifier"),
    }
}

fn unary_operator_for_token(token: &Token) -> Option<UnaryOperator> {
    match token {
        Token::MinusSign => return Some(UnaryOperator::Negation),
        Token::BitwiseComplement => return Some(UnaryOperator::BitwiseComplement),
        Token::LogicalNegation => return Some(UnaryOperator::LogicalNegation),
        _ => return None,
    }
}
