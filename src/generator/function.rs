use asm::Asm;
use asm::Register::Rax;
use generator::statement;
use parser::function::Function;
use parser::statement::Statement;

pub fn asm(asm: &mut Asm, function: Function) {
    let has_return: bool;
    match function.statements.last() {
        Some(Statement::Return(_)) => has_return = true,
        _ => has_return = false,
    }

    asm.declare_function(function.name);
    for statement in function.statements {
        statement::asm(asm, statement, &function.stack_frame);
    }

    // Return 0 if no return statement
    if !has_return {
        asm.mov(&0, &Rax);
    }

    asm.function_return();
}
