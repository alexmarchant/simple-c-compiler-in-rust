use asm::Asm;
use generator::term;
use asm::Register::{Rax, Rcx, Al};
use parser::expression::{
    Expression,
    ExpressionOperator,
    LogicalAndExpression,
    LogicalAndOperator,
    EqualityExpression,
    EqualityOperator,
    RelationalExpression,
    RelationalOperator,
    AdditiveExpression,
    AdditiveOperator,
};

pub fn asm(asm: &mut Asm, expression: Expression) {
    // e1
    logical_and_asm(asm, expression.expression);

    for binary_expression in expression.binary_expressions {
        match binary_expression.operator {
            ExpressionOperator::Or => {
                let mut clause = asm.new_clause();

		asm.cmp_int(0, Rax);
                asm.je(clause.start_id());
                asm.mov_int(1, Rax);
                asm.jmp(clause.end_id());

                asm.start_clause(&mut clause);
                // e2
                logical_and_asm(asm, binary_expression.right_expression);
                asm.cmp_int(0, Rax);
                asm.mov_int(0, Rax);
                asm.setne(Al);

                asm.end_clause(&mut clause);
            },
        }
    }
}

fn logical_and_asm(asm: &mut Asm, expression: LogicalAndExpression) {
    // e1
    equality_asm(asm, expression.expression);

    for binary_expression in expression.binary_expressions {
        match binary_expression.operator {
            LogicalAndOperator::And => {
                let mut clause = asm.new_clause();

		asm.cmp_int(0, Rax);
                asm.jne(clause.start_id());
                asm.jmp(clause.end_id());

                asm.start_clause(&mut clause);
                // e2
                equality_asm(asm, binary_expression.right_expression);
                asm.cmp_int(0, Rax);
                asm.mov_int(0, Rax);
                asm.setne(Al);

                asm.end_clause(&mut clause);
            },
        }
    }
}

fn equality_asm(asm: &mut Asm, expression: EqualityExpression) {
    relational_asm(asm, expression.expression);

    for binary_expression in expression.binary_expressions {
        asm.push(Rax);
        relational_asm(asm, binary_expression.right_expression);
        asm.pop(Rcx);
        asm.cmp(Rax, Rcx);
        asm.mov_int(0, Rax);

        match binary_expression.operator {
            EqualityOperator::Equal => {
                asm.sete(Al);
            },
            EqualityOperator::NotEqual => {
                asm.setne(Al);
            },
        }
    }
}

fn relational_asm(asm: &mut Asm, expression: RelationalExpression) {
    additive_asm(asm, expression.expression);

    for binary_expression in expression.binary_expressions {
        asm.push(Rax);
        additive_asm(asm, binary_expression.right_expression);
        asm.pop(Rcx);
        asm.cmp(Rax, Rcx);
        asm.mov_int(0, Rax);

        match binary_expression.operator {
            RelationalOperator::LessThan => {
                asm.setl(Al);
            },
            RelationalOperator::LessThanOrEqual => {
                asm.setle(Al);
            },
            RelationalOperator::GreaterThan => {
                asm.setg(Al);
            },
            RelationalOperator::GreaterThanOrEqual => {
                asm.setge(Al);
            },
        }
    }
}

fn additive_asm(asm: &mut Asm, expression: AdditiveExpression) {
    term::asm(asm, expression.term);

    for binary_term in expression.binary_terms {
        asm.push(Rax);
        term::asm(asm, binary_term.right_term);
        asm.pop(Rcx);

        match binary_term.operator {
            AdditiveOperator::Addition => {
                asm.add(Rcx, Rax);
            },
            AdditiveOperator::Subtraction => {
                asm.sub(Rax, Rcx);
                asm.mov(Rcx, Rax);
            },
        }
    }
}
