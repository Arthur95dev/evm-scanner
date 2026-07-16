use crate::analysis::detectors::{
    DelegateCallDetector, Detector, ExternalCallDetector, SelfDestructDetector,
};
use crate::analysis::finding::Finding;
use crate::evm::instruction::Instruction;

pub struct Scanner {
    detectors: Vec<Box<dyn Detector>>,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            detectors: vec![
                Box::new(DelegateCallDetector),
                Box::new(ExternalCallDetector),
                Box::new(SelfDestructDetector),
            ],
        }
    }

    pub fn scan(&self, instructions: &[Instruction]) -> Vec<Finding> {
        let mut findings = Vec::new();

        for detector in &self.detectors {
            findings.extend(detector.detect(instructions));
        }

        findings
    }
}
