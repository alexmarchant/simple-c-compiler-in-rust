use asm::Asm;
use asm::Register::{Rax, Al, Rbp};
use asm::RegisterOffset;
use generator::factor;
use parser::StackFrame;
use generator::expression;
use parser::factor::Factor;
use parser::factor::UnaryOperator;
use parser::factor::UnaryOperation;

pub fn asm(asm: &mut Asm, factor: Factor, stack_frame: &StackFrame) {
    match factor {
        Factor::Expression(expression) => {
            expression::asm(asm, *expression, stack_frame);
        },
        Factor::UnaryOperation(operation) => {
            unary_operation_asm(asm, *operation, stack_frame);
        },
        Factor::Constant(value) => {
            asm.mov(&value, &Rax);
        },
        Factor::Identifier(name) => {
            let offset = stack_frame.vars.get(&name)
                .expect(&format!(
                    "Var '{}' has not been declared",
                    name,
                ));
            let reg_offset = RegisterOffset {
                offset: *offset,
                register: Rbp,
            };
            asm.mov(&reg_offset, &Rax);
        },
    }
}

pub fn unary_operation_asm(asm: &mut Asm, operation: UnaryOperation, stack_frame: &StackFrame) {
    factor::asm(asm, operation.factor.clone(), stack_frame);

    match operation.operator {
        UnaryOperator::Negation => {
            asm.neg(&Rax);
        },
        UnaryOperator::LogicalNegation => {
            asm.cmp(&0, &Rax);
            asm.mov(&0, &Rax);
            asm.sete(&Al);
        },
        UnaryOperator::BitwiseComplement => {
            asm.not(&Rax);
        },
    }
}
