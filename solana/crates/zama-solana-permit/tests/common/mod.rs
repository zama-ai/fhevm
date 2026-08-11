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
/// Fixture cluster genesis hash the test chain id is derived from.
pub const GENESIS_HASH_HEX: &str =
    "4539cf79f66704d313b4047b712d24ee29653cdf7484b18bc05992c01c105576";
/// Fixture chain id: `derive_chain_id` applied to [`GENESIS_HASH_HEX`], written out as a
/// literal so a drifting derivation surfaces here instead of silently rewriting every
/// golden. The tie is asserted in `chain_id_derivation.rs`.
pub const CHAIN_ID: u64 = 10_037_641_751_006_774_702;

/// The settled chain-id derivation, `zama-solana-chain-id-v1`:
/// `0x8000000000000000 | (be_u64(SHA-256(tag ‖ genesis_hash)[0..8]) & 0x7fff…)`.
///
/// A deployment-time rule: no running component recomputes it — production reads the
/// chain id from configuration and checks only the chain-kind bit. It lives here, in
/// test code, because the fixture generator is the one place that plays the deployer.
pub fn derive_chain_id(genesis_hash: &[u8; 32]) -> u64 {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"zama-solana-chain-id-v1");
    hasher.update(genesis_hash);
    let digest = hasher.finalize();
    let leading = u64::from_be_bytes(digest[..8].try_into().expect("eight digest bytes"));
    0x8000_0000_0000_0000 | (leading & 0x7fff_ffff_ffff_ffff)
}
/// Fixture KMS context id.
pub const KMS_CONTEXT_ID_HEX: &str =
    "bb801121e2ea198af189c9331dfc57f675802c35206f96a5964deeac39f79d18";
/// Fixture KMS epoch id.
pub const KMS_EPOCH_ID_HEX: &str =
    "7772d6a5c7fc28db485c51abbe18cba52b775baf1015b59ac363e5bf5827a3f2";
/// Label the filler transport keys are expanded from.
pub const TRANSPORT_KEY_LABEL: &[u8] = b"zama-solana-permit-test-transport-key";
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
    "110049ad6aec706218ea59c4234642a394c15a1be630edb3efff36fbd718464e";
/// Signature over the reference envelope, from the independent implementation.
pub const REFERENCE_SIGNATURE_HEX: &str = concat!(
    "69259422cd12d75a04e39f6dbbce1f07bd45b0657df3cd7c41881dc387b13ec8",
    "87a106646a50ff9c709544f50c3bb2e8e896e5d25bb8b205c00392458e3d7207"
);
/// The same signature with its scalar replaced by `S + L`: a second encoding of the
/// same signature, which a lax verifier accepts and a strict one rejects.
pub const NON_CANONICAL_SIGNATURE_HEX: &str = concat!(
    "69259422cd12d75a04e39f6dbbce1f07bd45b0657df3cd7c41881dc387b13ec8",
    "7475fcc084b311f546323c98eb3491fde896e5d25bb8b205c00392458e3d7217"
);
/// Digest of the permissive permit's envelope.
pub const PERMISSIVE_ENVELOPE_DIGEST_HEX: &str =
    "e105f269f644f20c7025bb436b1cfea038f00c76bb215eec6ea25bccc44e96a2";
/// Signature over the permissive permit's envelope.
pub const PERMISSIVE_SIGNATURE_HEX: &str = concat!(
    "15b15779298fceec4cd4bf7201b6871cd03046968014853b471858cdc604360d",
    "821986ad98c846f22af1b9d34f5816a26fcf70961e2bfdd49b94690527dd5d0e"
);
/// Fingerprint of the reference transport key, computed outside this crate.
pub const REFERENCE_FINGERPRINT_HEX: &str =
    "b1316d23732ff02dbdd55a3f375199bccdcad68b9f90d4eb5d1140bdcd01eb07";

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

/// Deterministic transport key of the accepted length, expanded from a label. The
/// bytes are filler — the protocol never interprets them — but they are reproducible
/// in any language, which matters because four other implementations consume the same
/// fixture.
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

/// The reference transport key: a genuinely generated ML-KEM-512 encapsulation key in
/// its tfhe safe-serialized `UnifiedPublicEncKey::MlKem512` container — the one
/// representation the whole chain carries, signs and links. The permit layer itself
/// never interprets the bytes. The underlying key is generated by FIPS 203
/// ML-KEM-512.KeyGen_internal(d, z) with
/// d = SHAKE256("zama-solana-permit-reference-transport-key/d", 32) and
/// z = SHAKE256("zama-solana-permit-reference-transport-key/z", 32);
/// the container bytes are copied verbatim from the KMS linker vector set's
/// `reference-mlkem-512` entry, so the two halves of the specification's fixture set
/// bind byte-identical objects. The bare 800-byte encapsulation key survives in that
/// set only as a negative (`bare-mlkem-512-800`): request validation rejects its width.
pub const REFERENCE_TRANSPORT_KEY_HEX: &str = concat!(
    "0300000000000000302e35000000000300000000000000302e31130000000000",
    "0000556e69666965645075626c6963456e634b65790000000000000000200300",
    "000000000050813ec9f7b53c004191429786739a5660a2a05451c5431c081574",
    "5c43956f175449da6cd4702335acaa51064f761508cb262b0457561086a28ee5",
    "acd3ecbe7be095d554b34b61bde9744a7cc67f1189a2714180cf0c4475548813",
    "a970e26011a0c0bf28e56b5a16608ff2c7103583f95240a7a18bc37aaace2120",
    "9bea85b50c2ecbb08a6921b0d5909bd71c40461b099dd79b56d972e75b6c502b",
    "42075ab2dcc26f85b34a4ef34060452b440173fbb95082d2acc42565b4e9a954",
    "63b9c39ab14714b33349245447b25a42035d64a7371959bac5831f94b3bdc309",
    "5e31769d211f9b17b4966ba00cd68bfab42693205c7e349e08527bc5e982256b",
    "9d7e57ce59cb706b160519b1a7c473918e754540dc1c5b14707f6002c7aa7d87",
    "c300ead080296683a5c30a73e74b7107087f5760c90350b125bb9e5530af1409",
    "f5900b396116d37306c391a5b7137e1d7720ca58a41af79f9065b1bc01c5fe31",
    "cba5ea55b682b537b36cce9b23e5826d2fca0abe86b44ec639f5832d5f7b2278",
    "a4685e4249faa8441b31198f5566f3599fa3675e97aa40cf62a25d3715d5656a",
    "a3647641016bcd329717667f1a090dd5f94873c558366614f39a3e93b14f3886",
    "6812616e3b83189e17cc26526a96106696b6548cbaa8a4f222ee31ac72e16a50",
    "77c699fb1a3fe68673a8293a21c0e34ccb42e32249f22d3924974dc92f25d60c",
    "37424110892d5e1217a03c15efe4ad49248fe2776b2b65b71f140c7e0ba53c6b",
    "4c09992e17799a5ee65247f428b20739a92898eff5acd455c52d02285bcccd82",
    "0767ddf38e4a6026f32802bc9b6fba6b077becc1b5e23653607c592c3a86d56b",
    "f103c3d64b01fc7aba912138a8000a872b3af36c8346069fb2756e0ee41e9b03",
    "33bb9994b6cb4e6d1a190df45bc3369fa2f12ad63b8376952335b159b696ba5e",
    "c6771903c55361649428b856195dd37a1fbad85d5b9054e0e70250c8a590028e",
    "4bc1385864549be00d96a642ce4b0a88f7b447e627a4702bb4278c4d09812f24",
    "293a129f56c96a83faa779504c165c97eec81d535a0458904a73905404b16db9",
    "f5bd23f420cca76e5b13555b19f7e2d43cde771f93dcda9d280f2967e16a9c1f",
    "7db8cc2d48",
);

/// The bare 800-byte encapsulation key inside [`REFERENCE_TRANSPORT_KEY_HEX`]'s
/// container — the representation this permit once accepted. Kept as a fixture for
/// exactly one purpose: the negative record showing that the old wire form is
/// refused. Byte-identical to the KMS linker vector set's `bare-mlkem-512-800`.
pub const REFERENCE_BARE_KEY_HEX: &str = concat!(
    "27b792b8740145b905d2b19f3575cb5cc27746ec1869b89a80217182c06d9e66",
    "877492ac1a515d31e275244712769b10fde778e1e9b6ff9c1fbeec2f51b63377",
    "e621ce408035fc9c7fb6a4846c12ab82b3fe75cc91544a5ea511aed07712f602",
    "50989d9ff46a38f7a999120b924a6100f278eb25422a1278732a1082d85b9397",
    "8e31d5b462b7ad319c8749519238d0b82e5760de9ca49a6b52826b07a6a81340",
    "51aef3b63b225c0201126f36e23657dc0f67a9b59c0a5bea89393952674a3156",
    "fdd2035594bd0c9a537e3b49278091becc6845c43e2022700c0478edb7b9d8a0",
    "801bd9b98ad2006ccb3db1a2c36d005736d87d28aa96bfd329bf9aa3c05cb521",
    "660c36211e61d65e63abb295aa2ade8096d3f41136fa713b61bf84d75cf45c4c",
    "d287861cd7a96e106ff6336cdda88a92a9a29c14a5cab0c537e318d629bae996",
    "9631a1a8eff0b02b3904ed17a7361733758abcae984d0fb55a1d628302151627",
    "db570828c11c74b278597d672671fc00a5c25c7aadfc8f7be008532c9ff59b01",
    "aca534e8382a297a7dfc4965f56619e3ec8fb5380137c45f12404cbe772ffce5",
    "c9b24ccb28e2193681651d43b07f83aa4b662e19b98c4197666f9b626a4c880b",
    "0a14b40b8c8e7629557931ae901abf16a06c6b561e1861b40a9c933553d8f28b",
    "4a80b3d2d865488941d77a128155064884c67cb8c6921a4cc0b7c9bf059c5101",
    "521b0119157489e8c5566f86386f3905d9987825f5045f957273a76a1af599a5",
    "85b674b6a696ba9affc12410b04c2a482730d95dd4023d0b1825b23792b8c0b2",
    "aa7116fcc4b959a35a68eb2a3fda078ad3bc3e69280b94636f6795c44a06dacc",
    "00156032aca9924e5cb618e11ec6215599807b57955871ec7f819228088c5304",
    "7379a72b8f0d6a7cd89c7f258c73876892236cae8cdb0b80640324e0c8c2f081",
    "f2079e7c53bea86435fa4563826b838213677515aef5160f4ddc1200b214fcd0",
    "1ca4976948b86c1bc512ffc08f948889fa058292c3b5369b56265435a1d993f1",
    "11ccfa6761547b86323779e40b0f4d41af6e4a34583cc28502353c7a1028b725",
    "8bfdd3c82d8e64cf6d91e5b1815df57d2791eb20bc6c0bc208eb7db167f454e0",
);

/// The bare encapsulation key, as bytes.
pub fn reference_bare_transport_key() -> Vec<u8> {
    let hex = REFERENCE_BARE_KEY_HEX;
    assert_eq!(hex.len(), 800 * 2, "expected 800 bare key bytes");
    (0..hex.len() / 2)
        .map(|index| u8::from_str_radix(&hex[index * 2..index * 2 + 2], 16).expect("hex fixture"))
        .collect()
}

/// The fixture transport key.
pub fn reference_transport_key() -> Vec<u8> {
    let hex = REFERENCE_TRANSPORT_KEY_HEX;
    assert_eq!(
        hex.len(),
        TRANSPORT_KEY_LEN * 2,
        "expected 869 container bytes"
    );
    (0..hex.len() / 2)
        .map(|index| u8::from_str_radix(&hex[index * 2..index * 2 + 2], 16).expect("hex fixture"))
        .collect()
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
