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
#![allow(dead_code)]

use alloy::primitives::U256;
use base64::{Engine, engine::general_purpose::STANDARD as BASE64_STANDARD};
use connector_utils::types::solana_request::{
    PermitWireFields, SolanaHandleEntryWire, SolanaUserDecryptRequestWire,
    SolanaUserDecryptionRequestV1,
};
use kms_worker::core::solana::{
    SolanaHost, SolanaPubkeyBytes,
    pipeline::AuthorizationContext,
    proof::{
        CoprocessorProofClient, HostProofReader, LeafKind, LeafProofOutcome, LeafQuery,
        ProofReadError, ProofResponses, leaf_proof_request_body,
    },
    snapshot::{
        HostSnapshot, HostStateReader, SYSTEM_PROGRAM_ID, SnapshotAccount, SnapshotError,
        SnapshotKeys, SolanaRpcClient,
    },
};
use mocktail::server::MockServer;
use reqwest::Client;
use ring::signature::{Ed25519KeyPair, KeyPair};
use solana_pubkey::Pubkey;
use std::collections::BTreeMap;
use std::num::NonZeroUsize;
use std::sync::Mutex;
use std::time::Duration;
use zama_solana_acl::{
    DELEGATION_SEED, EncryptedSlot, EncryptedStore, HOST_CONFIG_SEED, HostConfigRecord, MmrProof,
    PERMIT_INVALIDATION_SEED, PermitInvalidationRecord, USER_DECRYPTION_DELEGATION_DISCRIMINATOR,
    WILDCARD_AUTHORITY, encode_host_config, encode_permit_invalidation,
    encrypted_store_discriminator, historical_access_leaf_commitment, mmr_append, mmr_build_proof,
    public_decrypt_leaf_commitment,
};
use zama_solana_permit::{
    Identity, KmsRouting, PermitFields, Signature, TRANSPORT_KEY_LEN, build_envelope,
};

/// The host deployment every fixture is built against.
pub const PROGRAM_ID: SolanaPubkeyBytes = [7; 32];
/// Type byte `0x01` over a cluster tag, as every Solana host chain id.
pub const CHAIN_ID: u64 = 0x0123_4567_89ab_cdef;

/// The application program of the default encrypted store.
pub const APP_PROGRAM: SolanaPubkeyBytes = [1; 32];
/// The encrypted store authority of the default encrypted store.
pub const AUTHORITY: SolanaPubkeyBytes = [2; 32];
/// The program-declared scope of the default encrypted store.
pub const SCOPE: SolanaPubkeyBytes = [3; 32];
/// The label of the default encrypted store.
pub const LABEL: [u8; 32] = *b"balance_________________________";

/// FHE type byte of a boolean handle — the narrowest type, two bits.
pub const FHE_TYPE_BOOL: u8 = 0;
/// FHE type byte of a 64-bit handle.
pub const FHE_TYPE_UINT64: u8 = 5;
/// Handle format version byte.
pub const HANDLE_VERSION: u8 = 0;

// ---------------------------------------------------------------------------
// Handles
// ---------------------------------------------------------------------------

/// A handle of the fixture cluster: distinguishing byte, chain id big-endian at `[22..30]`,
/// FHE type at `[30]`, version at `[31]`.
pub fn handle(tag: u8, fhe_type: u8) -> [u8; 32] {
    let mut bytes = [tag; 32];
    bytes[22..30].copy_from_slice(&CHAIN_ID.to_be_bytes());
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
/// The fixture deployment inside the default window.
pub const CONTEXT: AuthorizationContext = context_at(NOW_INSIDE_WINDOW);

pub const fn context_at(now_unix_seconds: u64) -> AuthorizationContext {
    AuthorizationContext {
        program_id: PROGRAM_ID,
        now_unix_seconds,
    }
}
/// The KMS context of the default permit.
pub const KMS_CONTEXT: SolanaPubkeyBytes = [0x11; 32];
/// The KMS epoch of the default permit.
pub const KMS_EPOCH: SolanaPubkeyBytes = [0x12; 32];

impl PermitBuilder {
    /// A permit for `user`, scoped to the fixture application.
    pub fn new(user: SolanaPubkeyBytes) -> Self {
        Self {
            wire: PermitWireFields {
                user_address: user.to_vec(),
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

    /// Replaces the signed chain, which every handle of the request must name.
    pub fn chain_id(mut self, chain_id: u64) -> Self {
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
    pub fn direct(self, encrypted_store: &EncryptedStoreFixture, handle: [u8; 32]) -> Self {
        let signer = self.wallet.pubkey();
        self.entry(handle, signer, encrypted_store.account_key)
    }

    /// Adds a delegated entry: `delegator`'s allow leaf on the handle authorizes it, through a
    /// delegation to the signer.
    pub fn delegated(
        self,
        encrypted_store: &EncryptedStoreFixture,
        handle: [u8; 32],
        delegator: SolanaPubkeyBytes,
    ) -> Self {
        self.entry(handle, delegator, encrypted_store.account_key)
    }

    /// Adds an entry verbatim, for the malformed and substitution cases.
    pub fn entry(
        mut self,
        handle: [u8; 32],
        owner_address: SolanaPubkeyBytes,
        encrypted_store: SolanaPubkeyBytes,
    ) -> Self {
        self.entries.push(SolanaHandleEntryWire {
            handle: handle.to_vec(),
            owner_address: owner_address.to_vec(),
            encrypted_store: encrypted_store.to_vec(),
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
    pub fn typed(&self) -> SolanaUserDecryptionRequestV1 {
        SolanaUserDecryptionRequestV1::new(U256::from(1), &self.wire())
            .expect("fixture request is well formed")
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

/// An encrypted store as the host program would hold it, with the leaves the
/// coprocessors' record would hold for it.
#[derive(Clone, Debug)]
pub struct EncryptedStoreFixture {
    /// The encrypted store state.
    pub encrypted_store: EncryptedStore,
    /// Its canonical address.
    pub account_key: SolanaPubkeyBytes,
    /// Every leaf sealed so far, in leaf order, so proofs can be rebuilt.
    pub leaves: Vec<SealedLeaf>,
}

impl EncryptedStoreFixture {
    /// An encrypted store of the fixture application holding `current_handle`, with no
    /// leaf sealed yet.
    pub fn new(current_handle: [u8; 32]) -> Self {
        Self::in_application(APP_PROGRAM, AUTHORITY, SCOPE, LABEL, current_handle)
    }

    /// An encrypted store of an arbitrary application, authority, scope and label.
    pub fn in_application(
        program: SolanaPubkeyBytes,
        authority: SolanaPubkeyBytes,
        scope: SolanaPubkeyBytes,
        label: [u8; 32],
        current_handle: [u8; 32],
    ) -> Self {
        let (account_key, bump) = Pubkey::find_program_address(
            &[b"encrypted-state", &program, &authority, &scope],
            &Pubkey::new_from_array(PROGRAM_ID),
        );
        Self {
            encrypted_store: EncryptedStore {
                program,
                authority,
                scope,
                slots: vec![EncryptedSlot {
                    key: label,
                    handle: current_handle,
                }],
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
        self.encrypted_store.slots[0].handle
    }

    /// Seals an allow leaf naming `key` on the current handle — what the host program does when
    /// the application allows a key.
    pub fn allow(&mut self, key: SolanaPubkeyBytes) {
        let handle = self.current_handle();
        self.allow_handle(handle, key);
    }

    /// Seals an allow leaf for an exact handle. The handle need not still occupy a slot.
    pub fn allow_handle(&mut self, handle: [u8; 32], key: SolanaPubkeyBytes) {
        let leaf_index = self.encrypted_store.leaf_count;
        let commitment =
            historical_access_leaf_commitment(self.account_key, leaf_index, handle, key);
        self.append(
            LeafQuery {
                encrypted_store: self.account_key,
                handle,
                kind: LeafKind::Allowed { key },
            },
            commitment,
        );
    }

    /// Seals a public-decrypt leaf for the current handle.
    pub fn mark_public(&mut self) {
        let handle = self.current_handle();
        let leaf_index = self.encrypted_store.leaf_count;
        let commitment = public_decrypt_leaf_commitment(self.account_key, leaf_index, handle);
        self.append(
            LeafQuery {
                encrypted_store: self.account_key,
                handle,
                kind: LeafKind::Public,
            },
            commitment,
        );
    }

    /// Replaces the current handle, as a write does. Seals nothing: the leaves already sealed
    /// keep naming the handle they were sealed for.
    pub fn update(&mut self, new_handle: [u8; 32]) {
        self.encrypted_store.slots[0].handle = new_handle;
    }

    /// Appends a commitment, keeping the leaf list in step with the MMR.
    pub fn append(&mut self, query: LeafQuery, commitment: [u8; 32]) {
        mmr_append(
            &mut self.encrypted_store.peaks,
            &mut self.encrypted_store.leaf_count,
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
        let leaf_count = self.encrypted_store.leaf_count;
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
            encrypted_store: self.account_key,
            handle,
            kind: LeafKind::Allowed { key },
        }
    }

    /// The public query for `handle` under this account.
    pub fn public_query(&self, handle: [u8; 32]) -> LeafQuery {
        LeafQuery {
            encrypted_store: self.account_key,
            handle,
            kind: LeafKind::Public,
        }
    }

    /// The account as the host program would write it: discriminator then body.
    pub fn account(&self) -> SnapshotAccount {
        let mut data = encrypted_store_discriminator().to_vec();
        data.extend_from_slice(
            &borsh::to_vec(&self.encrypted_store).expect("the encrypted store serializes"),
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
    pub authority: SolanaPubkeyBytes,
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
            authority: AUTHORITY,
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
            authority: WILDCARD_AUTHORITY,
            ..Self::live(delegator, delegate, observed_slot)
        }
    }

    /// Its canonical address and bump, derived here rather than taken from the code under test.
    pub fn address(&self) -> (SolanaPubkeyBytes, u8) {
        let (address, bump) = Pubkey::find_program_address(
            &[
                DELEGATION_SEED,
                &self.delegator,
                &self.delegate,
                &self.authority,
            ],
            &Pubkey::new_from_array(PROGRAM_ID),
        );
        (address.to_bytes(), bump)
    }

    /// The account as the host program would write it.
    pub fn account(&self) -> SnapshotAccount {
        let (_, bump) = self.address();
        let mut data = USER_DECRYPTION_DELEGATION_DISCRIMINATOR.to_vec();
        data.extend_from_slice(&self.delegator);
        data.extend_from_slice(&self.delegate);
        data.extend_from_slice(&self.authority);
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
        &[PERMIT_INVALIDATION_SEED, user.as_ref()],
        &Pubkey::new_from_array(PROGRAM_ID),
    );
    (address.to_bytes(), bump)
}

/// An invalidation record holding `watermark` for `user`.
pub fn invalidation_account(user: SolanaPubkeyBytes, watermark: u64) -> SnapshotAccount {
    let (_, bump) = invalidation_address(user);
    SnapshotAccount {
        owner: PROGRAM_ID,
        data: encode_permit_invalidation(&PermitInvalidationRecord {
            user,
            invalidation_watermark: watermark,
            bump,
        }),
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
    /// The encrypted stores placed whole, so the leaf record that agrees with this world
    /// can be derived from it.
    sealed: BTreeMap<SolanaPubkeyBytes, EncryptedStoreFixture>,
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

    /// Places an encrypted store in the world.
    pub fn with_encrypted_store(mut self, encrypted_store: &EncryptedStoreFixture) -> Self {
        self.accounts
            .insert(encrypted_store.account_key, encrypted_store.account());
        self.sealed
            .insert(encrypted_store.account_key, encrypted_store.clone());
        self
    }

    /// The leaf record that has sealed exactly what this world's encrypted stores hold:
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

    /// Projects the world onto the requested keys, exactly as an account read would.
    pub fn read(&self, keys: &SnapshotKeys) -> HostSnapshot {
        let accounts = keys
            .as_slice()
            .iter()
            .map(|key| (*key, self.accounts.get(key).cloned()));
        HostSnapshot::new(self.slot, accounts)
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
        Ok(world.read(keys))
    }
}

// ---------------------------------------------------------------------------
// The leaf record and its reader
// ---------------------------------------------------------------------------

/// The coprocessors' leaf record as one read of it would answer: a set of accounts whose leaves
/// it has sealed, possibly an older state than the chain shows. An account not in the record is
/// unknown to it. A record can also be given its answers outright, query by query.
#[derive(Clone, Debug, Default)]
pub struct ProofRecord {
    accounts: BTreeMap<SolanaPubkeyBytes, EncryptedStoreFixture>,
    answers: BTreeMap<LeafQuery, LeafProofOutcome>,
}

impl ProofRecord {
    /// A record that has sealed the leaves of the given accounts, as they stand.
    pub fn of(accounts: &[&EncryptedStoreFixture]) -> Self {
        accounts
            .iter()
            .fold(Self::default(), |record, account| record.with(account))
    }

    /// Adds an account's leaves to the record.
    pub fn with(mut self, encrypted_store: &EncryptedStoreFixture) -> Self {
        self.accounts
            .insert(encrypted_store.account_key, encrypted_store.clone());
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
        self.accounts
            .get(&query.encrypted_store)
            .map_or(LeafProofOutcome::UnknownAccount, |account| {
                account.outcome(query)
            })
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
    pub fn serving(accounts: &[&EncryptedStoreFixture]) -> Self {
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
    async fn read_proofs(&self, queries: &[LeafQuery]) -> Result<ProofResponses, ProofReadError> {
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
        Ok(ProofResponses {
            candidates: queries
                .iter()
                .map(|query| vec![record.answer(query)])
                .collect(),
            unavailable: None,
        })
    }
}

/// A proof reader whose record is unreachable: every read fails as the transport would.
pub struct UnavailableProofReader;

impl HostProofReader for UnavailableProofReader {
    async fn read_proofs(&self, _queries: &[LeafQuery]) -> Result<ProofResponses, ProofReadError> {
        Err(ProofReadError::Unavailable {
            reason: "no coprocessor answered".to_owned(),
        })
    }
}

// ---------------------------------------------------------------------------
// A host behind real HTTP
// ---------------------------------------------------------------------------

/// The route the connector reads leaf proofs from. A literal, so a moved route fails the tests.
pub const LEAF_PROOFS_ROUTE: &str = "/v1/solana/leaf-proofs";

/// A Solana node and a coprocessor behind real HTTP, for tests that go through the production
/// readers. Each answers only the exact request body it is scripted for, which pins the keys,
/// encoding and commitment of a read, and the queries of a proof batch.
pub struct HttpHost {
    pub rpc: MockServer,
    pub coprocessor: MockServer,
}

impl HttpHost {
    pub async fn start() -> Self {
        let rpc = MockServer::new_http("solana-rpc");
        rpc.start().await.expect("the mock RPC starts");
        let coprocessor = MockServer::new_http("coprocessor");
        coprocessor
            .start()
            .await
            .expect("the mock coprocessor starts");
        Self { rpc, coprocessor }
    }

    /// Replaces the node's state: one `getMultipleAccounts` read of these keys, in this order.
    pub fn serve_accounts(&mut self, accounts: &[(SolanaPubkeyBytes, Option<SnapshotAccount>)]) {
        let keys: Vec<_> = accounts.iter().map(|(key, _)| *key).collect();
        let request = multiple_accounts_request(&keys);
        let value: Vec<_> = accounts
            .iter()
            .map(|(_, account)| {
                account.as_ref().map(|account| {
                    serde_json::json!({
                        "owner": Pubkey::new_from_array(account.owner).to_string(),
                        "data": [BASE64_STANDARD.encode(&account.data), "base64"],
                        "lamports": 1,
                        "executable": false,
                        "rentEpoch": 0,
                    })
                })
            })
            .collect();
        let response = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": { "context": { "slot": 1 }, "value": value },
        });
        self.rpc.mocks().clear();
        self.rpc.mock(move |when, then| {
            when.post().json(request.clone());
            then.json(response.clone());
        });
    }

    /// Replaces the coprocessor's record: one leaf-proof batch of these queries.
    pub fn serve_proofs(&mut self, answers: &[(LeafQuery, LeafProofOutcome)]) {
        serve_proofs(&mut self.coprocessor, answers);
    }

    pub fn host(&self) -> SolanaHost {
        solana_host(&self.rpc, &[&self.coprocessor])
    }
}

/// The `getMultipleAccounts` body the connector sends for `keys`, at confirmed commitment.
pub fn multiple_accounts_request(keys: &[SolanaPubkeyBytes]) -> serde_json::Value {
    serde_json::json!({
        "jsonrpc": "2.0",
        "id": 0,
        "method": "getMultipleAccounts",
        "params": [
            keys.iter()
                .map(|key| Pubkey::new_from_array(*key).to_string())
                .collect::<Vec<_>>(),
            {"encoding": "base64", "commitment": "confirmed", "dataSlice": null, "minContextSlot": null},
        ],
    })
}

/// Scripts `coprocessor` to answer one leaf-proof batch.
pub fn serve_proofs(coprocessor: &mut MockServer, answers: &[(LeafQuery, LeafProofOutcome)]) {
    let queries: Vec<_> = answers.iter().map(|(query, _)| *query).collect();
    // Mocktail compares bytes; going through a `Value` would reorder the typed fields.
    let request = serde_json::to_string(&leaf_proof_request_body(&queries)).unwrap();
    let response = serde_json::json!({
        "proofs": answers.iter().map(|(_, outcome)| wire_outcome(outcome)).collect::<Vec<_>>(),
    });
    coprocessor.mocks().clear();
    coprocessor.mock(move |when, then| {
        when.post().path(LEAF_PROOFS_ROUTE).text(request.clone());
        then.json(response.clone());
    });
}

/// A host of the fixture deployment reading from `rpc` and asking every one of `coprocessors`.
pub fn solana_host(rpc: &MockServer, coprocessors: &[&MockServer]) -> SolanaHost {
    let endpoints: Vec<_> = coprocessors
        .iter()
        .map(|coprocessor| coprocessor.base_url().unwrap().clone())
        .collect();
    SolanaHost {
        program_id: PROGRAM_ID,
        reader: SolanaRpcClient::new(
            rpc.base_url().unwrap().clone(),
            Duration::from_secs(10),
            NonZeroUsize::MIN,
        ),
        proofs: CoprocessorProofClient::new(&endpoints, "test-key".to_owned(), Client::new()),
    }
}

/// A leaf-proof answer as the coprocessor route serializes it.
fn wire_outcome(outcome: &LeafProofOutcome) -> serde_json::Value {
    match outcome {
        LeafProofOutcome::Found {
            leaf_index,
            leaf_count,
            siblings,
        } => serde_json::json!({
            "status": "found",
            "leafIndex": leaf_index,
            "leafCount": leaf_count,
            "siblings": siblings.iter().map(alloy::hex::encode).collect::<Vec<_>>(),
        }),
        LeafProofOutcome::NotFound { leaf_count } => {
            serde_json::json!({ "status": "notFound", "leafCount": leaf_count })
        }
        LeafProofOutcome::UnknownAccount => serde_json::json!({ "status": "unknownAccount" }),
        LeafProofOutcome::HistoryIncomplete => {
            serde_json::json!({ "status": "historyIncomplete" })
        }
    }
}
