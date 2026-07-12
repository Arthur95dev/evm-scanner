use crate::evm::instruction::Instruction;
use crate::evm::opcodes::Opcode;
use crate::analysis::finding::{Finding, Severity};

pub struct Scanner;

impl Scanner {
    pub fn scan(instructions: &[Instruction]) -> Vec<Finding> {
        let mut findings = Vec::new();

        for instruction in instructions {
            if let Instruction::Op { opcode, offset } = instruction {
                match opcode {
                    Opcode::CALL => {
                        findings.push(Finding {
                            severity: Severity::Medium,
                            title: "External CALL found".into(),
                            offset: *offset,
                        });
                    }

                    Opcode::DELEGATECALL => {
                        findings.push(Finding {
                            severity: Severity::High,
                            title: "DELEGATECALL found".into(),
                            offset: *offset,
                        });
                    }

                    Opcode::SELFDESTRUCT => {
                        findings.push(Finding {
                            severity: Severity::High,
                            title: "SELFDESTRUCT found".into(),
                            offset: *offset,
                        });
                    }

                    _ => {}
                }
            }
        }

        findings
    }
}
