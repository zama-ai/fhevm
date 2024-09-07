use fhevm_engine_common::types::{Handle, SupportedFheCiphertexts};

use crate::server::InMemoryCiphertext;

pub type DFGTaskResult = Option<InMemoryCiphertext>;

#[derive(Clone)]
pub enum DFGTaskInput {
    Val(SupportedFheCiphertexts),
    Handle(Handle),
}
