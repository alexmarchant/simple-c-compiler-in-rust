use asm::Asm;
use generator::statement;
use parser::function::Function;

pub fn asm(asm: &mut Asm, function: Function) {
    asm.declare_function(function.name);
    statement::asm(asm, function.statement);
}
