//! KMS context and epoch servability.
//!
//! The pair is parsed from the permit's signed routing field, so it is the pair the wallet
//! agreed to and not one the transport chose. Validation is the same one this connector
//! already applies to the EVM routing blob: the local database maintained from protocol
//! configuration events, backed by the on-chain check.
//!
//! Rotation is not invalidation. A context switch or an epoch rotation does not by itself
//! kill outstanding permits — old-epoch shares are retained for in-flight use, and the signed
//! pair stays servable until the permit expires, is invalidated, or governance explicitly
//! destroys that epoch or context. Treating rotation as invalidation would make every key
//! resharing a mass revocation.
//!
//! ## The classification is inherited, not decided here
//!
//! This rule is the one the Connector already applies to the EVM routing blob, and it is
//! inherited whole — the same local database fed by protocol-configuration events, the same
//! on-chain check behind it. Which means its resolution is inherited too, and it is coarser than
//! the vocabulary of the management state:
//!
//! * a **destroyed context** is visible on its own, because context destruction arrives as an
//!   event and lands in the local database as a flag — terminal;
//! * everything else about the pair arrives as a single boolean from one on-chain view, which is
//!   true only when the epoch is active *and* belongs to the signed context. A `false` therefore
//!   covers three different worlds at once — the epoch is not active yet, the epoch belongs to
//!   another context, the epoch was destroyed — and no view exposes the epoch's state or its
//!   context separately, so they cannot be told apart. All three are transient.
//!
//! The consequence worth stating plainly: a request naming a *destroyed epoch* is retried within
//! the attempt budget rather than failed fast. That is inherited behaviour, not a decision of the
//! Solana path, and it is fail-closed either way. Making it terminal would mean feeding epoch
//! destruction into the local database — a change to validation the EVM path shares, and so a
//! change to EVM behaviour, which this work does not make.

use crate::core::solana_acl::SolanaPubkeyBytes;
use std::future::Future;

/// Validates a signed `(context, epoch)` pair against KMS management state.
///
/// A trait for the same reason [`super::snapshot::HostStateReader`] is one: it is the only
/// other part of authorization that is not a pure function, so keeping it behind a seam is
/// what lets the pipeline be driven from canned state in a test.
pub trait KmsPairValidator: Send + Sync {
    /// Whether this pair is currently servable.
    fn validate_pair(
        &self,
        kms_context_id: &SolanaPubkeyBytes,
        kms_epoch_id: &SolanaPubkeyBytes,
    ) -> impl Future<Output = Result<(), KmsPairFailure>> + Send;
}

/// Why a signed KMS pair is not servable.
///
/// One variant per outcome the inherited validation can actually tell apart — no finer. A
/// vocabulary richer than the source would read as knowledge nobody has: a caller would branch on
/// "the epoch was destroyed" and get a variant that nothing ever produces.
#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum KmsPairFailure {
    /// The context is unknown to this party. It may appear later.
    #[error("KMS context is unknown")]
    ContextUnknown,
    /// Governance destroyed the context. Nothing will make it servable again — the one terminal
    /// outcome, and the only one with a signal of its own.
    #[error("KMS context has been destroyed")]
    ContextDestroyed,
    /// The signed pair is not servable, and the source cannot say which of three reasons applies:
    /// the epoch is not active yet, it belongs to another context, or it was destroyed. Transient
    /// for all three, because two of them are indistinguishable from the first.
    #[error("KMS pair is not servable (epoch not active, of another context, or destroyed)")]
    PairNotServable,
    /// The management state could not be reached.
    #[error("KMS management state unavailable: {reason}")]
    Unavailable {
        /// What went wrong, for the log.
        reason: String,
    },
}
