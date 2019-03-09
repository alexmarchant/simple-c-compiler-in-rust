use parser;

pub fn program_asm(program: parser::Program) -> String {
    let asm = function_asm(program.function);
    return asm
}

pub fn function_asm(function: parser::Function) -> String {
    let mut asm = format!("\t.globl\t_{}\n", function.name);
    asm.push_str(&format!("_{}:\n", function.name));
    asm.push_str(&statement_asm(function.statement));
    return asm;
}

pub fn statement_asm(statement: parser::Statement) -> String {
    match statement {
        parser::Statement::Return(expression) => {
            let mut asm = expression_asm(expression);
            asm.push_str("\tret\n");
            return asm;
        },
    }
}

pub fn expression_asm(expression: parser::Expression) -> String {
    let mut asm = term_asm(expression.term);

    for term in expression.binary_terms {
        asm.push_str("\tpush\t%rax\n");
        asm.push_str(&term_asm(term.right_term));
        asm.push_str("\tpop\t%rcx\n");

        match term.operator {
            parser::BinaryTermOperator::Addition => {
                asm.push_str("\tadd\t%rcx, %rax\n");
            },
            parser::BinaryTermOperator::Subtraction => {
                asm.push_str("\tsub\t%rax, %rcx\n");
                asm.push_str("\tmov\t%rcx, %rax\n");
            },
        }
    }

    return asm;
}

pub fn term_asm(term: parser::Term) -> String {
    let mut asm = factor_asm(term.factor);

    for factor in term.binary_factors {
        asm.push_str("\tpush\t%rax\n");
        asm.push_str(&factor_asm(factor.right_factor));
        asm.push_str("\tpop\t%rcx\n");

        match factor.operator {
            parser::BinaryFactorOperator::Multiplication => {
                asm.push_str("\timul\t%rcx, %rax\n");
            },
            parser::BinaryFactorOperator::Division => {
                asm.push_str("\tmov\t%rcx, %rdx\n");
                asm.push_str("\tmov\t%rax, %rcx\n");
                asm.push_str("\tmov\t%rdx, %rax\n");
                asm.push_str("\tmov\t$0, %rdx\n");
                asm.push_str("\tidiv\t%rcx\n");
            },
        }
    }

    return asm;
}

pub fn factor_asm(factor: parser::Factor) -> String {
    match factor {
        parser::Factor::Constant(value) => {
            return format!("\tmov\t${}, %rax\n", value);
        },
        parser::Factor::UnaryOperation(operation) => {
            return unary_operation_asm(*operation);
        },
        parser::Factor::Expression(expression) => {
            return expression_asm(*expression);
        },
    }
}

pub fn unary_operation_asm(operation: parser::UnaryOperation) -> String {
    let mut asm = factor_asm(operation.factor.clone());

    match operation.operator {
        parser::UnaryOperator::Negation => {
            asm.push_str("\tneg\t%rax\n");
        },
        parser::UnaryOperator::LogicalNegation => {
            asm.push_str("\tcmp\t$0, %rax\n");
            asm.push_str("\tmov\t$0, %rax\n");
            asm.push_str("\tsete\t%al\n");
        },
        parser::UnaryOperator::BitwiseComplement => {
            asm.push_str("\tnot\t%rax\n");
        },
    }

    return asm;
}
