use serde_json::Value;
use zama_solana_acl::{
    create_encrypted_store_address, find_delegation_record_address, find_host_config_address,
    find_permit_invalidation_address, EncryptedStore,
};

fn key(value: &Value) -> [u8; 32] {
    bs58::decode(value.as_str().unwrap())
        .into_vec()
        .unwrap()
        .try_into()
        .unwrap()
}

#[test]
fn pda_golden() {
    let fixture: Value = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../test-fixtures/pda/pda_v1.json"
    )))
    .unwrap();
    let host = &fixture["programs"]["zamaHost"];
    let host_program = key(&host["id"]);
    let inputs = &fixture["inputs"];
    let pdas = &host["pdas"];
    for (name, (address, bump)) in [
        ("hostConfig", find_host_config_address(&host_program)),
        (
            "delegationRecord",
            find_delegation_record_address(
                &host_program,
                &key(&inputs["delegator"]),
                &key(&inputs["delegate"]),
                &key(&inputs["program"]),
                &key(&inputs["scope"]),
            ),
        ),
        (
            "invalidation",
            find_permit_invalidation_address(&host_program, &key(&inputs["user"])),
        ),
    ] {
        assert_eq!(address, key(&pdas[name]["address"]), "{name} address");
        assert_eq!(
            u64::from(bump),
            pdas[name]["bump"].as_u64().unwrap(),
            "{name} bump"
        );
    }
    let mut store = EncryptedStore {
        program: key(&inputs["program"]),
        authority: key(&inputs["authority"]),
        scope: key(&inputs["scope"]),
        bump: pdas["encryptedStore"]["bump"]
            .as_u64()
            .unwrap()
            .try_into()
            .unwrap(),
        ..Default::default()
    };
    let expected = Some(key(&pdas["encryptedStore"]["address"]));
    assert_eq!(
        create_encrypted_store_address(&host_program, &store),
        expected
    );
    store.bump = store.bump.wrapping_sub(1);
    assert_ne!(
        create_encrypted_store_address(&host_program, &store),
        expected
    );
}
