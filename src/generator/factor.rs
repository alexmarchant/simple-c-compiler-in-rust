use asm::Asm;
use asm::Register::{Rax, Al};
use generator::factor;
use generator::expression;
use parser::factor::Factor;
use parser::factor::UnaryOperator;
use parser::factor::UnaryOperation;

pub fn asm(asm: &mut Asm, factor: Factor) {
    match factor {
        Factor::Constant(value) => {
            asm.mov_int(value, Rax);
        },
        Factor::UnaryOperation(operation) => {
            unary_operation_asm(asm, *operation);
        },
        Factor::Expression(expression) => {
            expression::asm(asm, *expression);
        },
    }
}

pub fn unary_operation_asm(asm: &mut Asm, operation: UnaryOperation) {
    factor::asm(asm, operation.factor.clone());

    match operation.operator {
        UnaryOperator::Negation => {
            asm.neg(Rax);
        },
        UnaryOperator::LogicalNegation => {
            asm.cmp_int(0, Rax);
            asm.mov_int(0, Rax);
            asm.sete(Al);
        },
        UnaryOperator::BitwiseComplement => {
            asm.not(Rax);
        },
    }
}
