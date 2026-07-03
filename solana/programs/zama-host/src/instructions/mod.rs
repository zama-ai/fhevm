//! Instruction account contexts and handlers for the ZamaHost program.
//!
//! Each instruction lives in its own module. Shared account contexts and
//! validation helpers stay in `common` so generated Anchor account types remain
//! re-exported from this module.

mod common;
pub mod define_kms_context;
pub mod delegate_for_user_decryption;
pub mod destroy_kms_context;
mod encrypted_value;
pub mod fhe_eval;
pub mod initialize_host_config;
pub mod input_verification;
pub mod revoke_delegation_for_user_decryption;
pub mod set_deny_subject;
pub mod set_grant_deny_list_enabled;
pub mod set_host_pause;
pub mod set_max_hcu_depth_per_tx;
pub mod set_max_hcu_per_tx;
#[cfg(feature = "poc")]
pub mod set_mock_input_enabled;
#[cfg(feature = "poc")]
pub mod set_test_shims_enabled;
#[cfg(feature = "poc")]
pub mod test_emit_fhe_rand;
#[cfg(feature = "poc")]
pub mod test_emit_trivial_encrypt;

pub use define_kms_context::*;
pub use delegate_for_user_decryption::*;
pub use destroy_kms_context::*;
pub use encrypted_value::*;
pub use fhe_eval::*;
pub use initialize_host_config::*;
pub use revoke_delegation_for_user_decryption::*;
pub use set_deny_subject::*;
pub use set_grant_deny_list_enabled::*;
pub use set_host_pause::*;
pub use set_max_hcu_depth_per_tx::*;
pub use set_max_hcu_per_tx::*;
#[cfg(feature = "poc")]
pub use set_mock_input_enabled::*;
#[cfg(feature = "poc")]
pub use set_test_shims_enabled::*;
#[cfg(feature = "poc")]
pub use test_emit_fhe_rand::*;
#[cfg(feature = "poc")]
pub use test_emit_trivial_encrypt::*;
