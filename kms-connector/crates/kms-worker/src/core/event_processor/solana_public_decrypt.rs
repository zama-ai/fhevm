//! Solana public-decryption authorization, and the per-host handle both Solana paths read through.
//!
//! Public-ness has no live on-chain flag: it is a `PublicDecryptLeaf` sealed in the encrypted
//! value account's MMR, and the account holds only the peaks. The check here is the user-decrypt
//! pipeline's last three steps on their own — read the account, fetch the leaf proof from the
//! coprocessors' record, verify it against the account's peaks — with the same readers, the same
//! resolution rule and the same binding rule. The one thing the request supplies is which
//! account the handle lives in, carried in the version-`0x03` `extraData` blob.
//!
//! The external suite `kms-worker/tests/solana_public_decrypt_carrier.rs` pins the carrier and the
//! whole path from outside the modules that make it up.

use crate::core::{
    event_processor::ProcessingError,
    solana::{
        deployment::DeploymentIdentity,
        encrypted_value_account::{EncryptedValueAccountFailure, resolve_encrypted_value_account},
        failure::FailureClass,
        handle_binding::{HandleBindingFailure, check_public_binding},
        proof::{
            HostProofReader, HttpHostProofReader, LeafKind, LeafQuery, ProofBatch, ProofReadError,
            read_proofs_with_one_retry,
        },
        snapshot::{HostStateReader, RpcHostStateReader, SnapshotError, SnapshotKeys},
    },
    solana_acl::HandleBytes,
};
use anyhow::anyhow;
use connector_utils::types::solana_extra_data::parse_solana_public_decrypt_extra_data;
use kms_connector_api::ErrorCode;

/// Per-chain Solana host: the deployment identity authorization is decided against, the
/// `confirmed`-commitment account reader, and the leaf-proof reader over the coprocessors. Both
/// Solana paths — user decrypt and public decrypt — go through these three and nothing else.
#[derive(Clone, Debug)]
pub struct SolanaHost {
    /// Which program and cluster this Connector authorizes against.
    pub deployment: DeploymentIdentity,
    /// The atomic `getMultipleAccounts` snapshot reader.
    pub reader: RpcHostStateReader,
    /// The leaf-proof reader, fanning out to every configured coprocessor.
    pub proofs: HttpHostProofReader,
}

/// The public-decrypt check as the event processor calls it.
pub async fn check_solana_handles_public_decrypt(
    host: &SolanaHost,
    handles: &[HandleBytes],
    extra_data: &[u8],
) -> Result<(), ProcessingError> {
    check_public_decrypt(
        &host.deployment,
        &host.reader,
        &host.proofs,
        handles,
        extra_data,
    )
    .await
    .map_err(|failure| {
        let message = anyhow!("Solana public-decryption authorization failed: {failure}");
        match failure.class() {
            FailureClass::Terminal => {
                ProcessingError::irrecoverable(ErrorCode::Unprocessable, message)
            }
            FailureClass::Transient | FailureClass::Retryable => {
                ProcessingError::recoverable(ErrorCode::UpstreamTransient, message)
            }
        }
    })
}

/// Authorizes one public decryption, or says why not.
///
/// Generic over the two readers for the same reason the user-decrypt pipeline is: a test drives
/// the whole path against canned state and canned proofs, and counts the reads.
pub async fn check_public_decrypt<R, P>(
    deployment: &DeploymentIdentity,
    reader: &R,
    proofs: &P,
    handles: &[HandleBytes],
    extra_data: &[u8],
) -> Result<(), PublicDecryptFailure>
where
    R: HostStateReader,
    P: HostProofReader,
{
    // A `PublicDecryptLeaf` names one handle, and the carrier names one account: one handle per
    // request.
    let handle = match handles {
        [single] => *single,
        other => {
            return Err(PublicDecryptFailure::NotSingleHandle {
                handles: other.len(),
            });
        }
    };
    let extra = parse_solana_public_decrypt_extra_data(extra_data)
        .ok_or(PublicDecryptFailure::MalformedExtraData)?;

    let program_id = deployment.program_id();
    let keys = SnapshotKeys::new([extra.encrypted_value_account]);
    let observation = reader.read_accounts(&keys).await?;
    let encrypted_value_account =
        resolve_encrypted_value_account(&observation, program_id, extra.encrypted_value_account)?;

    let batch = ProofBatch::new([LeafQuery {
        encrypted_value_account: encrypted_value_account.account_key(),
        handle,
        kind: LeafKind::Public,
    }]);
    let live_leaf_count = encrypted_value_account.encrypted_value().leaf_count;
    let outcomes = read_proofs_with_one_retry(proofs, &batch, |_| live_leaf_count).await?;

    check_public_binding(&encrypted_value_account, handle, &outcomes[0])?;
    Ok(())
}

/// Why a public decryption was not authorized.
#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum PublicDecryptFailure {
    /// The request named a number of handles other than one.
    #[error("Solana public decryption authorizes exactly one handle per request, got {handles}")]
    NotSingleHandle {
        /// How many arrived.
        handles: usize,
    },
    /// The `extraData` is not the version-3 carrier naming the encrypted value account.
    #[error(
        "Solana public decryption requires the version-3 extraData naming the handle's encrypted \
         value account"
    )]
    MalformedExtraData,
    /// The account could not be observed.
    #[error("host state: {0}")]
    Snapshot(#[from] SnapshotError),
    /// The named account is not a valid encrypted value account.
    #[error("encrypted value account: {0}")]
    EncryptedValueAccount(#[from] EncryptedValueAccountFailure),
    /// The leaf record could not be read at all.
    #[error("leaf proofs: {0}")]
    ProofRead(#[from] ProofReadError),
    /// No public-decrypt leaf binds the handle.
    #[error("handle binding: {0}")]
    HandleBinding(#[from] HandleBindingFailure),
}

impl PublicDecryptFailure {
    /// Which action the failure implies, delegating to the taxonomy that produced it; the two
    /// local variants describe a request that is wrong forever.
    pub fn class(&self) -> FailureClass {
        match self {
            Self::NotSingleHandle { .. } | Self::MalformedExtraData => FailureClass::Terminal,
            Self::Snapshot(source) => source.class(),
            Self::EncryptedValueAccount(source) => source.class(),
            Self::ProofRead(source) => source.class(),
            Self::HandleBinding(source) => source.class(),
        }
    }
}
