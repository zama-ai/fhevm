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
//! route without it: the chain-type byte.

use crate::core::solana_acl::SolanaPubkeyBytes;

/// Solana type-byte helpers. Re-exported so tests share the worker's definition.
pub use crate::core::config::{SOLANA_CHAIN_TYPE, is_solana_host_chain_id, solana_host_chain_id};

/// The Connector's own deployment identity.
///
/// No public constructor: the only way to obtain one is [`DeploymentIdentity::resolve`], so a
/// chain id that never passed the type-byte check cannot enter through a struct literal.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DeploymentIdentity {
    program_id: SolanaPubkeyBytes,
    chain_id: u64,
}

impl DeploymentIdentity {
    /// Resolves the identity at startup from configuration.
    ///
    /// The one rejection is a chain id whose high byte is not `0x01`, and it stops the process rather
    /// than each request: handles embed the chain id and routing reads the type byte out of it.
    pub fn resolve(
        program_id: SolanaPubkeyBytes,
        chain_id: u64,
    ) -> Result<Self, DeploymentIdentityError> {
        if !is_solana_host_chain_id(chain_id) {
            return Err(DeploymentIdentityError::ChainTypeByteInvalid { chain_id });
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
    request: &connector_utils::types::solana_request::SolanaUserDecryptRequest,
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

    Ok(())
}

/// Why an identity could not be resolved at startup.
#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum DeploymentIdentityError {
    /// The configured chain id does not have Solana type byte `0x01`.
    #[error(
        "configured chain id {chain_id} is not a Solana host chain id (high byte must be 0x01)"
    )]
    ChainTypeByteInvalid {
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
}
