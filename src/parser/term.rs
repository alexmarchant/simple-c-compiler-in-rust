use parser::factor;
use parser::factor::Factor;
use parser::factor::BinaryFactor;
use lexer::Token;

#[derive(Debug, Clone)]
pub struct Term {
    pub factor: factor::Factor,
    pub binary_factors: Vec<BinaryFactor>,
}


pub fn parse(tokens: Vec<Token>) -> Result<(Term, Vec<Token>), &'static str> {
    let factor: Factor;
    let mut binary_factors: Vec<BinaryFactor> = Vec::new();
    let mut leftover_tokens: Vec<Token>;

    match factor::parse(tokens) {
        Ok((matched_factor, tokens)) => {
            factor = matched_factor;
            leftover_tokens = tokens;
        },
        Err(err) => return Err(err),
    }

    while let Some(operator) = factor::binary_factor_operator_for_token(&leftover_tokens[0]) {
        match factor::parse(leftover_tokens[1..].to_vec()) {
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
