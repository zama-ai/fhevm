//! Deployment identity: which program, which cluster.
//!
//! A permit carries no handle at signing time, so it cannot derive its environment from one
//! — the deployment is signed explicitly as `(verifying_program_id, chain_id)`. The
//! Connector compares that pair against its **own** identity, and both halves of it come from
//! configuration: the program id, and the chain id of the cluster this Connector serves.
//!
//! The chain id is configured rather than computed here on purpose. It is not a free constant: it
//! is the same u64 the host program's `HostConfig` was initialized with and that every handle of
//! the cluster embeds in bytes `[22..30]`, and the rule that turns a cluster into that number is a
//! deployment-time rule stated in the protocol specification, applied once per cluster. Recomputing
//! it here — from a genesis hash fetched at startup, say — would put a second implementation of
//! that rule in this process, and a second implementation is a second answer.
//!
//! What protects a deployment is therefore not the provenance of the value but its uniqueness per
//! cluster, and [`check_deployment`] is where that is spent: a permit signed for another cluster
//! names another chain id, and so do the handles it lists. A Connector configured with the wrong
//! cluster's chain id does not quietly accept foreign permits — it rejects everything, loudly, from
//! the first request.
//!
//! One thing about the value is still checked here, because it is cheap and because handles cannot
//! route without it: the chain-kind high bit.

use crate::core::solana_acl::SolanaPubkeyBytes;

/// The chain-kind high bit: set for a Solana host chain, clear for an EVM one.
pub use crate::core::config::SOLANA_CHAIN_TYPE_BIT;

/// The Connector's own deployment identity.
///
/// No public constructor: the only way to obtain one is [`DeploymentIdentity::resolve`], so a
/// chain id that never passed the chain-kind check cannot enter through a struct literal.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DeploymentIdentity {
    program_id: SolanaPubkeyBytes,
    chain_id: u64,
}

impl DeploymentIdentity {
    /// Resolves the identity at startup from configuration.
    ///
    /// The one rejection is a chain id without the chain-kind bit, and it stops the process rather
    /// than each request: handles embed the chain id and routing reads the bit out of it, so an
    /// identity missing it matches no handle of any Solana cluster and every rejection downstream
    /// would look like a user error.
    pub fn resolve(
        program_id: SolanaPubkeyBytes,
        chain_id: u64,
    ) -> Result<Self, DeploymentIdentityError> {
        if chain_id & SOLANA_CHAIN_TYPE_BIT == 0 {
            return Err(DeploymentIdentityError::ChainKindBitMissing { chain_id });
        }

        Ok(Self {
            program_id,
            chain_id,
        })
    }

    /// Which program owns the host state this Connector authorizes against.
    pub fn program_id(&self) -> SolanaPubkeyBytes {
        self.program_id
    }

    /// Which cluster.
    pub fn chain_id(&self) -> u64 {
        self.chain_id
    }
}

/// Checks the signed deployment pair and the handle-embedded chain ids against this
/// identity.
///
/// One equality, three values: the signed `chain_id`, the u64 every handle embeds, and this
/// Connector's configured value. There is no "first handle as source of truth" — a batch mixing
/// clusters is a rejection, not a majority vote.
pub fn check_deployment(
    request: &super::request::SolanaUserDecryptRequest,
    deployment: &DeploymentIdentity,
) -> Result<(), DeploymentFailure> {
    let permit = request.permit();

    let signed_program = *permit.verifying_program_id().as_bytes();
    if signed_program != deployment.program_id() {
        return Err(DeploymentFailure::ProgramIdMismatch {
            signed: signed_program,
            own: deployment.program_id(),
        });
    }

    let signed_chain_id = permit.chain_id();
    if signed_chain_id != deployment.chain_id() {
        return Err(DeploymentFailure::ChainIdMismatch {
            signed: signed_chain_id,
            own: deployment.chain_id(),
        });
    }

    let Some(first) = request.handles().first() else {
        // A validated request always names at least one handle; decoding rejects an empty list.
        return Ok(());
    };
    let first_embedded = embedded_chain_id(&first.handle());

    // The handles agree among themselves before any of them is compared with the signed value.
    // Testing each handle against the signature directly would report the second cluster of a
    // mixed batch as an ordinary mismatch and hide that the batch was mixed at all.
    for (index, entry) in request.handles().iter().enumerate().skip(1) {
        let embedded = embedded_chain_id(&entry.handle());
        if embedded != first_embedded {
            return Err(DeploymentFailure::MixedEmbeddedChainIds {
                index,
                found: embedded,
                expected: first_embedded,
            });
        }
    }

    // One equality, three values: at this point the handles are unanimous, so comparing one of
    // them with the signed chain id compares all of them.
    if first_embedded != signed_chain_id {
        return Err(DeploymentFailure::EmbeddedChainIdMismatch {
            index: 0,
            embedded: first_embedded,
            signed: signed_chain_id,
        });
    }

    Ok(())
}

/// Offset of the embedded chain id inside a handle.
const HANDLE_CHAIN_ID_RANGE: std::ops::Range<usize> = 22..30;

/// The chain id embedded in a handle's bytes `[22..30]`.
pub fn embedded_chain_id(handle: &[u8; 32]) -> u64 {
    let mut bytes = [0; 8];
    bytes.copy_from_slice(&handle[HANDLE_CHAIN_ID_RANGE]);
    u64::from_be_bytes(bytes)
}

/// Why an identity could not be resolved at startup.
#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum DeploymentIdentityError {
    /// The configured chain id does not carry the Solana chain-kind bit, so handles of this
    /// cluster could not route.
    #[error("configured chain id {chain_id} does not carry the Solana chain-kind bit")]
    ChainKindBitMissing {
        /// The configured value.
        chain_id: u64,
    },
}

/// Why a request's deployment did not match.
#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum DeploymentFailure {
    /// The permit was signed for another program.
    #[error("permit names program {signed:?}, this deployment is {own:?}")]
    ProgramIdMismatch {
        /// What the permit signed.
        signed: SolanaPubkeyBytes,
        /// This Connector's program id.
        own: SolanaPubkeyBytes,
    },
    /// The permit was signed for another cluster.
    #[error("permit names chain id {signed}, this deployment is {own}")]
    ChainIdMismatch {
        /// What the permit signed.
        signed: u64,
        /// This Connector's derived chain id.
        own: u64,
    },
    /// Handles of the same request embed different chain ids.
    #[error("handle {index} embeds chain id {found}, an earlier handle embeds {expected}")]
    MixedEmbeddedChainIds {
        /// Which entry disagreed.
        index: usize,
        /// The value it embeds.
        found: u64,
        /// The value the earlier handles embed.
        expected: u64,
    },
    /// A handle embeds a chain id other than the signed one.
    #[error("handle {index} embeds chain id {embedded}, the permit signs {signed}")]
    EmbeddedChainIdMismatch {
        /// Which entry.
        index: usize,
        /// The value it embeds.
        embedded: u64,
        /// What the permit signed.
        signed: u64,
    },
}
