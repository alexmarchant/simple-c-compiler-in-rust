use asm::Asm;
use generator::function;
use parser::program::Program;

pub fn asm(asm: &mut Asm, program: Program) {
    function::asm(asm, program.function);
}
