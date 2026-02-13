#[derive(Clone, Debug, Default)]
pub struct AsyncComputation {
    pub operation: i32,
    pub transaction_id: Vec<u8>,
    pub output_handle: Vec<u8>,
    pub inputs: Vec<AsyncComputationInput>,
    pub is_allowed: bool,
}

#[derive(Clone, Debug, Default)]
pub struct AsyncComputationInput {
    pub input: Option<async_computation_input::Input>,
}

pub mod async_computation_input {
    #[derive(Clone, Debug)]
    pub enum Input {
        InputHandle(Vec<u8>),
        Scalar(Vec<u8>),
    }
}
