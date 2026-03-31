pub use solana_test_input_core::*;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;
pub mod onchain;
