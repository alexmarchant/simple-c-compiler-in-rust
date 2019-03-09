use parser::function;
use parser::function::Function;
use lexer::Token;

#[derive(Debug)]
struct Program {
    function: Function,
}

pub fn parse(tokens: Vec<Token>) -> Result<Program, String> {
    match function::parse(&tokens[..]) {
        Ok(function) => return Ok(Program { function: function }),
        Err(err) => return Err(err),
    }
}
