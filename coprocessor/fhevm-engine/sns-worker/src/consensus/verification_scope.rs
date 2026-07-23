use alloy_primitives::{Address, B256, U256};
use ciphertext_attestation::manifest::{ManifestVersion, SignedManifest};

#[derive(Clone, Debug)]
pub(super) struct VerificationScope {
    pub(super) local_publisher: Address,
    pub(super) version: ManifestVersion,
    pub(super) coprocessor_context_id: U256,
    pub(super) host_chain_id: i64,
    pub(super) publication_block_number: i64,
    pub(super) publication_block_hash: B256,
    pub(super) revision: u64,
    pub(super) local_manifest_digest: B256,
}

impl VerificationScope {
    pub(super) fn matches_manifest(&self, manifest: &SignedManifest) -> bool {
        let Ok(host_chain_id) = u64::try_from(self.host_chain_id) else {
            return false;
        };
        let Ok(publication_block_number) = u64::try_from(self.publication_block_number) else {
            return false;
        };
        let payload = &manifest.payload;
        payload.version == self.version
            && payload.coprocessor_context_id == self.coprocessor_context_id
            && payload.host_chain_id == U256::from(host_chain_id)
            && payload.publication_block_number == U256::from(publication_block_number)
            && payload.publication_block_hash == self.publication_block_hash
    }
}
