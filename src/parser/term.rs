use parser::factor;
use parser::StackFrame;
use parser::factor::Factor;
use parser::factor::BinaryFactor;
use lexer::Token;

#[derive(Debug, Clone)]
pub struct Term {
    pub factor: factor::Factor,
    pub binary_factors: Vec<BinaryFactor>,
}


pub fn parse(tokens: Vec<Token>, stack_frame: &StackFrame) -> Result<(Term, Vec<Token>), String> {
    let factor: Factor;
    let mut binary_factors: Vec<BinaryFactor> = Vec::new();
    let mut leftover_tokens: Vec<Token>;

    let (matched_factor, tokens) = factor::parse(tokens, stack_frame)?;
    factor = matched_factor;
    leftover_tokens = tokens;

    while let Some(operator) = factor::binary_factor_operator_for_token(&leftover_tokens[0]) {
        let (matched_factor, tokens) = factor::parse(leftover_tokens[1..].to_vec(), stack_frame)?;
        binary_factors.push(BinaryFactor {
            operator: operator,
            right_factor: matched_factor,
        });
        leftover_tokens = tokens;
    }

    return Ok((
        Term { factor: factor, binary_factors: binary_factors },
        leftover_tokens,
    ));
}
