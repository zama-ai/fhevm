//! Deployment identity: which program, which cluster, and the one equality that ties the
//! permit, the handles and this Connector together.
//!
//! A permit signs its deployment explicitly because it carries no handle at signing time and
//! so cannot derive its environment from one. The Connector's side of that comparison is its own
//! identity, and both halves of it are configuration: the program id, and the chain id of the
//! cluster it serves. That chain id is not computed here — the rule mapping a cluster to its number
//! is a deployment-time rule of the protocol, applied once per cluster and pinned in the host
//! program's own config, which every handle then embeds.
//!
//! What protects a deployment is that the number is unique per cluster, not where this process read
//! it from. A permit signed for another cluster names another chain id, and so do its handles; a
//! Connector configured with the wrong cluster's number does not accept foreign permits, it rejects
//! everything from the first request.
//!
//! The chain-id agreement itself is one equality over three values — signed, embedded in every
//! handle, configured here. There is no first-handle-wins and no majority: a batch that mixes
//! clusters is refused.

mod solana_support;

use kms_worker::core::solana::{
    deployment::{
        DeploymentFailure, DeploymentIdentity, DeploymentIdentityError, SOLANA_CHAIN_TYPE_BIT,
        check_deployment, embedded_chain_id,
    },
    failure::FailureClass,
};
use solana_support::*;

// ---------------------------------------------------------------------------
// Resolving this Connector's own identity
// ---------------------------------------------------------------------------

/// The identity is the configured pair, taken as given.
#[test]
fn the_identity_is_the_configured_program_and_chain_id() {
    let identity = DeploymentIdentity::resolve(PROGRAM_ID, CHAIN_ID)
        .expect("a Solana chain id and a program id are an identity");

    assert_eq!(identity.chain_id(), CHAIN_ID);
    assert_eq!(identity.program_id(), PROGRAM_ID);
}

/// The one thing checked about the configured value: handles embed it and routing reads the
/// chain-kind bit out of it, so a chain id without the bit matches no handle of any Solana cluster.
/// Caught at startup, where it is one log line, rather than per request, where it would look like a
/// user error every time.
#[test]
fn a_configured_chain_id_without_the_chain_kind_bit_fails_at_startup() {
    let without_bit = CHAIN_ID & !SOLANA_CHAIN_TYPE_BIT;

    let error = DeploymentIdentity::resolve(PROGRAM_ID, without_bit)
        .expect_err("a Solana chain id carries the chain-kind bit");

    assert!(matches!(
        error,
        DeploymentIdentityError::ChainKindBitMissing { chain_id } if chain_id == without_bit
    ));
    assert_eq!(error.class(), FailureClass::Terminal);
}

// ---------------------------------------------------------------------------
// Matching a request against it
// ---------------------------------------------------------------------------

/// The reference case: signed pair, embedded chain ids and this Connector's identity all agree.
#[test]
fn a_permit_naming_this_deployment_is_accepted() {
    let wallet = Wallet::new(1);
    let live = handle(0x10, FHE_TYPE_UINT64);
    let encrypted_value_account = EncryptedValueAccountFixture::new(live, &[wallet.pubkey()]);
    let request = RequestBuilder::new(&wallet)
        .direct_current(&encrypted_value_account, live)
        .typed();

    check_deployment(&request, &deployment()).expect("the permit names this deployment");
}

/// One program id may be deployed to several clusters, so the program half of the pair is not
/// enough on its own — but it is still checked, and a permit signed for another program is
/// refused here rather than surviving to the account reads.
#[test]
fn a_permit_signed_for_another_program_is_rejected() {
    let wallet = Wallet::new(1);
    let live = handle(0x11, FHE_TYPE_UINT64);
    let encrypted_value_account = EncryptedValueAccountFixture::new(live, &[wallet.pubkey()]);
    let other_program = [0x55; 32];
    let request = RequestBuilder::new(&wallet)
        .permit(PermitBuilder::new(wallet.pubkey()).deployment_pair(other_program, CHAIN_ID))
        .direct_current(&encrypted_value_account, live)
        .typed();

    let failure = check_deployment(&request, &deployment())
        .expect_err("another program's permit authorizes nothing here");

    assert!(matches!(
        failure,
        DeploymentFailure::ProgramIdMismatch { signed, own }
            if signed == other_program && own == PROGRAM_ID
    ));
}

/// A permit signed for another cluster is refused even though the program id matches — which is
/// the whole reason the cluster is signed at all. A local validator reset produces a new genesis
/// hash and therefore a new deployment, and its old permits land here.
#[test]
fn a_permit_signed_for_another_cluster_is_rejected() {
    let wallet = Wallet::new(1);
    let other_chain = SOLANA_CHAIN_TYPE_BIT | 0xdead_beef;
    let live = handle_on_chain(0x12, FHE_TYPE_UINT64, other_chain);
    let encrypted_value_account = EncryptedValueAccountFixture::new(live, &[wallet.pubkey()]);
    let request = RequestBuilder::new(&wallet)
        .permit(PermitBuilder::new(wallet.pubkey()).deployment_pair(PROGRAM_ID, other_chain))
        .direct_current(&encrypted_value_account, live)
        .typed();

    let failure = check_deployment(&request, &deployment())
        .expect_err("another cluster's permit authorizes nothing here");

    assert!(matches!(
        failure,
        DeploymentFailure::ChainIdMismatch { signed, own }
            if signed == other_chain && own == CHAIN_ID
    ));
}

/// Handles of one request must embed one chain id. A mixed batch is refused rather than resolved
/// by taking the first handle as the truth — under that rule, appending a foreign-cluster handle
/// to a valid request would smuggle it in.
#[test]
fn handles_embedding_different_chain_ids_are_rejected() {
    let wallet = Wallet::new(1);
    let local = handle(0x13, FHE_TYPE_UINT64);
    let foreign_chain = SOLANA_CHAIN_TYPE_BIT | 0x1234;
    let foreign = handle_on_chain(0x14, FHE_TYPE_UINT64, foreign_chain);
    let encrypted_value_account = EncryptedValueAccountFixture::new(local, &[wallet.pubkey()]);
    let request = RequestBuilder::new(&wallet)
        .direct_current(&encrypted_value_account, local)
        .entry(
            foreign,
            wallet.pubkey(),
            encrypted_value_account.encrypted_value_id(),
            0,
            Vec::new(),
        )
        .typed();

    let failure = check_deployment(&request, &deployment()).expect_err("one request, one cluster");

    assert!(
        matches!(
            failure,
            DeploymentFailure::MixedEmbeddedChainIds {
                index: 1,
                found,
                expected
            } if found == foreign_chain && expected == CHAIN_ID
        ),
        "the rejection names the entry that disagrees, got {failure}"
    );
}

/// Every handle's embedded chain id must equal the signed one. Handles agreeing among themselves
/// is not enough: a batch of consistently foreign handles under a locally valid permit is still
/// a request about another cluster's ciphertexts.
#[test]
fn handles_embedding_a_cluster_other_than_the_signed_one_are_rejected() {
    let wallet = Wallet::new(1);
    let foreign_chain = SOLANA_CHAIN_TYPE_BIT | 0x4321;
    let foreign = handle_on_chain(0x15, FHE_TYPE_UINT64, foreign_chain);
    let encrypted_value_account = EncryptedValueAccountFixture::new(foreign, &[wallet.pubkey()]);
    let request = RequestBuilder::new(&wallet)
        .direct_current(&encrypted_value_account, foreign)
        .typed();

    let failure = check_deployment(&request, &deployment())
        .expect_err("the signed chain id and the embedded one are one value");

    assert!(matches!(
        failure,
        DeploymentFailure::EmbeddedChainIdMismatch { index: 0, embedded, signed }
            if embedded == foreign_chain && signed == CHAIN_ID
    ));
}

/// The embedded chain id is read from the bytes the handle format puts it in, big-endian, and
/// the chain-kind bit comes along with it — the same u64 the permit signs.
#[test]
fn the_embedded_chain_id_is_read_from_the_handle_bytes() {
    let live = handle(0x16, FHE_TYPE_UINT64);

    assert_eq!(embedded_chain_id(&live), CHAIN_ID);
    assert_ne!(
        CHAIN_ID & SOLANA_CHAIN_TYPE_BIT,
        0,
        "the fixture chain id carries the chain-kind bit, so the read above proves the bit \
         survives the round trip"
    );
}
