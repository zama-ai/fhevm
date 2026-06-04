//! Instruction account contexts and handlers for the ZamaHost program.
//!
//! Each instruction lives in its own module. Shared account contexts and
//! validation helpers stay in `common` so generated Anchor account types remain
//! re-exported from this module.

pub mod allow_acl_subjects;
pub mod allow_for_decryption;
pub mod allow_transient_handle;
pub mod assert_acl_record;
pub mod close_transient_session;
pub mod commit_handle_material;
mod common;
pub mod create_transient_session;
pub mod delegate_for_user_decryption;
pub mod fhe_binary_op;
pub mod fhe_binary_op_and_bind_output;
pub mod fhe_eval;
pub mod fhe_rand_and_bind;
pub mod fhe_rand_bounded_and_bind;
pub mod fhe_ternary_op_and_bind_output;
pub mod initialize_host_config;
pub mod mock_input_verified_and_bind;
pub mod revoke_delegation_for_user_decryption;
pub mod seal_transient_session;
pub mod set_deny_subject;
pub mod set_grant_deny_list_enabled;
pub mod set_host_pause;
pub mod set_mock_input_enabled;
pub mod set_test_shims_enabled;
pub mod test_emit_acl_allowed;
pub mod test_emit_fhe_rand;
pub mod test_emit_input_verified;
pub mod test_emit_trivial_encrypt;
pub mod trivial_encrypt_and_bind;
pub mod verify_coprocessor_input_and_bind;
pub mod verify_input_and_bind;

pub use allow_acl_subjects::*;
pub use allow_for_decryption::*;
pub use allow_transient_handle::*;
pub use assert_acl_record::*;
pub use close_transient_session::*;
pub use commit_handle_material::*;
pub use create_transient_session::*;
pub use delegate_for_user_decryption::*;
pub use fhe_binary_op::*;
pub use fhe_binary_op_and_bind_output::*;
pub use fhe_eval::*;
pub use fhe_rand_and_bind::*;
pub use fhe_rand_bounded_and_bind::*;
pub use fhe_ternary_op_and_bind_output::*;
pub use initialize_host_config::*;
pub use mock_input_verified_and_bind::*;
pub use revoke_delegation_for_user_decryption::*;
pub use seal_transient_session::*;
pub use set_deny_subject::*;
pub use set_grant_deny_list_enabled::*;
pub use set_host_pause::*;
pub use set_mock_input_enabled::*;
pub use set_test_shims_enabled::*;
pub use test_emit_acl_allowed::*;
pub use test_emit_fhe_rand::*;
pub use test_emit_input_verified::*;
pub use test_emit_trivial_encrypt::*;
pub use trivial_encrypt_and_bind::*;
pub use verify_coprocessor_input_and_bind::*;
pub use verify_input_and_bind::*;
