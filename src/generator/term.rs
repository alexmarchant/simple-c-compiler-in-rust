use asm::Asm;
use asm::Register::{Rax, Rcx, Rdx};
use generator::factor;
use parser::term::Term;
use parser::factor::BinaryFactorOperator;

pub fn asm(asm: &mut Asm, term: Term) {
    factor::asm(asm, term.factor);

    for factor in term.binary_factors {
        asm.push(Rax);
        factor::asm(asm, factor.right_factor);
        asm.pop(Rcx);

        match factor.operator {
            BinaryFactorOperator::Multiplication => {
                asm.imul(Rcx, Rax);
            },
            BinaryFactorOperator::Division => {
                asm.mov(Rcx, Rdx);
                asm.mov(Rax, Rcx);
                asm.mov(Rdx, Rax);
                asm.mov_int(0, Rdx);
                asm.idiv(Rcx);
            },
        }
    }
}

