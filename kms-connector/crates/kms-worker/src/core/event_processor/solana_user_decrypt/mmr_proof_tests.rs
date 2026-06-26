//! End-to-end MMR-proof tests: build a real lineage with the shared `zama_solana_acl` crate, sign a
//! v2 request over the canonical preimage, and drive the signature + dispatch paths. The lineage
//! fetch (`fetch_encrypted_value_acl`) is I/O so it is exercised separately; everything load-bearing
//! for authorization (the staleness gate, the historical/public verifier, the proof-binding into
//! the signature, the v1→v2 domain bump) is covered here with pure helpers.

use super::*;
use alloy::primitives::{Address, Bytes, FixedBytes};
use connector_utils::types::solana_extra_data::SOLANA_USER_DECRYPT_DOMAIN_TAG;
use fhevm_gateway_bindings::decryption::{
    Decryption::{HandleEntry, SnsCiphertextMaterial, UserDecryptionRequestSolana},
    IDecryption::{RequestValiditySeconds, UserDecryptionRequestSolanaPayload},
};
use ring::signature::{Ed25519KeyPair, KeyPair};
use zama_solana_acl::{
    acl_nonce_key, historical_access_leaf_commitment, mmr_append, mmr_build_proof,
    public_decrypt_leaf_commitment,
};

const CHAIN_ID: u64 = 7777;
const HOST: SolanaPubkeyBytes = [42u8; 32];
const DOMAIN: SolanaPubkeyBytes = [1u8; 32];
const APP: SolanaPubkeyBytes = [2u8; 32];
const LABEL: [u8; 32] = *b"balance_________________________";

/// A lineage whose account bytes and proofs are produced by the shared crate, mirroring the
/// helper in `solana_encrypted_value_acl.rs`.
struct Lineage {
    acl: EncryptedValueAcl,
    account: SolanaPubkeyBytes,
    leaves: Vec<[u8; 32]>,
}

fn lineage(handle: HandleBytes, subjects: &[SolanaPubkeyBytes]) -> Lineage {
    let value_key = acl_nonce_key(DOMAIN, APP, LABEL);
    let (account, bump) = encrypted_value_acl_address(HOST, value_key);
    Lineage {
        acl: EncryptedValueAcl {
            acl_domain_key: DOMAIN,
            app_account: APP,
            encrypted_value_label: LABEL,
            current_handle: handle,
            subjects: subjects.to_vec(),
            leaf_count: 0,
            peaks: Vec::new(),
            bump,
        },
        account,
        leaves: Vec::new(),
    }
}

impl Lineage {
    fn value_key(&self) -> [u8; 32] {
        acl_nonce_key(
            self.acl.acl_domain_key,
            self.acl.app_account,
            self.acl.encrypted_value_label,
        )
    }
    fn append(&mut self, commitment: [u8; 32]) {
        mmr_append(&mut self.acl.peaks, &mut self.acl.leaf_count, commitment).unwrap();
        self.leaves.push(commitment);
    }
    fn rotate(&mut self, new_handle: HandleBytes) {
        let old = self.acl.current_handle;
        for i in 0..self.acl.subjects.len() {
            let idx = self.acl.leaf_count;
            self.append(historical_access_leaf_commitment(
                self.account,
                idx,
                old,
                self.acl.subjects[i],
            ));
        }
        self.acl.current_handle = new_handle;
    }
    fn mark_public(&mut self) {
        let idx = self.acl.leaf_count;
        self.append(public_decrypt_leaf_commitment(
            self.account,
            idx,
            self.acl.current_handle,
        ));
    }
    fn proof(&self, i: u64) -> MmrProof {
        mmr_build_proof(&self.leaves, i).unwrap()
    }
}

/// The full transport blob: 1-byte mode prefix ‖ Borsh(MmrProof).
fn proof_blob(mode: u8, proof: &MmrProof) -> Vec<u8> {
    let mut blob = vec![mode];
    blob.extend_from_slice(&borsh::to_vec(proof).unwrap());
    blob
}

/// Builds a v2-signed single-handle request carrying an MMR-proof tail, signed by `identity_kp`
/// over the canonical preimage. `proof_blob`/`value_key`/`proof_slot` are bound into the
/// signature exactly as production does.
fn signed_mmr_request(
    identity_kp: &Ed25519KeyPair,
    handle: HandleBytes,
    value_key: [u8; 32],
    proof_blob: Vec<u8>,
    proof_slot: u64,
) -> UserDecryptionRequestSolana {
    let identity: SolanaPubkeyBytes = identity_kp.public_key().as_ref().try_into().unwrap();
    let public_key = b"reencryption-public-key".to_vec();
    let nonce = [5u8; 32];
    let context_id = [0u8; 32];
    let start: u64 = 1_000;
    let duration: u64 = 3_600;

    let preimage = solana_user_decrypt_signing_preimage(&SolanaUserDecryptSigningInput {
        contracts_chain_id: CHAIN_ID,
        public_key: &public_key,
        handles: &[handle],
        identity: &identity,
        context_id: &context_id,
        nonce: &nonce,
        allowed_acl_domain_keys: &[DOMAIN],
        start_timestamp: start,
        duration_seconds: duration,
        acl_value_key: &value_key,
        mmr_proof_bytes: &proof_blob,
        proof_slot,
    });
    let signature = identity_kp.sign(&preimage);

    let mut extra_data = vec![0x01u8];
    extra_data.extend_from_slice(&context_id);

    let payload = UserDecryptionRequestSolanaPayload {
        userIdentity: FixedBytes::from(identity),
        publicKey: Bytes::from(public_key),
        allowedAclDomainKeys: vec![FixedBytes::from(DOMAIN)],
        requestValidity: RequestValiditySeconds {
            startTimestamp: U256::from(start),
            durationSeconds: U256::from(duration),
        },
        nonce: FixedBytes::from(nonce),
        extraData: Bytes::from(extra_data),
        signature: Bytes::from(signature.as_ref().to_vec()),
        aclValueKey: FixedBytes::from(value_key),
        mmrProof: Bytes::from(proof_blob),
        proofSlot: proof_slot,
    };
    UserDecryptionRequestSolana {
        decryptionId: U256::from(1u64),
        snsCtMaterials: vec![SnsCiphertextMaterial {
            ctHandle: FixedBytes::from(handle),
            ..Default::default()
        }],
        handles: vec![HandleEntry {
            handle: FixedBytes::from(handle),
            contractAddress: Address::ZERO,
            ownerAddress: Address::ZERO,
        }],
        payload,
    }
}

fn identity_kp(seed: u8) -> Ed25519KeyPair {
    Ed25519KeyPair::from_pkcs8_maybe_unchecked(&pkcs8_from_seed(&[seed; 32])).unwrap()
}

fn h(tag: u8) -> HandleBytes {
    [tag; 32]
}

// (1) HISTORICAL ACCEPT: a rotated-away handle with a valid historical-access proof, signed v2,
// verifies and authorizes.
#[test]
fn historical_accept() {
    let kp = identity_kp(11);
    let owner: SolanaPubkeyBytes = kp.public_key().as_ref().try_into().unwrap();
    let mut l = lineage(h(10), &[owner]);
    l.rotate(h(11));
    let proof = l.proof(0);
    let blob = proof_blob(MMR_MODE_HISTORICAL, &proof);

    let request = signed_mmr_request(&kp, h(10), l.value_key(), blob, l.acl.leaf_count);
    let auth = verify_solana_user_decrypt_signature(&request, CHAIN_ID).unwrap();

    let verifier = SolanaAclVerifier::new(HOST);
    dispatch_solana_mmr_proof(&verifier, l.account, HOST, &l.acl, h(10), &auth)
        .expect("historical decrypt must authorize");
}

// (2) PUBLIC ACCEPT: a handle marked public with a valid public-decrypt proof, mode 0x02.
#[test]
fn public_accept() {
    let kp = identity_kp(12);
    let owner: SolanaPubkeyBytes = kp.public_key().as_ref().try_into().unwrap();
    let mut l = lineage(h(20), &[owner]);
    l.mark_public();
    l.rotate(h(21));
    let proof = l.proof(0);
    let blob = proof_blob(MMR_MODE_PUBLIC, &proof);

    let request = signed_mmr_request(&kp, h(20), l.value_key(), blob, l.acl.leaf_count);
    let auth = verify_solana_user_decrypt_signature(&request, CHAIN_ID).unwrap();

    let verifier = SolanaAclVerifier::new(HOST);
    dispatch_solana_mmr_proof(&verifier, l.account, HOST, &l.acl, h(20), &auth)
        .expect("public decrypt must authorize");
}

// (3) STALE RETRYABLE: a proof that no longer verifies against the LIVE peaks because the lineage
// rotated (its mountain merged) since the proof was built → Recoverable (NOT Irrecoverable), with
// both counts named. Verify-FIRST: a mere proof_slot != live leaf_count does NOT reject — the
// proof must actually fail verification (see test 3b). Budget-safe (Solana never hits the budget
// arm).
#[test]
fn stale_merged_proof_is_recoverable() {
    let kp = identity_kp(11);
    let owner: SolanaPubkeyBytes = kp.public_key().as_ref().try_into().unwrap();
    // lc 0 -> 3: leaf 2 = (h(12), owner) sits in a size-1 mountain at lc=3.
    let mut l = lineage(h(10), &[owner]);
    l.rotate(h(11)); // lc=1
    l.rotate(h(12)); // lc=2
    l.rotate(h(13)); // lc=3, leaf 2 commits handle h(12)
    assert_eq!(l.acl.leaf_count, 3);
    // Build leaf 2's proof at lc=3 and capture it (the client's just-superseded-handle proof).
    let blob = proof_blob(MMR_MODE_HISTORICAL, &l.proof(2));
    let proof_slot = 3u64;
    // One more rotation merges leaf 2's size-1 mountain into the size-4 mountain, so the captured
    // proof's path is now too short and no longer verifies against the live peaks.
    l.rotate(h(14)); // lc=4
    assert_eq!(l.acl.leaf_count, 4);

    let request = signed_mmr_request(&kp, h(12), l.value_key(), blob, proof_slot);
    let auth = verify_solana_user_decrypt_signature(&request, CHAIN_ID).unwrap();

    let verifier = SolanaAclVerifier::new(HOST);
    let err = dispatch_solana_mmr_proof(&verifier, l.account, HOST, &l.acl, h(12), &auth)
        .expect_err("a stale (merged) proof must be rejected");
    match err {
        ProcessingError::Recoverable(e) => {
            let msg = e.to_string();
            assert!(
                msg.contains("leaf_count=3"),
                "must name the proof's leaf_count, got: {msg}"
            );
            assert!(
                msg.contains("leaf_count=4"),
                "must name the live leaf_count, got: {msg}"
            );
        }
        other => {
            panic!("a stale (merged) proof must be Recoverable (budget-safe), got {other:?}")
        }
    }
}

// (3b) VERIFY-FIRST ACCEPTS COUNT DRIFT WITHOUT A MERGE: a proof built against an earlier
// leaf_count whose mountain has NOT merged still verifies against the live peaks and MUST be
// accepted, even though proof_slot != live leaf_count. This is exactly the case the old
// leaf_count-equality gate wrongly rebuilt.
#[test]
fn valid_proof_survives_count_drift_without_merge() {
    let kp = identity_kp(13);
    let owner: SolanaPubkeyBytes = kp.public_key().as_ref().try_into().unwrap();
    let mut l = lineage(h(30), &[owner]);
    l.rotate(h(31)); // lc=1
    l.rotate(h(32)); // lc=2, leaf 0 = (h(30), owner) in the size-2 mountain
    assert_eq!(l.acl.leaf_count, 2);
    // Build leaf 0's proof (the oldest, most stable leaf) at lc=2 and capture it.
    let blob = proof_blob(MMR_MODE_HISTORICAL, &l.proof(0));
    let proof_slot = 2u64;
    // Advance to lc=3: leaf 0's size-2 mountain does NOT merge (that needs lc=4), so the captured
    // proof still verifies against the live peaks.
    l.rotate(h(33)); // lc=3
    assert_eq!(l.acl.leaf_count, 3);

    // proof_slot (2) != live leaf_count (3), yet the proof verifies → must be accepted.
    let request = signed_mmr_request(&kp, h(30), l.value_key(), blob, proof_slot);
    let auth = verify_solana_user_decrypt_signature(&request, CHAIN_ID).unwrap();

    let verifier = SolanaAclVerifier::new(HOST);
    dispatch_solana_mmr_proof(&verifier, l.account, HOST, &l.acl, h(30), &auth).expect(
        "a proof that still verifies against the live peaks must be accepted despite count drift",
    );
}

// (3c) CURRENT ACCEPT: a no-proof request (empty mmrProof) carrying a non-zero aclValueKey
// authorizes the LIVE lineage handle against current_handle + membership — the path balances /
// total_supply take now that they have no V1 AclRecord.
#[test]
fn current_lineage_accept() {
    let kp = identity_kp(21);
    let owner: SolanaPubkeyBytes = kp.public_key().as_ref().try_into().unwrap();
    let l = lineage(h(40), &[owner]);

    let request = signed_mmr_request(&kp, h(40), l.value_key(), Vec::new(), 0);
    let auth = verify_solana_user_decrypt_signature(&request, CHAIN_ID).unwrap();
    assert!(auth.mmr_proof_bytes.is_empty());
    assert_ne!(auth.acl_value_key, [0u8; 32]);

    let verifier = SolanaAclVerifier::new(HOST);
    dispatch_solana_current(&verifier, l.account, HOST, &l.acl, h(40), &auth)
        .expect("current-lineage decrypt of the live handle by a subject must authorize");
}

// (3d) CURRENT NON-SUBJECT REJECTED: the live handle is correct but the requester is not a
// subject of the lineage → not authorized.
#[test]
fn current_lineage_non_subject_rejected() {
    let kp = identity_kp(22);
    let other: SolanaPubkeyBytes = [99u8; 32];
    let l = lineage(h(50), &[other]); // requester (kp) is NOT a subject

    let request = signed_mmr_request(&kp, h(50), l.value_key(), Vec::new(), 0);
    let auth = verify_solana_user_decrypt_signature(&request, CHAIN_ID).unwrap();

    let verifier = SolanaAclVerifier::new(HOST);
    let err = dispatch_solana_current(&verifier, l.account, HOST, &l.acl, h(50), &auth)
        .expect_err("a non-subject must not be authorized for the current handle");
    assert!(matches!(err, ProcessingError::Recoverable(_)));
}

// (3e) CURRENT REJECTS A ROTATED-AWAY HANDLE: the no-proof current path authorizes only the
// LIVE handle. A past (rotated-away) handle must NOT slip through here — it requires a
// historical-access MMR proof. Guards against the current path leaking history.
#[test]
fn current_lineage_rejects_rotated_away_handle() {
    let kp = identity_kp(23);
    let owner: SolanaPubkeyBytes = kp.public_key().as_ref().try_into().unwrap();
    let mut l = lineage(h(60), &[owner]);
    l.rotate(h(61)); // current_handle is now h(61); h(60) is historical

    // Request the OLD handle with no proof.
    let request = signed_mmr_request(&kp, h(60), l.value_key(), Vec::new(), 0);
    let auth = verify_solana_user_decrypt_signature(&request, CHAIN_ID).unwrap();

    let verifier = SolanaAclVerifier::new(HOST);
    let err = dispatch_solana_current(&verifier, l.account, HOST, &l.acl, h(60), &auth)
        .expect_err("a rotated-away handle must not authorize via the no-proof current path");
    assert!(matches!(err, ProcessingError::Recoverable(_)));
}

// (4) V1-SIGNED REJECTED UNDER V2: a signature over a preimage built with the OLD v1 domain tag
// (proof fields populated) fails verify against the v2 preimage the connector rebuilds.
#[test]
fn v1_signature_rejected_under_v2() {
    let kp = identity_kp(11);
    let owner: SolanaPubkeyBytes = kp.public_key().as_ref().try_into().unwrap();
    let mut l = lineage(h(10), &[owner]);
    l.rotate(h(11));
    let proof = l.proof(0);
    let blob = proof_blob(MMR_MODE_HISTORICAL, &proof);
    let value_key = l.value_key();

    // Confirm the current tag is v2 (guards against an accidental revert of the bump).
    assert_eq!(
        SOLANA_USER_DECRYPT_DOMAIN_TAG,
        b"zama-solana-user-decrypt-v2"
    );

    // Build a v1-tag preimage by hand (everything else identical to the v2 builder) and sign it.
    let identity: SolanaPubkeyBytes = kp.public_key().as_ref().try_into().unwrap();
    let public_key = b"reencryption-public-key".to_vec();
    let nonce = [5u8; 32];
    let context_id = [0u8; 32];
    let (start, duration) = (1_000u64, 3_600u64);
    let mut v1_preimage = b"zama-solana-user-decrypt-v1".to_vec();
    v1_preimage.extend_from_slice(&CHAIN_ID.to_be_bytes());
    v1_preimage.extend_from_slice(&(public_key.len() as u32).to_be_bytes());
    v1_preimage.extend_from_slice(&public_key);
    v1_preimage.extend_from_slice(&1u32.to_be_bytes());
    v1_preimage.extend_from_slice(&h(10));
    v1_preimage.extend_from_slice(&identity);
    v1_preimage.extend_from_slice(&context_id);
    v1_preimage.extend_from_slice(&nonce);
    v1_preimage.extend_from_slice(&1u32.to_be_bytes());
    v1_preimage.extend_from_slice(&DOMAIN);
    v1_preimage.extend_from_slice(&start.to_be_bytes());
    v1_preimage.extend_from_slice(&duration.to_be_bytes());
    let v1_signature = kp.sign(&v1_preimage);

    let mut extra_data = vec![0x01u8];
    extra_data.extend_from_slice(&context_id);
    let payload = UserDecryptionRequestSolanaPayload {
        userIdentity: FixedBytes::from(identity),
        publicKey: Bytes::from(public_key),
        allowedAclDomainKeys: vec![FixedBytes::from(DOMAIN)],
        requestValidity: RequestValiditySeconds {
            startTimestamp: U256::from(start),
            durationSeconds: U256::from(duration),
        },
        nonce: FixedBytes::from(nonce),
        extraData: Bytes::from(extra_data),
        signature: Bytes::from(v1_signature.as_ref().to_vec()),
        aclValueKey: FixedBytes::from(value_key),
        mmrProof: Bytes::from(blob),
        proofSlot: l.acl.leaf_count,
    };
    let request = UserDecryptionRequestSolana {
        decryptionId: U256::from(1u64),
        snsCtMaterials: vec![SnsCiphertextMaterial {
            ctHandle: FixedBytes::from(h(10)),
            ..Default::default()
        }],
        handles: vec![HandleEntry {
            handle: FixedBytes::from(h(10)),
            contractAddress: Address::ZERO,
            ownerAddress: Address::ZERO,
        }],
        payload,
    };

    let result = verify_solana_user_decrypt_signature(&request, CHAIN_ID);
    assert!(
        matches!(result, Err(ProcessingError::Irrecoverable(_))),
        "a v1-domain signature must not verify under v2, got {result:?}",
    );
}

// (5) PROOF-FIELD BINDING: mutating the proof blob (or value_key/slot) after signing breaks the
// signature — proving the new fields are load-bearing in the preimage.
#[test]
fn proof_fields_bound_into_signature() {
    let kp = identity_kp(11);
    let owner: SolanaPubkeyBytes = kp.public_key().as_ref().try_into().unwrap();
    let mut l = lineage(h(10), &[owner]);
    l.rotate(h(11));
    let blob = proof_blob(MMR_MODE_HISTORICAL, &l.proof(0));

    // Mutate the proof blob.
    let mut req = signed_mmr_request(&kp, h(10), l.value_key(), blob.clone(), l.acl.leaf_count);
    let mut tampered = blob.clone();
    *tampered.last_mut().unwrap() ^= 0xff;
    req.payload.mmrProof = Bytes::from(tampered);
    assert!(
        matches!(
            verify_solana_user_decrypt_signature(&req, CHAIN_ID),
            Err(ProcessingError::Irrecoverable(_))
        ),
        "a mutated proof blob must break the signature",
    );

    // Mutate the value_key.
    let mut req = signed_mmr_request(&kp, h(10), l.value_key(), blob.clone(), l.acl.leaf_count);
    req.payload.aclValueKey = FixedBytes::from([0x99u8; 32]);
    assert!(matches!(
        verify_solana_user_decrypt_signature(&req, CHAIN_ID),
        Err(ProcessingError::Irrecoverable(_))
    ));

    // Mutate the proof_slot.
    let mut req = signed_mmr_request(&kp, h(10), l.value_key(), blob, l.acl.leaf_count);
    req.payload.proofSlot += 1;
    assert!(matches!(
        verify_solana_user_decrypt_signature(&req, CHAIN_ID),
        Err(ProcessingError::Irrecoverable(_))
    ));
}

// (6) CURRENT-ACL → MMR PATH-CONFUSION: a request signed with the current-ACL zero tail
// (empty proof, zero value_key, zero slot) cannot be coerced into the MMR branch by injecting a
// non-empty proof blob post-signing — the injected bytes change the rebuilt preimage so the
// signature check rejects it. Guards the dispatch-branch decision behind the signature gate.
#[test]
fn current_acl_mmr_proof_injection_rejected() {
    let kp = identity_kp(11);
    let l = lineage(h(10), &[kp.public_key().as_ref().try_into().unwrap()]);

    // Sign a current-ACL-shaped request: empty proof, zero value_key, zero slot.
    let mut req = signed_mmr_request(&kp, h(10), [0u8; 32], Vec::new(), 0);

    // Inject a non-empty MMR proof blob after signing (the path-confusion attempt).
    let injected = proof_blob(MMR_MODE_HISTORICAL, &l.proof_for_empty());
    req.payload.mmrProof = Bytes::from(injected);

    let result = verify_solana_user_decrypt_signature(&req, CHAIN_ID);
    assert!(
        matches!(result, Err(ProcessingError::Irrecoverable(_))),
        "a post-sign-injected MMR proof must break the signature, got {result:?}",
    );
}

// MULTI-HANDLE GUARD: the single-handle scope of an MMR-proof request is enforced by the pure
// `require_single_handle` helper — a two-handle request is Irrecoverable, a one-handle
// request returns that handle. (The production caller in decryption.rs delegates to this helper,
// so the rejection is now covered without a live host.)
#[test]
fn mmr_proof_multi_handle_rejected() {
    let two = [h(10), h(11)];
    let err =
        require_single_handle(&two).expect_err("a two-handle MMR-proof request must be rejected");
    match err {
        ProcessingError::Irrecoverable(e) => {
            assert!(
                e.to_string().contains("exactly one handle"),
                "unexpected message: {e}"
            );
        }
        other => panic!("multi-handle MMR request must be Irrecoverable, got {other:?}"),
    }
    assert_eq!(require_single_handle(&[h(10)]).unwrap(), h(10));
    assert!(matches!(
        require_single_handle(&[]),
        Err(ProcessingError::Irrecoverable(_))
    ));
}

// (7) SIBLINGS CAP: a proof carrying more than MAX_MMR_SIBLINGS siblings is rejected as
// Irrecoverable at the decode site (bounds untrusted allocation).
#[test]
fn siblings_cap_rejected() {
    let kp = identity_kp(11);
    let oversized = MmrProof {
        leaf_index: 0,
        siblings: vec![[0u8; 32]; MAX_MMR_SIBLINGS + 1],
    };
    let blob = proof_blob(MMR_MODE_HISTORICAL, &oversized);
    // Slot must match so we reach the verifier-decode path, not the staleness gate. Use an
    // empty lineage (leaf_count 0) and a request slot of 0.
    let l = lineage(h(10), &[kp.public_key().as_ref().try_into().unwrap()]);
    let request = signed_mmr_request(&kp, h(10), l.value_key(), blob, 0);
    let auth = verify_solana_user_decrypt_signature(&request, CHAIN_ID).unwrap();

    let verifier = SolanaAclVerifier::new(HOST);
    let err = dispatch_solana_mmr_proof(&verifier, l.account, HOST, &l.acl, h(10), &auth)
        .expect_err("oversized sibling list must be rejected");
    assert!(matches!(err, ProcessingError::Irrecoverable(_)));
}

// Unknown mode byte → Irrecoverable.
#[test]
fn unknown_mode_rejected() {
    let kp = identity_kp(11);
    let l = lineage(h(10), &[kp.public_key().as_ref().try_into().unwrap()]);
    let blob = proof_blob(0x09, &l.proof_for_empty());
    let request = signed_mmr_request(&kp, h(10), l.value_key(), blob, 0);
    let auth = verify_solana_user_decrypt_signature(&request, CHAIN_ID).unwrap();
    let verifier = SolanaAclVerifier::new(HOST);
    let err = dispatch_solana_mmr_proof(&verifier, l.account, HOST, &l.acl, h(10), &auth)
        .expect_err("unknown mode must be rejected");
    assert!(matches!(err, ProcessingError::Irrecoverable(_)));
}

impl Lineage {
    fn proof_for_empty(&self) -> MmrProof {
        MmrProof {
            leaf_index: 0,
            siblings: Vec::new(),
        }
    }
}
