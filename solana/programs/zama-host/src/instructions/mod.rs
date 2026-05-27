//! Instruction account contexts and handlers for the ZamaHost program.
//!
//! Each instruction lives in its own module. Shared account contexts and
//! validation helpers stay in `common` so generated Anchor account types remain
//! re-exported from this module.

mod allow_acl_subjects;
mod allow_for_decryption;
mod allow_transient_handle;
mod assert_acl_record;
mod close_transient_session;
mod commit_handle_material;
mod common;
mod create_transient_session;
mod delegate_for_user_decryption;
mod fhe_binary_op;
mod fhe_binary_op_and_bind_output;
mod fhe_eval;
mod fhe_rand_and_bind;
mod fhe_rand_bounded_and_bind;
mod fhe_ternary_op_and_bind_output;
mod initialize_host_config;
mod mock_input_verified_and_bind;
mod revoke_delegation_for_user_decryption;
mod seal_transient_session;
mod set_deny_subject;
mod set_grant_deny_list_enabled;
mod set_host_pause;
mod set_mock_input_enabled;
mod set_test_shims_enabled;
mod test_emit_acl_allowed;
mod test_emit_fhe_rand;
mod test_emit_input_verified;
mod test_emit_trivial_encrypt;
mod trivial_encrypt_and_bind;
mod verify_input_and_bind;

pub use allow_acl_subjects::*;
pub use allow_for_decryption::*;
pub use allow_transient_handle::*;
pub use assert_acl_record::*;
pub use close_transient_session::*;
pub use commit_handle_material::*;
pub use common::*;
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
pub use verify_input_and_bind::*;
