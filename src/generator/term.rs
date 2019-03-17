use asm::Asm;
use generator::factor;
use parser::term::Term;
use parser::StackFrame;
use asm::Register::{Rax, Rcx, Rdx};
use parser::factor::BinaryFactorOperator;

pub fn asm(asm: &mut Asm, term: Term, stack_frame: &StackFrame) {
    factor::asm(asm, term.factor, stack_frame);

    for factor in term.binary_factors {
        asm.push(&Rax);
        factor::asm(asm, factor.right_factor, stack_frame);
        asm.pop(&Rcx);

        match factor.operator {
            BinaryFactorOperator::Multiplication => {
                asm.imul(&Rcx, &Rax);
            },
            BinaryFactorOperator::Division => {
                asm.mov(&Rcx, &Rdx);
                asm.mov(&Rax, &Rcx);
                asm.mov(&Rdx, &Rax);
                asm.mov(&0, &Rdx);
                asm.idiv(&Rcx);
            },
        }
    }
}

