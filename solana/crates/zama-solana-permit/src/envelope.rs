//! Envelope construction and signature verification.
//!
//! The wallet signs the canonical text inside an offchain-message envelope whose
//! first byte is illegal as a transaction's first byte — that is the structural
//! guarantee that a permit signature can never be replayed as a transaction.
//!
//! Verification is reconstruction: the envelope is rebuilt locally from validated
//! typed fields and the signature is checked over those bytes. Neither the text nor
//! the envelope is ever accepted from a caller, which is why the signed bytes are
//! never parsed for a security decision.
//!
//! # What "verifies" means here
//!
//! Ed25519 is not one behaviour. Implementations disagree about non-reduced signature
//! scalars, small-order public keys and non-canonically encoded coordinates, and these
//! permits are verified by five implementations on five libraries. Any disagreement
//! means they authorize different sets of permits, so the strict reading is spelled out
//! rather than inherited from whichever library each side happens to link:
//!
//! * the signature scalar must be reduced, and neither `R` nor `A` may be of small
//!   order — this is `verify_strict`, not the permissive entry point;
//! * the user pubkey must be a point on the curve;
//! * the user pubkey's encoding must be canonical, i.e. its y-coordinate below the field
//!   modulus. No attack rides on this one — the encoding sits inside the signed envelope,
//!   so a re-encoded key changes the message — but libraries disagree about accepting it
//!   and every real wallet key is canonical, so the disagreement is removed rather than
//!   left open.
//!
//! The first is a property of the signature and fails as a mismatch. The last three are
//! properties of the key: they make the permit unusable no matter what accompanies it,
//! and they say so with their own rejection.

use ed25519_dalek::{Signature as DalekSignature, VerifyingKey};

use crate::{
    error::PermitError,
    render::render_canonical_text,
    types::{Identity, PermitFields, Signature, IDENTITY_LEN},
};

/// The offchain-message preamble. Its leading `0xff` cannot begin a transaction.
pub const ENVELOPE_PREAMBLE: &[u8; 16] = b"\xffsolana offchain";
/// Envelope format version.
pub const ENVELOPE_VERSION: u8 = 1;
/// A permit envelope always has exactly one signer, the permit's own user.
pub const ENVELOPE_SIGNER_COUNT: u8 = 1;

/// Reconstructs the envelope bytes the wallet signed.
pub fn build_envelope(fields: &PermitFields) -> Vec<u8> {
    let text = render_canonical_text(fields);

    let mut envelope = Vec::with_capacity(ENVELOPE_PREAMBLE.len() + 2 + IDENTITY_LEN + text.len());
    envelope.extend_from_slice(ENVELOPE_PREAMBLE);
    envelope.push(ENVELOPE_VERSION);
    envelope.push(ENVELOPE_SIGNER_COUNT);
    // The sole signer is the permit's own user, which is also what the text's `User:` line
    // names — so the screen a human read and the bytes their wallet signed cannot disagree
    // about who is consenting.
    envelope.extend_from_slice(fields.user_pubkey().as_bytes());
    // No length prefix and no application domain: the text runs to the end of the message.
    envelope.extend_from_slice(text.as_bytes());
    envelope
}

/// Verifies the signature over the locally reconstructed envelope.
///
/// The parameters are the whole contract: validated fields and a signature. There
/// is deliberately no variant of this function accepting a text or an envelope.
pub fn verify_signature(fields: &PermitFields, signature: &Signature) -> Result<(), PermitError> {
    let verifying_key = usable_verifying_key(fields.user_pubkey())?;

    verifying_key
        .verify_strict(
            &build_envelope(fields),
            &DalekSignature::from_bytes(signature.as_bytes()),
        )
        .map_err(|_| PermitError::SignatureMismatch)
}

/// Turns a permit's user pubkey into a key that can verify, or says it cannot.
///
/// Three ways a key is unusable, all reported alike because they are all "this permit
/// names something that is not a wallet key", none of them a statement about the
/// signature that arrived with it.
fn usable_verifying_key(user_pubkey: &Identity) -> Result<VerifyingKey, PermitError> {
    let encoded = user_pubkey.as_bytes();

    if !is_canonically_encoded(encoded) {
        return Err(PermitError::UnusableUserPubkey);
    }

    let verifying_key =
        VerifyingKey::from_bytes(encoded).map_err(|_| PermitError::UnusableUserPubkey)?;

    // Small-order keys are checked here rather than left to `verify_strict`, which also
    // rejects them but cannot distinguish them from an ordinary mismatch. The difference
    // matters: under a cofactored verifier an all-zero signature verifies against such a
    // key, so a permit could carry the consent of a wallet nobody owns — that is a fact
    // about the permit, and every verifier has to report it the same way.
    if verifying_key.is_weak() {
        return Err(PermitError::UnusableUserPubkey);
    }

    Ok(verifying_key)
}

/// Little-endian encoding of the field modulus, 2^255 - 19.
const FIELD_MODULUS_LE: [u8; IDENTITY_LEN] = {
    let mut bytes = [0xff; IDENTITY_LEN];
    bytes[0] = 0xed;
    bytes[IDENTITY_LEN - 1] = 0x7f;
    bytes
};

/// True when a compressed point encodes its y-coordinate canonically, i.e. below the
/// field modulus.
///
/// The top bit carries the sign of x and is not part of the coordinate, so it is masked
/// off before the comparison. That masking is not an optimization: roughly half of all
/// legitimate wallet keys have it set, and a check that skipped the mask would reject
/// them.
fn is_canonically_encoded(encoded: &[u8; IDENTITY_LEN]) -> bool {
    let mut coordinate = *encoded;
    coordinate[IDENTITY_LEN - 1] &= 0x7f;

    // Little-endian, so the comparison walks from the most significant byte down. Equality
    // with the modulus is itself non-canonical: it encodes zero the long way.
    for (byte, modulus_byte) in coordinate.iter().zip(FIELD_MODULUS_LE.iter()).rev() {
        if byte != modulus_byte {
            return byte < modulus_byte;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A y-coordinate one below the modulus is canonical; the modulus itself and
    /// everything above it is not.
    ///
    /// The boundary is pinned from both sides because it is invisible from the outside:
    /// the public suite can only show that some chosen encoding is refused, not that the
    /// refusal starts in exactly the right place.
    #[test]
    fn the_canonical_encoding_boundary_is_the_field_modulus() {
        let mut below = FIELD_MODULUS_LE;
        below[0] -= 1;
        assert!(is_canonically_encoded(&below), "modulus minus one");

        assert!(
            !is_canonically_encoded(&FIELD_MODULUS_LE),
            "the modulus encodes zero the long way"
        );

        let mut above = FIELD_MODULUS_LE;
        above[0] += 1;
        assert!(!is_canonically_encoded(&above), "modulus plus one");

        assert!(
            !is_canonically_encoded(&[0xff; IDENTITY_LEN]),
            "the largest 255-bit coordinate"
        );

        assert!(is_canonically_encoded(&[0u8; IDENTITY_LEN]), "zero");
    }

    /// The sign bit is not part of the coordinate.
    ///
    /// This is the check the public suite cannot make, and the one with the most to lose:
    /// about half of all real wallet keys have this bit set, so a comparison that forgot
    /// to mask it would refuse them — while still passing every fixture, because the
    /// fixture wallet's key happens to have it clear.
    #[test]
    fn the_sign_bit_is_not_part_of_the_coordinate() {
        for coordinate in [[0u8; IDENTITY_LEN], {
            let mut below = FIELD_MODULUS_LE;
            below[0] -= 1;
            below
        }] {
            let mut with_sign_bit = coordinate;
            with_sign_bit[IDENTITY_LEN - 1] |= 0x80;

            assert!(
                is_canonically_encoded(&with_sign_bit),
                "setting the sign bit must not change whether {coordinate:?} is canonical"
            );
        }
    }
}
