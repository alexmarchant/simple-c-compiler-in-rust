use asm::Asm;
use parser::StackFrame;
use asm::Register::Rax;
use generator::expression;
use parser::statement::Statement;

pub fn asm(asm: &mut Asm, statement: Statement, stack_frame: &StackFrame) {
    match statement {
        Statement::Return(expression) => {
            expression::asm(asm, expression, stack_frame);
        },
        Statement::Expression(expression) => {
            expression::asm(asm, expression, stack_frame);
        },
        Statement::VariableDeclaration(declaration) => {
            match declaration.expression {
                Some(expression) => expression::asm(
                    asm,
                    expression,
                    stack_frame,
                ),
                None => asm.mov_int(0, Rax),
            }
            asm.push(Rax);
        },
    }
}

