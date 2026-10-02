//! The answers the Merkle proof server gave, by the signing hash of the request that asked.
//!
//! A request's signature names no recipient, so a coprocessor that received a request can resend
//! it to the other servers until it expires. A server answers such a repeat from here, without
//! charging the signer or reading the database again, so a replay neither spends the signer's rate
//! nor adds load (DD-067). The connector's own hedge to a server that a replay reached first gets
//! the same answer. An answer lives until its signature expires. Past [`MAX_CACHED_BYTES`] a new
//! answer is not kept, and a later repeat of it is new work again.

use std::{collections::HashMap, sync::Mutex};

use alloy::primitives::B256;
use axum::body::Bytes;

/// About a minute of answers at a few thousand requests per second.
pub const MAX_CACHED_BYTES: usize = 64 * 1024 * 1024;

pub struct AnswerCache {
    max_bytes: usize,
    inner: Mutex<Answers>,
}

#[derive(Default)]
struct Answers {
    by_signing_hash: HashMap<B256, (u64, Bytes)>,
    bytes: usize,
}

impl AnswerCache {
    pub fn new(max_bytes: usize) -> Self {
        Self {
            max_bytes,
            inner: Mutex::default(),
        }
    }

    /// The answer to `signing_hash`, while its signature is valid at Unix time `now`.
    pub fn get(&self, signing_hash: &B256, now: u64) -> Option<Bytes> {
        let answers = self.inner.lock().unwrap_or_else(|err| err.into_inner());
        answers
            .by_signing_hash
            .get(signing_hash)
            .filter(|(expires, _)| *expires >= now)
            .map(|(_, answer)| answer.clone())
    }

    /// Keeps `answer` to the request signed over `signing_hash` until `expires`. When it does
    /// not fit, the expired answers go first; an answer that still does not fit is not kept.
    pub fn insert(
        &self,
        signing_hash: B256,
        expires: u64,
        answer: Bytes,
        now: u64,
    ) {
        let mut answers =
            self.inner.lock().unwrap_or_else(|err| err.into_inner());
        if answers.bytes + answer.len() > self.max_bytes {
            let Answers {
                by_signing_hash,
                bytes,
            } = &mut *answers;
            by_signing_hash.retain(|_, (kept_until, kept)| {
                let live = *kept_until >= now;
                if !live {
                    *bytes -= kept.len();
                }
                live
            });
            if *bytes + answer.len() > self.max_bytes {
                return;
            }
        }
        answers.bytes += answer.len();
        if let Some((_, replaced)) = answers
            .by_signing_hash
            .insert(signing_hash, (expires, answer))
        {
            answers.bytes -= replaced.len();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NOW: u64 = 1_790_950_000;

    #[test]
    fn an_answer_is_kept_until_its_signature_expires() {
        let cache = AnswerCache::new(1024);
        let answer = Bytes::from_static(b"proofs");
        cache.insert(B256::repeat_byte(1), NOW + 60, answer.clone(), NOW);
        assert_eq!(cache.get(&B256::repeat_byte(1), NOW + 60), Some(answer));
        assert_eq!(cache.get(&B256::repeat_byte(1), NOW + 61), None);
        assert_eq!(cache.get(&B256::repeat_byte(2), NOW), None);
    }

    #[test]
    fn a_full_cache_drops_expired_answers_then_keeps_no_more() {
        let cache = AnswerCache::new(8);
        cache.insert(
            B256::repeat_byte(1),
            NOW,
            Bytes::from_static(b"1234"),
            NOW,
        );
        cache.insert(
            B256::repeat_byte(2),
            NOW + 60,
            Bytes::from_static(b"5678"),
            NOW,
        );
        // Full: the expired first answer makes room for the third.
        cache.insert(
            B256::repeat_byte(3),
            NOW + 60,
            Bytes::from_static(b"9abc"),
            NOW + 1,
        );
        assert_eq!(cache.get(&B256::repeat_byte(1), NOW), None);
        assert!(cache.get(&B256::repeat_byte(3), NOW + 1).is_some());
        // Full of live answers: the fourth is not kept.
        cache.insert(
            B256::repeat_byte(4),
            NOW + 60,
            Bytes::from_static(b"defg"),
            NOW + 1,
        );
        assert_eq!(cache.get(&B256::repeat_byte(4), NOW + 1), None);
        assert!(cache.get(&B256::repeat_byte(2), NOW + 1).is_some());
    }
}
