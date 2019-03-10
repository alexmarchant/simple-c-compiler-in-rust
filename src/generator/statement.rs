use asm::Asm;
use generator::expression;
use parser::statement::Statement;

pub fn asm(asm: &mut Asm, statement: Statement) {
    match statement {
        Statement::Return(expression) => {
            expression::asm(asm, expression);
            asm.ret();
        },
    }
}

