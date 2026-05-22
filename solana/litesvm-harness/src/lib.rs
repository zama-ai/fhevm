//! Shared LiteSVM harness for Solana PoC tests.
//!
//! Stack: `litesvm 0.11`, `anchor-litesvm 0.4`, `anchor-lang 1.0.2`, `solana-sdk 3.0`.

mod acl;
mod fixture;
mod instructions;
mod kms;
mod programs;
mod transaction;
mod util;

pub use acl::{
    acl_record_address, balance_acl_record_address, event_authority, read_acl_record,
    record_subjects, token_account_address, transfer_amount_acl_address, vault_authority_address,
};
pub use fixture::{
    create_spl_token_account, spl_token_amount, token_account, token_fixture, TokenFixture,
    TransferOutputAccounts, WrapOutputAccounts,
};
pub use instructions::{
    authorize_transfer_amount, self_transfer_ix, transfer_ix, transfer_ix_with_amount_acl,
    transfer_ix_with_amount_nonce, transfer_ix_with_current_acl,
    transfer_ix_with_current_acl_and_amount_nonce, transfer_output_accounts, wrap_output_accounts,
    wrap_usdc_ix,
};
pub use kms::{
    authorization_payload_bytes, kms_like_user_decrypt_check,
    signed_current_balance_user_decrypt_request, signed_user_decrypt_request,
    signed_user_decrypt_request_with_domains, UserDecryptAuthorizationPayload,
    UserDecryptHandleEntry, UserDecryptRequest,
};
pub use programs::{
    assert_program_built, host_program_so_path, svm_with_program, svm_with_programs,
    token_program_so_path,
};
pub use transaction::{
    anchor_ix, send, send_many_with_signers, send_with_meta, send_with_meta_and_signature,
    send_with_signers, try_send,
};
pub use util::{
    amount_plaintext, execute_frame_log_count, expected_trivial_handle,
    DEFAULT_INPUT_NONCE_SEQUENCE,
};
