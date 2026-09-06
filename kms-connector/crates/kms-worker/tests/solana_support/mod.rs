//! Fixtures for the Solana authorization suite.
//!
//! The centre of this module is two scripted readers over two kinds of state. [`World`] plus
//! [`ScriptedReader`] is a set of accounts at a slot and a reader that answers from it while
//! recording what it was asked for; [`ProofRecord`] plus [`ScriptedProofReader`] is the
//! coprocessors' leaf record and a reader that answers leaf-proof queries from it, likewise
//! counting. Together they turn a race into a value — a scenario is two worlds, or a world and a
//! record that disagree, not two moments — and they let the suite assert how many times
//! authorization reads either source, which is otherwise an invisible property.
//!
//! Everything else here builds the three account layouts authorization reads (encrypted value
//! account, delegation record, invalidation record), seals the leaves the record serves, and
//! signs real permits with a real wallet key, so no test depends on a signature the code under
//! test produced.

// Groups land one at a time; a builder written for a later group is early, not dead.
#![allow(dead_code)]

use kms_worker::core::solana::{
    delegation::WILDCARD_ENCRYPTED_VALUE_ACCOUNT_AUTHORITY,
    deployment::{DeploymentIdentity, SOLANA_CHAIN_TYPE_BIT},
    kms_pair::{KmsPairFailure, KmsPairValidator},
    proof::{HostProofReader, LeafKind, LeafProofOutcome, LeafQuery, ProofReadError},
    request::{SolanaHandleEntryWire, SolanaUserDecryptRequest, SolanaUserDecryptRequestWire},
    snapshot::{
        HostSnapshot, HostStateReader, SYSTEM_PROGRAM_ID, SnapshotAccount, SnapshotError,
        SnapshotKeys,
    },
};
use kms_worker::core::solana_acl::SolanaPubkeyBytes;
use ring::signature::{Ed25519KeyPair, KeyPair};
use sha2::{Digest, Sha256};
use solana_pubkey::Pubkey;
use std::collections::BTreeMap;
use std::sync::Mutex;
use zama_solana_acl::{
    EncryptedValue, HOST_CONFIG_SEED, HostConfigRecord, MmrProof, encode_host_config,
    encrypted_value_discriminator, encrypted_value_seeds, historical_access_leaf_commitment,
    mmr_append, mmr_build_proof, public_decrypt_leaf_commitment,
};
use zama_solana_permit::{
    Identity, KmsRouting, PermitFields, PermitWireFields, Signature, TRANSPORT_KEY_LEN,
    build_envelope,
};

/// The host deployment every fixture is built against.
pub const PROGRAM_ID: SolanaPubkeyBytes = [7; 32];
/// The genesis hash of the cluster these fixtures stand for. Provenance only: it names which
/// cluster [`CHAIN_ID`] belongs to, and no check in the authorization path reads it — the rule that
/// ties a cluster to its chain id is applied once per cluster at deployment, not per request.
pub const GENESIS_HASH: [u8; 32] = [9; 32];
/// The chain id of the fixture cluster, carrying the chain-kind high bit as every Solana host
/// chain id must.
pub const CHAIN_ID: u64 = SOLANA_CHAIN_TYPE_BIT | 0x0123_4567_89ab_cdef;

/// The application program of the default encrypted value account.
pub const APP_PROGRAM: SolanaPubkeyBytes = [1; 32];
/// The encrypted value account authority of the default encrypted value account.
pub const AUTHORITY: SolanaPubkeyBytes = [2; 32];
/// The program-declared scope of the default encrypted value account.
pub const SCOPE: SolanaPubkeyBytes = [3; 32];
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

/// One signed `program ‖ scope` entry in transport form.
pub fn scope_entry(program: SolanaPubkeyBytes, scope: SolanaPubkeyBytes) -> Vec<u8> {
    let mut entry = program.to_vec();
    entry.extend_from_slice(&scope);
    entry
}

/// Builds permits in transport form.
///
/// Defaults describe a permit that passes every state-free rule: scoped to the fixture
/// application, one hour of validity from [`DEFAULT_START`], this deployment, the only known
/// routing version.
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
    /// A permit for `user`, scoped to the fixture application.
    pub fn new(user: SolanaPubkeyBytes) -> Self {
        Self {
            wire: PermitWireFields {
                user_pubkey: user.to_vec(),
                transport_key: vec![0xa5; TRANSPORT_KEY_LEN],
                allowed_scopes: vec![scope_entry(APP_PROGRAM, SCOPE)],
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

    /// Drops the scope list, making the permit permissive.
    pub fn permissive(mut self) -> Self {
        self.wire.allowed_scopes.clear();
        self
    }

    /// Replaces the signed application scope with the given `(program, scope)` pairs, sorted
    /// into the canonical ascending order the typed form demands.
    pub fn scope(mut self, pairs: &[(SolanaPubkeyBytes, SolanaPubkeyBytes)]) -> Self {
        let mut entries: Vec<Vec<u8>> = pairs
            .iter()
            .map(|(program, scope)| scope_entry(*program, *scope))
            .collect();
        entries.sort();
        self.wire.allowed_scopes = entries;
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

    /// Adds a direct entry: the signer's own allow leaf on the handle authorizes it.
    pub fn direct(
        self,
        encrypted_value_account: &EncryptedValueAccountFixture,
        handle: [u8; 32],
    ) -> Self {
        let signer = self.wallet.pubkey();
        self.entry(handle, signer, encrypted_value_account.account_key)
    }

    /// Adds a delegated entry: `delegator`'s allow leaf on the handle authorizes it, through a
    /// delegation to the signer.
    pub fn delegated(
        self,
        encrypted_value_account: &EncryptedValueAccountFixture,
        handle: [u8; 32],
        delegator: SolanaPubkeyBytes,
    ) -> Self {
        self.entry(handle, delegator, encrypted_value_account.account_key)
    }

    /// Adds an entry verbatim, for the malformed and substitution cases.
    pub fn entry(
        mut self,
        handle: [u8; 32],
        allowed_key: SolanaPubkeyBytes,
        encrypted_value_account: SolanaPubkeyBytes,
    ) -> Self {
        self.entries.push(SolanaHandleEntryWire {
            handle: handle.to_vec(),
            allowed_key: allowed_key.to_vec(),
            encrypted_value_account: encrypted_value_account.to_vec(),
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

// ---------------------------------------------------------------------------
// Account layouts
// ---------------------------------------------------------------------------

/// One sealed leaf: the query that names it and the commitment the MMR holds.
#[derive(Clone, Copy, Debug)]
pub struct SealedLeaf {
    /// Which `(account, handle, kind)` the leaf answers.
    pub query: LeafQuery,
    /// The leaf commitment.
    pub commitment: [u8; 32],
}

/// An encrypted value account as the host program would hold it, with the leaves the
/// coprocessors' record would hold for it.
#[derive(Clone, Debug)]
pub struct EncryptedValueAccountFixture {
    /// The encrypted value account state.
    pub encrypted_value: EncryptedValue,
    /// Its canonical address.
    pub account_key: SolanaPubkeyBytes,
    /// Every leaf sealed so far, in leaf order, so proofs can be rebuilt.
    pub leaves: Vec<SealedLeaf>,
}

impl EncryptedValueAccountFixture {
    /// An encrypted value account of the fixture application holding `current_handle`, with no
    /// leaf sealed yet.
    pub fn new(current_handle: [u8; 32]) -> Self {
        Self::in_application(APP_PROGRAM, AUTHORITY, SCOPE, LABEL, current_handle)
    }

    /// An encrypted value account of an arbitrary application, authority, scope and label.
    pub fn in_application(
        program: SolanaPubkeyBytes,
        encrypted_value_account_authority: SolanaPubkeyBytes,
        scope: SolanaPubkeyBytes,
        label: [u8; 32],
        current_handle: [u8; 32],
    ) -> Self {
        let (account_key, bump) = Pubkey::find_program_address(
            &encrypted_value_seeds(&program, &encrypted_value_account_authority, &scope, &label),
            &Pubkey::new_from_array(PROGRAM_ID),
        );
        Self {
            encrypted_value: EncryptedValue {
                program,
                encrypted_value_account_authority,
                scope,
                label,
                current_handle,
                leaf_count: 0,
                peaks: Vec::new(),
                bump,
            },
            account_key: account_key.to_bytes(),
            leaves: Vec::new(),
        }
    }

    /// An account holding `current_handle` with `key` already allowed on it: the state one
    /// write plus one allow leaves behind, and the reference state of most scenarios.
    pub fn allowing(current_handle: [u8; 32], key: SolanaPubkeyBytes) -> Self {
        let mut fixture = Self::new(current_handle);
        fixture.allow(key);
        fixture
    }

    /// The current handle.
    pub fn current_handle(&self) -> [u8; 32] {
        self.encrypted_value.current_handle
    }

    /// Seals an allow leaf naming `key` on the current handle — what the host program does when
    /// the application allows a key.
    pub fn allow(&mut self, key: SolanaPubkeyBytes) {
        let handle = self.encrypted_value.current_handle;
        let leaf_index = self.encrypted_value.leaf_count;
        let commitment =
            historical_access_leaf_commitment(self.account_key, leaf_index, handle, key);
        self.append(
            LeafQuery {
                encrypted_value_account: self.account_key,
                handle,
                kind: LeafKind::Allowed { key },
            },
            commitment,
        );
    }

    /// Seals a public-decrypt leaf for the current handle.
    pub fn mark_public(&mut self) {
        let handle = self.encrypted_value.current_handle;
        let leaf_index = self.encrypted_value.leaf_count;
        let commitment = public_decrypt_leaf_commitment(self.account_key, leaf_index, handle);
        self.append(
            LeafQuery {
                encrypted_value_account: self.account_key,
                handle,
                kind: LeafKind::Public,
            },
            commitment,
        );
    }

    /// Replaces the current handle, as a write does. Seals nothing: the leaves already sealed
    /// keep naming the handle they were sealed for.
    pub fn update(&mut self, new_handle: [u8; 32]) {
        self.encrypted_value.current_handle = new_handle;
    }

    /// Appends a commitment, keeping the leaf list in step with the MMR.
    pub fn append(&mut self, query: LeafQuery, commitment: [u8; 32]) {
        mmr_append(
            &mut self.encrypted_value.peaks,
            &mut self.encrypted_value.leaf_count,
            commitment,
        )
        .expect("the fixture MMR accepts an append");
        self.leaves.push(SealedLeaf { query, commitment });
    }

    /// The commitments in leaf order.
    pub fn commitments(&self) -> Vec<[u8; 32]> {
        self.leaves.iter().map(|leaf| leaf.commitment).collect()
    }

    /// A proof of the leaf at `leaf_index`, against the current MMR.
    pub fn proof(&self, leaf_index: u64) -> MmrProof {
        mmr_build_proof(&self.commitments(), leaf_index).expect("the fixture MMR builds a proof")
    }

    /// What the coprocessors' record answers for `query` when it has sealed exactly this
    /// account's leaves.
    pub fn outcome(&self, query: &LeafQuery) -> LeafProofOutcome {
        let leaf_count = self.encrypted_value.leaf_count;
        match self.leaves.iter().position(|leaf| leaf.query == *query) {
            Some(index) => {
                let proof = self.proof(index as u64);
                LeafProofOutcome::Found {
                    leaf_index: proof.leaf_index,
                    leaf_count,
                    siblings: proof.siblings,
                }
            }
            None => LeafProofOutcome::NotFound { leaf_count },
        }
    }

    /// The allow query for `key` on `handle` under this account.
    pub fn allowed_query(&self, handle: [u8; 32], key: SolanaPubkeyBytes) -> LeafQuery {
        LeafQuery {
            encrypted_value_account: self.account_key,
            handle,
            kind: LeafKind::Allowed { key },
        }
    }

    /// The public query for `handle` under this account.
    pub fn public_query(&self, handle: [u8; 32]) -> LeafQuery {
        LeafQuery {
            encrypted_value_account: self.account_key,
            handle,
            kind: LeafKind::Public,
        }
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
    /// Which authority it covers.
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
    /// A live delegation covering the fixture authority.
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

/// The canonical config-singleton address of the fixture deployment, derived here rather than
/// taken from the code under test.
pub fn host_config_address() -> (SolanaPubkeyBytes, u8) {
    let (address, bump) =
        Pubkey::find_program_address(&[HOST_CONFIG_SEED], &Pubkey::new_from_array(PROGRAM_ID));
    (address.to_bytes(), bump)
}

/// The config singleton as the host program would write it, through the shared crate's own
/// encoder — the inverse of the decoder under test, so no field table is restated here.
pub fn host_config_account(paused: bool) -> SnapshotAccount {
    let (_, bump) = host_config_address();
    SnapshotAccount {
        owner: PROGRAM_ID,
        data: encode_host_config(&HostConfigRecord { paused, bump }),
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
    /// The encrypted value accounts placed whole, so the leaf record that agrees with this world
    /// can be derived from it.
    sealed: BTreeMap<SolanaPubkeyBytes, EncryptedValueAccountFixture>,
}

impl World {
    /// A world at `slot` holding a running host: the config singleton is present and unpaused, so
    /// a scenario that says nothing about pause is a scenario in which pause is not the point.
    /// Named for what it holds rather than for the slot, because it is not empty.
    pub fn running_at_slot(slot: u64) -> Self {
        let mut accounts = BTreeMap::new();
        let (key, _) = host_config_address();
        accounts.insert(key, host_config_account(false));
        Self {
            slot,
            accounts,
            sealed: BTreeMap::new(),
        }
    }

    /// The same world with the host paused.
    pub fn paused(mut self) -> Self {
        let (key, _) = host_config_address();
        self.accounts.insert(key, host_config_account(true));
        self
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
        self.sealed.insert(
            encrypted_value_account.account_key,
            encrypted_value_account.clone(),
        );
        self
    }

    /// The leaf record that has sealed exactly what this world's encrypted value accounts hold:
    /// the record a coprocessor in step with this observation would serve.
    pub fn record(&self) -> ProofRecord {
        self.sealed
            .values()
            .fold(ProofRecord::default(), |record, account| {
                record.with(account)
            })
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
            sealed: BTreeMap::new(),
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

// ---------------------------------------------------------------------------
// The leaf record and its reader
// ---------------------------------------------------------------------------

/// What the coprocessors' record holds for one account.
#[derive(Clone, Debug)]
enum RecordedAccount {
    /// The record has sealed exactly this account's leaves — possibly an older state of the
    /// account than the chain shows, which is how a record behind the chain is expressed.
    Sealed(Box<EncryptedValueAccountFixture>),
    /// The record's history for the account has a gap.
    Incomplete,
}

/// The coprocessors' leaf record as one read of it would answer: a set of accounts whose leaves
/// it has sealed. An account not in the record is unknown to it.
///
/// A record can also be given its answers outright, query by query, which is how a recorded
/// observation is replayed without the fixtures that produced it.
#[derive(Clone, Debug, Default)]
pub struct ProofRecord {
    accounts: BTreeMap<SolanaPubkeyBytes, RecordedAccount>,
    answers: BTreeMap<LeafQuery, LeafProofOutcome>,
}

impl ProofRecord {
    /// A record that has sealed the leaves of the given accounts, as they stand.
    pub fn of(accounts: &[&EncryptedValueAccountFixture]) -> Self {
        accounts
            .iter()
            .fold(Self::default(), |record, account| record.with(account))
    }

    /// Adds an account's leaves to the record.
    pub fn with(mut self, encrypted_value_account: &EncryptedValueAccountFixture) -> Self {
        self.accounts.insert(
            encrypted_value_account.account_key,
            RecordedAccount::Sealed(Box::new(encrypted_value_account.clone())),
        );
        self
    }

    /// Marks an account's history as having a gap the record cannot close.
    pub fn incomplete(mut self, account_key: SolanaPubkeyBytes) -> Self {
        self.accounts
            .insert(account_key, RecordedAccount::Incomplete);
        self
    }

    /// Forgets an account, making it unknown to the record.
    pub fn without(mut self, account_key: &SolanaPubkeyBytes) -> Self {
        self.accounts.remove(account_key);
        self
    }

    /// A record answering exactly these queries with exactly these outcomes. Any other query is
    /// an account unknown to it.
    pub fn answering(answers: impl IntoIterator<Item = (LeafQuery, LeafProofOutcome)>) -> Self {
        Self {
            accounts: BTreeMap::new(),
            answers: answers.into_iter().collect(),
        }
    }

    /// What the record answers for one query.
    pub fn answer(&self, query: &LeafQuery) -> LeafProofOutcome {
        if let Some(outcome) = self.answers.get(query) {
            return outcome.clone();
        }
        match self.accounts.get(&query.encrypted_value_account) {
            Some(RecordedAccount::Sealed(account)) => account.outcome(query),
            Some(RecordedAccount::Incomplete) => LeafProofOutcome::HistoryIncomplete,
            None => LeafProofOutcome::UnknownAccount,
        }
    }
}

/// A proof reader that answers from a script of records and counts every read.
///
/// Like [`ScriptedReader`], it answers in order and panics past the end of its script: the
/// pipeline reads proofs once, or twice when the record is behind, and a third read is a
/// defect to surface rather than a case to absorb.
pub struct ScriptedProofReader {
    records: Vec<ProofRecord>,
    calls: Mutex<Vec<Vec<LeafQuery>>>,
}

impl ScriptedProofReader {
    /// A reader whose every read sees the same record. Two copies, because the pipeline may
    /// legitimately read twice.
    pub fn constant(record: ProofRecord) -> Self {
        Self::scripted(vec![record.clone(), record])
    }

    /// A reader whose reads see the given records in order.
    pub fn scripted(records: Vec<ProofRecord>) -> Self {
        Self {
            records,
            calls: Mutex::new(Vec::new()),
        }
    }

    /// A reader over a record that has sealed exactly these accounts' leaves.
    pub fn serving(accounts: &[&EncryptedValueAccountFixture]) -> Self {
        Self::constant(ProofRecord::of(accounts))
    }

    /// A reader that must never be asked: for the scenarios that are rejected before the proof
    /// read, where reaching it would mean a rule ran out of order.
    pub fn unreachable() -> Self {
        Self::scripted(Vec::new())
    }

    /// How many times the record was read.
    pub fn call_count(&self) -> usize {
        self.calls.lock().expect("proof reader lock").len()
    }

    /// The query batches that were read, in order.
    pub fn calls(&self) -> Vec<Vec<LeafQuery>> {
        self.calls.lock().expect("proof reader lock").clone()
    }
}

impl HostProofReader for ScriptedProofReader {
    async fn read_proofs(
        &self,
        queries: &[LeafQuery],
    ) -> Result<Vec<LeafProofOutcome>, ProofReadError> {
        let index = {
            let mut calls = self.calls.lock().expect("proof reader lock");
            calls.push(queries.to_vec());
            calls.len() - 1
        };
        let record = self.records.get(index).unwrap_or_else(|| {
            panic!(
                "authorization read leaf proofs {} time(s); the script provides {}",
                index + 1,
                self.records.len()
            )
        });
        Ok(queries.iter().map(|query| record.answer(query)).collect())
    }
}

/// A proof reader whose record is unreachable: every read fails as the transport would.
pub struct UnavailableProofReader;

impl HostProofReader for UnavailableProofReader {
    async fn read_proofs(
        &self,
        _queries: &[LeafQuery],
    ) -> Result<Vec<LeafProofOutcome>, ProofReadError> {
        Err(ProofReadError::Unavailable {
            reason: "no coprocessor answered".to_owned(),
        })
    }
}

// ---------------------------------------------------------------------------
// KMS pair validators
// ---------------------------------------------------------------------------

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
