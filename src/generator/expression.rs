use asm::Asm;
use generator;
use generator::term;
use parser::StackFrame;
use asm::Register::{Rax, Rcx, Al, Rbp};
use asm::RegisterOffset;
use parser::expression::{
    Expression,
    LogicalOrExpression,
    LogicalOrOperator,
    LogicalAndExpression,
    LogicalAndOperator,
    EqualityExpression,
    EqualityOperator,
    RelationalExpression,
    RelationalOperator,
    AdditiveExpression,
    AdditiveOperator,
    Assignment,
};

pub fn asm(asm: &mut Asm, expression: Expression, stack_frame: &StackFrame) {
    match expression {
        Expression::Assignment(assignment) => {
            assignment_asm(asm, assignment, stack_frame);
        },
        Expression::LogicalOrExpression(expression) => {
            logical_or_asm(asm, expression, stack_frame);
        },
    }
}

pub fn logical_or_asm(asm: &mut Asm, expression: LogicalOrExpression, stack_frame: &StackFrame) {
    // e1
    logical_and_asm(asm, expression.expression, stack_frame);

    for binary_expression in expression.binary_expressions {
        match binary_expression.operator {
            LogicalOrOperator::Or => {
                let mut clause = asm.new_clause();

		asm.cmp(&0, &Rax);
                asm.je(clause.start_id());
                asm.mov(&1, &Rax);
                asm.jmp(clause.end_id());

                asm.start_clause(&mut clause);
                // e2
                logical_and_asm(asm, binary_expression.right_expression, stack_frame);
                asm.cmp(&0, &Rax);
                asm.mov(&0, &Rax);
                asm.setne(&Al);

                asm.end_clause(&mut clause);
            },
        }
    }
}

fn logical_and_asm(asm: &mut Asm, expression: LogicalAndExpression, stack_frame: &StackFrame) {
    // e1
    equality_asm(asm, expression.expression, stack_frame);

    for binary_expression in expression.binary_expressions {
        match binary_expression.operator {
            LogicalAndOperator::And => {
                let mut clause = asm.new_clause();

		asm.cmp(&0, &Rax);
                asm.jne(clause.start_id());
                asm.jmp(clause.end_id());

                asm.start_clause(&mut clause);
                // e2
                equality_asm(asm, binary_expression.right_expression, stack_frame);
                asm.cmp(&0, &Rax);
                asm.mov(&0, &Rax);
                asm.setne(&Al);

                asm.end_clause(&mut clause);
            },
        }
    }
}

fn equality_asm(asm: &mut Asm, expression: EqualityExpression, stack_frame: &StackFrame) {
    relational_asm(asm, expression.expression, stack_frame);

    for binary_expression in expression.binary_expressions {
        asm.push(&Rax);
        relational_asm(asm, binary_expression.right_expression, stack_frame);
        asm.pop(&Rcx);
        asm.cmp(&Rax, &Rcx);
        asm.mov(&0, &Rax);

        match binary_expression.operator {
            EqualityOperator::Equal => {
                asm.sete(&Al);
            },
            EqualityOperator::NotEqual => {
                asm.setne(&Al);
            },
        }
    }
}

fn relational_asm(asm: &mut Asm, expression: RelationalExpression, stack_frame: &StackFrame) {
    additive_asm(asm, expression.expression, stack_frame);

    for binary_expression in expression.binary_expressions {
        asm.push(&Rax);
        additive_asm(asm, binary_expression.right_expression, stack_frame);
        asm.pop(&Rcx);
        asm.cmp(&Rax, &Rcx);
        asm.mov(&0, &Rax);

        match binary_expression.operator {
            RelationalOperator::LessThan => {
                asm.setl(&Al);
            },
            RelationalOperator::LessThanOrEqual => {
                asm.setle(&Al);
            },
            RelationalOperator::GreaterThan => {
                asm.setg(&Al);
            },
            RelationalOperator::GreaterThanOrEqual => {
                asm.setge(&Al);
            },
        }
    }
}

fn additive_asm(asm: &mut Asm, expression: AdditiveExpression, stack_frame: &StackFrame) {
    term::asm(asm, expression.term, stack_frame);

    for binary_term in expression.binary_terms {
        asm.push(&Rax);
        term::asm(asm, binary_term.right_term, stack_frame);
        asm.pop(&Rcx);

        match binary_term.operator {
            AdditiveOperator::Addition => {
                asm.add(&Rcx, &Rax);
            },
            AdditiveOperator::Subtraction => {
                asm.sub(&Rax, &Rcx);
                asm.mov(&Rcx, &Rax);
            },
        }
    }
}

fn assignment_asm(asm: &mut Asm, assignment: Box<Assignment>, stack_frame: &StackFrame) {
    let offset = stack_frame.vars.get(&assignment.var.name)
        .expect(&format!(
            "Var '{}' has not been declared",
            &assignment.var.name,
        ));
    generator::expression::asm(asm, assignment.expression, stack_frame);
    let reg_offset = RegisterOffset {
        offset: *offset,
        register: Rbp,
    };
    asm.mov(&Rax, &reg_offset);
}
