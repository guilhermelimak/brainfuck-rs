use crate::parser::{IoMode, Statement, StatementKind, Value};
struct Vm {
    pub program: Vec<Statement>,
    pub memory: Vec<usize>,
    pub index: usize,
}

impl Vm {
    pub fn new(ast: Vec<Statement>) -> Vm {
        return Vm {
            program: ast,
            memory: vec![0, 30000],
            index: 0,
        };
    }

    pub fn run_loop(&mut self) {}

    pub fn run_program(&mut self) {
        let pg = self.program.clone();

        match pg.into_iter().next() {
            Some(st) => match st.kind {
                StatementKind::Loop => {}
                StatementKind::Ptr(val) => match val {
                    Value::Inc => {
                        self.index = self.index + 1;
                    }
                    Value::Dec => {
                        self.index = self.index - 1;
                    }
                },
                StatementKind::Io(val) => match val {
                    IoMode::In => todo!(),
                    IoMode::Out => todo!(),
                },
                StatementKind::Math(val) => match val {
                    Value::Inc => {
                        let cell = self.memory[self.index];
                        self.memory[self.index] = cell + 1;
                    }
                    Value::Dec => {
                        let cell = self.memory[self.index];
                        if cell <= 0 {
                            return;
                        }
                        self.memory[self.index] = cell - 1;
                    }
                },
            },
            None => todo!(),
        }
    }
}

#[cfg(test)]
mod interpreter_tests {}
