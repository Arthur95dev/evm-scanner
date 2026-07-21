use crate::bytecode::decoder::decode;
use crate::evm::instruction::Instruction;
use crate::evm::stack::Stack;

pub struct Interpreter {
    stack: Stack,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            stack: Stack::new(),
        }
    }

    pub fn execute(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            self.execute_instruction(instruction);
        }
    }

    pub fn execute_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Push { size: _, value, .. } => {
                // Возможная замена
                let diff = 32 - value.len();
                let mut word = [0u8; 32];
                word[diff..].copy_from_slice(value.as_slice());
                self.stack.push(word);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push1_values() {
        let bytecode = vec![
            0x60, 0x02, // PUSH1 2
            0x60, 0x03, // PUSH1 3
            0x00, // STOP
        ];

        let instructions = decode(&bytecode);

        let mut interpreter = Interpreter::new();
        interpreter.execute(&instructions);

        assert_eq!(interpreter.stack.len(), 2);

        let first = interpreter.stack.get(0).unwrap();
        let second = interpreter.stack.get(1).unwrap();

        let mut expected_first = [0u8; 32];
        expected_first[31] = 0x02;

        let mut expected_second = [0u8; 32];
        expected_second[31] = 0x03;

        assert_eq!(first, &expected_first);
        assert_eq!(second, &expected_second);
    }

    #[test]
    fn test_push32() {
        let bytecode = vec![
            0x7f, // PUSH32
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32,
        ];

        let instructions = decode(&bytecode);

        let mut interpreter = Interpreter::new();
        interpreter.execute(&instructions);

        let mut expected = [0u8; 32];
        expected.copy_from_slice(&[
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32,
        ]);

        assert_eq!(interpreter.stack.get(0), Some(&expected));
    }
}
