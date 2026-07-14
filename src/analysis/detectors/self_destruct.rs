use crate::analysis::detectors::Detector;
use crate::analysis::finding::{Finding, Severity};
use crate::evm::instruction::Instruction;
use crate::evm::opcodes::Opcode;

pub struct SelfDestructDetector;

impl Detector for SelfDestructDetector {
    fn name(&self) -> &'static str {
        "SelfDestruct"
    }

    fn detect(&self, instructions: &[Instruction]) -> Vec<Finding> {
        let mut findings = Vec::new();

        for instruction in instructions {
            if let Instruction::Op {
                opcode: Opcode::SELFDESTRUCT,
                offset,
            } = instruction
            {
                findings.push(Finding {
                    severity: Severity::High,
                    title: "SelfDestruct found".into(),
                    offset: *offset,
                });
            }
        }

        findings
    }
}
