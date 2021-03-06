use std::fmt;

pub enum Register {
    Rax,
    Rcx,
    Rdx,
    Rbp,
    Rsp,
    Al,
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Register::Rax => write!(f, "%rax"),
            Register::Rcx => write!(f, "%rcx"),
            Register::Rdx => write!(f, "%rdx"),
            Register::Rbp => write!(f, "%rbp"),
            Register::Rsp => write!(f, "%rsp"),
            Register::Al => write!(f, "%al"),
        }
    }
}

pub struct RegisterOffset {
    pub register: Register,
    pub offset: i64,
}

impl fmt::Display for RegisterOffset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({})", self.offset, self.register)
    }
}

#[derive(Default)]
pub struct Clause {
    pub state: ClauseState,
    pub count: i64, 
}

pub enum ClauseState {
    New,
    Started,
    Ended,
}

impl Default for ClauseState {
    fn default() -> Self { ClauseState::New }
}

impl Clause {
    pub fn start_id(&self) -> String {
        return format!("_clause_{}", self.count);
    }

    pub fn end_id(&self) -> String {
        return format!("_end_{}", self.count);
    }
}

#[derive(Default)]
pub struct Asm {
    pub clause_count: i64,
    pub source: String,
}

impl Asm {
    pub fn declare_function(&mut self, name: String) {
        self.source.push_str(
            &format!("\t.globl\t_{}\n", name)
        );
        self.source.push_str(
            &format!("_{}:\n", name)
        );
        // Function prologue (new stack frame)
        self.push(&Register::Rbp);
        self.mov(&Register::Rsp, &Register::Rbp);
    }

    pub fn function_return(&mut self) {
        // Function epilogue (clear stack frame)
        self.mov(&Register::Rbp, &Register::Rsp);
        self.pop(&Register::Rbp);
        // Return
        self.ret();
    }

    pub fn mov(&mut self, src: &fmt::Display, dest: &fmt::Display) {
        self.source.push_str(
            &format!("\tmov\t{}, {}\n", src, dest)
        );
    }

    pub fn push(&mut self, src: &fmt::Display) {
        self.source.push_str(
            &format!("\tpush\t{}\n", src)
        );
    }

    pub fn pop(&mut self, src: &fmt::Display) {
        self.source.push_str(
            &format!("\tpop\t{}\n", src)
        );
    }

    pub fn ret(&mut self) {
        self.source.push_str(
            "\tret\n",
        );
    }

    pub fn add(&mut self, src: &fmt::Display, dest: &fmt::Display) {
        self.source.push_str(
            &format!("\tadd\t{}, {}\n", src, dest)
        );
    }

    pub fn sub(&mut self, src: &fmt::Display, dest: &fmt::Display) {
        self.source.push_str(
            &format!("\tsub\t{}, {}\n", src, dest)
        );
    }

    pub fn imul(&mut self, src: &fmt::Display, dest: &fmt::Display) {
        self.source.push_str(
            &format!("\timul\t{}, {}\n", src, dest)
        );
    }

    pub fn idiv(&mut self, src: &fmt::Display) {
        self.source.push_str(
            &format!("\tidiv\t{}\n", src)
        );
    }

    pub fn neg(&mut self, src: &fmt::Display) {
        self.source.push_str(
            &format!("\tneg\t{}\n", src)
        );
    }

    pub fn cmp(&mut self, src_a: &fmt::Display, src_b: &fmt::Display) {
        self.source.push_str(
            &format!("\tcmp\t{}, {}\n", src_a, src_b)
        );
    }

    pub fn sete(&mut self, dest: &fmt::Display) {
        self.source.push_str(
            &format!("\tsete\t{}\n", dest)
        );
    }

    pub fn setne(&mut self, dest: &fmt::Display) {
        self.source.push_str(
            &format!("\tsetne\t{}\n", dest)
        );
    }

    pub fn setl(&mut self, dest: &fmt::Display) {
        self.source.push_str(
            &format!("\tsetl\t{}\n", dest)
        );
    }

    pub fn setle(&mut self, dest: &fmt::Display) {
        self.source.push_str(
            &format!("\tsetle\t{}\n", dest)
        );
    }

    pub fn setg(&mut self, dest: &fmt::Display) {
        self.source.push_str(
            &format!("\tsetg\t{}\n", dest)
        );
    }

    pub fn setge(&mut self, dest: &fmt::Display) {
        self.source.push_str(
            &format!("\tsetge\t{}\n", dest)
        );
    }

    pub fn not(&mut self, src: &fmt::Display) {
        self.source.push_str(
            &format!("\tnot\t{}\n", src)
        );
    }

    pub fn jmp(&mut self, clause_id: String) {
        self.source.push_str(
            &format!("\tjmp\t{}\n", clause_id)
        );
    }

    pub fn je(&mut self, clause_id: String) {
        self.source.push_str(
            &format!("\tje\t{}\n", clause_id)
        );
    }

    pub fn jne(&mut self, clause_id: String) {
        self.source.push_str(
            &format!("\tjne\t{}\n", clause_id)
        );
    }

    pub fn new_clause(&mut self) -> Clause {
        let clause = Clause { count: self.clause_count, ..Default::default() };
        self.clause_count += 1;
        return clause;
    }

    pub fn start_clause(&mut self, clause: &mut Clause) {
        match clause.state {
            ClauseState::New => (),
            ClauseState::Started => panic!("Clause already started"),
            ClauseState::Ended => panic!("Clause already ended"),
        }

        self.source.push_str(
            &format!("{}:\n", clause.start_id())
        );
        clause.state = ClauseState::Started;
    }

    pub fn end_clause(&mut self, clause: &mut Clause) {
        match clause.state {
            ClauseState::New => panic!("Clause not started"),
            ClauseState::Started => (),
            ClauseState::Ended => panic!("Clause already ended"),
        }

        self.source.push_str(
            &format!("{}:\n", clause.end_id())
        );
        clause.state = ClauseState::Ended;
    }
}
