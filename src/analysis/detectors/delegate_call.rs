use crate::analysis::detectors::Detector;
use crate::analysis::finding::{Finding, Severity};
use crate::evm::instruction::Instruction;
use crate::evm::opcodes::Opcode;

pub struct DelegateCallDetector;

impl Detector for DelegateCallDetector {
    fn name(&self) -> &'static str {
        "DelegateCall"
    }

    fn detect(&self, instructions: &[Instruction]) -> Vec<Finding> {
        let mut findings = Vec::new();

        for instruction in instructions {
            if let Instruction::Op {
                opcode: Opcode::DELEGATECALL,
                offset,
            } = instruction
            {
                findings.push(Finding {
                    severity: Severity::High,
                    title: "DelegateCall instruction found".into(),
                    offset: *offset,
                });
            }
        }

        findings
    }
}
