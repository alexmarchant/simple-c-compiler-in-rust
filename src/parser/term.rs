use parser::factor;
use parser::factor::Factor;
use lexer::Token;

#[derive(Debug, Clone)]
pub struct Term {
    pub factor: factor::Factor,
    pub binary_factors: Vec<BinaryFactor>,
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

pub fn parse(tokens: &[Token]) -> Result<(Term, &[Token]), String> {
    let factor: Factor;
    let mut binary_factors: Vec<BinaryFactor> = Vec::new();
    let mut leftover_tokens: &[Token];

    match factor::parse(tokens) {
        Ok((matched_factor, tokens)) => {
            factor = matched_factor;
            leftover_tokens = tokens;
        },
        Err(err) => return Err(err),
    }

    while let Some(operator) = binary_factor_operator_for_token(&leftover_tokens[0]) {
        match factor::parse(&leftover_tokens[1..]) {
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


fn binary_factor_operator_for_token(token: &Token) -> Option<BinaryFactorOperator> {
    match token {
        Token::MultiplicationSign => return Some(BinaryFactorOperator::Multiplication),
        Token::DivisionSign => return Some(BinaryFactorOperator::Division),
        _ => return None,
    }
}
