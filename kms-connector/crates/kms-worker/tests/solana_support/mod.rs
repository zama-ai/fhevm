//! Fixtures for the Solana authorization suite.
//!
//! The centre of this module is [`World`] plus [`ScriptedReader`]: a set of accounts at a slot, and
//! a reader that answers from it while recording what it was asked for. That pair is what turns a
//! race into a value — a scenario is two worlds, not two moments — and it is what lets the suite
//! assert how many times authorization reads state, which is otherwise an invisible property.
//!
//! Everything else here builds the three account layouts authorization reads (encrypted value
//! account, delegation record, invalidation record) and signs real permits with a real wallet key,
//! so no test depends on a signature the code under test produced.

// Groups land one at a time; a builder written for a later group is early, not dead.
#![allow(dead_code)]

use kms_worker::core::solana::{
    delegation::WILDCARD_ENCRYPTED_VALUE_ACCOUNT_AUTHORITY,
    deployment::{DeploymentIdentity, SOLANA_CHAIN_TYPE_BIT},
    kms_pair::{KmsPairFailure, KmsPairValidator},
    request::{SolanaHandleEntryWire, SolanaUserDecryptRequest, SolanaUserDecryptRequestWire},
    snapshot::{
        HostSnapshot, HostStateReader, SYSTEM_PROGRAM_ID, SnapshotAccount, SnapshotError,
        SnapshotKeys,
    },
};
use kms_worker::core::solana_acl::SolanaPubkeyBytes;
use kms_worker::core::solana_encrypted_value_acl::encrypted_value_acl_address;
use ring::signature::{Ed25519KeyPair, KeyPair};
use sha2::{Digest, Sha256};
use solana_pubkey::Pubkey;
use std::collections::BTreeMap;
use std::sync::Mutex;
use zama_solana_acl::{
    EncryptedValue, MmrProof, derive_encrypted_value_id, encrypted_value_discriminator,
    historical_access_leaf_commitment, mmr_append, mmr_build_proof, public_decrypt_leaf_commitment,
};
use zama_solana_permit::{
    Identity, KmsRouting, PermitFields, PermitWireFields, Signature, TRANSPORT_KEY_LEN,
    build_envelope,
};

/// The deployment every fixture is built against.
pub const PROGRAM_ID: SolanaPubkeyBytes = [7; 32];
/// The genesis hash of the cluster these fixtures stand for. Provenance only: it names which
/// cluster [`CHAIN_ID`] belongs to, and no check in the authorization path reads it — the rule that
/// ties a cluster to its chain id is applied once per cluster at deployment, not per request.
pub const GENESIS_HASH: [u8; 32] = [9; 32];
/// The chain id of the fixture cluster, carrying the chain-kind high bit as every Solana host
/// chain id must.
pub const CHAIN_ID: u64 = SOLANA_CHAIN_TYPE_BIT | 0x0123_4567_89ab_cdef;

/// The ACL domain of the default encrypted value account.
pub const DOMAIN: SolanaPubkeyBytes = [1; 32];
/// The encrypted value account authority of the default encrypted value account.
pub const AUTHORITY: SolanaPubkeyBytes = [2; 32];
/// The label of the default encrypted value account.
pub const LABEL: [u8; 32] = *b"balance_________________________";

/// FHE type byte of a boolean handle — the narrowest type, two bits.
pub const FHE_TYPE_BOOL: u8 = 0;
/// FHE type byte of a 64-bit handle.
pub const FHE_TYPE_UINT64: u8 = 5;
/// Handle format version byte.
pub const HANDLE_VERSION: u8 = 0;

/// Discriminator of the invalidation record, as a literal.
///
/// Deliberately a literal and not a call into the host program's framework: this is the
/// account the Connector decodes by hand, and the point of the pin is that a foreign
/// implementation's bytes are compared against a constant. It is checked against its own
/// preimage in [`permit_invalidation_discriminator`].
pub const PERMIT_INVALIDATION_DISCRIMINATOR: [u8; 8] =
    [0xec, 0x8b, 0xdb, 0xa9, 0xb9, 0x22, 0xe9, 0x88];

/// The invalidation record's discriminator, recomputed from the account name.
///
/// The suite asserts this equals the literal above. Both sides are computed here rather than
/// taken from the program, so a rename or a derivation change on the host side surfaces as a
/// mismatch against the constant a reader was told to look for.
pub fn permit_invalidation_discriminator() -> [u8; 8] {
    let digest = Sha256::digest(b"account:PermitInvalidation");
    let mut out = [0; 8];
    out.copy_from_slice(&digest[..8]);
    out
}

/// Discriminator of the delegation record.
pub fn user_decryption_delegation_discriminator() -> [u8; 8] {
    kms_worker::core::solana_acl::anchor_account_discriminator("UserDecryptionDelegation")
}

// ---------------------------------------------------------------------------
// Deployment identity
// ---------------------------------------------------------------------------

/// The deployment identity of the fixture cluster.
pub fn deployment() -> DeploymentIdentity {
    DeploymentIdentity::resolve(PROGRAM_ID, CHAIN_ID).expect("fixture deployment resolves")
}

// ---------------------------------------------------------------------------
// Handles
// ---------------------------------------------------------------------------

/// A handle of the fixture cluster: distinguishing byte, chain id big-endian at `[22..30]`,
/// FHE type at `[30]`, version at `[31]`.
pub fn handle(tag: u8, fhe_type: u8) -> [u8; 32] {
    handle_on_chain(tag, fhe_type, CHAIN_ID)
}

/// A handle carrying an arbitrary embedded chain id, for the deployment-mismatch cases.
pub fn handle_on_chain(tag: u8, fhe_type: u8, chain_id: u64) -> [u8; 32] {
    let mut bytes = [tag; 32];
    bytes[22..30].copy_from_slice(&chain_id.to_be_bytes());
    bytes[30] = fhe_type;
    bytes[31] = HANDLE_VERSION;
    bytes
}

// ---------------------------------------------------------------------------
// Wallets and permits
// ---------------------------------------------------------------------------

/// A wallet that signs permits the way a real one does: over the reconstructed envelope.
pub struct Wallet {
    keypair: Ed25519KeyPair,
}

impl Wallet {
    /// A deterministic wallet for a seed byte.
    pub fn new(seed: u8) -> Self {
        // A minimal PKCS#8 v1 document wrapping the raw seed, which is what `ring` accepts.
        let prefix: [u8; 16] = [
            0x30, 0x2e, 0x02, 0x01, 0x00, 0x30, 0x05, 0x06, 0x03, 0x2b, 0x65, 0x70, 0x04, 0x22,
            0x04, 0x20,
        ];
        let mut document = prefix.to_vec();
        document.extend_from_slice(&[seed; 32]);
        Self {
            keypair: Ed25519KeyPair::from_pkcs8_maybe_unchecked(&document)
                .expect("fixture keypair is well formed"),
        }
    }

    /// The wallet's public key, which is also the permit's user and its recipient.
    pub fn pubkey(&self) -> SolanaPubkeyBytes {
        self.keypair
            .public_key()
            .as_ref()
            .try_into()
            .expect("an Ed25519 public key is 32 bytes")
    }

    /// Signs the envelope of a validated permit.
    pub fn sign(&self, fields: &PermitFields) -> Signature {
        let signature = self.keypair.sign(&build_envelope(fields));
        Signature::new(
            signature
                .as_ref()
                .try_into()
                .expect("an Ed25519 signature is 64 bytes"),
        )
    }
}

/// Builds permits in transport form.
///
/// Defaults describe a permit that passes every state-free rule: scoped to [`DOMAIN`], one
/// hour of validity from [`DEFAULT_START`], this deployment, the only known routing version.
#[derive(Clone, Debug)]
pub struct PermitBuilder {
    wire: PermitWireFields,
}

/// Start of the default validity window.
pub const DEFAULT_START: u64 = 1_700_000_000;
/// Length of the default validity window.
pub const DEFAULT_DURATION: u64 = 3_600;
/// A time inside the default window.
pub const NOW_INSIDE_WINDOW: u64 = DEFAULT_START + 60;
/// The KMS context of the default permit.
pub const KMS_CONTEXT: SolanaPubkeyBytes = [0x11; 32];
/// The KMS epoch of the default permit.
pub const KMS_EPOCH: SolanaPubkeyBytes = [0x12; 32];

impl PermitBuilder {
    /// A permit for `user`, scoped to the fixture domain.
    pub fn new(user: SolanaPubkeyBytes) -> Self {
        Self {
            wire: PermitWireFields {
                user_pubkey: user.to_vec(),
                transport_key: vec![0xa5; TRANSPORT_KEY_LEN],
                allowed_acl_domain_keys: vec![DOMAIN.to_vec()],
                start_timestamp: DEFAULT_START,
                duration_seconds: DEFAULT_DURATION,
                verifying_program_id: PROGRAM_ID.to_vec(),
                chain_id: CHAIN_ID,
                extra_data: KmsRouting::ContextAndEpoch {
                    kms_context_id: Identity::new(KMS_CONTEXT),
                    kms_epoch_id: Identity::new(KMS_EPOCH),
                }
                .to_extra_data(),
            },
        }
    }

    /// Drops the domain list, making the permit permissive.
    pub fn permissive(mut self) -> Self {
        self.wire.allowed_acl_domain_keys.clear();
        self
    }

    /// Replaces the signed domain scope.
    pub fn scope(mut self, domains: &[SolanaPubkeyBytes]) -> Self {
        self.wire.allowed_acl_domain_keys = domains.iter().map(|key| key.to_vec()).collect();
        self
    }

    /// Replaces the validity window.
    pub fn window(mut self, start_timestamp: u64, duration_seconds: u64) -> Self {
        self.wire.start_timestamp = start_timestamp;
        self.wire.duration_seconds = duration_seconds;
        self
    }

    /// Replaces the signed deployment pair.
    pub fn deployment_pair(mut self, program_id: SolanaPubkeyBytes, chain_id: u64) -> Self {
        self.wire.verifying_program_id = program_id.to_vec();
        self.wire.chain_id = chain_id;
        self
    }

    /// Replaces the signed KMS routing pair.
    pub fn kms_pair(mut self, context: SolanaPubkeyBytes, epoch: SolanaPubkeyBytes) -> Self {
        self.wire.extra_data = KmsRouting::ContextAndEpoch {
            kms_context_id: Identity::new(context),
            kms_epoch_id: Identity::new(epoch),
        }
        .to_extra_data();
        self
    }

    /// The permit in transport form.
    pub fn wire(&self) -> PermitWireFields {
        self.wire.clone()
    }

    /// The permit's validated form, for signing.
    pub fn typed(&self) -> PermitFields {
        PermitFields::decode(&self.wire).expect("fixture permit is well formed")
    }
}

/// Builds requests in transport form: a permit, the wallet signature over its envelope, and
/// the handle entries.
pub struct RequestBuilder<'a> {
    wallet: &'a Wallet,
    permit: PermitBuilder,
    entries: Vec<SolanaHandleEntryWire>,
}

impl<'a> RequestBuilder<'a> {
    /// A request signed by `wallet` under a default permit.
    pub fn new(wallet: &'a Wallet) -> Self {
        Self {
            wallet,
            permit: PermitBuilder::new(wallet.pubkey()),
            entries: Vec::new(),
        }
    }

    /// Replaces the permit.
    pub fn permit(mut self, permit: PermitBuilder) -> Self {
        self.permit = permit;
        self
    }

    /// Adds a direct current-access entry: the signer owns the handle and claims it is live.
    pub fn direct_current(
        self,
        encrypted_value_account: &EncryptedValueAccountFixture,
        handle: [u8; 32],
    ) -> Self {
        let owner = self.wallet.pubkey();
        self.entry(
            handle,
            owner,
            encrypted_value_account.encrypted_value_id(),
            0,
            Vec::new(),
        )
    }

    /// Adds a delegated current-access entry: `delegator` owns the handle.
    pub fn delegated_current(
        self,
        encrypted_value_account: &EncryptedValueAccountFixture,
        handle: [u8; 32],
        delegator: SolanaPubkeyBytes,
    ) -> Self {
        self.entry(
            handle,
            delegator,
            encrypted_value_account.encrypted_value_id(),
            0,
            Vec::new(),
        )
    }

    /// Adds a historical-access entry carrying a proof.
    pub fn historical(
        self,
        encrypted_value_account: &EncryptedValueAccountFixture,
        handle: [u8; 32],
        owner: SolanaPubkeyBytes,
        proof: &MmrProof,
        proof_leaf_count: u64,
    ) -> Self {
        let bytes = borsh::to_vec(proof).expect("a proof serializes");
        self.entry(
            handle,
            owner,
            encrypted_value_account.encrypted_value_id(),
            proof_leaf_count,
            bytes,
        )
    }

    /// Adds an entry verbatim, for the malformed cases.
    pub fn entry(
        mut self,
        handle: [u8; 32],
        owner: SolanaPubkeyBytes,
        encrypted_value_id: [u8; 32],
        proof_leaf_count: u64,
        access_proof: Vec<u8>,
    ) -> Self {
        self.entries.push(SolanaHandleEntryWire {
            handle: handle.to_vec(),
            owner: owner.to_vec(),
            encrypted_value_id: encrypted_value_id.to_vec(),
            proof_leaf_count,
            access_proof,
        });
        self
    }

    /// The request in transport form, signed.
    pub fn wire(&self) -> SolanaUserDecryptRequestWire {
        let signature = self.wallet.sign(&self.permit.typed());
        SolanaUserDecryptRequestWire {
            permit: self.permit.wire(),
            signature: signature.as_bytes().to_vec(),
            handles: self.entries.clone(),
        }
    }

    /// The request in validated form.
    pub fn typed(&self) -> SolanaUserDecryptRequest {
        SolanaUserDecryptRequest::decode(&self.wire()).expect("fixture request is well formed")
    }
}

/// The event-typed ACL-scope declaration an honest gateway carries for this request: the signed
/// list's actual length. Scenarios probing the declaration rule itself state a lying value
/// directly instead of using this.
pub fn declared_acl_domain_key_count(request: &SolanaUserDecryptRequest) -> u8 {
    u8::try_from(request.permit().allowed_acl_domain_keys().as_slice().len())
        .expect("test permits stay under the ACL-scope cap")
}

// ---------------------------------------------------------------------------
// Account layouts
// ---------------------------------------------------------------------------

/// An encrypted value account as the host program would hold it, with the leaves needed to build
/// proofs.
#[derive(Clone, Debug)]
pub struct EncryptedValueAccountFixture {
    /// The encrypted value account state.
    pub encrypted_value: EncryptedValue,
    /// Its canonical address.
    pub account_key: SolanaPubkeyBytes,
    /// Every leaf appended so far, so proofs can be rebuilt.
    pub leaves: Vec<[u8; 32]>,
}

impl EncryptedValueAccountFixture {
    /// An encrypted value account in the fixture domain and app, holding `current_handle` for
    /// `subjects`.
    pub fn new(current_handle: [u8; 32], subjects: &[SolanaPubkeyBytes]) -> Self {
        Self::in_domain(DOMAIN, AUTHORITY, LABEL, current_handle, subjects)
    }

    /// An encrypted value account in an arbitrary domain, app and label.
    pub fn in_domain(
        domain: SolanaPubkeyBytes,
        encrypted_value_account_authority: SolanaPubkeyBytes,
        label: [u8; 32],
        current_handle: [u8; 32],
        subjects: &[SolanaPubkeyBytes],
    ) -> Self {
        let encrypted_value_id =
            derive_encrypted_value_id(domain, encrypted_value_account_authority, label);
        let (account_key, bump) = encrypted_value_acl_address(PROGRAM_ID, encrypted_value_id);
        Self {
            encrypted_value: EncryptedValue {
                domain,
                encrypted_value_account_authority,
                label,
                current_handle,
                subjects: subjects.to_vec(),
                leaf_count: 0,
                peaks: Vec::new(),
                bump,
            },
            account_key,
            leaves: Vec::new(),
        }
    }

    /// The encrypted value account identity a request names.
    pub fn encrypted_value_id(&self) -> [u8; 32] {
        self.encrypted_value.encrypted_value_id()
    }

    /// Replaces the current handle, sealing a historical leaf for each current subject —
    /// what the host program does on a write.
    pub fn update(&mut self, new_handle: [u8; 32]) {
        let replaced = self.encrypted_value.current_handle;
        for index in 0..self.encrypted_value.subjects.len() {
            let leaf_index = self.encrypted_value.leaf_count;
            let commitment = historical_access_leaf_commitment(
                self.account_key,
                leaf_index,
                replaced,
                self.encrypted_value.subjects[index],
            );
            self.append(commitment);
        }
        self.encrypted_value.current_handle = new_handle;
    }

    /// Seals a public-decrypt leaf for the current handle.
    pub fn mark_public(&mut self) {
        let leaf_index = self.encrypted_value.leaf_count;
        let commitment = public_decrypt_leaf_commitment(
            self.account_key,
            leaf_index,
            self.encrypted_value.current_handle,
        );
        self.append(commitment);
    }

    /// Appends a commitment, keeping the leaf list in step with the MMR.
    pub fn append(&mut self, commitment: [u8; 32]) {
        mmr_append(
            &mut self.encrypted_value.peaks,
            &mut self.encrypted_value.leaf_count,
            commitment,
        )
        .expect("the fixture MMR accepts an append");
        self.leaves.push(commitment);
    }

    /// A proof of the leaf at `leaf_index`.
    pub fn proof(&self, leaf_index: u64) -> MmrProof {
        mmr_build_proof(&self.leaves, leaf_index).expect("the fixture MMR builds a proof")
    }

    /// Replaces the subject set, as a membership rotation does.
    pub fn rotate_subjects(&mut self, subjects: &[SolanaPubkeyBytes]) {
        self.encrypted_value.subjects = subjects.to_vec();
    }

    /// The account as the host program would write it: discriminator then body.
    pub fn account(&self) -> SnapshotAccount {
        let mut data = encrypted_value_discriminator().to_vec();
        data.extend_from_slice(
            &borsh::to_vec(&self.encrypted_value).expect("the encrypted value account serializes"),
        );
        SnapshotAccount {
            owner: PROGRAM_ID,
            data,
        }
    }
}

/// A delegation record as the host program would hold it.
#[derive(Clone, Copy, Debug)]
pub struct DelegationFixture {
    /// Who granted it.
    pub delegator: SolanaPubkeyBytes,
    /// Who received it.
    pub delegate: SolanaPubkeyBytes,
    /// Which app it covers.
    pub encrypted_value_account_authority: SolanaPubkeyBytes,
    /// Last slot it is valid at.
    pub expiration_slot: u64,
    /// The counter no rule reads and no signature commits to.
    pub delegation_counter: u64,
    /// When it was last written.
    pub last_update_slot: u64,
    /// Whether the delegator revoked it.
    pub revoked: bool,
}

impl DelegationFixture {
    /// A live delegation covering the fixture app.
    pub fn live(
        delegator: SolanaPubkeyBytes,
        delegate: SolanaPubkeyBytes,
        observed_slot: u64,
    ) -> Self {
        Self {
            delegator,
            delegate,
            encrypted_value_account_authority: AUTHORITY,
            expiration_slot: observed_slot + 100,
            delegation_counter: 1,
            last_update_slot: observed_slot.saturating_sub(1),
            revoked: false,
        }
    }

    /// A live wildcard row: the same grant with the reserved sentinel in place of an encrypted
    /// value account authority, which is how a delegator covers every authority of theirs at once.
    pub fn live_wildcard(
        delegator: SolanaPubkeyBytes,
        delegate: SolanaPubkeyBytes,
        observed_slot: u64,
    ) -> Self {
        Self {
            encrypted_value_account_authority: WILDCARD_ENCRYPTED_VALUE_ACCOUNT_AUTHORITY,
            ..Self::live(delegator, delegate, observed_slot)
        }
    }

    /// Its canonical address and bump.
    pub fn address(&self) -> (SolanaPubkeyBytes, u8) {
        kms_worker::core::solana::delegation::delegation_address(
            PROGRAM_ID,
            self.delegator,
            self.delegate,
            self.encrypted_value_account_authority,
        )
    }

    /// The account as the host program would write it.
    pub fn account(&self) -> SnapshotAccount {
        let (_, bump) = self.address();
        let mut data = user_decryption_delegation_discriminator().to_vec();
        data.extend_from_slice(&self.delegator);
        data.extend_from_slice(&self.delegate);
        data.extend_from_slice(&self.encrypted_value_account_authority);
        data.extend_from_slice(&self.expiration_slot.to_le_bytes());
        data.extend_from_slice(&self.delegation_counter.to_le_bytes());
        data.extend_from_slice(&self.last_update_slot.to_le_bytes());
        data.push(self.revoked as u8);
        data.push(bump);
        SnapshotAccount {
            owner: PROGRAM_ID,
            data,
        }
    }
}

/// The canonical invalidation-record address for a user, derived here rather than taken from
/// the code under test.
pub fn invalidation_address(user: SolanaPubkeyBytes) -> (SolanaPubkeyBytes, u8) {
    let (address, bump) = Pubkey::find_program_address(
        &[b"permit-invalidation", user.as_ref()],
        &Pubkey::new_from_array(PROGRAM_ID),
    );
    (address.to_bytes(), bump)
}

/// An invalidation record holding `watermark` for `user`.
pub fn invalidation_account(user: SolanaPubkeyBytes, watermark: u64) -> SnapshotAccount {
    let (_, bump) = invalidation_address(user);
    let mut data = PERMIT_INVALIDATION_DISCRIMINATOR.to_vec();
    data.extend_from_slice(&user);
    data.extend_from_slice(&watermark.to_le_bytes());
    data.push(bump);
    SnapshotAccount {
        owner: PROGRAM_ID,
        data,
    }
}

/// The account a bare transfer to a not-yet-created PDA leaves behind: System-program-owned, no
/// data. Every address in this path is derivable by anyone, so any sender can produce this at any
/// of them, for any user.
pub fn prefunded_account() -> SnapshotAccount {
    SnapshotAccount {
        owner: SYSTEM_PROGRAM_ID,
        data: Vec::new(),
    }
}

// ---------------------------------------------------------------------------
// Worlds and readers
// ---------------------------------------------------------------------------

/// Host state at one slot: what a read of it would return.
#[derive(Clone, Debug, Default)]
pub struct World {
    /// The slot a read of this world reports as its observation point.
    pub slot: u64,
    accounts: BTreeMap<SolanaPubkeyBytes, SnapshotAccount>,
}

impl World {
    /// An empty world at `slot`.
    pub fn at_slot(slot: u64) -> Self {
        Self {
            slot,
            accounts: BTreeMap::new(),
        }
    }

    /// Places an encrypted value account in the world.
    pub fn with_encrypted_value_account(
        mut self,
        encrypted_value_account: &EncryptedValueAccountFixture,
    ) -> Self {
        self.accounts.insert(
            encrypted_value_account.account_key,
            encrypted_value_account.account(),
        );
        self
    }

    /// Places a delegation record in the world.
    pub fn with_delegation(mut self, delegation: &DelegationFixture) -> Self {
        let (key, _) = delegation.address();
        self.accounts.insert(key, delegation.account());
        self
    }

    /// Places an invalidation record in the world.
    pub fn with_watermark(mut self, user: SolanaPubkeyBytes, watermark: u64) -> Self {
        let (key, _) = invalidation_address(user);
        self.accounts
            .insert(key, invalidation_account(user, watermark));
        self
    }

    /// Places an arbitrary account in the world, for the wrong-owner and wrong-type cases.
    pub fn with_account(mut self, key: SolanaPubkeyBytes, account: SnapshotAccount) -> Self {
        self.accounts.insert(key, account);
        self
    }

    /// Removes an account, for the absent cases.
    pub fn without_account(mut self, key: &SolanaPubkeyBytes) -> Self {
        self.accounts.remove(key);
        self
    }

    /// The same world observed at another slot.
    pub fn at(mut self, slot: u64) -> Self {
        self.slot = slot;
        self
    }

    /// A world assembled from a recorded account set, for the vector runner.
    pub fn from_accounts(
        slot: u64,
        accounts: impl IntoIterator<Item = (SolanaPubkeyBytes, SnapshotAccount)>,
    ) -> Self {
        Self {
            slot,
            accounts: accounts.into_iter().collect(),
        }
    }

    /// The accounts this world holds, in key order, for recording a vector.
    pub fn accounts(&self) -> impl Iterator<Item = (&SolanaPubkeyBytes, &SnapshotAccount)> {
        self.accounts.iter()
    }

    /// Projects the world onto the requested keys, exactly as an account read would.
    pub fn read(&self, keys: &SnapshotKeys) -> Result<HostSnapshot, SnapshotError> {
        let accounts = keys
            .as_slice()
            .iter()
            .map(|key| self.accounts.get(key).cloned())
            .collect();
        HostSnapshot::new(self.slot, keys, accounts)
    }
}

/// A reader that answers from a script of worlds and records every call.
///
/// Reads are answered in order: the first call sees the first world, the second the second. A
/// call beyond the script panics rather than repeating the last world — an authorization that
/// reads state a third time is a defect, and it should surface as a loud failure in whichever
/// test provoked it rather than as a passing assertion elsewhere.
pub struct ScriptedReader {
    worlds: Vec<World>,
    calls: Mutex<Vec<SnapshotKeys>>,
}

impl ScriptedReader {
    /// A reader whose every read sees the same world.
    pub fn constant(world: World) -> Self {
        Self::scripted(vec![world.clone(), world])
    }

    /// A reader whose reads see the given worlds in order.
    pub fn scripted(worlds: Vec<World>) -> Self {
        Self {
            worlds,
            calls: Mutex::new(Vec::new()),
        }
    }

    /// How many times state was read.
    pub fn call_count(&self) -> usize {
        self.calls.lock().expect("reader lock").len()
    }

    /// The key sets that were read, in order.
    pub fn calls(&self) -> Vec<SnapshotKeys> {
        self.calls.lock().expect("reader lock").clone()
    }

    /// The keys of the read at `index`.
    pub fn call(&self, index: usize) -> SnapshotKeys {
        self.calls()
            .get(index)
            .cloned()
            .unwrap_or_else(|| panic!("expected at least {} host-state read(s)", index + 1))
    }
}

impl HostStateReader for ScriptedReader {
    async fn read_accounts(&self, keys: &SnapshotKeys) -> Result<HostSnapshot, SnapshotError> {
        let index = {
            let mut calls = self.calls.lock().expect("reader lock");
            calls.push(keys.clone());
            calls.len() - 1
        };
        let world = self.worlds.get(index).unwrap_or_else(|| {
            panic!(
                "authorization read host state {} time(s); the script provides {}",
                index + 1,
                self.worlds.len()
            )
        });
        world.read(keys)
    }
}

/// A KMS pair validator that serves the fixture pair and nothing else.
pub struct ServableKmsPair;

impl KmsPairValidator for ServableKmsPair {
    async fn validate_pair(
        &self,
        kms_context_id: &SolanaPubkeyBytes,
        kms_epoch_id: &SolanaPubkeyBytes,
    ) -> Result<(), KmsPairFailure> {
        if kms_context_id == &KMS_CONTEXT && kms_epoch_id == &KMS_EPOCH {
            Ok(())
        } else {
            Err(KmsPairFailure::ContextUnknown)
        }
    }
}

/// A KMS pair validator that always fails with a given reason.
pub struct UnservableKmsPair(pub KmsPairFailure);

impl KmsPairValidator for UnservableKmsPair {
    async fn validate_pair(
        &self,
        _kms_context_id: &SolanaPubkeyBytes,
        _kms_epoch_id: &SolanaPubkeyBytes,
    ) -> Result<(), KmsPairFailure> {
        Err(self.0.clone())
    }
}
