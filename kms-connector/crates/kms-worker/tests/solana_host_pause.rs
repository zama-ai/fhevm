//! The host pause switch, from the Connector's side.
//!
//! A user decryption is the one plaintext-releasing path that never touches the host program, so
//! the operator's switch reaches it only by being read here. Three properties: a paused
//! deployment authorizes nothing and a running one authorizes the same bytes, the switch is read
//! on the *first* read so a paused host costs one round trip rather than two, and a singleton
//! this Connector cannot read is a refusal rather than a `false`.

mod solana_support;

use kms_worker::core::solana::{
    failure::{AuthorizationFailure, FailureClass},
    pause::PauseFailure,
    pipeline::{AuthorizationContext, authorize_request},
    snapshot::SnapshotAccount,
};
use solana_support::*;

fn context<'a>(
    deployment: &'a kms_worker::core::solana::deployment::DeploymentIdentity,
) -> AuthorizationContext<'a> {
    AuthorizationContext {
        deployment,
        now_unix_seconds: NOW_INSIDE_WINDOW,
    }
}

/// A running world whose single direct entry every other rule authorizes, so a refusal in these
/// tests can only be the pause rule.
fn direct_scenario() -> (Wallet, EncryptedValueAccountFixture, [u8; 32], World) {
    let wallet = Wallet::new(1);
    let handle = handle(0x31, FHE_TYPE_UINT64);
    let encrypted_value_account = EncryptedValueAccountFixture::new(handle, &[wallet.pubkey()]);
    let world = World::running_at_slot(100)
        .with_encrypted_value_account(&encrypted_value_account)
        .with_watermark(wallet.pubkey(), 0);
    (wallet, encrypted_value_account, handle, world)
}

/// A paused host refuses a request every other rule authorizes, and the refusal is transient:
/// nothing about the request died, so the identical bytes authorize once the operator lifts it.
#[tokio::test]
async fn a_paused_host_refuses_until_the_switch_is_lifted() {
    let (wallet, encrypted_value_account, handle, running) = direct_scenario();
    let request = RequestBuilder::new(&wallet)
        .direct_current(&encrypted_value_account, handle)
        .typed();
    let deployment = deployment();

    let failure = authorize_request(
        &ScriptedReader::constant(running.clone().paused()),
        &ServableKmsPair,
        context(&deployment),
        &request,
    )
    .await
    .expect_err("a paused host releases no plaintext");
    assert!(matches!(
        failure,
        AuthorizationFailure::Pause(PauseFailure::Paused)
    ));
    assert_eq!(failure.class(), FailureClass::Transient);

    authorize_request(
        &ScriptedReader::constant(running),
        &ServableKmsPair,
        context(&deployment),
        &request,
    )
    .await
    .expect("the identical request authorizes once the host is running again");
}

/// The switch is read on the first read, not the deciding one. A delegated request reads twice and
/// the two worlds disagree about pause: a paused first read refuses without ever taking the second,
/// and a running first read authorizes even though the second world is paused. That asymmetry is
/// the design rather than a tolerance — see [`kms_worker::core::solana::pause`] for why pause is
/// decided one observation earlier than every authorization rule.
#[tokio::test]
async fn the_switch_is_read_on_the_first_read_of_a_delegated_request() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let handle = handle(0x32, FHE_TYPE_UINT64);
    let encrypted_value_account = EncryptedValueAccountFixture::new(handle, &[delegator.pubkey()]);
    let delegation = DelegationFixture::live(delegator.pubkey(), signer.pubkey(), 100);
    let request = RequestBuilder::new(&signer)
        .delegated_current(&encrypted_value_account, handle, delegator.pubkey())
        .typed();
    let running = World::running_at_slot(100)
        .with_encrypted_value_account(&encrypted_value_account)
        .with_watermark(signer.pubkey(), 0)
        .with_delegation(&delegation);
    let deployment = deployment();

    let paused_first = ScriptedReader::scripted(vec![running.clone().paused(), running.clone()]);
    let failure = authorize_request(
        &paused_first,
        &ServableKmsPair,
        context(&deployment),
        &request,
    )
    .await
    .expect_err("a paused first read refuses however the second read reads");
    assert!(matches!(
        failure,
        AuthorizationFailure::Pause(PauseFailure::Paused)
    ));
    assert_eq!(
        paused_first.call_count(),
        1,
        "a paused host is refused before the second round trip"
    );

    let running_first = ScriptedReader::scripted(vec![running.clone(), running.paused()]);
    authorize_request(
        &running_first,
        &ServableKmsPair,
        context(&deployment),
        &request,
    )
    .await
    .expect("the switch was off when it was read; the deciding read does not carry it");
    assert_eq!(running_first.call_count(), 2);
}

/// Everything that is not this deployment's own singleton is a refusal, never a `false`. Absence
/// is the sharpest case: the address is derivable by anyone, so both "nothing here" and "somebody
/// funded it" must refuse rather than read as "not paused".
#[tokio::test]
async fn a_singleton_this_connector_cannot_read_refuses_rather_than_reading_as_running() {
    let (wallet, encrypted_value_account, handle, running) = direct_scenario();
    let request = RequestBuilder::new(&wallet)
        .direct_current(&encrypted_value_account, handle)
        .typed();
    let (key, _) = host_config_address();
    let deployment = deployment();

    let mut truncated = host_config_account(true);
    truncated.data.truncate(truncated.data.len() - 1);
    let foreign_owner = SnapshotAccount {
        owner: [0x77; 32],
        ..host_config_account(false)
    };
    let cases = [
        (
            "no account at the singleton address",
            running.clone().without_account(&key),
            PauseFailure::Absent { account_key: key },
        ),
        (
            "a bare transfer to the address, before the program ever wrote there",
            running.clone().with_account(key, prefunded_account()),
            PauseFailure::Absent { account_key: key },
        ),
        (
            "an account of another layout",
            running.clone().with_account(key, truncated),
            PauseFailure::NotAHostConfig { account_key: key },
        ),
        (
            "another program's account",
            running.with_account(key, foreign_owner),
            PauseFailure::ForeignOwner {
                account_key: key,
                owner: [0x77; 32],
                expected: PROGRAM_ID,
            },
        ),
    ];

    for (what, world, expected) in cases {
        let outcome = authorize_request(
            &ScriptedReader::constant(world),
            &ServableKmsPair,
            context(&deployment),
            &request,
        )
        .await;
        let Err(failure) = outcome else {
            panic!("{what}: an unreadable switch is a closed switch");
        };
        assert_eq!(failure, AuthorizationFailure::Pause(expected), "{what}");
    }
}
