use parser;

pub fn program_asm(program: parser::Program) -> String {
    let function_asm = function_asm(program.function);
    return format!("{}\n", function_asm);
}

pub fn function_asm(function: parser::Function) -> String {
    let statement_asm = statement_asm(function.statement);
    return format!("\t.globl\t_{}\n_{}:\n{}", function.name, function.name, statement_asm);
}

pub fn statement_asm(statement: parser::Statement) -> String {
    match statement {
        parser::Statement::Return(expression) => {
            let expression_asm = expression_asm(expression);
            return format!("\tmovl\t${}, %eax\n\tret", expression_asm);
        },
    }
}

pub fn expression_asm(expression: parser::Expression) -> String {
    match expression {
        parser::Expression::Constant(value) => return value.to_string(),
    }
}

