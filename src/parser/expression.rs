use parser::term;
use parser::StackFrame;
use parser::term::Term;
use lexer::Token;

#[derive(Debug, Clone)]
pub enum Expression {
    Assignment(Box<Assignment>),
    LogicalOrExpression(LogicalOrExpression),
}

#[derive(Debug, Clone)]
pub struct LogicalOrExpression {
    pub expression: LogicalAndExpression,
    pub binary_expressions: Vec<BinaryLogicalAndExpression>,
}

#[derive(Debug, Clone)]
pub struct BinaryLogicalAndExpression {
    pub operator: LogicalOrOperator,
    pub right_expression: LogicalAndExpression,
}

#[derive(Debug, Clone)]
pub enum LogicalOrOperator {
    Or,
}

#[derive(Debug, Clone)]
pub struct LogicalAndExpression {
    pub expression: EqualityExpression,
    pub binary_expressions: Vec<BinaryEqualityExpression>,
}

#[derive(Debug, Clone)]
pub struct BinaryEqualityExpression {
    pub operator: LogicalAndOperator,
    pub right_expression: EqualityExpression,
}

#[derive(Debug, Clone)]
pub enum LogicalAndOperator {
    And,
}

#[derive(Debug, Clone)]
pub struct EqualityExpression {
    pub expression: RelationalExpression,
    pub binary_expressions: Vec<BinaryRelationalExpression>,
}

#[derive(Debug, Clone)]
pub struct BinaryRelationalExpression {
    pub operator: EqualityOperator,
    pub right_expression: RelationalExpression,
}

#[derive(Debug, Clone)]
pub enum EqualityOperator {
    Equal,
    NotEqual,
}

#[derive(Debug, Clone)]
pub struct RelationalExpression {
    pub expression: AdditiveExpression,
    pub binary_expressions: Vec<BinaryAdditiveExpression>,
}

#[derive(Debug, Clone)]
pub struct BinaryAdditiveExpression {
    pub operator: RelationalOperator,
    pub right_expression: AdditiveExpression,
}

#[derive(Debug, Clone)]
pub enum RelationalOperator {
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

#[derive(Debug, Clone)]
pub struct AdditiveExpression {
    pub term: Term,
    pub binary_terms: Vec<BinaryTerms>,
}

#[derive(Debug, Clone)]
pub struct BinaryTerms {
    pub operator: AdditiveOperator,
    pub right_term: Term,
}


#[derive(Debug, Clone)]
pub enum AdditiveOperator {
    Subtraction,
    Addition,
}

#[derive(Debug, Clone)]
pub struct Var {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub var: Var,
    pub expression: Expression,
}

pub fn parse(
    tokens: Vec<Token>,
    stack_frame: &StackFrame,
) -> Result<(Expression, Vec<Token>), String> {
    match parse_assignment(tokens.clone(), stack_frame) {
        Ok((assignment, leftover_tokens)) => {
            match stack_frame.vars.get(&assignment.var.name) {
                None => return Err(format!("Var '{}' hasn't been declared", assignment.var.name)),
                _ => (),
            }

            return Ok((
                Expression::Assignment(Box::new(assignment)),
                leftover_tokens
            ));
        },
        Err(_) => (),
    }

    let (expression, leftover_tokens) = parse_logical_or(tokens.clone(), stack_frame)?;
    return Ok((
        Expression::LogicalOrExpression(expression),
        leftover_tokens,
    ));
}

pub fn parse_with_parens(tokens: Vec<Token>, stack_frame: &StackFrame) -> Result<(Expression, Vec<Token>), String> {
    match tokens.get(0) {
        Some(Token::OpenParen) => (),
        _ => return Err("Expecting '('".to_string()),
    }

    let expression: Expression;
    let leftover_tokens: Vec<Token>;
    let (matched_expression, tokens) = parse(tokens[1..].to_vec(), stack_frame)?;
    expression = matched_expression;
    leftover_tokens = tokens;

    match leftover_tokens[0] {
        Token::CloseParen => (),
        _ => return Err("Expecting ')'".to_string()),
    }

    return Ok((expression, leftover_tokens[1..].to_vec()))
}

pub fn parse_logical_or(tokens: Vec<Token>, stack_frame: &StackFrame) -> Result<(LogicalOrExpression, Vec<Token>), String> {
    let expression: LogicalAndExpression;
    let mut binary_expressions: Vec<BinaryLogicalAndExpression> = Vec::new();
    let mut leftover_tokens: Vec<Token>;

    let (matched_expression, tokens) = parse_logical_and(tokens, stack_frame)?;
    expression = matched_expression;
    leftover_tokens = tokens;

    while let Some(operator) = expression_operator_for_token(&leftover_tokens[0]) {
        let (matched_expression, tokens) = parse_logical_and(leftover_tokens[1..].to_vec(), stack_frame)?;
        binary_expressions.push(BinaryLogicalAndExpression {
            operator: operator,
            right_expression: matched_expression,
        });
        leftover_tokens = tokens;
    }

    return Ok((
        LogicalOrExpression {
            expression: expression,
            binary_expressions: binary_expressions
        },
        leftover_tokens,
    ));
}

fn parse_logical_and(tokens: Vec<Token>, stack_frame: &StackFrame) -> Result<(LogicalAndExpression, Vec<Token>), String> {
    let mut binary_expressions: Vec<BinaryEqualityExpression> = Vec::new();
    let mut leftover_tokens: Vec<Token>;

    let (expression, tokens) = parse_equality(tokens, stack_frame)?;
    leftover_tokens = tokens;

    while let Some(operator) = logical_and_operator_for_token(&leftover_tokens[0]) {
        let (matched_expression, tokens) = parse_equality(leftover_tokens[1..].to_vec(), stack_frame)?;
        binary_expressions.push(BinaryEqualityExpression {
            operator: operator,
            right_expression: matched_expression,
        });
        leftover_tokens = tokens;
    }

    return Ok((
        LogicalAndExpression {
            expression: expression,
            binary_expressions: binary_expressions
        },
        leftover_tokens,
    ));
}

fn parse_equality(tokens: Vec<Token>, stack_frame: &StackFrame) -> Result<(EqualityExpression, Vec<Token>), String> {
    let mut binary_expressions: Vec<BinaryRelationalExpression> = Vec::new();
    let mut leftover_tokens: Vec<Token>;

    let (expression, tokens) = parse_relational(tokens, stack_frame)?;
    leftover_tokens = tokens;

    while let Some(operator) = equality_operator_for_token(&leftover_tokens[0]) {
        let (matched_expression, tokens) = parse_relational(leftover_tokens[1..].to_vec(), stack_frame)?;
        binary_expressions.push(BinaryRelationalExpression {
            operator: operator,
            right_expression: matched_expression,
        });
        leftover_tokens = tokens;
    }

    return Ok((
        EqualityExpression {
            expression: expression,
            binary_expressions: binary_expressions
        },
        leftover_tokens,
    ));
}

fn parse_relational(tokens: Vec<Token>, stack_frame: &StackFrame) -> Result<(RelationalExpression, Vec<Token>), String> {
    let mut binary_expressions: Vec<BinaryAdditiveExpression> = Vec::new();
    let mut leftover_tokens: Vec<Token>;

    let (expression, tokens) = parse_additive(tokens, stack_frame)?;
    leftover_tokens = tokens;

    while let Some(operator) = relational_operator_for_token(&leftover_tokens[0]) {
        let (matched_expression, tokens) = parse_additive(leftover_tokens[1..].to_vec(), stack_frame)?;
        binary_expressions.push(BinaryAdditiveExpression {
            operator: operator,
            right_expression: matched_expression,
        });
        leftover_tokens = tokens;
    }

    return Ok((
        RelationalExpression {
            expression: expression,
            binary_expressions: binary_expressions
        },
        leftover_tokens,
    ));
}

fn parse_additive(tokens: Vec<Token>, stack_frame: &StackFrame) -> Result<(AdditiveExpression, Vec<Token>), String> {
    let mut binary_terms: Vec<BinaryTerms> = Vec::new();
    let mut leftover_tokens: Vec<Token>;

    let (term, tokens) = term::parse(tokens, stack_frame)?;
    leftover_tokens = tokens;

    while let Some(operator) = additive_operator_for_token(&leftover_tokens[0]) {
        let (matched_term, tokens) = term::parse(leftover_tokens[1..].to_vec(), stack_frame)?;
        binary_terms.push(BinaryTerms {
            operator: operator,
            right_term: matched_term,
        });
        leftover_tokens = tokens;
    }

    return Ok((
        AdditiveExpression { term: term, binary_terms: binary_terms },
        leftover_tokens,
    ));
}

fn parse_assignment(tokens: Vec<Token>, stack_frame: &StackFrame) -> Result<(Assignment, Vec<Token>), String> {
    let var: Var;

    let mut leftover_tokens: Vec<Token>;
    match tokens.get(0) {
        Some(Token::Identifier(ref name)) => {
            var = Var { name: name.clone() };
            leftover_tokens = tokens[1..].to_vec();
        },
        _ => return Err("Invalid assignment: Expecting identifier".to_string()),
    }

    match leftover_tokens.get(0) {
        Some(Token::Assignment) => {
            leftover_tokens = leftover_tokens[1..].to_vec();
        },
        _ => return Err("Invalid assignment: Expecting '='".to_string()),
    }

    let (expression, leftover_tokens) = parse(leftover_tokens, stack_frame)?;
    return Ok((
        Assignment { var: var, expression: expression },
        leftover_tokens,
    ));
}

fn expression_operator_for_token(token: &Token) -> Option<LogicalOrOperator> {
    match token {
        Token::Or => return Some(LogicalOrOperator::Or),
        _ => return None,
    }
}

fn logical_and_operator_for_token(token: &Token) -> Option<LogicalAndOperator> {
    match token {
        Token::And => return Some(LogicalAndOperator::And),
        _ => return None,
    }
}

fn equality_operator_for_token(token: &Token) -> Option<EqualityOperator> {
    match token {
        Token::Equal => return Some(EqualityOperator::Equal),
        Token::NotEqual => return Some(EqualityOperator::NotEqual),
        _ => return None,
    }
}

fn relational_operator_for_token(token: &Token) -> Option<RelationalOperator> {
    match token {
        Token::LessThan => return Some(RelationalOperator::LessThan),
        Token::LessThanOrEqual => return Some(RelationalOperator::LessThanOrEqual),
        Token::GreaterThan => return Some(RelationalOperator::GreaterThan),
        Token::GreaterThanOrEqual => return Some(RelationalOperator::GreaterThanOrEqual),
        _ => return None,
    }
}

fn additive_operator_for_token(token: &Token) -> Option<AdditiveOperator> {
    match token {
        Token::MinusSign => return Some(AdditiveOperator::Subtraction),
        Token::PlusSign => return Some(AdditiveOperator::Addition),
        _ => return None,
    }
}
