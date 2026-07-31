//! Deployment identity: which program, which cluster, and the one equality that ties the
//! permit, the handles and this Connector together.
//!
//! A permit signs its deployment explicitly because it carries no handle at signing time and
//! so cannot derive its environment from one. The Connector's side of that comparison is its
//! own identity, and the load-bearing decision is that the chain id is *derived* from the
//! cluster's genesis hash rather than configured. A configured constant is identical across
//! environments, which is exactly the condition under which a signature for one deployment
//! verifies against another.
//!
//! Two failure surfaces follow, and they are deliberately different in kind. A configured pin
//! that disagrees with the derivation is a startup failure: the process must not run, because
//! every request would fail deployment matching for a reason no log line would explain. A
//! permit naming another deployment is an ordinary request rejection.
//!
//! The chain-id agreement itself is one equality over three values — signed, embedded in every
//! handle, derived here. There is no first-handle-wins and no majority: a batch that mixes
//! clusters is refused.

mod solana_support;

use kms_worker::core::solana::{
    deployment::{
        ChainIdDerivation, DeploymentFailure, DeploymentIdentity, DeploymentIdentityError,
        SOLANA_CHAIN_TYPE_BIT, check_deployment, embedded_chain_id,
    },
    failure::FailureClass,
};
use solana_support::*;

/// A derivation that returns whatever it was told to, so a test can describe a cluster whose
/// derived chain id is inconvenient.
struct FixedDerivation(u64);

impl ChainIdDerivation for FixedDerivation {
    fn derive_chain_id(&self, _genesis_hash: &[u8; 32]) -> u64 {
        self.0
    }
}

// ---------------------------------------------------------------------------
// Resolving this Connector's own identity
// ---------------------------------------------------------------------------

/// With no pin in configuration, the chain id is whatever the cluster's genesis hash derives.
/// Nothing about it is configurable, which is the property that makes a devnet permit useless
/// against mainnet.
#[test]
fn the_chain_id_is_derived_from_the_genesis_hash() {
    let identity = DeploymentIdentity::resolve(PROGRAM_ID, &GENESIS_HASH, None, &StandInDerivation)
        .expect("a cluster with a genesis hash has a chain id");

    assert_eq!(identity.chain_id(), CHAIN_ID);
    assert_eq!(identity.program_id(), PROGRAM_ID);
}

/// A pin is allowed as an operational convenience, and it is cross-checked. Agreeing changes
/// nothing.
#[test]
fn a_configured_chain_id_that_agrees_with_the_derivation_is_accepted() {
    let identity = DeploymentIdentity::resolve(
        PROGRAM_ID,
        &GENESIS_HASH,
        Some(CHAIN_ID),
        &StandInDerivation,
    )
    .expect("a pin equal to the derivation is redundant, not wrong");

    assert_eq!(identity.chain_id(), CHAIN_ID);
}

/// A pin that disagrees stops the process. The alternative — preferring one of the two values —
/// would run a Connector whose idea of its own cluster matches nothing on that cluster, and
/// every rejection downstream would look like a user error.
#[test]
fn a_configured_chain_id_that_disagrees_with_the_derivation_fails_at_startup() {
    let wrong_pin = CHAIN_ID ^ 1;

    let error = DeploymentIdentity::resolve(
        PROGRAM_ID,
        &GENESIS_HASH,
        Some(wrong_pin),
        &StandInDerivation,
    )
    .expect_err("a pin disagreeing with the cluster is a misconfiguration, not a request problem");

    assert!(matches!(
        error,
        DeploymentIdentityError::PinnedChainIdMismatch {
            pinned,
            derived
        } if pinned == wrong_pin && derived == CHAIN_ID
    ));
    assert_eq!(error.class(), FailureClass::Terminal);
}

/// The derived value has to be usable as a host chain id: handles embed it, and routing reads
/// the chain-kind bit out of it. A derivation that loses the bit is caught where it is cheap to
/// notice rather than per request.
#[test]
fn a_derived_chain_id_without_the_chain_kind_bit_fails_at_startup() {
    let without_bit = CHAIN_ID & !SOLANA_CHAIN_TYPE_BIT;

    let error = DeploymentIdentity::resolve(
        PROGRAM_ID,
        &GENESIS_HASH,
        None,
        &FixedDerivation(without_bit),
    )
    .expect_err("a Solana chain id carries the chain-kind bit");

    assert!(matches!(
        error,
        DeploymentIdentityError::ChainKindBitMissing { chain_id } if chain_id == without_bit
    ));
}

// ---------------------------------------------------------------------------
// Matching a request against it
// ---------------------------------------------------------------------------

/// The reference case: signed pair, embedded chain ids and this Connector's identity all agree.
#[test]
fn a_permit_naming_this_deployment_is_accepted() {
    let wallet = Wallet::new(1);
    let live = handle(0x10, FHE_TYPE_UINT64);
    let lineage = LineageFixture::new(live, &[wallet.pubkey()]);
    let request = RequestBuilder::new(&wallet)
        .direct_current(&lineage, live)
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
    let lineage = LineageFixture::new(live, &[wallet.pubkey()]);
    let other_program = [0x55; 32];
    let request = RequestBuilder::new(&wallet)
        .permit(PermitBuilder::new(wallet.pubkey()).deployment_pair(other_program, CHAIN_ID))
        .direct_current(&lineage, live)
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
    let lineage = LineageFixture::new(live, &[wallet.pubkey()]);
    let request = RequestBuilder::new(&wallet)
        .permit(PermitBuilder::new(wallet.pubkey()).deployment_pair(PROGRAM_ID, other_chain))
        .direct_current(&lineage, live)
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
    let lineage = LineageFixture::new(local, &[wallet.pubkey()]);
    let request = RequestBuilder::new(&wallet)
        .direct_current(&lineage, local)
        .entry(foreign, wallet.pubkey(), lineage.value_key(), 0, Vec::new())
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
    let lineage = LineageFixture::new(foreign, &[wallet.pubkey()]);
    let request = RequestBuilder::new(&wallet)
        .direct_current(&lineage, foreign)
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
