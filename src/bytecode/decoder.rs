use crate::evm::opcodes::Opcode;
use crate::evm::instruction::Instruction;

pub fn decode(bytecode: &[u8]) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    let mut offset = 0;

    while offset < bytecode.len() {
        let byte = bytecode[offset];

        if (0x60..=0x7f).contains(&byte) {
            let size = byte - 0x60 + 1;

            let start = offset + 1;
            let end = start + size as usize;

            if end > bytecode.len() {
                let available = bytecode.len().saturating_sub(start);

                instructions.push(Instruction::InvalidPush {
                    opcode: byte,
                    expected: size,
                    available,
                    offset,
                });

                break;
            }

            let value = bytecode[start..end].to_vec();

            instructions.push(Instruction::Push {
                size,
                value,
                offset,
            });

            offset = end;
            continue;
        }

        match Opcode::from_byte(byte) {
            Some(opcode) => {
                instructions.push(Instruction::Op { opcode, offset });
            }

            None => {
                instructions.push(Instruction::Unknown { byte, offset });
            }
        }

        offset += 1;
    }

    instructions
}
