use crate::evm::stack::Stack;
use crate::evm::instruction::Instruction;

pub struct Interpreter {
    stack: Stack,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { stack: Stack::new() }
    }

    pub fn execute(&self, instructions: &[Instruction]) {
        for instruction in instructions {
            self.execute_instructions(instruction);
        }
    }

    pub fn execute_instructions(&self, instruction: &Instruction) {
        match instruction {
            Instruction::Push { size, value, offset } => {
                println!("should realize");
            }
            _ => {}
        }
    }
}