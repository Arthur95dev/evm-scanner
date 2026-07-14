use crate::analysis::finding::Finding;
use crate::evm::instruction::Instruction;

mod external_call;
mod delegate_call;
mod self_destruct;

pub trait Detector {
    fn name(&self) -> &'static str;
    fn detect(&self, instructions: &[Instruction]) -> Vec<Finding>;
}
