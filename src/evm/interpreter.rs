use crate::bytecode::decoder::decode;
use crate::evm::instruction::Instruction;
use crate::evm::opcodes::Opcode;
use crate::evm::stack::{Stack, StackError};
use alloy_primitives::U256;

pub struct Interpreter {
    stack: Stack,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            stack: Stack::new(),
        }
    }

    pub fn execute(&mut self, instructions: &[Instruction]) -> Result<(), StackError> {
        for instruction in instructions {
            self.execute_instruction(instruction)?;
        }
        Ok(())
    }

    pub fn execute_instruction(&mut self, instruction: &Instruction) -> Result<(), StackError> {
        match instruction {
            Instruction::Push { size: _, value, .. } => {
                // Возможная замена
                let diff = 32 - value.len();
                let mut word = [0u8; 32];
                word[diff..].copy_from_slice(value.as_slice());
                self.stack.push(word);
                Ok(())
            }

            Instruction::Op { opcode, .. } => match opcode {
                Opcode::POP => {
                    self.stack.pop()?;
                    Ok(())
                }

                Opcode::ADD => {
                    let a = U256::from_be_bytes(self.stack.pop()?);
                    let b = U256::from_be_bytes(self.stack.pop()?);
                    self.stack.push(a.wrapping_add(b).to_be_bytes());
                    Ok(())
                }

                _ => Ok(()),
            },

            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push1() {
        let bytecode = vec![
            0x60, 0x02, // PUSH1 2
            0x60, 0x03, // PUSH1 3
            0x00, // STOP
        ];

        let instructions = decode(&bytecode);

        let mut interpreter = Interpreter::new();
        interpreter.execute(&instructions).unwrap();

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
        interpreter.execute(&instructions).unwrap();

        let mut expected = [0u8; 32];
        expected.copy_from_slice(&[
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32,
        ]);

        assert_eq!(interpreter.stack.get(0), Some(&expected));
    }

    #[test]
    fn test_pop_removes_top_value() {
        let bytecode = vec![
            0x60, 0x02, // PUSH1 2
            0x60, 0x03, // PUSH1 3
            0x50, // POP
        ];

        let instructions = decode(&bytecode);

        let mut interpreter = Interpreter::new();
        interpreter.execute(&instructions).unwrap();

        assert_eq!(interpreter.stack.len(), 1);

        let mut expected = [0u8; 32];
        expected[31] = 0x02;

        assert_eq!(interpreter.stack.get(0), Some(&expected));
    }

    #[test]
    fn test_pop_on_empty_stack_returns_underflow_error() {
        let bytecode = vec![0x50]; // POP

        let instructions = decode(&bytecode);

        let mut interpreter = Interpreter::new();

        assert_eq!(interpreter.execute(&instructions), Err(StackError::Underflow));
    }

    #[test]
    fn test_add_sums_top_two_values() {
        let bytecode = vec![
            0x60, 0x02, // PUSH1 2
            0x60, 0x03, // PUSH1 3
            0x01, // ADD
        ];

        let instructions = decode(&bytecode);

        let mut interpreter = Interpreter::new();
        interpreter.execute(&instructions).unwrap();

        assert_eq!(interpreter.stack.len(), 1);

        let mut expected = [0u8; 32];
        expected[31] = 0x05;

        assert_eq!(interpreter.stack.get(0), Some(&expected));
    }

    #[test]
    fn test_add_wraps_on_overflow() {
        let mut bytecode = vec![0x7f]; // PUSH32 U256::MAX
        bytecode.extend_from_slice(&[0xffu8; 32]);
        bytecode.extend_from_slice(&[0x60, 0x01]); // PUSH1 1
        bytecode.push(0x01); // ADD

        let instructions = decode(&bytecode);

        let mut interpreter = Interpreter::new();
        interpreter.execute(&instructions).unwrap();

        assert_eq!(interpreter.stack.get(0), Some(&[0u8; 32]));
    }
}
