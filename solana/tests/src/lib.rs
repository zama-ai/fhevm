//! Solana FHEVM PoC tests and shared fixtures.
//!
//! Integration tests live in `host_events.rs` (same crate, no separate `tests/` tree).
//! tfhe-worker slow-path tests import the public helpers from this crate.

#![allow(clippy::too_many_arguments, clippy::result_large_err)]

#[cfg(test)]
mod host_events;

mod acl;
mod cleartext;
mod events;
mod fixture;
mod host_ix;
mod invariants;
mod instructions;
mod kms;
mod programs;
mod scenarios;
mod semantic;
mod transaction;
mod util;

pub use acl::{
    acl_record_address, assert_acl_record, assert_balance_acl, balance_acl_record_address,
    created_acl_count, event_authority, rand_acl_record_address, rand_counter_address,
    read_acl_record, read_rand_counter, record_subjects, seed_authorizing_acl_record,
    token_account_address, transfer_amount_acl_address, vault_authority_address,
};
pub use cleartext::{
    cleartext_rand_value, CleartextBackend, ClearValue, FheBackend, Handle, TypedClearValue,
};
pub use events::{
    acl_allowed_events, balance_handle_updated_events, binary_op_events, collect_cpi_events,
    collect_zama_host_events, count_acl_allowed_events,
    count_tfhe_host_events, decode_token_cpi_event, decode_zama_host_cpi_event, fhe_rand_events,
    max_cpi_depth, trivial_encrypt_events, AclAllowedEvent, FheBinaryOpEvent, FheRandEvent,
    TrivialEncryptEvent, ANCHOR_EVENT_IX_TAG_LE, ZamaHostEvent,
};
pub use zama_host_events::FheBinaryOpCode;
pub use fixture::{
    create_spl_token_account, spl_token_amount, token_account, token_fixture, TokenFixture,
    TransferOutputAccounts, WrapOutputAccounts,
};
pub use host_ix::{allow_for_decryption_ix, execute_frame_ix, label};
pub use invariants::{
    assert_balance_acl_subjects, assert_no_zama_host_events_on_failure,
    assert_tfhe_event_count, assert_transfer_output_invariants, assert_wrap_output_invariants,
};
pub use instructions::{
    authorize_transfer_amount, poc_demo_confidential_rand, poc_demo_confidential_rand_ix,
    self_transfer_ix, transfer_ix, transfer_ix_with_amount_acl, transfer_ix_with_amount_nonce,
    transfer_ix_with_current_acl, transfer_ix_with_current_acl_and_amount_nonce,
    transfer_output_accounts, wrap_output_accounts, wrap_usdc_ix,
};
pub use kms::{
    authorization_payload_bytes, kms_like_public_decrypt_check, kms_like_user_decrypt_check,
    signed_current_balance_user_decrypt_request, signed_confidential_rand_user_decrypt_request,
    signed_user_decrypt_request,
    signed_user_decrypt_request_with_domains, PublicDecryptHandleEntry,
    UserDecryptAuthorizationPayload, UserDecryptHandleEntry, UserDecryptRequest,
};
pub use programs::{
    host_program_so_path, svm_with_program, svm_with_programs,
    token_program_so_path,
};
pub use scenarios::{
    run_rand_demo_scenario, run_transfer_scenario, run_transfer_scenario_meta, run_wrap_scenario,
    RandDemoScenario, TransferScenario, TransferSetup, WrapScenario, WrapSetup, BALANCE_FHE_TYPE,
};
pub use semantic::{
    assert_transfer_cleartext, assert_transfer_semantics, compute_backend_kind_from_env,
    decrypt_transfer_balances, seed_transfer_inputs, BackendError, ComputeBackendKind,
    SemanticBackend, TransferExpect,
};
pub use transaction::{
    anchor_ix, send, send_many_with_signers, send_with_meta, send_with_meta_and_signature,
    send_with_signers, try_send, try_send_with_meta,
};
pub use util::{
    amount_plaintext, execute_frame_log_count, expected_trivial_handle,
    previous_bank_hash_from_sysvar, set_previous_slot_hash, DEFAULT_INPUT_NONCE_SEQUENCE,
    DEFAULT_TEST_PREVIOUS_BANK_HASH,
};
