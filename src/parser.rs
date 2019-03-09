use lexer::Token;

#[derive(Debug)]
pub struct Program {
    pub function: Function,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub statement: Statement,
}

#[derive(Debug)]
pub enum Statement {
    Return(Expression),
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub term: Term,
    pub binary_terms: Vec<BinaryTerm>,
}

#[derive(Debug, Clone)]
pub struct Term {
    pub factor: Factor,
    pub binary_factors: Vec<BinaryFactor>,
}

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

#[derive(Debug, Clone)]
pub enum BinaryTermOperator {
    Subtraction,
    Addition,
}

#[derive(Debug, Clone)]
pub struct BinaryTerm {
    pub operator: BinaryTermOperator,
    pub right_term: Term,
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

pub fn parse_program(tokens: Vec<Token>) -> Result<Program, String> {
    match parse_function(&tokens[..]) {
        Ok(function) => return Ok(Program { function: function }),
        Err(err) => return Err(err),
    }
}

pub fn parse_function(tokens: &[Token]) -> Result<Function, String> {
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
    match parse_statement(&tokens[5..]) {
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

pub fn parse_statement(tokens: &[Token]) -> Result<(Statement, &[Token]), String> {
    let statement: Statement;

    match tokens[0] {
        Token::KeywordReturn => {},
        _ => return Err("Expecting 'return'".to_string()),
    }

    let leftover_tokens: &[Token];
    match parse_expression(&tokens[1..]) {
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

pub fn parse_expression(tokens: &[Token]) -> Result<(Expression, &[Token]), String> {
    let term: Term;
    let mut binary_terms: Vec<BinaryTerm> = Vec::new();
    let mut leftover_tokens: &[Token];

    match parse_term(tokens) {
        Ok((matched_term, tokens)) => {
            term = matched_term;
            leftover_tokens = tokens;
        },
        Err(err) => return Err(err),
    }

    while let Some(operator) = binary_term_operator_for_token(&leftover_tokens[0]) {
        match parse_term(&leftover_tokens[1..]) {
            Ok((matched_term, tokens)) => {
                binary_terms.push(BinaryTerm {
                    operator: operator,
                    right_term: matched_term,
                });
                leftover_tokens = tokens;
            },
            Err(err) => return Err(err),
        }
    }

    return Ok((
        Expression { term: term, binary_terms: binary_terms },
        leftover_tokens,
    ));
}

fn parse_term(tokens: &[Token]) -> Result<(Term, &[Token]), String> {
    let factor: Factor;
    let mut binary_factors: Vec<BinaryFactor> = Vec::new();
    let mut leftover_tokens: &[Token];

    match parse_factor(tokens) {
        Ok((matched_factor, tokens)) => {
            factor = matched_factor;
            leftover_tokens = tokens;
        },
        Err(err) => return Err(err),
    }

    while let Some(operator) = binary_factor_operator_for_token(&leftover_tokens[0]) {
        match parse_factor(&leftover_tokens[1..]) {
            Ok((matched_factor, tokens)) => {
                binary_factors.push(BinaryFactor {
                    operator: operator,
                    right_factor: matched_factor,
                });
                leftover_tokens = tokens;
            },
            Err(err) => return Err(err),
        }
    }

    return Ok((
        Term { factor: factor, binary_factors: binary_factors },
        leftover_tokens,
    ));
}

fn parse_factor(tokens: &[Token]) -> Result<(Factor, &[Token]), String> {
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
    match parse_expression(&tokens[1..]) {
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
    match parse_factor(&tokens[1..]) {
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

fn binary_term_operator_for_token(token: &Token) -> Option<BinaryTermOperator> {
    match token {
        Token::MinusSign => return Some(BinaryTermOperator::Subtraction),
        Token::PlusSign => return Some(BinaryTermOperator::Addition),
        _ => return None,
    }
}

fn binary_factor_operator_for_token(token: &Token) -> Option<BinaryFactorOperator> {
    match token {
        Token::MultiplicationSign => return Some(BinaryFactorOperator::Multiplication),
        Token::DivisionSign => return Some(BinaryFactorOperator::Division),
        _ => return None,
    }
}
