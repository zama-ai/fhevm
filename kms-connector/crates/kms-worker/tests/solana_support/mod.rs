//! Fixtures for the Solana authorization suite.
//!
//! The centre of this module is two scripted readers over two kinds of state. [`World`] plus
//! [`ScriptedReader`] is a set of accounts at a slot and a reader that answers from it while
//! recording what it was asked for; [`ProofRecord`] plus [`ScriptedProofReader`] is a
//! coprocessor's leaf record and a reader that answers leaf-proof queries from a list of them,
//! likewise recording. Together they turn a race into a value — a scenario is two worlds, or a world and a
//! record that disagree, not two moments — and they let the suite assert how many times
//! authorization reads either source, which is otherwise an invisible property.
//!
//! Everything else here builds the three account layouts authorization reads (encrypted store,
//! delegation record, invalidation record), seals the leaves the record serves, and
//! signs real permits with a real wallet key, so no test depends on a signature the code under
//! test produced.
#![allow(dead_code)]

use alloy::primitives::B256;
use alloy::primitives::U256;
use base64::{Engine, engine::general_purpose::STANDARD as BASE64_STANDARD};
use connector_utils::types::solana_request::{
    SolanaEntryClaims, SolanaRequestBlob, SolanaUserDecryptFields, SolanaUserDecryptionRequestV1,
};
use kms_worker::core::solana::{
    SolanaHost,
    pipeline::AuthorizationContext,
    proof::{
        CoprocessorProofClient, HostProofReader, LeafKind, LeafProofOutcome, LeafQuery,
        ProofReadError, leaf_proof_request_body,
    },
    snapshot::{
        AccountsRead, DerivedAddress, HostStateReader, ObservedRow, ObservedRows, SnapshotAccount,
        SnapshotError, SolanaRpcClient,
    },
};
use kms_worker::core::{ApiKey, ProofRoute};
use mocktail::server::MockServer;
use reqwest::Client;
use ring::signature::{Ed25519KeyPair, KeyPair};
use solana_pubkey::Pubkey;
use std::collections::BTreeMap;
use std::num::NonZeroUsize;
use std::sync::Mutex;
use std::time::Duration;
use zama_solana_acl::{
    CLOCK_SYSVAR_ID, DELEGATION_SEED, EncryptedSlot, EncryptedStore, MmrProof,
    PERMIT_INVALIDATION_SEED, PermitInvalidationRecord, SYSVAR_OWNER_ID,
    UserDecryptionDelegationRecord, WILDCARD_APP, encode_clock, encode_permit_invalidation,
    encode_user_decryption_delegation, encrypted_store_discriminator,
    historical_access_leaf_commitment, mmr_append, mmr_build_proof, public_decrypt_leaf_commitment,
};
use zama_solana_permit::{
    Identity, KmsRouting, PermitFields, PermitWireFields, Signature, TRANSPORT_KEY_LEN,
    build_envelope,
};
use zama_solana_request::HandleEntry;

/// The System program: the owner of an account no program has taken over.
pub const SYSTEM_PROGRAM_ID: Pubkey = Pubkey::new_from_array(zama_solana_acl::SYSTEM_PROGRAM_ID);

/// A key made of one repeated byte.
pub const fn pubkey(byte: u8) -> Pubkey {
    Pubkey::new_from_array([byte; 32])
}

/// The host deployment every fixture is built against.
pub const PROGRAM_ID: Pubkey = pubkey(7);
/// Type byte `0x01` over a cluster tag, as every Solana host chain id.
pub const CHAIN_ID: u64 = 0x0123_4567_89ab_cdef;

/// The application program of the default encrypted store.
pub const APP_PROGRAM: Pubkey = pubkey(1);
/// The encrypted store authority of the default encrypted store.
pub const AUTHORITY: Pubkey = pubkey(2);
/// The scope of the default encrypted store: an account its program owns.
pub const SCOPE: Pubkey = pubkey(3);
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
    pub fn pubkey(&self) -> Pubkey {
        Pubkey::try_from(self.keypair.public_key().as_ref())
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
pub fn scope_entry(program: Pubkey, scope: Pubkey) -> Vec<u8> {
    let mut entry = program.to_bytes().to_vec();
    entry.extend_from_slice(scope.as_ref());
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
/// The Unix time the host's Clock reads in every world, unless a test sets another.
pub const HOST_NOW: u64 = NOW_INSIDE_WINDOW;
/// The fixture deployment inside the default window.
pub const CONTEXT: AuthorizationContext = context_at(NOW_INSIDE_WINDOW);

pub const fn context_at(now_unix_seconds: u64) -> AuthorizationContext {
    AuthorizationContext {
        program_id: PROGRAM_ID,
        now_unix_seconds,
    }
}
/// The KMS context of the default permit.
pub const KMS_CONTEXT: [u8; 32] = [0x11; 32];
/// The KMS epoch of the default permit.
pub const KMS_EPOCH: [u8; 32] = [0x12; 32];

impl PermitBuilder {
    /// A permit for `user`, scoped to the fixture application.
    pub fn new(user: Pubkey) -> Self {
        Self {
            wire: PermitWireFields {
                user_address: user.to_bytes().to_vec(),
                transport_key: vec![0xa5; TRANSPORT_KEY_LEN],
                allowed_scopes: vec![scope_entry(APP_PROGRAM, SCOPE)],
                start_timestamp: DEFAULT_START,
                duration_seconds: DEFAULT_DURATION,
                verifying_program_id: PROGRAM_ID.to_bytes().to_vec(),
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
    pub fn scope(mut self, pairs: &[(Pubkey, Pubkey)]) -> Self {
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
    pub fn kms_pair(mut self, context: [u8; 32], epoch: [u8; 32]) -> Self {
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
    entries: Vec<HandleEntry>,
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
        delegator: Pubkey,
    ) -> Self {
        self.entry(handle, delegator, encrypted_store.account_key)
    }

    /// Adds an entry verbatim, for the malformed and substitution cases.
    pub fn entry(
        mut self,
        handle: [u8; 32],
        owner_address: Pubkey,
        encrypted_store: Pubkey,
    ) -> Self {
        self.entries.push(HandleEntry {
            handle,
            owner_address: owner_address.to_bytes(),
            encrypted_store: encrypted_store.to_bytes(),
        });
        self
    }

    /// The request in its two carriers, signed: the fields the Gateway types, and the blob.
    pub fn parts(&self) -> (SolanaUserDecryptFields, SolanaRequestBlob) {
        let signature = self.wallet.sign(&self.permit.typed());
        let permit = self.permit.wire();
        let fixed = |bytes: Vec<u8>| bytes.try_into().expect("fixture permit is well formed");
        let gateway = SolanaUserDecryptFields {
            handles: self.entries.iter().map(|e| e.handle).collect(),
            transport_key: permit.transport_key,
            start_timestamp: permit.start_timestamp,
            duration_seconds: permit.duration_seconds,
            extra_data: permit.extra_data,
        };
        let blob = SolanaRequestBlob {
            user_address: fixed(permit.user_address),
            allowed_scopes: permit
                .allowed_scopes
                .into_iter()
                .map(|scope| scope.try_into().expect("fixture scope is well formed"))
                .collect(),
            verifying_program_id: fixed(permit.verifying_program_id),
            signature: *signature.as_bytes(),
            entries: self
                .entries
                .iter()
                .map(|e| SolanaEntryClaims {
                    owner_address: e.owner_address,
                    encrypted_store: e.encrypted_store,
                })
                .collect(),
        };
        (gateway, blob)
    }

    /// The request in validated form.
    pub fn typed(&self) -> SolanaUserDecryptionRequestV1 {
        let (gateway, blob) = self.parts();
        SolanaUserDecryptionRequestV1::new(U256::from(1), gateway, blob)
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
    pub account_key: Pubkey,
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
        program: Pubkey,
        authority: Pubkey,
        scope: Pubkey,
        label: [u8; 32],
        current_handle: [u8; 32],
    ) -> Self {
        let (account_key, bump) = Pubkey::find_program_address(
            &[
                b"encrypted-state",
                program.as_ref(),
                authority.as_ref(),
                scope.as_ref(),
            ],
            &PROGRAM_ID,
        );
        Self {
            encrypted_store: EncryptedStore {
                program: program.to_bytes(),
                authority: authority.to_bytes(),
                scope: scope.to_bytes(),
                slots: vec![EncryptedSlot {
                    key: label,
                    handle: current_handle,
                }],
                leaf_count: 0,
                peaks: Vec::new(),
                bump,
            },
            account_key,
            leaves: Vec::new(),
        }
    }

    /// An account holding `current_handle` with `key` already allowed on it: the state one
    /// write plus one allow leaves behind, and the reference state of most scenarios.
    pub fn allowing(current_handle: [u8; 32], key: Pubkey) -> Self {
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
    pub fn allow(&mut self, key: Pubkey) {
        let handle = self.current_handle();
        self.allow_handle(handle, key);
    }

    /// Seals an allow leaf for an exact handle. The handle need not still occupy a slot.
    pub fn allow_handle(&mut self, handle: [u8; 32], key: Pubkey) {
        let leaf_index = self.encrypted_store.leaf_count;
        let commitment = historical_access_leaf_commitment(
            self.account_key.to_bytes(),
            leaf_index,
            handle,
            key.to_bytes(),
        );
        self.append(
            LeafQuery {
                encrypted_store: self.account_key,
                handle: B256::new(handle),
                kind: LeafKind::Allowed { key },
            },
            commitment,
        );
    }

    /// Seals a public-decrypt leaf for the current handle.
    pub fn mark_public(&mut self) {
        let handle = self.current_handle();
        let leaf_index = self.encrypted_store.leaf_count;
        let commitment =
            public_decrypt_leaf_commitment(self.account_key.to_bytes(), leaf_index, handle);
        self.append(
            LeafQuery {
                encrypted_store: self.account_key,
                handle: B256::new(handle),
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
    pub fn allowed_query(&self, handle: [u8; 32], key: Pubkey) -> LeafQuery {
        LeafQuery {
            encrypted_store: self.account_key,
            handle: B256::new(handle),
            kind: LeafKind::Allowed { key },
        }
    }

    /// The public query for `handle` under this account.
    pub fn public_query(&self, handle: [u8; 32]) -> LeafQuery {
        LeafQuery {
            encrypted_store: self.account_key,
            handle: B256::new(handle),
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
    pub delegator: Pubkey,
    /// Who received it.
    pub delegate: Pubkey,
    /// The application's program, or the wildcard.
    pub program: Pubkey,
    /// The application's scope, or the wildcard.
    pub scope: Pubkey,
    /// Unix second it ends at, exclusive; 0 once revoked.
    pub expires_at: u64,
    /// The counter no rule reads and no signature commits to.
    pub delegation_counter: u64,
    /// When it was last written, which no rule reads.
    pub last_update_slot: u64,
}

impl DelegationFixture {
    /// A delegation live at [`HOST_NOW`] in the default encrypted store's application.
    pub fn live(delegator: Pubkey, delegate: Pubkey) -> Self {
        Self {
            delegator,
            delegate,
            program: APP_PROGRAM,
            scope: SCOPE,
            expires_at: HOST_NOW + 3_600,
            delegation_counter: 1,
            last_update_slot: 1,
        }
    }

    /// A live wildcard row: the same grant with the sentinel in place of the application, which is
    /// how a delegator covers every application at once.
    pub fn live_wildcard(delegator: Pubkey, delegate: Pubkey) -> Self {
        Self {
            program: Pubkey::new_from_array(WILDCARD_APP),
            scope: Pubkey::new_from_array(WILDCARD_APP),
            ..Self::live(delegator, delegate)
        }
    }

    /// The same record in the application of `encrypted_store`.
    pub fn in_application_of(self, encrypted_store: &EncryptedStoreFixture) -> Self {
        Self {
            program: Pubkey::new_from_array(encrypted_store.encrypted_store.program),
            scope: Pubkey::new_from_array(encrypted_store.encrypted_store.scope),
            ..self
        }
    }

    /// The same record revoked, which zeroes its expiry.
    pub fn revoked(self) -> Self {
        Self {
            expires_at: 0,
            ..self
        }
    }

    /// Its canonical address and bump, derived here rather than taken from the code under test.
    pub fn address(&self) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                DELEGATION_SEED,
                self.delegator.as_ref(),
                self.delegate.as_ref(),
                self.program.as_ref(),
                self.scope.as_ref(),
            ],
            &PROGRAM_ID,
        )
    }

    /// The account as the host program would write it.
    pub fn account(&self) -> SnapshotAccount {
        let (_, bump) = self.address();
        SnapshotAccount {
            owner: PROGRAM_ID,
            data: encode_user_decryption_delegation(&UserDecryptionDelegationRecord {
                delegator: self.delegator.to_bytes(),
                delegate: self.delegate.to_bytes(),
                program: self.program.to_bytes(),
                scope: self.scope.to_bytes(),
                expires_at: self.expires_at,
                delegation_counter: self.delegation_counter,
                last_update_slot: self.last_update_slot,
                bump,
            }),
        }
    }
}

/// The Clock sysvar reading `unix_timestamp`, as a node returns it.
pub fn clock_account(unix_timestamp: u64) -> SnapshotAccount {
    SnapshotAccount {
        owner: Pubkey::new_from_array(SYSVAR_OWNER_ID),
        data: encode_clock(unix_timestamp),
    }
}

/// The canonical invalidation-record address for a user, derived here rather than taken from
/// the code under test.
pub fn invalidation_address(user: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[PERMIT_INVALIDATION_SEED, user.as_ref()], &PROGRAM_ID)
}

/// An invalidation record holding `watermark` for `user`.
pub fn invalidation_account(user: Pubkey, watermark: u64) -> SnapshotAccount {
    let (_, bump) = invalidation_address(user);
    SnapshotAccount {
        owner: PROGRAM_ID,
        data: encode_permit_invalidation(&PermitInvalidationRecord {
            user: user.to_bytes(),
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
    accounts: BTreeMap<Pubkey, SnapshotAccount>,
    /// The encrypted stores placed whole, so the leaf record that agrees with this world
    /// can be derived from it.
    sealed: BTreeMap<Pubkey, EncryptedStoreFixture>,
}

impl World {
    /// A world at `slot` holding only the Clock, at [`HOST_NOW`].
    pub fn at_slot(slot: u64) -> Self {
        Self {
            slot,
            ..Self::default()
        }
        .with_clock(HOST_NOW)
    }

    /// The same world with the Clock reading `unix_timestamp`.
    pub fn with_clock(mut self, unix_timestamp: u64) -> Self {
        self.accounts.insert(
            Pubkey::new_from_array(CLOCK_SYSVAR_ID),
            clock_account(unix_timestamp),
        );
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
    pub fn with_watermark(mut self, user: Pubkey, watermark: u64) -> Self {
        let (key, _) = invalidation_address(user);
        self.accounts
            .insert(key, invalidation_account(user, watermark));
        self
    }

    /// Places an arbitrary account in the world, for the wrong-owner and wrong-type cases.
    pub fn with_account(mut self, key: Pubkey, account: SnapshotAccount) -> Self {
        self.accounts.insert(key, account);
        self
    }

    /// Removes an account, for the absent cases.
    pub fn without_account(mut self, key: &Pubkey) -> Self {
        self.accounts.remove(key);
        self
    }

    /// The same world observed at another slot.
    pub fn at(mut self, slot: u64) -> Self {
        self.slot = slot;
        self
    }

    /// Projects the world onto the requested keys, exactly as an account read would.
    pub fn read(&self, keys: &[Pubkey]) -> AccountsRead {
        AccountsRead {
            slot: self.slot,
            accounts: keys.iter().map(|key| self.account(key)).collect(),
        }
    }

    /// The account at `key`, if the world holds one.
    pub fn account(&self, key: &Pubkey) -> Option<SnapshotAccount> {
        self.accounts.get(key).cloned()
    }

    /// The account at a derived address, as a read of this world observes it.
    pub fn row(&self, (key, bump): DerivedAddress) -> ObservedRow {
        ObservedRow {
            key,
            bump,
            account: self.account(&key),
        }
    }

    /// Both delegation rows of a delegated entry, judged at this world's Clock.
    pub fn rows(&self, exact: DerivedAddress, wildcard: DerivedAddress) -> ObservedRows {
        let clock = self
            .account(&Pubkey::new_from_array(CLOCK_SYSVAR_ID))
            .expect("the world has a Clock");
        ObservedRows {
            exact: self.row(exact),
            wildcard: self.row(wildcard),
            now: zama_solana_acl::decode_clock_unix_timestamp(clock.owner.as_array(), &clock.data)
                .expect("the world's Clock decodes"),
        }
    }
}

/// One host-state read as the reader was asked for it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ReadCall {
    pub keys: Vec<Pubkey>,
    pub min_context_slot: Option<u64>,
}

/// A reader that answers from a script of worlds and records every call.
///
/// Reads are answered in order: the first call sees the first world, the second the second. A
/// call beyond the script panics rather than repeating the last world — an authorization that
/// reads state a third time is a defect, and it should surface as a loud failure in whichever
/// test provoked it rather than as a passing assertion elsewhere. Like a node, it refuses a read
/// whose world is older than the read's `min_context_slot`.
pub struct ScriptedReader {
    worlds: Vec<World>,
    calls: Mutex<Vec<ReadCall>>,
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

    /// The reads that were made, in order.
    pub fn calls(&self) -> Vec<ReadCall> {
        self.calls.lock().expect("reader lock").clone()
    }

    /// The read at `index`.
    pub fn call(&self, index: usize) -> ReadCall {
        self.calls()
            .get(index)
            .cloned()
            .unwrap_or_else(|| panic!("expected at least {} host-state read(s)", index + 1))
    }
}

impl HostStateReader for ScriptedReader {
    async fn read_accounts(
        &self,
        keys: &[Pubkey],
        min_context_slot: Option<u64>,
    ) -> Result<AccountsRead, SnapshotError> {
        let index = {
            let mut calls = self.calls.lock().expect("reader lock");
            calls.push(ReadCall {
                keys: keys.to_vec(),
                min_context_slot,
            });
            calls.len() - 1
        };
        let world = self.worlds.get(index).unwrap_or_else(|| {
            panic!(
                "authorization read host state {} time(s); the script provides {}",
                index + 1,
                self.worlds.len()
            )
        });
        if min_context_slot.is_some_and(|slot| world.slot < slot) {
            return Err(SnapshotError::NodeBehind);
        }
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
    accounts: BTreeMap<Pubkey, EncryptedStoreFixture>,
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
    pub fn without(mut self, account_key: &Pubkey) -> Self {
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

/// One coprocessor as the scripted proof reader sees it.
#[derive(Clone, Debug)]
pub enum ProofSource {
    /// Answers every query from this record.
    Serving(ProofRecord),
    /// Fails every read, as an unreachable coprocessor's transport would.
    Down,
    /// Answers one outcome fewer than it was asked for.
    Truncated(ProofRecord),
    /// Never answers, as a coprocessor that accepted the connection and stalled.
    Stalled,
    /// Panics if asked: for scenarios rejected before the proof read, where reaching it would
    /// mean a rule ran out of order.
    MustNotBeAsked,
}

/// A proof reader over coprocessors, recording every read as `(coprocessor, queries)` in the
/// order the reads started.
pub struct ScriptedProofReader {
    sources: Vec<ProofSource>,
    calls: Mutex<Vec<(usize, Vec<LeafQuery>)>>,
}

impl ScriptedProofReader {
    /// One coprocessor serving `record`.
    pub fn constant(record: ProofRecord) -> Self {
        Self::coprocessors(vec![ProofSource::Serving(record)])
    }

    /// Coprocessors serving these records, in this order.
    pub fn in_order(records: Vec<ProofRecord>) -> Self {
        Self::coprocessors(records.into_iter().map(ProofSource::Serving).collect())
    }

    /// These coprocessors, in this order.
    pub fn coprocessors(sources: Vec<ProofSource>) -> Self {
        Self {
            sources,
            calls: Mutex::new(Vec::new()),
        }
    }

    /// One coprocessor over a record that has sealed exactly these accounts' leaves.
    pub fn serving(accounts: &[&EncryptedStoreFixture]) -> Self {
        Self::constant(ProofRecord::of(accounts))
    }

    /// One coprocessor that must never be asked.
    pub fn unreachable() -> Self {
        Self::coprocessors(vec![ProofSource::MustNotBeAsked])
    }

    /// One coprocessor that is down.
    pub fn down() -> Self {
        Self::coprocessors(vec![ProofSource::Down])
    }

    /// How many reads were made, across all coprocessors.
    pub fn call_count(&self) -> usize {
        self.calls.lock().expect("proof reader lock").len()
    }

    /// The reads that were made, in order, as `(coprocessor, queries)`.
    pub fn calls(&self) -> Vec<(usize, Vec<LeafQuery>)> {
        self.calls.lock().expect("proof reader lock").clone()
    }
}

impl HostProofReader for ScriptedProofReader {
    fn source_count(&self) -> usize {
        self.sources.len()
    }

    async fn read_proofs(
        &self,
        source: usize,
        queries: &[LeafQuery],
    ) -> Result<Vec<LeafProofOutcome>, ProofReadError> {
        self.calls
            .lock()
            .expect("proof reader lock")
            .push((source, queries.to_vec()));
        let answer = |record: &ProofRecord| -> Vec<_> {
            queries.iter().map(|query| record.answer(query)).collect()
        };
        match &self.sources[source] {
            ProofSource::Serving(record) => Ok(answer(record)),
            ProofSource::Truncated(record) => Ok(answer(record)[1..].to_vec()),
            ProofSource::Down => Err(ProofReadError::Unavailable {
                reason: format!("coprocessor {source} is down"),
            }),
            ProofSource::Stalled => std::future::pending().await,
            ProofSource::MustNotBeAsked => {
                panic!("authorization read leaf proofs where no read was expected")
            }
        }
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
    pub fn serve_accounts(&mut self, accounts: &[(Pubkey, Option<SnapshotAccount>)]) {
        let keys: Vec<_> = accounts.iter().map(|(key, _)| *key).collect();
        let request = multiple_accounts_request(&keys, None);
        let value: Vec<_> = accounts
            .iter()
            .map(|(_, account)| {
                account.as_ref().map(|account| {
                    serde_json::json!({
                        "owner": account.owner.to_string(),
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
pub fn multiple_accounts_request(
    keys: &[Pubkey],
    min_context_slot: Option<u64>,
) -> serde_json::Value {
    serde_json::json!({
        "jsonrpc": "2.0",
        "id": 0,
        "method": "getMultipleAccounts",
        "params": [
            keys.iter()
                .map(Pubkey::to_string)
                .collect::<Vec<_>>(),
            {"encoding": "base64", "commitment": "confirmed", "dataSlice": null, "minContextSlot": min_context_slot},
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
    let routes: Vec<_> = coprocessors
        .iter()
        .map(|coprocessor| proof_route(coprocessor.base_url().unwrap()))
        .collect();
    SolanaHost {
        program_id: PROGRAM_ID,
        reader: SolanaRpcClient::new(
            rpc.base_url().unwrap().clone(),
            Duration::from_secs(10),
            NonZeroUsize::MIN,
        ),
        proofs: CoprocessorProofClient::new(&routes, Client::new()),
    }
}

/// A leaf-proof route to the coprocessor at `url`.
pub fn proof_route(url: &url::Url) -> ProofRoute {
    ProofRoute {
        url: url.clone(),
        api_key: ApiKey::from("test-key".to_owned()),
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
