use parser;

pub fn program_asm(program: parser::Program) -> String {
    let function_asm = function_asm(program.function);
    return format!("{}\n", function_asm);
}

pub fn function_asm(function: parser::Function) -> String {
    let statement_asm = statement_asm(function.statement);
    let mut res = format!("\t.globl\t_{}", function.name);
    res = format!("{}\n_{}:", res, function.name);
    res = format!("{}\n{}", res, statement_asm);
    return res;
}

pub fn statement_asm(statement: parser::Statement) -> String {
    match statement {
        parser::Statement::Return(expression) => {
            let expression_asm = expression_asm(expression);
            return format!("{}\n\tret", expression_asm);
        },
    }
}

pub fn expression_asm(expression: parser::Expression) -> String {
    match expression {
        parser::Expression::Constant(value) => {
            return format!("\tmovq\t${}, %rax", value);
        },
        parser::Expression::UnaryOperation(operation) => {
            let expression_asm = expression_asm(operation.expression.clone());
            match operation.operator {
                parser::UnaryOperator::Negation => {
                    return format!("{}\n\tneg\t%rax", expression_asm);
                },
                parser::UnaryOperator::LogicalNegation => {
                    let mut res = format!("{}", expression_asm);
                    res = format!("{}\n\tcmpq\t$0, %rax", res);
                    res = format!("{}\n\tmovq\t$0, %rax", res);
                    res = format!("{}\n\tsete\t%al", res);
                    return res;
                },
                parser::UnaryOperator::BitwiseComplement => {
                    return format!("{}\n\tnot\t%rax", expression_asm);
                },
            }
        },
    }
}

