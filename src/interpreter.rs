use crate::parser::{IoMode, Statement, StatementKind, Value};
pub struct Vm {
    pub program: Vec<Statement>,
    pub memory: Vec<usize>,
    pub index: usize,
}

const MEMORY_SIZE: usize = 30000;

impl Vm {
    pub fn new(ast: Vec<Statement>) -> Vm {
        return Vm {
            program: ast,
            memory: vec![0; MEMORY_SIZE],
            index: 0,
        };
    }

    pub fn run(&mut self) {
        self.run_statements(self.program.clone())
    }

    fn get_cell(&mut self) -> usize {
        self.memory[self.index]
    }

    fn set_cell(&mut self, value: usize) {
        let cell = self.get_cell();

        if cell == usize::MAX {
            return;
        }

        self.memory[self.index] = value;
    }

    pub fn run_statements(&mut self, pg: Vec<Statement>) {
        for st in pg.into_iter() {
            match st.kind {
                StatementKind::Loop => {
                    if st.children.len() > 0 {
                        self.run_statements(st.children.clone());
                    }
                }
                StatementKind::Io(val) => match val {
                    IoMode::In => todo!(),
                    IoMode::Out => todo!(),
                },
                StatementKind::Ptr(val) => match val {
                    Value::Inc => {
                        if self.index < MEMORY_SIZE {
                            self.index = self.index + 1;
                        }
                    }
                    Value::Dec => {
                        if self.index > 0 {
                            self.index = self.index - 1;
                        }
                    }
                },
                StatementKind::Math(val) => match val {
                    Value::Inc => {
                        let cell = self.get_cell();

                        if cell == usize::MAX {
                            return;
                        }

                        self.set_cell(cell + 1);
                    }
                    Value::Dec => {
                        let cell = self.get_cell();

                        if cell <= 0 {
                            return;
                        }

                        self.set_cell(cell - 1);
                    }
                },
            }
        }
    }
}

#[cfg(test)]
mod interpreter_tests {
    use super::*;
    use crate::{lexer::Lexer, parser::Parser};

    fn vm_with_input(program: &str) -> Vm {
        let tokens = Lexer::new(program).lex();
        let vm = Vm::new(Parser::parse(&mut tokens.into_iter()));
        vm
    }

    #[test]
    fn math_statements() {
        let mut vm = vm_with_input("++");
        vm.run();
        assert_eq!(vm.memory.get(0).unwrap(), &2);

        let mut vm = vm_with_input("+-+");
        vm.run();
        assert_eq!(vm.memory.get(0).unwrap(), &1);
    }

    #[test]
    fn ptr_statements() {
        let mut vm = vm_with_input("+>+>+");
        vm.run();
        assert_eq!(vm.memory.get(0).unwrap(), &1);
        assert_eq!(vm.memory.get(1).unwrap(), &1);
        assert_eq!(vm.memory.get(2).unwrap(), &1);

        let mut vm = vm_with_input("+>+<+");
        vm.run();
        assert_eq!(vm.memory.get(0).unwrap(), &2);
        assert_eq!(vm.memory.get(1).unwrap(), &1);
        assert_eq!(vm.memory.get(2).unwrap(), &0);
    }

    #[test]
    fn loop_statements() {
        let mut vm = vm_with_input("+++++[-]");
        vm.run();
        assert_eq!(vm.memory.get(0).unwrap(), &1);
    }
}
