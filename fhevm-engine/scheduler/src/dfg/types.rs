use fhevm_engine_common::types::SupportedFheCiphertexts;

pub type DFGTaskResult = Option<SupportedFheCiphertexts>;

#[derive(Clone)]
pub enum DFGTaskInput {
    Val(SupportedFheCiphertexts),
    Dep(Option<usize>),
}

pub enum SchedulerError {
    SchedulerError,
}
