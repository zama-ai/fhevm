use serde_json::Value;

const HOST_IDL: &str =
    include_str!("../../../coprocessor/fhevm-engine/host-listener/idl/zama_host.json");
const TOKEN_IDL: &str =
    include_str!("../../../coprocessor/fhevm-engine/host-listener/idl/confidential_token.json");
const HOST_CARGO: &str = include_str!("../../programs/zama-host/Cargo.toml");
const TOKEN_CARGO: &str = include_str!("../../programs/confidential-token/Cargo.toml");
const HOST_LIB: &str = include_str!("../../programs/zama-host/src/lib.rs");
const HOST_STATE: &str = include_str!("../../programs/zama-host/src/state/mod.rs");
const HOST_CONFIG: &str = include_str!("../../programs/zama-host/src/state/host_config.rs");
const TOKEN_LIB: &str = include_str!("../../programs/confidential-token/src/lib.rs");
const TOKEN_COMMON: &str =
    include_str!("../../programs/confidential-token/src/instructions/common.rs");
const TOKEN_WRAP_USDC: &str =
    include_str!("../../programs/confidential-token/src/instructions/wrap_usdc.rs");
const IDL_CHECK_SCRIPT: &str = include_str!("../../scripts/check-zama-host-idl.sh");
const SOLANA_ABI_CHECK: &str = include_str!("../../scripts/check_solana_abi.py");
const HOST_LISTENER_BUILD: &str =
    include_str!("../../../coprocessor/fhevm-engine/host-listener/build.rs");
const SOLANA_ABI_GOLDEN: &str =
    include_str!("../../../coprocessor/fhevm-engine/host-listener/idl/solana_abi_golden.json");

#[test]
fn host_idl_drops_verifier_set_and_keeps_secp_kms_context_path() {
    let idl = parse_idl(HOST_IDL);
    let accounts = names(&idl, "accounts");
    let types = names(&idl, "types");
    let instructions = names(&idl, "instructions");

    // The threshold-Ed25519 VerifierSet subsystem is fully removed: input binding, disclosure,
    // and redemption all authenticate against the secp256k1 KMS context now.
    assert!(
        !accounts
            .iter()
            .any(|name| name == "verifier_set" || name == "VerifierSet"),
        "zama-host IDL must not expose a VerifierSet account"
    );
    assert!(
        !types.iter().any(|name| name == "VerifierSet"),
        "zama-host IDL must not define a VerifierSet layout"
    );
    assert!(
        !instructions
            .iter()
            .any(|name| name.contains("verifier_set")),
        "zama-host IDL must not expose any verifier-set instruction"
    );

    // The canonical KMS context is the signer authority for secp256k1 certificates.
    assert!(
        accounts.iter().any(|name| name == "KmsContext"),
        "zama-host IDL must expose the KmsContext account"
    );
    let fields = type_field_names(&idl, "KmsContext");
    for required in ["context_id", "signers", "thresholds", "destroyed", "bump"] {
        assert!(
            fields.iter().any(|field| field == required),
            "KmsContext is missing field `{required}`"
        );
    }
    // Encrypted inputs are verified in-frame by the `fhe_eval` `VerifiedInput` operand
    // (the fromExternal path); there is no standalone verify_coprocessor_input instruction.
    assert!(
        !instructions
            .iter()
            .any(|name| name == "verify_coprocessor_input"),
        "standalone verify_coprocessor_input must be removed (input verification is inline in fhe_eval)"
    );
}

#[test]
fn token_idl_removed_operator_surface_and_splits_payer_from_owner() {
    let idl = parse_idl(TOKEN_IDL);
    let instructions = names(&idl, "instructions");
    let accounts = names(&idl, "accounts");
    let types = names(&idl, "types");

    for removed in [
        "set_operator",
        "close_operator",
        "confidential_transfer_from",
        "confidential_call_transfer_receiver_from",
    ] {
        assert!(
            !instructions.iter().any(|name| name == removed),
            "production token IDL must not expose removed operator instruction `{removed}`"
        );
    }
    for removed in ["confidential_operator", "ConfidentialOperator"] {
        assert!(
            !accounts.iter().any(|name| name == removed)
                && !types.iter().any(|name| name == removed),
            "production token IDL must not expose removed operator state `{removed}`"
        );
    }

    let transfer_accounts = instruction_account_names(&idl, "confidential_transfer");
    let owner_index = transfer_accounts
        .iter()
        .position(|account| account == "owner")
        .expect("confidential_transfer must keep owner authority account");
    let payer_index = transfer_accounts
        .iter()
        .position(|account| account == "payer")
        .expect("confidential_transfer must add distinct payer account");
    assert_ne!(
        owner_index, payer_index,
        "owner authority and payer must be separate account metas even when callers pass the same key"
    );
    // fromExternal: the transfer amount is a coprocessor-attested instruction argument, not a
    // durable amount_compute_acl witness account.
    assert!(
        !transfer_accounts
            .iter()
            .any(|account| account == "amount_compute_acl"),
        "confidential_transfer amount is an attested external input, not a durable amount_compute_acl account"
    );

    // There is no per-mint verifier-set rotation surface; disclosure/redemption requests pin a
    // KMS context id and the response verifies a secp256k1 cert against that context.
    for removed in ["update_mint_verifier_sets", "migrate_mint_verifier_sets"] {
        assert!(
            !instructions.iter().any(|name| name == removed),
            "production token IDL must not expose removed verifier-set instruction `{removed}`"
        );
    }
    let mint_fields = type_field_names(&idl, "ConfidentialMint");
    for removed in ["disclosure_verifier_set", "redemption_verifier_set"] {
        assert!(
            !mint_fields.iter().any(|field| field == removed),
            "ConfidentialMint must not retain verifier-set field `{removed}`"
        );
    }
    let disclosure_fields = type_field_names(&idl, "DisclosureRequest");
    assert!(
        disclosure_fields
            .iter()
            .any(|field| field == "kms_context_id"),
        "DisclosureRequest must pin a kms_context_id"
    );
    assert!(
        !disclosure_fields
            .iter()
            .any(|field| field == "verifier_set" || field == "verifier_set_version"),
        "DisclosureRequest must not retain verifier-set fields"
    );

    let source = format!("{TOKEN_LIB}\n{TOKEN_COMMON}");
    for removed in [
        "ConfidentialTransferFrom",
        "ConfidentialCallTransferReceiverFrom",
        "SetOperator",
        "CloseOperator",
        "ConfidentialOperator",
    ] {
        assert!(
            !source.contains(removed),
            "operator symbol `{removed}` must be removed from production token source"
        );
    }
}

#[test]
fn transient_wrap_does_not_leave_durable_acl_contracts() {
    let idl = parse_idl(TOKEN_IDL);
    let wrap_accounts = instruction_account_names(&idl, "wrap_usdc");
    assert!(
        !wrap_accounts
            .iter()
            .any(|account| account == "amount_compute_acl"),
        "wrap_usdc must not require a durable amount_compute_acl for public deposit amount"
    );
    assert!(
        TOKEN_WRAP_USDC.contains("Output::transient()")
            || TOKEN_WRAP_USDC.contains("transient_output")
            || TOKEN_WRAP_USDC.contains("TrivialAmount::transient"),
        "wrap_usdc should trivial-encrypt the public amount as an instruction-local transient value"
    );
}

#[test]
fn token_idl_drops_transfer_and_call_callback_surface() {
    let idl = parse_idl(TOKEN_IDL);
    let instructions = names(&idl, "instructions");
    let accounts = names(&idl, "accounts");
    let types = names(&idl, "types");

    // The ported transfer-and-call callback flow (issue #1593) is replaced by app-driven CPI
    // composition, so its instructions and settlement state must be gone.
    for removed in [
        "confidential_call_transfer_receiver",
        "confidential_prepare_transfer_callback",
        "confidential_finalize_transfer_callback",
        "test_receiver_return_callback",
    ] {
        assert!(
            !instructions.iter().any(|name| name == removed),
            "callback instruction `{removed}` must be removed from the token IDL"
        );
    }
    for removed in ["TransferCallbackSettlement", "TransferReceiverHookCall"] {
        assert!(
            !accounts.iter().any(|name| name == removed)
                && !types.iter().any(|name| name == removed),
            "callback state `{removed}` must be removed from the token IDL"
        );
    }
}

#[test]
fn token_request_witnesses_bind_handle_lineage_and_secp_kms_context() {
    let source = TOKEN_COMMON;
    // The request witness binds the request to its accounts, the handle and its
    // `EncryptedValue` lineage account (which replaced the deleted
    // `HandleMaterialCommitment` material_* fields), host config, chain id, and
    // the pinned KMS context id; the response then verifies a secp256k1 KMS cert.
    for required in [
        "request_hash",
        "kms_context_id",
        "request.handle == handle",
        "request.encrypted_value == encrypted_value",
        "host_config",
        "chain_id",
        "assert_kms_public_decrypt_cert_for_request",
        "extract_kms_context_id",
        "verify_kms_public_decrypt",
    ] {
        assert!(
            source.contains(required),
            "token request/consume path must bind `{required}`"
        );
    }
    // The dead Ed25519 verifier-set message helpers and the removed
    // handle-material commitment surface must be gone.
    for removed in [
        "proof_message_v2",
        "assert_threshold_verifier_signature",
        "verifier_set_version",
        "material_commitment_hash",
        "material_key_id",
        "HandleMaterialCommitment",
    ] {
        assert!(
            !source.contains(removed),
            "legacy verifier-set helper `{removed}` should not remain in production paths"
        );
    }
}

#[test]
fn production_poc_paths_are_compile_feature_gated_and_runtime_rejected() {
    assert!(
        HOST_CARGO.contains("poc"),
        "zama-host Cargo.toml must define a `poc` feature for mock/test-only paths"
    );
    assert!(
        TOKEN_CARGO.contains("poc"),
        "confidential-token Cargo.toml must define a `poc` feature for receiver test paths"
    );

    for symbol in [
        "mock_input_verified_and_bind",
        "test_emit_input_verified",
        "test_emit_acl_allowed",
        "test_emit_trivial_encrypt",
        "test_emit_fhe_rand",
    ] {
        assert!(
            cfg_gated_symbol(HOST_LIB, symbol) || cfg_gated_symbol(HOST_LIB, &camelish(symbol)),
            "`{symbol}` must be gated out of production/default builds"
        );
    }
    assert!(
        cfg_gated_symbol(TOKEN_LIB, "test_receiver_return_callback")
            || cfg_gated_symbol(TOKEN_LIB, "TestReceiverReturnCallback"),
        "`test_receiver_return_callback` must be gated out of production/default token builds"
    );
    for symbol in ["create_random_amount", "create_random_bounded_amount"] {
        assert!(
            cfg_gated_symbol(TOKEN_LIB, symbol) || cfg_gated_symbol(TOKEN_LIB, &camelish(symbol)),
            "`{symbol}` must be gated out of production/default token builds"
        );
    }

    let token_idl = parse_idl(TOKEN_IDL);
    let token_instructions = names(&token_idl, "instructions");
    assert!(
        !token_instructions
            .iter()
            .any(|name| name == "test_receiver_return_callback"),
        "production token IDL must not expose test_receiver_return_callback"
    );
    for removed in ["create_random_amount", "create_random_bounded_amount"] {
        assert!(
            !token_instructions.iter().any(|name| name == removed),
            "production token IDL must not expose `poc`-gated demo helper `{removed}`"
        );
    }

    assert!(
        HOST_CONFIG.contains("SOLANA_POC_CHAIN_ID") && HOST_CONFIG.contains("poc"),
        "HostConfig validation must reject SOLANA_POC_CHAIN_ID outside the poc feature"
    );
    assert!(
        HOST_CONFIG.contains("mock_input_enabled") && HOST_CONFIG.contains("test_shims_enabled"),
        "production initialization must reject enabled mock/test flags"
    );
}

#[test]
fn handle_byte_layout_is_preserved_while_entropy_policy_is_deferred() {
    for required in [
        "handle_chain_id(handle: [u8; 32])",
        "handle[22..30]",
        "handle_fhe_type(handle: [u8; 32])",
        "handle[30]",
        "handle[31] == HANDLE_VERSION",
        "HANDLE_VERSION",
        "COMPUTED_HANDLE_MARKER",
    ] {
        assert!(
            HOST_STATE.contains(required),
            "handle byte layout must preserve `{required}`"
        );
    }
}

#[test]
fn abi_golden_drift_checks_cover_host_token_listener_and_kms_layouts() {
    let golden = parse_idl(SOLANA_ABI_GOLDEN);
    assert!(
        golden
            .get("pending_schemas")
            .and_then(Value::as_array)
            .is_some_and(|pending| pending.is_empty()),
        "Solana ABI golden manifest must not carry pending schemas"
    );

    let schemas = golden
        .get("schemas")
        .and_then(Value::as_array)
        .expect("ABI golden schemas should be an array");
    for required in ["KmsContext", "DisclosureRequest", "BurnRedemptionRequest"] {
        assert!(
            schemas
                .iter()
                .any(|schema| { schema.get("name").and_then(Value::as_str) == Some(required) }),
            "ABI golden manifest must pin `{required}`"
        );
    }
    for removed in [
        "VerifierSet",
        "OperatorSetEvent",
        "OperatorClosedEvent",
        // Deleted by the EncryptedValue ACL rewrite.
        "AclRecord",
        "HandleMaterialCommitment",
    ] {
        assert!(
            !schemas
                .iter()
                .any(|schema| { schema.get("name").and_then(Value::as_str) == Some(removed) }),
            "ABI golden manifest must not pin removed schema `{removed}`"
        );
    }

    for required in [
        "zama_host.json",
        "confidential_token.json",
        "HostConfig",
        "KmsContext",
        // The ACL rewrite's lineage account replaces AclRecord/HandleMaterialCommitment.
        "EncryptedValue",
    ] {
        assert!(
            IDL_CHECK_SCRIPT.contains(required)
                || SOLANA_ABI_CHECK.contains(required)
                || HOST_LISTENER_BUILD.contains(required),
            "ABI drift tooling must include `{required}`"
        );
    }
    assert!(
        SOLANA_ABI_CHECK.contains("schema_hash") || SOLANA_ABI_CHECK.contains("golden"),
        "listener generated decode path must expose schema hashes or golden decode checks"
    );
}

fn parse_idl(json: &str) -> Value {
    serde_json::from_str(json).expect("IDL JSON should parse")
}

fn names(idl: &Value, section: &str) -> Vec<String> {
    idl.get(section)
        .and_then(Value::as_array)
        .expect("IDL section should be an array")
        .iter()
        .filter_map(|item| item.get("name").and_then(Value::as_str))
        .map(ToOwned::to_owned)
        .collect()
}

fn instruction_account_names(idl: &Value, instruction_name: &str) -> Vec<String> {
    idl.get("instructions")
        .and_then(Value::as_array)
        .expect("IDL instructions should be an array")
        .iter()
        .find(|instruction| {
            instruction.get("name").and_then(Value::as_str) == Some(instruction_name)
        })
        .unwrap_or_else(|| panic!("missing instruction `{instruction_name}`"))
        .get("accounts")
        .and_then(Value::as_array)
        .expect("instruction accounts should be an array")
        .iter()
        .filter_map(|account| account.get("name").and_then(Value::as_str))
        .map(ToOwned::to_owned)
        .collect()
}

fn type_field_names(idl: &Value, type_name: &str) -> Vec<String> {
    idl.get("types")
        .and_then(Value::as_array)
        .expect("IDL types should be an array")
        .iter()
        .find(|ty| ty.get("name").and_then(Value::as_str) == Some(type_name))
        .unwrap_or_else(|| panic!("missing type `{type_name}`"))
        .get("type")
        .and_then(|ty| ty.get("fields"))
        .and_then(Value::as_array)
        .expect("type fields should be an array")
        .iter()
        .filter_map(|field| field.get("name").and_then(Value::as_str))
        .map(ToOwned::to_owned)
        .collect()
}

fn cfg_gated_symbol(source: &str, symbol: &str) -> bool {
    let Some(index) = source.find(symbol) else {
        return true;
    };
    let prefix_start = index.saturating_sub(256);
    let prefix = &source[prefix_start..index];
    prefix.contains("cfg(feature = \"poc\")")
        || prefix.contains("cfg_attr(feature = \"poc\"")
        || prefix.contains("feature = \"poc\"")
}

fn camelish(symbol: &str) -> String {
    symbol
        .split('_')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<String>()
}
