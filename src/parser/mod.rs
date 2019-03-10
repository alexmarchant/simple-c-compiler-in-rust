use std::collections::HashMap;

pub mod program;
pub mod function;
pub mod statement;
pub mod expression;
pub mod term;
pub mod factor;

#[derive(Debug, Default)]
pub struct StackFrame {
    pub current_offset: i64,
    pub vars: HashMap<String, i64>,
}

impl StackFrame {
    pub fn add_var(&mut self, name: String) {
        self.current_offset -= 8;
        self.vars.insert(name, self.current_offset);
    }
}
