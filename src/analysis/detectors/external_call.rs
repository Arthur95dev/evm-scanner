use crate::analysis::detectors::Detector;
use crate::analysis::finding::{Finding, Severity};
use crate::evm::instruction::{Instruction};
use crate::evm::opcodes::Opcode;

pub struct ExternalCallDetector;

impl Detector for ExternalCallDetector {
    fn name(&self) -> &'static str {
        "ExternalCall"
    }

    fn detect(&self, instructions: &[Instruction]) -> Vec<Finding> {
        let mut findings = Vec::new();

        for instruction in instructions {
            if let Instruction::Op {
                opcode: Opcode::CALL,
                offset,
            } = instruction
            {
                findings.push(Finding {
                    severity: Severity::High,
                    title: "ExternalCall instruction found".into(),
                    offset: *offset,
                });
            }
        }

        findings
    }
}
