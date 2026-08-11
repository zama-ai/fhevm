//! The normative permit vectors: generator and runner in one place.
//!
//! The vector set is owned by this file and committed as JSON under
//! `solana/test-fixtures/permit/`, next to the schema every other implementation
//! includes. Generation and checking share one code path — the tests below build the
//! set in memory and compare it against the committed file, and the same build is
//! written to disk when the update gate is set:
//!
//! ```text
//! bash scripts/update-permit-vectors.sh
//! ```
//!
//! There is deliberately no separate generator binary. A generator that is not the
//! runner drifts from it, and then the committed file agrees with neither.
//!
//! Two disciplines are enforced on the set itself, not just on the crate:
//!
//! * a rejecting record is derived from an accepted one by exactly one documented
//!   mutation, and the runner checks that the base is accepted — so "this permit is
//!   rejected" cannot be passing for an unrelated reason;
//! * every rule in the shared rule list is exercised by at least one record, so
//!   dropping a class during a regeneration fails the suite instead of silently
//!   shrinking coverage.
//!
//! The chain-id derivation is settled (`zama-solana-chain-id-v1`) and these records
//! carry ids derived by it — see `chain_id_derivation.rs` for the rule and its parity
//! with the public cluster registry. With the derivation settled and the transport
//! key in its canonical container representation, this set is byte-frozen: a
//! regeneration that changes any committed byte is a protocol change, not a refresh.

mod common;

// The schema lives with the fixtures rather than in this crate, because four other
// implementations include the same file from their own test targets.
#[path = "../../../test-fixtures/permit/permit_vectors.rs"]
mod schema;

use common::*;
use schema::{
    from_hex, rule, to_hex, Deployment, KmsRoutingRecord, PermitVector, PermitVectorFile,
    VectorResult, WirePermit, PERMIT_VECTOR_SCHEMA,
};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use zama_solana_permit::{
    build_envelope, render_canonical_text, verify_signature, IdentityField, PermitError,
    PermitFields, PermitWireFields, MAX_DURATION_SECONDS, MAX_START_TIMESTAMP,
};

/// Setting this rewrites the committed file from the in-memory build.
const UPDATE_ENV: &str = "ZAMA_UPDATE_PERMIT_VECTORS";

fn vector_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../test-fixtures/permit/permit_v1.json")
}

// ---------------------------------------------------------------------------
// Rule naming: the crate's error taxonomy mapped onto the shared names
// ---------------------------------------------------------------------------

/// Maps this implementation's error onto the shared rule name.
///
/// The shared names are coarser than the errors on purpose — no field indices, no
/// lengths — because they have to be expressible in five implementations with five
/// different error types. This function is this crate's half of that contract.
fn rule_name(error: &PermitError) -> &'static str {
    match error {
        PermitError::IdentityWidth { .. } => rule::IDENTITY_WIDTH,
        PermitError::TooManyAclDomainKeys { .. } => rule::TOO_MANY_ACL_DOMAIN_KEYS,
        PermitError::AclDomainKeysNotAscending { .. } => rule::ACL_DOMAIN_KEYS_NOT_ASCENDING,
        PermitError::DuplicateAclDomainKey { .. } => rule::DUPLICATE_ACL_DOMAIN_KEY,
        PermitError::DurationOutOfRange { .. } => rule::DURATION_OUT_OF_RANGE,
        PermitError::StartTimestampOutOfRange { .. } => rule::START_TIMESTAMP_OUT_OF_RANGE,
        PermitError::TransportKeyLength { .. } => rule::TRANSPORT_KEY_LENGTH,
        PermitError::UnknownKmsRoutingVersion { .. } => rule::UNKNOWN_KMS_ROUTING_VERSION,
        PermitError::KmsRoutingLength { .. } => rule::KMS_ROUTING_LENGTH,
        PermitError::SignatureMismatch => rule::SIGNATURE_MISMATCH,
        PermitError::UnusableUserPubkey => rule::UNUSABLE_USER_PUBKEY,
    }
}

// ---------------------------------------------------------------------------
// Building the set
// ---------------------------------------------------------------------------

/// Accumulates records and the transport-key table.
struct Builder {
    transport_keys: BTreeMap<String, String>,
    vectors: Vec<PermitVector>,
}

impl Builder {
    fn new() -> Self {
        Self {
            transport_keys: BTreeMap::new(),
            vectors: Vec::new(),
        }
    }

    /// Registers a transport key under a name and returns the name.
    fn transport_key(&mut self, name: &str, bytes: &[u8]) -> String {
        let hex = to_hex(bytes);
        if let Some(existing) = self.transport_keys.get(name) {
            assert_eq!(existing, &hex, "transport key {name} registered twice");
        } else {
            self.transport_keys.insert(name.to_string(), hex);
        }
        name.to_string()
    }

    /// Renders the wire form of a permit, naming its transport key.
    fn wire_record(&mut self, key_name: &str, wire: &PermitWireFields) -> WirePermit {
        let key_name = self.transport_key(key_name, &wire.transport_key);
        WirePermit {
            user_pubkey: to_hex(&wire.user_pubkey),
            transport_key: key_name,
            allowed_acl_domain_keys: wire
                .allowed_acl_domain_keys
                .iter()
                .map(|key| to_hex(key))
                .collect(),
            start_timestamp: wire.start_timestamp.to_string(),
            duration_seconds: wire.duration_seconds.to_string(),
            verifying_program_id: to_hex(&wire.verifying_program_id),
            chain_id: wire.chain_id.to_string(),
            extra_data: to_hex(&wire.extra_data),
        }
    }

    /// Builds a record, filling in the derived material (text, envelope, parsed
    /// routing) for permits whose typed form is well formed.
    #[allow(clippy::too_many_arguments)]
    fn push(
        &mut self,
        name: &str,
        comment: &str,
        result: VectorResult,
        rule: Option<&str>,
        derived_from: Option<&str>,
        mutation: Option<&str>,
        key_name: &str,
        wire: PermitWireFields,
        signature: Signature,
        signed_text: Option<String>,
    ) {
        let permit = self.wire_record(key_name, &wire);

        let decoded = PermitFields::decode(&wire).ok();
        let text = decoded.as_ref().map(render_canonical_text);
        let envelope = decoded.as_ref().map(build_envelope);
        let kms_routing = decoded.as_ref().map(|fields| {
            let (context, epoch) = kms_routing(fields);
            KmsRoutingRecord {
                version: zama_solana_permit::KMS_ROUTING_VERSION_BYTE.to_string(),
                kms_context_id: to_hex(context.as_bytes()),
                kms_epoch_id: to_hex(epoch.as_bytes()),
            }
        });

        self.vectors.push(PermitVector {
            name: name.to_string(),
            comment: comment.to_string(),
            result,
            rule: rule.map(str::to_string),
            derived_from: derived_from.map(str::to_string),
            mutation: mutation.map(str::to_string),
            permit,
            kms_routing,
            signature: to_hex(signature.as_bytes()),
            permit_text_bytes: text.as_ref().map(|text| to_hex(text.as_bytes())),
            permit_text: text,
            envelope_bytes: envelope.as_ref().map(|bytes| to_hex(bytes)),
            signed_text,
        });
    }

    /// An accepted record: signed honestly over its own reconstructed envelope.
    fn accept(&mut self, name: &str, comment: &str, key_name: &str, wire: PermitWireFields) {
        let signature = sign_wire(&wire);
        self.push(
            name,
            comment,
            VectorResult::Valid,
            None,
            None,
            None,
            key_name,
            wire,
            signature,
            None,
        );
    }

    /// A record this layer accepts while later rules decide whether it may be used.
    fn accept_conditionally(
        &mut self,
        name: &str,
        comment: &str,
        key_name: &str,
        wire: PermitWireFields,
    ) {
        let signature = sign_wire(&wire);
        self.push(
            name,
            comment,
            VectorResult::Acceptable,
            None,
            None,
            None,
            key_name,
            wire,
            signature,
            None,
        );
    }

    /// A rejected record, derived from a named accepted record by one mutation.
    #[allow(clippy::too_many_arguments)]
    fn reject(
        &mut self,
        name: &str,
        comment: &str,
        rule: &str,
        derived_from: &str,
        mutation: &str,
        key_name: &str,
        wire: PermitWireFields,
        signature: Signature,
        signed_text: Option<String>,
    ) {
        self.push(
            name,
            comment,
            VectorResult::Invalid,
            Some(rule),
            Some(derived_from),
            Some(mutation),
            key_name,
            wire,
            signature,
            signed_text,
        );
    }
}

use zama_solana_permit::Signature;

/// Signs a well-formed permit as the fixture wallet.
fn sign_wire(wire: &PermitWireFields) -> Signature {
    let fields = PermitFields::decode(wire).expect("a signable record must be well formed");
    sign_with_seed(USER_SEED, &build_envelope(&fields))
}

/// The whole vector set.
fn build_vector_file() -> PermitVectorFile {
    let mut builder = Builder::new();
    const REFERENCE: &str = "reference-permit-two-domains";
    const REFERENCE_KEY: &str = "reference-mlkem-512";

    // -- accepted ----------------------------------------------------------

    builder.accept(
        REFERENCE,
        "Two ACL domains in byte order. The two keys are the base58 length \
         counterexample pair: one encodes to 43 characters and the other to 44, so \
         their byte order is the reverse of their string order. Every other record is \
         derived from this one.",
        REFERENCE_KEY,
        reference_wire(),
    );

    builder.accept(
        "permissive-permit",
        "Empty domain list. The canonical text must state the breadth of the grant on \
         one explicit line rather than render an empty enumeration block.",
        REFERENCE_KEY,
        permissive_wire(),
    );

    builder.accept(
        "permit-with-one-domain",
        "The smallest enumerated list.",
        REFERENCE_KEY,
        wire_with_domain_count(1),
    );

    builder.accept(
        "permit-with-ten-domains",
        "The largest admissible list.",
        REFERENCE_KEY,
        wire_with_domain_count(10),
    );

    builder.accept(
        "window-at-upper-bounds",
        "Start at the latest representable timestamp and the longest permitted \
         duration together. Documents that the typed-form bounds keep the sum away \
         from the 64-bit edge, so the authorization layer needs no overflow check of \
         its own.",
        REFERENCE_KEY,
        PermitWireFields {
            start_timestamp: MAX_START_TIMESTAMP,
            duration_seconds: MAX_DURATION_SECONDS,
            ..reference_wire()
        },
    );

    builder.accept(
        "minimal-duration",
        "A one-second validity window: the shortest permitted.",
        REFERENCE_KEY,
        PermitWireFields {
            duration_seconds: 1,
            ..reference_wire()
        },
    );

    builder.accept(
        "chain-id-above-the-javascript-safe-integer",
        "Chain id at the 64-bit maximum. A consumer that parses the field through a \
         double-precision number loses precision here and will compute a different \
         canonical text — which is why every 64-bit field in this file is a decimal \
         string.",
        REFERENCE_KEY,
        PermitWireFields {
            chain_id: u64::MAX,
            ..reference_wire()
        },
    );

    builder.accept(
        "widest-permit",
        "The largest signed bytes the protocol admits: ten domains, every identity at \
         its longest base58 form, the longest chain id and duration, the latest \
         timestamp. Pins the clear-signing budget.",
        "worst-case-mlkem-512",
        worst_case_wire(10),
    );

    // -- accepted here, decided elsewhere ----------------------------------

    builder.accept_conditionally(
        "future-start-permit",
        "A permit whose validity window has not opened. The typed form and the \
         signature are impeccable; whether it may be used is a question for the rules \
         that own a clock and the revocation watermark, which are not in this layer.",
        REFERENCE_KEY,
        PermitWireFields {
            start_timestamp: 4_102_444_800, // 2100-01-01T00:00:00Z
            ..reference_wire()
        },
    );

    // -- rejected: signature ----------------------------------------------

    let reference_fields = decoded(&reference_wire());
    let reference_envelope = build_envelope(&reference_fields);
    let reference_text = render_canonical_text(&reference_fields);

    builder.reject(
        "signature-from-another-signer",
        "A different wallet signed the same envelope. The sole accepted signer is the \
         key the permit names.",
        rule::SIGNATURE_MISMATCH,
        REFERENCE,
        "signature replaced with one made by a different wallet",
        REFERENCE_KEY,
        reference_wire(),
        sign_with_seed(b"zama-solana-permit-user-seed-002", &reference_envelope),
        None,
    );

    let with_trailing = format!("{reference_text}\n");
    builder.reject(
        "signature-over-text-with-trailing-newline",
        "The wallet was shown a text with one extra line feed. A verifier that \
         reconstructs the text never produces that byte string, so the signature does \
         not match. This is how the no-trailing-content rule is enforced: not by \
         inspecting the signed bytes, but by never parsing them.",
        rule::SIGNATURE_MISMATCH,
        REFERENCE,
        "signed a text with a trailing line feed",
        REFERENCE_KEY,
        reference_wire(),
        sign_text_as_wallet(USER_SEED, &with_trailing),
        Some(with_trailing),
    );

    let with_lookalike = reference_text.replacen("Zama", "Z\u{0430}ma", 1);
    builder.reject(
        "signature-over-text-with-non-ascii-lookalike",
        "The wallet was shown a text in which a Latin letter is replaced by a \
         visually identical Cyrillic one. A human reader cannot tell the difference; \
         reconstruction can.",
        rule::SIGNATURE_MISMATCH,
        REFERENCE,
        "signed a text containing a non-ASCII homoglyph",
        REFERENCE_KEY,
        reference_wire(),
        sign_text_as_wallet(USER_SEED, &with_lookalike),
        Some(with_lookalike),
    );

    builder.reject(
        "non-canonical-signature-scalar",
        "A genuine signature with its scalar replaced by S + L: a second encoding of \
         the same signature. The naive verification equation holds for both, so a lax \
         implementation accepts this record and a strict one rejects it. Implementations \
         that disagree here accept different sets of permits — and the signature is part \
         of the relayer's deduplication identity, so a malleable one also multiplies one \
         wallet consent into many distinct requests.",
        rule::SIGNATURE_MISMATCH,
        REFERENCE,
        "signature scalar replaced by S + L",
        REFERENCE_KEY,
        reference_wire(),
        signature_from_hex(NON_CANONICAL_SIGNATURE_HEX),
        None,
    );

    // -- rejected: unusable key -------------------------------------------

    builder.reject(
        "small-order-user-pubkey",
        "The permit names a small-order point as its user. Under cofactored \
         verification an all-zero signature verifies against such a key, so a permit \
         can carry the consent of a wallet nobody owns. Nothing may verify here.",
        rule::UNUSABLE_USER_PUBKEY,
        REFERENCE,
        "user pubkey replaced with a small-order point",
        REFERENCE_KEY,
        PermitWireFields {
            user_pubkey: bytes32(SMALL_ORDER_PUBKEY_HEX).to_vec(),
            ..reference_wire()
        },
        Signature::new([0u8; 64]),
        None,
    );

    builder.reject(
        "user-pubkey-not-on-the-curve",
        "The user pubkey is a coordinate for which no curve point exists. Note that \
         most byte patterns one would reach for are valid points, so this encoding is \
         chosen rather than guessed.",
        rule::UNUSABLE_USER_PUBKEY,
        REFERENCE,
        "user pubkey replaced with a non-curve coordinate",
        REFERENCE_KEY,
        PermitWireFields {
            user_pubkey: bytes32(NOT_A_CURVE_POINT_HEX).to_vec(),
            ..reference_wire()
        },
        signature_from_hex(REFERENCE_SIGNATURE_HEX),
        None,
    );

    builder.reject(
        "non-canonically-encoded-user-pubkey",
        "A valid point whose coordinate is written above the field modulus. No attack \
         rides on this — the encoding is inside the signed envelope, so a re-encoded \
         key changes the message — but Ed25519 libraries disagree about accepting it, \
         and every real wallet key is canonical, so the disagreement is removed rather \
         than left open.",
        rule::UNUSABLE_USER_PUBKEY,
        REFERENCE,
        "user pubkey re-encoded above the field modulus",
        REFERENCE_KEY,
        PermitWireFields {
            user_pubkey: bytes32(NON_CANONICAL_PUBKEY_HEX).to_vec(),
            ..reference_wire()
        },
        signature_from_hex(REFERENCE_SIGNATURE_HEX),
        None,
    );

    // -- rejected: typed form ---------------------------------------------
    //
    // For these the signature is copied from the base record unchanged: the rejection
    // precedes signature verification, and a record that also carried a bad signature
    // would no longer isolate one rule.
    let base_signature = signature_from_hex(REFERENCE_SIGNATURE_HEX);

    builder.reject(
        "user-pubkey-of-wrong-width",
        "A 31-byte user pubkey. Widths are checked where transport values are decoded, \
         because they are unrepresentable once the typed form is reached.",
        rule::IDENTITY_WIDTH,
        REFERENCE,
        "user pubkey truncated to 31 bytes",
        REFERENCE_KEY,
        PermitWireFields {
            user_pubkey: bytes32(USER_PUBKEY_HEX)[..31].to_vec(),
            ..reference_wire()
        },
        base_signature,
        None,
    );

    builder.reject(
        "verifying-program-id-of-wrong-width",
        "A 33-byte verifying program id.",
        rule::IDENTITY_WIDTH,
        REFERENCE,
        "verifying program id extended to 33 bytes",
        REFERENCE_KEY,
        PermitWireFields {
            verifying_program_id: {
                let mut bytes = bytes32(VERIFYING_PROGRAM_ID_HEX).to_vec();
                bytes.push(0);
                bytes
            },
            ..reference_wire()
        },
        base_signature,
        None,
    );

    builder.reject(
        "acl-domain-key-of-wrong-width",
        "A 31-byte ACL domain key.",
        rule::IDENTITY_WIDTH,
        REFERENCE,
        "second ACL domain key truncated to 31 bytes",
        REFERENCE_KEY,
        {
            let mut wire = reference_wire();
            wire.allowed_acl_domain_keys[1] = bytes32(ACL_DOMAIN_KEY_44_HEX)[..31].to_vec();
            wire
        },
        base_signature,
        None,
    );

    builder.reject(
        "eleven-acl-domains",
        "One domain past the maximum.",
        rule::TOO_MANY_ACL_DOMAIN_KEYS,
        REFERENCE,
        "domain list extended to eleven keys",
        REFERENCE_KEY,
        {
            let mut keys: Vec<[u8; 32]> = (0..11).map(distinct_domain_key).collect();
            keys.sort_unstable();
            PermitWireFields {
                allowed_acl_domain_keys: keys.iter().map(|key| key.to_vec()).collect(),
                ..reference_wire()
            }
        },
        base_signature,
        None,
    );

    builder.reject(
        "acl-domains-in-base58-string-order",
        "The reference permit's two domain keys, ordered by their base58 strings \
         instead of their bytes. This is the natural mistake — it is what sorting the \
         values a user sees produces — and it diverges from byte order exactly when \
         the encodings differ in length.",
        rule::ACL_DOMAIN_KEYS_NOT_ASCENDING,
        REFERENCE,
        "the two domain keys swapped into base58-string order",
        REFERENCE_KEY,
        PermitWireFields {
            allowed_acl_domain_keys: vec![
                bytes32(ACL_DOMAIN_KEY_44_HEX).to_vec(),
                bytes32(ACL_DOMAIN_KEY_43_HEX).to_vec(),
            ],
            ..reference_wire()
        },
        base_signature,
        None,
    );

    builder.reject(
        "duplicate-acl-domain-key",
        "The same domain key twice. Rejected rather than collapsed: two different \
         signed lists must never render the same text.",
        rule::DUPLICATE_ACL_DOMAIN_KEY,
        REFERENCE,
        "second domain key replaced by a copy of the first",
        REFERENCE_KEY,
        PermitWireFields {
            allowed_acl_domain_keys: vec![
                bytes32(ACL_DOMAIN_KEY_43_HEX).to_vec(),
                bytes32(ACL_DOMAIN_KEY_43_HEX).to_vec(),
            ],
            ..reference_wire()
        },
        base_signature,
        None,
    );

    builder.reject(
        "zero-duration",
        "A validity window of zero seconds authorizes nothing and has no legitimate \
         producer.",
        rule::DURATION_OUT_OF_RANGE,
        REFERENCE,
        "duration set to zero",
        REFERENCE_KEY,
        PermitWireFields {
            duration_seconds: 0,
            ..reference_wire()
        },
        base_signature,
        None,
    );

    builder.reject(
        "duration-past-one-year",
        "One second past the longest permitted window.",
        rule::DURATION_OUT_OF_RANGE,
        REFERENCE,
        "duration set one second above the maximum",
        REFERENCE_KEY,
        PermitWireFields {
            duration_seconds: MAX_DURATION_SECONDS + 1,
            ..reference_wire()
        },
        base_signature,
        None,
    );

    builder.reject(
        "start-timestamp-past-the-representable-range",
        "One second past the latest representable timestamp. Beyond this bound the \
         timestamp rendering would stop being a total function.",
        rule::START_TIMESTAMP_OUT_OF_RANGE,
        REFERENCE,
        "start set one second above the maximum",
        REFERENCE_KEY,
        PermitWireFields {
            start_timestamp: MAX_START_TIMESTAMP + 1,
            ..reference_wire()
        },
        base_signature,
        None,
    );

    builder.reject(
        "transport-key-of-the-larger-variant-length",
        "A transport key of the deprecated larger variant's public-key size. The \
         permit carries no variant field, so the length is the variant declaration: \
         accepting this length would leave implementations free to disagree about what \
         the key means.",
        rule::TRANSPORT_KEY_LENGTH,
        REFERENCE,
        "transport key replaced with one of the larger variant's length",
        "mlkem-1024-length",
        PermitWireFields {
            transport_key: transport_key_bytes_of_len(1568),
            ..reference_wire()
        },
        base_signature,
        None,
    );

    builder.reject(
        "transport-key-truncated",
        "A transport key one byte short.",
        rule::TRANSPORT_KEY_LENGTH,
        REFERENCE,
        "transport key truncated by one byte",
        "truncated-mlkem-512",
        PermitWireFields {
            transport_key: transport_key_bytes_of_len(868),
            ..reference_wire()
        },
        base_signature,
        None,
    );

    builder.reject(
        "transport-key-bare-of-the-accepted-variant",
        "The bare 800-byte ML-KEM-512 encapsulation key: the representation this \
         permit once accepted, and byte-for-byte the payload of the accepted \
         safe-serialized container. The accepted length declares the representation \
         as well as the variant, so the old wire form is refused rather than read as \
         a shorter container.",
        rule::TRANSPORT_KEY_LENGTH,
        REFERENCE,
        "transport key replaced with the bare encapsulation key",
        "bare-mlkem-512-800",
        PermitWireFields {
            // The genuine bare key, not filler: the KMS linker vector set carries the
            // same bytes under the same name, and a shared name must mean shared bytes.
            transport_key: reference_bare_transport_key(),
            ..reference_wire()
        },
        base_signature,
        None,
    );

    builder.reject(
        "kms-routing-of-unknown-version",
        "A routing field whose version byte this protocol version does not know. \
         Rejected before rendering, which is what keeps rendering total.",
        rule::UNKNOWN_KMS_ROUTING_VERSION,
        REFERENCE,
        "routing version byte set to 0x03",
        REFERENCE_KEY,
        {
            let mut wire = reference_wire();
            wire.extra_data[0] = 0x03;
            wire
        },
        base_signature,
        None,
    );

    builder.reject(
        "kms-routing-of-wrong-length",
        "A routing field of the known version with one byte too many. A length that \
         merely contains the fields would be a second encoding of the same routing \
         material.",
        rule::KMS_ROUTING_LENGTH,
        REFERENCE,
        "one byte appended to the routing field",
        REFERENCE_KEY,
        {
            let mut wire = reference_wire();
            wire.extra_data.push(0);
            wire
        },
        base_signature,
        None,
    );

    PermitVectorFile {
        schema: PERMIT_VECTOR_SCHEMA.to_string(),
        description: "Normative vectors for the Solana user-decrypt permit: typed form, \
                      canonical text, envelope and signature. Authorization rules — the \
                      validity window, the revocation watermark, deployment identity, the \
                      KMS pair and domain scope — are a separate layer and are not \
                      covered here."
            .to_string(),
        regenerate_with: "bash scripts/update-permit-vectors.sh".to_string(),
        deployment: Deployment {
            genesis_hash: GENESIS_HASH_HEX.to_string(),
            chain_id_decimal: CHAIN_ID.to_string(),
            chain_id_hex: format!("{CHAIN_ID:#018x}"),
            chain_id_be_bytes: to_hex(&CHAIN_ID.to_be_bytes()),
            chain_id_derivation: "zama-solana-chain-id-v1: digest = SHA-256(ASCII(\
                                  \"zama-solana-chain-id-v1\") || genesis_hash); chain_id \
                                  = 0x8000000000000000 | (be_u64(digest[0..8]) & \
                                  0x7fffffffffffffff). Applied once per cluster at \
                                  deployment; running components read the id from \
                                  configuration and check only the chain-kind bit."
                .to_string(),
        },
        transport_keys: builder.transport_keys,
        vectors: builder.vectors,
    }
}

// ---------------------------------------------------------------------------
// The committed file
// ---------------------------------------------------------------------------

/// The committed JSON equals what the builder produces — or is rewritten from it when
/// the update gate is set.
#[test]
fn committed_vectors_match_the_generator() {
    let built = build_vector_file();
    let serialized = format!(
        "{}\n",
        serde_json::to_string_pretty(&built).expect("vectors serialize")
    );
    let path = vector_path();

    if std::env::var_os(UPDATE_ENV).is_some() {
        std::fs::create_dir_all(path.parent().expect("fixture directory"))
            .expect("create fixture directory");
        std::fs::write(&path, &serialized).expect("write vectors");
        eprintln!("wrote {}", path.display());
        return;
    }

    let committed = std::fs::read_to_string(&path).unwrap_or_else(|error| {
        panic!(
            "cannot read {}: {error}\nregenerate with: {UPDATE_ENV}=1 cargo test -p \
             zama-solana-permit --test vectors",
            path.display()
        )
    });

    assert_eq!(
        committed, serialized,
        "the committed vectors differ from the generator; regenerate with \
         `bash scripts/update-permit-vectors.sh` and review the diff"
    );
}

fn load_vectors() -> PermitVectorFile {
    if std::env::var_os(UPDATE_ENV).is_some() {
        // In update mode the file may not exist yet; check the in-memory build.
        return build_vector_file();
    }
    let path = vector_path();
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()));
    serde_json::from_str(&text).expect("committed vectors parse against the shared schema")
}

// ---------------------------------------------------------------------------
// Behaviour
// ---------------------------------------------------------------------------

/// Reconstructs the wire form of a record.
fn wire_of(file: &PermitVectorFile, vector: &PermitVector) -> PermitWireFields {
    PermitWireFields {
        user_pubkey: from_hex(&vector.permit.user_pubkey).expect("hex"),
        transport_key: vector
            .transport_key_bytes(file)
            .expect("the record's transport key is in the table"),
        allowed_acl_domain_keys: vector
            .permit
            .allowed_acl_domain_keys
            .iter()
            .map(|key| from_hex(key).expect("hex"))
            .collect(),
        start_timestamp: vector.permit.start_timestamp.parse().expect("decimal u64"),
        duration_seconds: vector.permit.duration_seconds.parse().expect("decimal u64"),
        verifying_program_id: from_hex(&vector.permit.verifying_program_id).expect("hex"),
        chain_id: vector.permit.chain_id.parse().expect("decimal u64"),
        extra_data: from_hex(&vector.permit.extra_data).expect("hex"),
    }
}

/// Every record behaves exactly as it declares, and a rejecting record fails by the
/// rule it names — not merely somehow.
#[test]
fn every_vector_behaves_as_declared() {
    let file = load_vectors();

    for vector in &file.vectors {
        let wire = wire_of(&file, vector);
        let signature = zama_solana_permit::Signature::new(
            vector.signature_bytes().expect("64-byte signature"),
        );
        let name = &vector.name;

        match vector.result {
            VectorResult::Valid | VectorResult::Acceptable => {
                let fields = PermitFields::decode(&wire)
                    .unwrap_or_else(|error| panic!("{name}: should decode, got {error}"));
                assert_eq!(
                    verify_signature(&fields, &signature),
                    Ok(()),
                    "{name}: should verify"
                );

                // The recorded text and envelope are what this implementation
                // reconstructs.
                let text = render_canonical_text(&fields);
                assert_eq!(
                    vector.permit_text.as_deref(),
                    Some(text.as_str()),
                    "{name}: recorded text"
                );
                assert_eq!(
                    vector.permit_text_bytes.as_deref(),
                    Some(to_hex(text.as_bytes()).as_str()),
                    "{name}: recorded text bytes"
                );
                assert_eq!(
                    vector.envelope_bytes.as_deref(),
                    Some(to_hex(&build_envelope(&fields)).as_str()),
                    "{name}: recorded envelope"
                );
            }
            VectorResult::Invalid => {
                let declared = vector
                    .rule
                    .as_deref()
                    .unwrap_or_else(|| panic!("{name}: a rejecting record must name its rule"));

                let error = match PermitFields::decode(&wire) {
                    Err(error) => error,
                    Ok(fields) => verify_signature(&fields, &signature)
                        .expect_err(&format!("{name}: should have been rejected")),
                };

                assert_eq!(
                    rule_name(&error),
                    declared,
                    "{name}: rejected by the wrong rule ({error})"
                );
            }
        }
    }
}

/// Every rejecting record isolates one violation: its base is an accepted record, and
/// exactly one thing about it was changed.
///
/// Without this, "rejected by the declared rule" would depend on the order in which an
/// implementation happens to run its checks — which is not something the protocol
/// should fix.
#[test]
fn every_rejecting_vector_isolates_a_single_violation() {
    let file = load_vectors();

    for vector in file
        .vectors
        .iter()
        .filter(|v| v.result == VectorResult::Invalid)
    {
        let name = &vector.name;
        let base_name = vector
            .derived_from
            .as_deref()
            .unwrap_or_else(|| panic!("{name}: must name the record it was derived from"));
        assert!(
            vector.mutation.is_some(),
            "{name}: must describe the single change applied"
        );

        let base = file
            .vectors
            .iter()
            .find(|candidate| candidate.name == base_name)
            .unwrap_or_else(|| panic!("{name}: base record {base_name} is missing"));
        assert_eq!(
            base.result,
            VectorResult::Valid,
            "{name}: base record {base_name} must itself be accepted"
        );

        // The base really is accepted by this implementation, so the rejection below
        // is attributable to the mutation.
        let base_wire = wire_of(&file, base);
        let base_fields = PermitFields::decode(&base_wire)
            .unwrap_or_else(|error| panic!("{base_name}: base must decode, got {error}"));
        let base_signature =
            zama_solana_permit::Signature::new(base.signature_bytes().expect("64-byte signature"));
        assert_eq!(
            verify_signature(&base_fields, &base_signature),
            Ok(()),
            "{base_name}: base must verify"
        );

        // And something actually changed.
        assert!(
            vector.permit != base.permit || vector.signature != base.signature,
            "{name}: nothing differs from {base_name}"
        );
    }
}

/// Every rule in the shared list is exercised. A regeneration that drops a class fails
/// here instead of quietly reducing coverage.
#[test]
fn every_rule_is_exercised_by_some_vector() {
    let file = load_vectors();
    let covered: BTreeSet<&str> = file
        .vectors
        .iter()
        .filter_map(|vector| vector.rule.as_deref())
        .collect();

    for expected in rule::ALL {
        assert!(
            covered.contains(expected),
            "no vector exercises the rule {expected}"
        );
    }

    // And no record names a rule the shared list does not define.
    for name in covered {
        assert!(
            rule::ALL.contains(&name),
            "vector names an unknown rule {name}"
        );
    }
}

/// The conditional class carries the explanation that makes it meaningful.
#[test]
fn the_accepted_classes_are_represented() {
    let file = load_vectors();

    assert!(
        file.vectors.iter().any(|v| v.result == VectorResult::Valid),
        "no accepted vectors"
    );
    let conditional: Vec<&PermitVector> = file
        .vectors
        .iter()
        .filter(|v| v.result == VectorResult::Acceptable)
        .collect();
    assert!(
        !conditional.is_empty(),
        "no vector documents behaviour this layer accepts and a later one decides"
    );
    for vector in conditional {
        assert!(
            vector.comment.len() > 40,
            "{}: a conditional record must explain what is undecided",
            vector.name
        );
    }
}

// ---------------------------------------------------------------------------
// Schema discipline
// ---------------------------------------------------------------------------

/// The file declares the schema this code understands, and every record carries the
/// fields the schema requires.
#[test]
fn records_carry_the_required_fields() {
    let file = load_vectors();

    assert_eq!(file.schema, PERMIT_VECTOR_SCHEMA);
    assert!(!file.description.is_empty());
    assert!(!file.regenerate_with.is_empty());

    let mut names = BTreeSet::new();
    for vector in &file.vectors {
        assert!(
            names.insert(vector.name.clone()),
            "duplicate vector name {}",
            vector.name
        );
        assert!(
            !vector.comment.is_empty(),
            "{}: needs a comment",
            vector.name
        );
        assert_eq!(
            vector.signature.len(),
            128,
            "{}: signature must be 64 bytes",
            vector.name
        );
        assert!(
            file.transport_keys
                .contains_key(&vector.permit.transport_key),
            "{}: transport key {} is not in the table",
            vector.name,
            vector.permit.transport_key
        );
    }
}

/// The deployment identity is recorded in all three forms, and they agree.
#[test]
fn deployment_chain_id_is_consistent_in_every_form() {
    let file = load_vectors();
    let decimal: u64 = file
        .deployment
        .chain_id_decimal
        .parse()
        .expect("decimal chain id");

    let hex = file
        .deployment
        .chain_id_hex
        .strip_prefix("0x")
        .expect("hex chain id is 0x-prefixed");
    assert_eq!(u64::from_str_radix(hex, 16).expect("hex chain id"), decimal);
    assert_eq!(
        from_hex(&file.deployment.chain_id_be_bytes).expect("hex bytes"),
        decimal.to_be_bytes().to_vec()
    );
    assert_eq!(
        from_hex(&file.deployment.genesis_hash).expect("hex").len(),
        32
    );

    // Every record is signed against that deployment, except where the record's whole
    // point is a different chain id.
    for vector in &file.vectors {
        let chain_id: u64 = vector.permit.chain_id.parse().expect("decimal chain id");
        assert!(
            chain_id == decimal
                || vector.name.contains("chain-id")
                || vector.name == "widest-permit",
            "{}: unexplained chain id {chain_id}",
            vector.name
        );
    }
}

/// Every 64-bit field is a JSON string, not a JSON number.
///
/// A number would arrive in a TypeScript consumer as a double and lose precision above
/// 2^53 — and the chain id is routinely above it. This test reads the raw JSON rather
/// than the typed form, because the typed form cannot tell the difference.
#[test]
fn sixty_four_bit_fields_are_strings_in_the_json() {
    if std::env::var_os(UPDATE_ENV).is_some() {
        return;
    }
    let text = std::fs::read_to_string(vector_path()).expect("read vectors");
    let json: serde_json::Value = serde_json::from_str(&text).expect("parse vectors");

    let deployment = &json["deployment"];
    for field in ["chain_id_decimal", "chain_id_hex", "chain_id_be_bytes"] {
        assert!(
            deployment[field].is_string(),
            "deployment.{field} must be a string"
        );
    }

    for vector in json["vectors"].as_array().expect("vectors array") {
        let permit = &vector["permit"];
        for field in ["chain_id", "start_timestamp", "duration_seconds"] {
            assert!(
                permit[field].is_string(),
                "{}: permit.{field} must be a string, not a JSON number",
                vector["name"]
            );
        }
    }
}

/// At least one record exceeds the range a double-precision number represents
/// exactly, so a consumer that parses through one is caught by the vectors rather than
/// in production.
#[test]
fn some_vector_exceeds_the_javascript_safe_integer_range() {
    const SAFE: u64 = 9_007_199_254_740_991; // 2^53 - 1
    let file = load_vectors();

    assert!(
        file.vectors.iter().any(|vector| {
            vector
                .permit
                .chain_id
                .parse::<u64>()
                .is_ok_and(|chain_id| chain_id > SAFE)
        }),
        "no vector exercises a 64-bit value above the safe-integer range"
    );
}

/// The record set is pinned to the independently computed goldens through the reference
/// record: its signature is the one the foreign implementation produced.
///
/// That is what stops the vectors from being self-consistent-but-wrong. The text and
/// envelope are pinned the same way, one step removed: the canonical-text suite checks
/// them against foreign literals, and the runner above checks the records against the
/// same rendering.
#[test]
fn the_reference_record_carries_the_independently_computed_signature() {
    let file = load_vectors();
    let reference = file
        .vectors
        .iter()
        .find(|vector| vector.name == "reference-permit-two-domains")
        .expect("the reference record");

    assert_eq!(reference.signature, REFERENCE_SIGNATURE_HEX);

    let permissive = file
        .vectors
        .iter()
        .find(|vector| vector.name == "permissive-permit")
        .expect("the permissive record");
    assert_eq!(permissive.signature, PERMISSIVE_SIGNATURE_HEX);
}

/// Records that model a wallet signing something other than the canonical text say so,
/// and the recorded signature really is over that text — so another implementation can
/// reproduce the record instead of trusting it.
#[test]
fn records_with_a_non_canonical_signed_text_are_reproducible() {
    let file = load_vectors();
    let mut checked = 0;

    for vector in &file.vectors {
        let Some(signed_text) = vector.signed_text.as_deref() else {
            continue;
        };
        let wire = wire_of(&file, vector);
        let fields = PermitFields::decode(&wire).expect("these records decode");

        assert_ne!(
            signed_text,
            render_canonical_text(&fields),
            "{}: signed_text should differ from the canonical text",
            vector.name
        );
        assert_eq!(
            vector.signature,
            to_hex(sign_text_as_wallet(USER_SEED, signed_text).as_bytes()),
            "{}: recorded signature is not the one over signed_text",
            vector.name
        );
        checked += 1;
    }

    assert!(
        checked >= 2,
        "expected records with a non-canonical signed text"
    );
}

/// The unused-import guard: `IdentityField` is referenced so that the error taxonomy
/// this file maps from stays visible at a glance.
#[test]
fn the_error_taxonomy_is_the_one_the_rule_names_map_from() {
    assert_eq!(
        rule_name(&PermitError::IdentityWidth {
            field: IdentityField::UserPubkey,
            len: 31
        }),
        rule::IDENTITY_WIDTH
    );
}
