use parser::term;
use parser::term::Term;
use lexer::Token;

#[derive(Debug, Clone)]
pub struct Expression {
    pub term: Term,
    pub binary_terms: Vec<BinaryTerm>,
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

pub fn parse(tokens: &[Token]) -> Result<(Expression, &[Token]), String> {
    let term: Term;
    let mut binary_terms: Vec<BinaryTerm> = Vec::new();
    let mut leftover_tokens: &[Token];

    match term::parse(tokens) {
        Ok((matched_term, tokens)) => {
            term = matched_term;
            leftover_tokens = tokens;
        },
        Err(err) => return Err(err),
    }

    while let Some(operator) = binary_term_operator_for_token(&leftover_tokens[0]) {
        match term::parse(&leftover_tokens[1..]) {
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

fn binary_term_operator_for_token(token: &Token) -> Option<BinaryTermOperator> {
    match token {
        Token::MinusSign => return Some(BinaryTermOperator::Subtraction),
        Token::PlusSign => return Some(BinaryTermOperator::Addition),
        _ => return None,
    }
}
