use parser::function;
use parser::function::Function;
use lexer::Token;

#[derive(Debug)]
pub struct Program {
    pub function: Function,
}

pub fn parse(tokens: Vec<Token>) -> Result<Program, String> {
    let function = function::parse(tokens)?;
    return Ok(Program { function: function });
}
