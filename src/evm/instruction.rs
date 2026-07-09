use crate::evm::opcodes::Opcode;
use std::fmt::{self};

#[derive(Debug)]
pub enum Instruction {
    Op {
        opcode: Opcode,
        offset: usize,
    },

    Push {
        size: u8,
        value: Vec<u8>,
        offset: usize,
    },

    InvalidPush {
        opcode: u8,
        expected: u8,
        available: usize,
        offset: usize,
    },

    Unknown {
        byte: u8,
        offset: usize,
    },
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Op { opcode, offset } => {
                write!(f, "{:04x} {}", offset, opcode.name())
            }

            Instruction::Push {
                size,
                value,
                offset,
            } => {
                write!(f, "{:04x} PUSH{} 0x{}", offset, size, hex::encode(value))
            }

            Instruction::InvalidPush {
                opcode,
                expected,
                available,
                offset,
            } => {
                write!(
                    f,
                    "{:04x} INVALID_PUSH 0x{:02x} expected={} available={}",
                    offset, opcode, expected, available
                )
            }

            Instruction::Unknown { byte, offset } => {
                write!(f, "{:04x} UNKNOWN 0x{:02x}", offset, byte)
            }
        }
    }
}
