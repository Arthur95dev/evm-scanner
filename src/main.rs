use serde_json::Value;
use std::env;
use std::fs;

mod opcodes;
use opcodes::{Instruction, Opcode};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_artifact.json>", args[0]);
        return;
    }

    let path = args.get(1).expect("Need path to file!");

    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", path, e);
            return;
        }
    };

    let json: Value = match serde_json::from_str(&content) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Error parsing JSON: {}", e);
            return;
        }
    };

    let bytecode = json
        .get("deployedBytecode")
        .and_then(|v| v.get("object"))
        .and_then(Value::as_str)
        .expect("Missing deployedBytecode.object");

    let hex_str = bytecode.strip_prefix("0x").unwrap_or(bytecode);
    if hex_str.is_empty() {
        eprintln!("Bytecode is empty.");
        return;
    }

    let bytecode_bytes = match hex::decode(hex_str) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Failed to decode hex: {}", e);
            return;
        }
    };
    
    let mut offset: usize = 0;
    while offset < bytecode_bytes.len() {
        let inst = Instruction::from_byte(bytecode_bytes[offset]);
        let size = inst.size();

        // Проверяем выход за границы (на случай битого байткода)
        if offset + size > bytecode_bytes.len() {
            eprintln!(
                "Warning: instruction at offset {} exceeds bytecode length, stopping.",
                offset
            );
            break;
        }

        if let Instruction::Op(op) = inst {
            if op.is_dangerous() {
                println!(
                    "Offset 0x{:04x} ({}): {} - potential vulnerability",
                    offset,
                    offset,
                    op.name()
                );
            }
        }

        offset += size;
    }
}
