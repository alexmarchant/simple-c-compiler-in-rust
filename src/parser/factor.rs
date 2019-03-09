use lexer::Token;
use parser::expression::Expression;
use parser::expression;
use parser::factor;

#[derive(Debug, Clone)]
pub enum Factor {
    Constant(i64),
    UnaryOperation(Box<UnaryOperation>),
    Expression(Box<Expression>)
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

pub fn parse(tokens: &[Token]) -> Result<(Factor, &[Token]), String> {
    match parse_expression_with_parens(tokens) {
        Ok((expression, leftover_tokens)) => {
            return Ok((Factor::Expression(Box::new(expression)), leftover_tokens));
        },
        Err(_) => (),
    }

    match parse_unary_operation(tokens) {
        Ok((operation, leftover_tokens)) => {
            return Ok((Factor::UnaryOperation(Box::new(operation)), leftover_tokens))
        },
        Err(_) => (),
    }

    match parse_integer_literal(tokens) {
        Ok((integer, leftover_tokens)) => {
            return Ok((Factor::Constant(integer), leftover_tokens))
        },
        Err(_) => (),
    }

    return Err("Invalid factor".to_string())
}

fn parse_expression_with_parens(tokens: &[Token]) -> Result<(Expression, &[Token]), String> {
    match tokens[0] {
        Token::OpenParen => (),
        _ => return Err("Expecting '('".to_string()),
    }

    let expression: Expression;
    let leftover_tokens: &[Token];
    match expression::parse(&tokens[1..]) {
        Ok((matched_expression, tokens)) => {
            expression = matched_expression;
            leftover_tokens = tokens;
        },
        Err(err) => return Err(err),
    }

    match leftover_tokens[0] {
        Token::CloseParen => (),
        _ => return Err("Expecting ')'".to_string()),
    }

    return Ok((expression, &leftover_tokens[1..]))
}

fn parse_unary_operation(tokens: &[Token]) -> Result<(UnaryOperation, &[Token]), String> {
    let operator: UnaryOperator;
    let factor: Factor;

    match unary_operator_for_token(&tokens[0]) {
        Some(matched_operator) => operator = matched_operator,
        None => return Err("Expecting ~ or ! or -".to_string()),
    }

    let leftover_tokens: &[Token];
    match factor::parse(&tokens[1..]) {
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

fn parse_integer_literal(tokens: &[Token]) -> Result<(i64, &[Token]), String> {
    match tokens[0] {
        Token::IntegerLiteral(value) => return Ok((value, &tokens[1..])),
        _ => return Err("Expecting integer literal".to_string()),
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
