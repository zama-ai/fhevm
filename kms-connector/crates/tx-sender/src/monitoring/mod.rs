pub mod health;
pub mod metrics;
pub mod operation_recovery;

/// DB table names of the decryption operations handled by the KMS connector.
const DECRYPTION_TABLES: &[&str] = &[
    "public_decryption_requests",
    "user_decryption_requests",
    "public_decryption_responses",
    "user_decryption_responses",
];

/// DB table names of the operations handled by the KMS connector.
const OPERATION_TABLES: &[&str] = &[
    "public_decryption_requests",
    "user_decryption_requests",
    "public_decryption_responses",
    "user_decryption_responses",
    "prep_keygen_requests",
    "keygen_requests",
    "crsgen_requests",
    "abort_keygen_requests",
    "abort_crsgen_requests",
    "prep_keygen_responses",
    "keygen_responses",
    "crsgen_responses",
    "new_kms_context",
    "new_kms_epoch",
    "new_kms_context_responses",
    "epoch_result_responses",
    "kms_context_destroyed",
    "kms_epoch_destroyed",
];
