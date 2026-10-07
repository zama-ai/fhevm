use crate::{pda_vectors::PdaVectors, state::*};
use anchor_lang::prelude::*;

#[test]
fn pda_golden() {
    let vectors = PdaVectors::load();
    let app = AppScope {
        program: vectors.key("program"),
        scope: vectors.key("scope"),
    };
    let pdas = [
        ("hostConfig", host_config_address()),
        (
            "kmsContext",
            kms_context_address(
                serde_json::from_value(vectors.fixture["inputs"]["contextId"].clone()).unwrap(),
            ),
        ),
        ("randNonce", rand_nonce_address()),
        (
            "encryptedStore",
            encrypted_store_address(app.program, vectors.key("authority"), app.scope),
        ),
        (
            "transientStore",
            transient_store_address(vectors.key("payer")),
        ),
        (
            "delegationRecord",
            user_decryption_delegation_address(
                vectors.key("delegator"),
                vectors.key("delegate"),
                app,
            ),
        ),
        (
            "invalidation",
            permit_invalidation_address(vectors.key("user")),
        ),
        ("denyScopeRecord", deny_scope_address(app)),
        ("hcuTrustedAppRecord", hcu_trusted_app_address(app)),
        ("hcuBlockMeter", hcu_block_meter_address(app)),
        ("pauserRecord", pauser_address(vectors.key("user"))),
        (
            "eventAuthority",
            Pubkey::find_program_address(&[b"__event_authority"], &crate::ID),
        ),
    ];
    vectors.check("zamaHost", crate::ID, &pdas);
}
