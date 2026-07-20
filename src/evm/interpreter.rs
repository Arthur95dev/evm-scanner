use crate::evm::stack::Stack;
use crate::evm::instruction::Instruction;

pub struct Interpreter {
    stack: Stack,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { stack: Stack::new() }
    }

    pub fn execute(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            self.execute_instruction(instruction);
        }
    }

    pub fn execute_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Push { size, value, offset } => {
                // Возможная замена
                let diff = 32 as usize - *size as usize;
                let mut word= [0u8; 32];
                word[diff..].copy_from_slice(value.as_slice());
                self.stack.push(word);
            }
            _ => {}
        }
    }
}