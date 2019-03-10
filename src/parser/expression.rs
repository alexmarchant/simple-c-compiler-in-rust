use parser::term;
use parser::term::Term;
use lexer::Token;

#[derive(Debug, Clone)]
pub struct Expression {
    pub expression: LogicalAndExpression,
    pub binary_expressions: Vec<BinaryLogicalAndExpression>,
}

#[derive(Debug, Clone)]
pub struct BinaryLogicalAndExpression {
    pub operator: ExpressionOperator,
    pub right_expression: LogicalAndExpression,
}

#[derive(Debug, Clone)]
pub enum ExpressionOperator {
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

pub fn parse(tokens: Vec<Token>) -> Result<(Expression, Vec<Token>), &'static str> {
    let expression: LogicalAndExpression;
    let mut binary_expressions: Vec<BinaryLogicalAndExpression> = Vec::new();
    let mut leftover_tokens: Vec<Token>;

    match parse_logical_and(tokens) {
        Ok((matched_expression, tokens)) => {
            expression = matched_expression;
            leftover_tokens = tokens;
        },
        Err(err) => return Err(err),
    }

    while let Some(operator) = expression_operator_for_token(&leftover_tokens[0]) {
        match parse_logical_and(leftover_tokens[1..].to_vec()) {
            Ok((matched_expression, tokens)) => {
                binary_expressions.push(BinaryLogicalAndExpression {
                    operator: operator,
                    right_expression: matched_expression,
                });
                leftover_tokens = tokens;
            },
            Err(err) => return Err(err),
        }
    }

    return Ok((
        Expression {
            expression: expression,
            binary_expressions: binary_expressions
        },
        leftover_tokens,
    ));
}

pub fn parse_with_parens(tokens: Vec<Token>) -> Result<(Expression, Vec<Token>), &'static str> {
    match tokens[0] {
        Token::OpenParen => (),
        _ => return Err("Expecting '('"),
    }

    let expression: Expression;
    let leftover_tokens: Vec<Token>;
    match parse(tokens[1..].to_vec()) {
        Ok((matched_expression, tokens)) => {
            expression = matched_expression;
            leftover_tokens = tokens;
        },
        Err(err) => return Err(err),
    }

    match leftover_tokens[0] {
        Token::CloseParen => (),
        _ => return Err("Expecting ')'"),
    }

    return Ok((expression, leftover_tokens[1..].to_vec()))
}

fn parse_logical_and(tokens: Vec<Token>) -> Result<(LogicalAndExpression, Vec<Token>), &'static str> {
    let expression: EqualityExpression;
    let mut binary_expressions: Vec<BinaryEqualityExpression> = Vec::new();
    let mut leftover_tokens: Vec<Token>;

    match parse_equality(tokens) {
        Ok((matched_expression, tokens)) => {
            expression = matched_expression;
            leftover_tokens = tokens;
        },
        Err(err) => return Err(err),
    }

    while let Some(operator) = logical_and_operator_for_token(&leftover_tokens[0]) {
        match parse_equality(leftover_tokens[1..].to_vec()) {
            Ok((matched_expression, tokens)) => {
                binary_expressions.push(BinaryEqualityExpression {
                    operator: operator,
                    right_expression: matched_expression,
                });
                leftover_tokens = tokens;
            },
            Err(err) => return Err(err),
        }
    }

    return Ok((
        LogicalAndExpression {
            expression: expression,
            binary_expressions: binary_expressions
        },
        leftover_tokens,
    ));
}

fn parse_equality(tokens: Vec<Token>) -> Result<(EqualityExpression, Vec<Token>), &'static str> {
    let expression: RelationalExpression;
    let mut binary_expressions: Vec<BinaryRelationalExpression> = Vec::new();
    let mut leftover_tokens: Vec<Token>;

    match parse_relational(tokens) {
        Ok((matched_expression, tokens)) => {
            expression = matched_expression;
            leftover_tokens = tokens;
        },
        Err(err) => return Err(err),
    }

    while let Some(operator) = equality_operator_for_token(&leftover_tokens[0]) {
        match parse_relational(leftover_tokens[1..].to_vec()) {
            Ok((matched_expression, tokens)) => {
                binary_expressions.push(BinaryRelationalExpression {
                    operator: operator,
                    right_expression: matched_expression,
                });
                leftover_tokens = tokens;
            },
            Err(err) => return Err(err),
        }
    }

    return Ok((
        EqualityExpression {
            expression: expression,
            binary_expressions: binary_expressions
        },
        leftover_tokens,
    ));
}

fn parse_relational(tokens: Vec<Token>) -> Result<(RelationalExpression, Vec<Token>), &'static str> {
    let expression: AdditiveExpression;
    let mut binary_expressions: Vec<BinaryAdditiveExpression> = Vec::new();
    let mut leftover_tokens: Vec<Token>;

    match parse_additive(tokens) {
        Ok((matched_expression, tokens)) => {
            expression = matched_expression;
            leftover_tokens = tokens;
        },
        Err(err) => return Err(err),
    }

    while let Some(operator) = relational_operator_for_token(&leftover_tokens[0]) {
        match parse_additive(leftover_tokens[1..].to_vec()) {
            Ok((matched_expression, tokens)) => {
                binary_expressions.push(BinaryAdditiveExpression {
                    operator: operator,
                    right_expression: matched_expression,
                });
                leftover_tokens = tokens;
            },
            Err(err) => return Err(err),
        }
    }

    return Ok((
        RelationalExpression {
            expression: expression,
            binary_expressions: binary_expressions
        },
        leftover_tokens,
    ));
}

fn parse_additive(tokens: Vec<Token>) -> Result<(AdditiveExpression, Vec<Token>), &'static str> {
    let term: Term;
    let mut binary_terms: Vec<BinaryTerms> = Vec::new();
    let mut leftover_tokens: Vec<Token>;

    match term::parse(tokens) {
        Ok((matched_term, tokens)) => {
            term = matched_term;
            leftover_tokens = tokens;
        },
        Err(err) => return Err(err),
    }

    while let Some(operator) = additive_operator_for_token(&leftover_tokens[0]) {
        match term::parse(leftover_tokens[1..].to_vec()) {
            Ok((matched_term, tokens)) => {
                binary_terms.push(BinaryTerms {
                    operator: operator,
                    right_term: matched_term,
                });
                leftover_tokens = tokens;
            },
            Err(err) => return Err(err),
        }
    }

    return Ok((
        AdditiveExpression { term: term, binary_terms: binary_terms },
        leftover_tokens,
    ));
}

fn expression_operator_for_token(token: &Token) -> Option<ExpressionOperator> {
    match token {
        Token::Or => return Some(ExpressionOperator::Or),
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
