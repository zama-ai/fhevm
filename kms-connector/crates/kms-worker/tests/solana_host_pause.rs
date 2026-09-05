//! The host pause switch, from the Connector's side.
//!
//! A user decryption is the one plaintext-releasing path that never touches the host program, so
//! the operator's pause switch reaches it only by being read here. These tests state the two
//! halves of that: a paused deployment authorizes nothing, and the pause is read from the
//! deployment's own config singleton rather than from anything a request can name.

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

/// The reference direct scenario: a signer naming their own live handle.
fn direct_scenario() -> (Wallet, EncryptedValueAccountFixture, [u8; 32]) {
    let wallet = Wallet::new(1);
    let handle = handle(0x31, FHE_TYPE_UINT64);
    let encrypted_value_account = EncryptedValueAccountFixture::new(handle, &[wallet.pubkey()]);
    (wallet, encrypted_value_account, handle)
}

/// A paused host refuses a request every other rule would authorize.
#[tokio::test]
async fn a_paused_host_refuses_a_direct_request() {
    let (wallet, encrypted_value_account, handle) = direct_scenario();
    let request = RequestBuilder::new(&wallet)
        .direct_current(&encrypted_value_account, handle)
        .typed();
    let world = World::at_slot(100)
        .with_encrypted_value_account(&encrypted_value_account)
        .with_watermark(wallet.pubkey(), 0)
        .paused();
    let reader = ScriptedReader::constant(world);
    let deployment = deployment();

    let failure = authorize_request(&reader, &ServableKmsPair, context(&deployment), &request)
        .await
        .expect_err("a paused host releases no plaintext");

    assert!(matches!(
        failure,
        AuthorizationFailure::Pause(PauseFailure::Paused)
    ));
}

/// The refusal is transient: nothing about the request died, so the same bytes authorize once the
/// operator lifts the pause.
#[tokio::test]
async fn the_pause_refusal_is_transient_and_lifts_with_the_switch() {
    let (wallet, encrypted_value_account, handle) = direct_scenario();
    let request = RequestBuilder::new(&wallet)
        .direct_current(&encrypted_value_account, handle)
        .typed();
    let running = World::at_slot(100)
        .with_encrypted_value_account(&encrypted_value_account)
        .with_watermark(wallet.pubkey(), 0);
    let deployment = deployment();

    let failure = authorize_request(
        &ScriptedReader::constant(running.clone().paused()),
        &ServableKmsPair,
        context(&deployment),
        &request,
    )
    .await
    .expect_err("a paused host releases no plaintext");
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

/// A delegated request is refused by the same rule, and before either delegation row is judged: a
/// paused host is a fact about the deployment, so the failure names the pause rather than an entry.
#[tokio::test]
async fn a_paused_host_refuses_a_delegated_request() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let handle = handle(0x32, FHE_TYPE_UINT64);
    let encrypted_value_account = EncryptedValueAccountFixture::new(handle, &[delegator.pubkey()]);
    let delegation = DelegationFixture::live(delegator.pubkey(), signer.pubkey(), 100);
    let request = RequestBuilder::new(&signer)
        .delegated_current(&encrypted_value_account, handle, delegator.pubkey())
        .typed();
    let world = World::at_slot(100)
        .with_encrypted_value_account(&encrypted_value_account)
        .with_watermark(signer.pubkey(), 0)
        .with_delegation(&delegation)
        .paused();
    let reader = ScriptedReader::constant(world);
    let deployment = deployment();

    let failure = authorize_request(&reader, &ServableKmsPair, context(&deployment), &request)
        .await
        .expect_err("a paused host releases no plaintext, delegated or not");

    assert!(matches!(
        failure,
        AuthorizationFailure::Pause(PauseFailure::Paused)
    ));
}

/// The switch costs no round trip of its own: the singleton is one of the keys the first read
/// already plans, and a paused deployment is decided from the same observation as everything else.
#[tokio::test]
async fn the_config_singleton_is_read_with_the_rest_of_the_state() {
    let (wallet, encrypted_value_account, handle) = direct_scenario();
    let request = RequestBuilder::new(&wallet)
        .direct_current(&encrypted_value_account, handle)
        .typed();
    let world = World::at_slot(100)
        .with_encrypted_value_account(&encrypted_value_account)
        .with_watermark(wallet.pubkey(), 0);
    let reader = ScriptedReader::constant(world);
    let deployment = deployment();

    authorize_request(&reader, &ServableKmsPair, context(&deployment), &request)
        .await
        .expect("a running host authorizes the reference request");

    let (host_config_key, _) = host_config_address();
    assert_eq!(reader.call_count(), 1, "one read, singleton included");
    assert!(
        reader.call(0).contains(&host_config_key),
        "the config singleton is planned with the rest of the first read"
    );
}

/// An absent singleton is a refusal, not a `false`. The address is derivable by anyone, so reading
/// "nothing here" as "not paused" would let a deployment that has never been configured — or a
/// Connector pointed at the wrong program — serve plaintext with the switch disarmed.
#[tokio::test]
async fn a_missing_config_singleton_refuses_rather_than_reading_as_running() {
    let (wallet, encrypted_value_account, handle) = direct_scenario();
    let request = RequestBuilder::new(&wallet)
        .direct_current(&encrypted_value_account, handle)
        .typed();
    let (host_config_key, _) = host_config_address();
    let world = World::at_slot(100)
        .with_encrypted_value_account(&encrypted_value_account)
        .with_watermark(wallet.pubkey(), 0)
        .without_account(&host_config_key);
    let reader = ScriptedReader::constant(world);
    let deployment = deployment();

    let failure = authorize_request(&reader, &ServableKmsPair, context(&deployment), &request)
        .await
        .expect_err("an unreadable switch is a closed switch");

    assert!(matches!(
        failure,
        AuthorizationFailure::Pause(PauseFailure::Absent { .. })
    ));
    assert_eq!(failure.class(), FailureClass::Transient);
}

/// An account of another layout at the singleton's address is refused too: decoding a pause flag
/// out of foreign bytes is exactly how the switch would silently read as off.
#[tokio::test]
async fn an_account_of_another_layout_at_the_singleton_address_is_refused() {
    let (wallet, encrypted_value_account, handle) = direct_scenario();
    let request = RequestBuilder::new(&wallet)
        .direct_current(&encrypted_value_account, handle)
        .typed();
    let (host_config_key, _) = host_config_address();
    let mut foreign = host_config_account(true);
    foreign.data.truncate(foreign.data.len() - 1);
    let world = World::at_slot(100)
        .with_encrypted_value_account(&encrypted_value_account)
        .with_watermark(wallet.pubkey(), 0)
        .with_account(host_config_key, foreign);
    let reader = ScriptedReader::constant(world);
    let deployment = deployment();

    let failure = authorize_request(&reader, &ServableKmsPair, context(&deployment), &request)
        .await
        .expect_err("a truncated singleton is not a singleton");

    assert!(matches!(
        failure,
        AuthorizationFailure::Pause(PauseFailure::NotAHostConfig { .. })
    ));
}

/// A singleton owned by another program is refused: only the host program can say whether the host
/// is paused.
#[tokio::test]
async fn a_foreign_owned_singleton_is_refused() {
    let (wallet, encrypted_value_account, handle) = direct_scenario();
    let request = RequestBuilder::new(&wallet)
        .direct_current(&encrypted_value_account, handle)
        .typed();
    let (host_config_key, _) = host_config_address();
    let impostor = SnapshotAccount {
        owner: [0x77; 32],
        ..host_config_account(false)
    };
    let world = World::at_slot(100)
        .with_encrypted_value_account(&encrypted_value_account)
        .with_watermark(wallet.pubkey(), 0)
        .with_account(host_config_key, impostor);
    let reader = ScriptedReader::constant(world);
    let deployment = deployment();

    let failure = authorize_request(&reader, &ServableKmsPair, context(&deployment), &request)
        .await
        .expect_err("another program's account is not this host's config");

    assert!(matches!(
        failure,
        AuthorizationFailure::Pause(PauseFailure::ForeignOwner { .. })
    ));
}
