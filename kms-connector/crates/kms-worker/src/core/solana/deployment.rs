//! Deployment identity: which program, which cluster.
//!
//! A permit carries no handle at signing time, so it cannot derive its environment from one
//! — the deployment is signed explicitly as `(verifying_program_id, chain_id)`. The
//! Connector compares that pair against its **own** identity: the program id from
//! configuration, and a chain id derived from its cluster's genesis hash.
//!
//! The chain id is derived, never configured. A configured constant is identical across
//! environments, which is precisely what makes a signature for one deployment verify
//! against another. A local validator's genesis hash changes on every reset, and that is
//! correct behaviour: the reset cluster is a different deployment and its old permits are
//! rejected.
//!
//! The derivation itself sits behind [`ChainIdDerivation`] because its algorithm and width
//! are still an open protocol question. What this module fixes is the contract — derived,
//! cross-checked against any pin, and equal to both the signed and the handle-embedded value
//! — not the arithmetic.

use crate::core::solana_acl::SolanaPubkeyBytes;

/// The chain-kind high bit: set for a Solana host chain, clear for an EVM one.
pub use crate::core::config::SOLANA_CHAIN_TYPE_BIT;

/// Derives a cluster's chain id from its genesis hash.
///
/// Behind a trait on purpose: the algorithm, the hash choice and the width are unsettled,
/// and everything else in this module tree only needs the value to exist and to be the same
/// u64 the handles embed.
pub trait ChainIdDerivation: Send + Sync {
    /// The chain id of the cluster whose genesis hash this is.
    fn derive_chain_id(&self, genesis_hash: &[u8; 32]) -> u64;
}

/// The Connector's own deployment identity.
///
/// No public constructor: the only way to obtain one is [`DeploymentIdentity::resolve`],
/// which derives the chain id. That is what keeps a configured chain id from re-entering
/// through a struct literal.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DeploymentIdentity {
    program_id: SolanaPubkeyBytes,
    chain_id: u64,
}

impl DeploymentIdentity {
    /// Resolves the identity at startup: derive the chain id from the cluster's genesis
    /// hash, and cross-check it against a configured pin when one is present.
    ///
    /// A pin that disagrees with the derivation is a startup failure, not a request-time
    /// rejection: the process would otherwise run with an identity nothing on its cluster
    /// agrees with, and every permit would fail deployment matching for a reason no log line
    /// explains.
    pub fn resolve(
        program_id: SolanaPubkeyBytes,
        genesis_hash: &[u8; 32],
        pinned_chain_id: Option<u64>,
        derivation: &dyn ChainIdDerivation,
    ) -> Result<Self, DeploymentIdentityError> {
        let chain_id = derivation.derive_chain_id(genesis_hash);

        // Whether the derived value is usable at all comes first: handles embed it and routing
        // reads the chain-kind bit out of it, so a derivation that loses the bit produces an
        // identity no handle of this cluster can match.
        if chain_id & SOLANA_CHAIN_TYPE_BIT == 0 {
            return Err(DeploymentIdentityError::ChainKindBitMissing { chain_id });
        }

        // A pin is an operational convenience and is cross-checked, never preferred: choosing
        // either side of a disagreement would run a Connector whose idea of its own cluster
        // matches nothing on that cluster.
        if let Some(pinned) = pinned_chain_id
            && pinned != chain_id
        {
            return Err(DeploymentIdentityError::PinnedChainIdMismatch {
                pinned,
                derived: chain_id,
            });
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
/// Connector's derived value. There is no "first handle as source of truth" — a batch mixing
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
    /// The configured chain id disagrees with the one derived from the cluster's genesis
    /// hash.
    #[error("configured chain id {pinned} disagrees with the derived {derived}")]
    PinnedChainIdMismatch {
        /// The configured value.
        pinned: u64,
        /// The derived value.
        derived: u64,
    },
    /// The derived chain id does not carry the Solana chain-kind bit, so handles of this
    /// cluster could not route.
    #[error("derived chain id {chain_id} does not carry the Solana chain-kind bit")]
    ChainKindBitMissing {
        /// The derived value.
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
