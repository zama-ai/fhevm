//! Shared fixtures for the integration suites.
//!
//! Every constant here is a deliberate choice, not filler:
//!
//! * the reference permit's two ACL-domain keys are the base58 length
//!   counterexample pair — one encodes to 43 characters, the other to 44, and their
//!   byte order is the opposite of their string order. Baking them into the *happy
//!   path* means the golden text itself would change if anyone ever sorted the list
//!   as strings;
//! * the verifying program id is the real host program id, so the golden text looks
//!   like a permit a wallet would actually be shown;
//! * the chain id is derived from a fixture genesis hash by a stand-in derivation.
//!   The real derivation is still an open protocol question, so goldens deliberately
//!   pin a fixed test chain id rather than a derivation result.

// Each integration binary compiles this module and uses a different subset of it.
#![allow(dead_code)]

use zama_solana_permit::{
    Identity, KmsRouting, PermitFields, PermitWireFields, Signature, TransportKey,
    KMS_ROUTING_VERSION_BYTE, MAX_ACL_DOMAIN_KEYS, MAX_DURATION_SECONDS, MAX_START_TIMESTAMP,
    TRANSPORT_KEY_LEN,
};

// ---------------------------------------------------------------------------
// Reference fixture
// ---------------------------------------------------------------------------

/// Ed25519 seed of the fixture wallet.
pub const USER_SEED: &[u8; 32] = b"zama-solana-permit-user-seed-001";
/// Public key of the fixture wallet. Asserted against the seed derivation, so a
/// drifting fixture surfaces here instead of silently rewriting every golden.
pub const USER_PUBKEY_HEX: &str =
    "c11a7cf8eb1cdfcb1bcb84b9d8314ddec3bb1410f0a95badd9c4384f643a6427";
/// The deployed host program id.
pub const VERIFYING_PROGRAM_ID_HEX: &str =
    "4cd3022dff504a675caf2d9b4f4014d0b3dc3ea17ffb97ba355cec5a933a30ee";
/// Fixture cluster genesis hash the test chain id stands in for.
pub const GENESIS_HASH_HEX: &str =
    "4539cf79f66704d313b4047b712d24ee29653cdf7484b18bc05992c01c105576";
/// Fixture chain id. Carries the host-kind high bit, as every Solana chain id does.
pub const CHAIN_ID: u64 = 14_211_618_221_876_249_811;
/// Fixture KMS context id.
pub const KMS_CONTEXT_ID_HEX: &str =
    "bb801121e2ea198af189c9331dfc57f675802c35206f96a5964deeac39f79d18";
/// Fixture KMS epoch id.
pub const KMS_EPOCH_ID_HEX: &str =
    "7772d6a5c7fc28db485c51abbe18cba52b775baf1015b59ac363e5bf5827a3f2";
/// Label the fixture transport key is expanded from.
pub const TRANSPORT_KEY_LABEL: &[u8] = b"zama-solana-permit-test-transport-key";
/// Fingerprint of the fixture transport key, computed outside this crate.
pub const TRANSPORT_KEY_FINGERPRINT_HEX: &str =
    "db6c3a40a21048811530040464f62a848576fd69d391c758bfb1373064b0df21";
/// `2026-01-01T01:03:00Z` — a minute-aligned start with a nonzero hour and minute,
/// so the golden exercises more of the timestamp rendering than midnight would.
pub const START_TIMESTAMP: u64 = 1_767_229_380;
/// Seven days.
pub const DURATION_SECONDS: u64 = 604_800;

// ---------------------------------------------------------------------------
// Goldens computed outside this crate
// ---------------------------------------------------------------------------
//
// These were produced by an independent RFC 8032 / SHAKE-256 implementation. Keeping
// them in one place means the envelope tests and the vector records are pinned to the
// same foreign values, rather than to each other.

/// Digest of the reference permit's envelope.
pub const REFERENCE_ENVELOPE_DIGEST_HEX: &str =
    "ee176c0d420e37694c9e385fd34dd7c30825b6b9fff65fd6489732e79c61bdda";
/// Signature over the reference envelope, from the independent implementation.
pub const REFERENCE_SIGNATURE_HEX: &str = concat!(
    "bf5741a5eeb51d60710cb67bf7d1c4418acf0f52a5de17c9178afc4f5210036b",
    "1da02900ad286508e4e0e97d51ae8d427207a2deddf0e103c1b17201743f9c05"
);
/// The same signature with its scalar replaced by `S + L`: a second encoding of the
/// same signature, which a lax verifier accepts and a strict one rejects.
pub const NON_CANONICAL_SIGNATURE_HEX: &str = concat!(
    "bf5741a5eeb51d60710cb67bf7d1c4418acf0f52a5de17c9178afc4f5210036b",
    "0a741f5dc78b7760ba7de12030a86c577207a2deddf0e103c1b17201743f9c15"
);
/// Digest of the permissive permit's envelope.
pub const PERMISSIVE_ENVELOPE_DIGEST_HEX: &str =
    "2f140e76c16b07ef52aded9a315553401a802e083a57dac113d04028d5ef8808";
/// Signature over the permissive permit's envelope.
pub const PERMISSIVE_SIGNATURE_HEX: &str = concat!(
    "7d9250e81b8fdaac9cd204ef06eaa4a51a1f409dd1466ebec9c0d04919d88170",
    "692487b5926847a5c82986fa2041e8710313268a8c31dba6f19acbaf1380730d"
);
/// Fingerprint of the fixture transport key.
pub const REFERENCE_FINGERPRINT_HEX: &str =
    "db6c3a40a21048811530040464f62a848576fd69d391c758bfb1373064b0df21";

/// A y-coordinate encoding that is not a point on the curve.
pub const NOT_A_CURVE_POINT_HEX: &str =
    "0200000000000000000000000000000000000000000000000000000000000000";
/// A small-order public key under which nothing may verify. Chosen from the eight
/// torsion encodings because a cofactored verifier accepts an all-zero signature under
/// this one.
pub const SMALL_ORDER_PUBKEY_HEX: &str =
    "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa";
/// A y-coordinate encoded above the field modulus: a valid point, non-canonically
/// written.
pub const NON_CANONICAL_PUBKEY_HEX: &str =
    "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";

/// Parses a 64-byte signature from hex.
pub fn signature_from_hex(hex: &str) -> Signature {
    assert_eq!(hex.len(), 128, "a signature is 64 bytes");
    let mut bytes = [0u8; 64];
    for (index, slot) in bytes.iter_mut().enumerate() {
        *slot = u8::from_str_radix(&hex[index * 2..index * 2 + 2], 16).expect("hex fixture");
    }
    Signature::new(bytes)
}

/// 32-byte SHAKE-256 digest, used to pin long byte strings compactly.
pub fn digest(bytes: &[u8]) -> [u8; 32] {
    use sha3::{
        digest::{ExtendableOutput, Update, XofReader},
        Shake256,
    };
    let mut hasher = Shake256::default();
    hasher.update(bytes);
    let mut out = [0u8; 32];
    hasher.finalize_xof().read(&mut out);
    out
}

/// ACL-domain key whose base58 encoding is 43 characters and sorts *above* its
/// byte-order successor as a string.
pub const ACL_DOMAIN_KEY_43_HEX: &str =
    "0edbafda67ca37188cf28263571f03b9716879e4acc9c514ab6727ffffffffff";
/// ACL-domain key whose base58 encoding is 44 characters and sorts *below* its
/// byte-order predecessor as a string.
pub const ACL_DOMAIN_KEY_44_HEX: &str =
    "0edbafda67ca37188cf28263571f03b9716879e4acc9c514ab67280000000000";
/// Base58 of the 43-character key, for the string-order assertions.
pub const ACL_DOMAIN_KEY_43_BASE58: &str = "zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz";
/// Base58 of the 44-character key.
pub const ACL_DOMAIN_KEY_44_BASE58: &str = "21111111111111111111111111111111111111111111";

// ---------------------------------------------------------------------------
// Byte helpers
// ---------------------------------------------------------------------------

/// Decodes a 64-character hex string. Panics on malformed input: fixtures are
/// literals in this file, so a bad one is a bug in the test, not an input to handle.
pub fn bytes32(hex: &str) -> [u8; 32] {
    let mut out = [0u8; 32];
    assert_eq!(hex.len(), 64, "expected 32 hex-encoded bytes");
    for (index, slot) in out.iter_mut().enumerate() {
        *slot = u8::from_str_radix(&hex[index * 2..index * 2 + 2], 16).expect("hex fixture");
    }
    out
}

/// Deterministic 800-byte transport key expanded from a label. The bytes are filler
/// — the protocol never interprets them — but they are reproducible in any language,
/// which matters because four other implementations consume the same fixture.
pub fn transport_key_bytes(label: &[u8]) -> Vec<u8> {
    use sha3::{
        digest::{ExtendableOutput, Update, XofReader},
        Shake256,
    };
    let mut hasher = Shake256::default();
    hasher.update(label);
    let mut reader = hasher.finalize_xof();
    let mut out = vec![0u8; TRANSPORT_KEY_LEN];
    reader.read(&mut out);
    out
}

/// The fixture transport key.
pub fn reference_transport_key() -> Vec<u8> {
    transport_key_bytes(TRANSPORT_KEY_LABEL)
}

/// Deterministic transport-key-shaped filler of an arbitrary length, for the tests
/// that vary only the length.
pub fn transport_key_bytes_of_len(len: usize) -> Vec<u8> {
    use sha3::{
        digest::{ExtendableOutput, Update, XofReader},
        Shake256,
    };
    let mut hasher = Shake256::default();
    hasher.update(TRANSPORT_KEY_LABEL);
    let mut reader = hasher.finalize_xof();
    let mut out = vec![0u8; len];
    reader.read(&mut out);
    out
}

/// Signed KMS routing bytes for the only known version.
pub fn extra_data(kms_context_id: [u8; 32], kms_epoch_id: [u8; 32]) -> Vec<u8> {
    let mut out = Vec::with_capacity(65);
    out.push(KMS_ROUTING_VERSION_BYTE);
    out.extend_from_slice(&kms_context_id);
    out.extend_from_slice(&kms_epoch_id);
    out
}

// ---------------------------------------------------------------------------
// Wire fixtures
// ---------------------------------------------------------------------------

/// The reference permit in transport form: two ACL domains, in byte order.
pub fn reference_wire() -> PermitWireFields {
    PermitWireFields {
        user_pubkey: bytes32(USER_PUBKEY_HEX).to_vec(),
        transport_key: reference_transport_key(),
        allowed_acl_domain_keys: vec![
            bytes32(ACL_DOMAIN_KEY_43_HEX).to_vec(),
            bytes32(ACL_DOMAIN_KEY_44_HEX).to_vec(),
        ],
        start_timestamp: START_TIMESTAMP,
        duration_seconds: DURATION_SECONDS,
        verifying_program_id: bytes32(VERIFYING_PROGRAM_ID_HEX).to_vec(),
        chain_id: CHAIN_ID,
        extra_data: extra_data(bytes32(KMS_CONTEXT_ID_HEX), bytes32(KMS_EPOCH_ID_HEX)),
    }
}

/// The reference permit with an empty ACL-domain list.
pub fn permissive_wire() -> PermitWireFields {
    PermitWireFields {
        allowed_acl_domain_keys: Vec::new(),
        ..reference_wire()
    }
}

/// The reference permit with `count` distinct ACL domains in byte order.
pub fn wire_with_domain_count(count: usize) -> PermitWireFields {
    let mut keys: Vec<[u8; 32]> = (0..count).map(distinct_domain_key).collect();
    keys.sort_unstable();
    PermitWireFields {
        allowed_acl_domain_keys: keys.iter().map(|key| key.to_vec()).collect(),
        ..reference_wire()
    }
}

/// A distinct 32-byte key per index, all encoding to 44 base58 characters (the
/// widest form), so a permit built from them measures the worst case.
pub fn distinct_domain_key(index: usize) -> [u8; 32] {
    let mut key = [0xffu8; 32];
    key[31] = 0xff - u8::try_from(index).expect("fixture index fits a byte");
    key
}

/// The widest permit the protocol admits at `domain_count` domains: every identity
/// at its longest base58 form, the longest chain id, the longest duration and the
/// latest timestamp.
pub fn worst_case_wire(domain_count: usize) -> PermitWireFields {
    let mut keys: Vec<[u8; 32]> = (10..10 + domain_count).map(distinct_domain_key).collect();
    keys.sort_unstable();
    PermitWireFields {
        // The fixture wallet, not a synthetic all-high key: the widest permit is also a
        // normative `valid` record, so it has to be a permit that can exist and be signed.
        // Its base58 form is 44 characters like every other identity here, so the size
        // claims made against this fixture are unaffected.
        user_pubkey: pubkey_of_seed(USER_SEED).as_bytes().to_vec(),
        transport_key: reference_transport_key(),
        allowed_acl_domain_keys: keys.iter().map(|key| key.to_vec()).collect(),
        start_timestamp: zama_solana_permit::MAX_START_TIMESTAMP,
        duration_seconds: zama_solana_permit::MAX_DURATION_SECONDS,
        verifying_program_id: distinct_domain_key(1).to_vec(),
        chain_id: u64::MAX,
        extra_data: extra_data(distinct_domain_key(2), distinct_domain_key(3)),
    }
}

// ---------------------------------------------------------------------------
// Deterministic field generation
// ---------------------------------------------------------------------------

/// SplitMix64. A named, self-contained generator rather than a random one: a
/// property test that fails must fail again on the next run, and the four other
/// implementations must be able to reproduce the same inputs.
pub fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// A pseudo-random 32-byte identity.
pub fn pseudo_identity(state: &mut u64) -> [u8; 32] {
    let mut out = [0u8; 32];
    for chunk in out.chunks_mut(8) {
        chunk.copy_from_slice(&splitmix64(state).to_le_bytes());
    }
    out
}

/// A well-formed permit derived from `seed`, spanning the whole admitted space:
/// every domain count, both ends of each bound, and identities that include
/// leading-zero and all-high byte patterns (the two base58 corner cases).
pub fn pseudo_valid_wire(seed: u64) -> PermitWireFields {
    let mut state = seed;
    let domain_count = (splitmix64(&mut state) as usize) % (MAX_ACL_DOMAIN_KEYS + 1);

    let mut keys: Vec<[u8; 32]> = (0..domain_count)
        .map(|index| {
            let mut key = pseudo_identity(&mut state);
            // Force the base58 corner cases into the sample rather than hoping for
            // them: a leading zero byte encodes shorter, an all-high key encodes
            // longer.
            match index % 3 {
                0 => key[0] = 0x00,
                1 => key[0] = 0xff,
                _ => {}
            }
            key
        })
        .collect();
    keys.sort_unstable();
    keys.dedup();

    let start_timestamp = match splitmix64(&mut state) % 4 {
        0 => 0,
        1 => MAX_START_TIMESTAMP,
        2 => splitmix64(&mut state) % (MAX_START_TIMESTAMP + 1),
        _ => START_TIMESTAMP,
    };
    let duration_seconds = match splitmix64(&mut state) % 4 {
        0 => 1,
        1 => MAX_DURATION_SECONDS,
        2 => 1 + splitmix64(&mut state) % MAX_DURATION_SECONDS,
        _ => DURATION_SECONDS,
    };
    let chain_id = match splitmix64(&mut state) % 4 {
        0 => 0,
        1 => u64::MAX,
        2 => splitmix64(&mut state),
        _ => CHAIN_ID,
    };

    PermitWireFields {
        user_pubkey: pseudo_identity(&mut state).to_vec(),
        transport_key: transport_key_bytes(&seed.to_le_bytes()),
        allowed_acl_domain_keys: keys.iter().map(|key| key.to_vec()).collect(),
        start_timestamp,
        duration_seconds,
        verifying_program_id: pseudo_identity(&mut state).to_vec(),
        chain_id,
        extra_data: extra_data(pseudo_identity(&mut state), pseudo_identity(&mut state)),
    }
}

/// Decodes a wire fixture, expecting it to be well formed.
pub fn decoded(wire: &PermitWireFields) -> PermitFields {
    PermitFields::decode(wire).expect("fixture is well formed")
}

/// The reference permit, decoded.
pub fn reference_fields() -> PermitFields {
    decoded(&reference_wire())
}

// ---------------------------------------------------------------------------
// Signing
// ---------------------------------------------------------------------------

/// Signs arbitrary bytes with a fixture wallet seed.
///
/// Signing lives in the tests, not in the crate: the crate only ever verifies. That
/// asymmetry is the point — the wallet is the only signer in the real protocol.
pub fn sign_with_seed(seed: &[u8; 32], message: &[u8]) -> Signature {
    use ed25519_dalek::{Signer, SigningKey};
    let signing_key = SigningKey::from_bytes(seed);
    Signature::new(signing_key.sign(message).to_bytes())
}

/// Builds an offchain-message envelope over an arbitrary text, the way a wallet
/// would.
///
/// The layout bytes are literals here, not the crate's constants: this helper plays
/// the wallet, and a test that borrowed the crate's idea of the envelope could not
/// catch the crate getting the envelope wrong.
pub fn envelope_over_text(user_pubkey: &Identity, text: &str) -> Vec<u8> {
    let mut out = Vec::with_capacity(18 + 32 + text.len());
    out.extend_from_slice(b"\xffsolana offchain");
    out.push(1); // envelope version
    out.push(1); // signer count
    out.extend_from_slice(user_pubkey.as_bytes());
    out.extend_from_slice(text.as_bytes());
    out
}

/// Signs a text inside a wallet-shaped envelope, as the fixture wallet.
pub fn sign_text_as_wallet(seed: &[u8; 32], text: &str) -> Signature {
    let pubkey = pubkey_of_seed(seed);
    sign_with_seed(seed, &envelope_over_text(&pubkey, text))
}

/// The public key belonging to a fixture seed.
pub fn pubkey_of_seed(seed: &[u8; 32]) -> Identity {
    use ed25519_dalek::SigningKey;
    Identity::new(SigningKey::from_bytes(seed).verifying_key().to_bytes())
}

/// Reads the typed KMS routing material out of decoded fields.
pub fn kms_routing(fields: &PermitFields) -> (Identity, Identity) {
    match fields.extra_data() {
        KmsRouting::ContextAndEpoch {
            kms_context_id,
            kms_epoch_id,
        } => (*kms_context_id, *kms_epoch_id),
    }
}

/// Wraps 800 bytes into a transport key, for tests that need the typed form.
pub fn transport_key(bytes: &[u8]) -> TransportKey {
    let array: Box<[u8; TRANSPORT_KEY_LEN]> = bytes
        .to_vec()
        .into_boxed_slice()
        .try_into()
        .expect("fixture transport key has the accepted length");
    TransportKey::new(array)
}
