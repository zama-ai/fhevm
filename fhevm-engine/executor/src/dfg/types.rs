use crate::server::InMemoryCiphertext;
use fhevm_engine_common::types::SupportedFheCiphertexts;

pub type DFGTaskResult = Option<InMemoryCiphertext>;

#[derive(Clone)]
pub enum DFGTaskInput {
    Val(SupportedFheCiphertexts),
    Dep(Option<usize>),
}
