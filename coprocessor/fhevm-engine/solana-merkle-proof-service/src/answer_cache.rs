//! The answer the Merkle proof server gives each signed request, by the signing hash of its
//! authorization, until the signature expires.
//!
//! A request's signature names no recipient, so a coprocessor that received a request can resend
//! it to the other servers until it expires. The first copy of a request to reach a server is
//! [`Admission::First`]: the server charges the signer and reads the record once. Every other
//! copy, sent at the same time or later, is an [`Admission::Repeat`] and waits for that answer,
//! a refusal included. A replay therefore neither spends the signer's rate nor adds load (DD-067).
//! When a server holds [`AnswerCache::new`]'s bytes of requests, it refuses new ones
//! ([`Admission::Full`]) rather than answer a request it could not recognize when replayed.

use std::{
    collections::{BTreeSet, HashMap},
    sync::Mutex,
};

use alloy::primitives::B256;
use tokio::sync::watch;

/// Bytes a request holds besides its answer: its map entry, its expiry entry and its channel.
pub const ENTRY_OVERHEAD_BYTES: usize = 256;

pub struct AnswerCache<A> {
    max_bytes: usize,
    inner: Mutex<Requests<A>>,
}

struct Requests<A> {
    by_signing_hash: HashMap<B256, Request<A>>,
    by_expiry: BTreeSet<(u64, B256)>,
    bytes: usize,
}

struct Request<A> {
    answer: watch::Receiver<Option<A>>,
    bytes: usize,
}

/// How a server treats a signed request.
pub enum Admission<A> {
    /// The first copy: send its answer here, then count its size with [`AnswerCache::settle`].
    First(watch::Sender<Option<A>>),
    /// A copy of a request already admitted: its answer, once the first copy has one.
    Repeat(watch::Receiver<Option<A>>),
    /// The cache holds as many requests as it may.
    Full,
}

impl<A> AnswerCache<A> {
    pub fn new(max_bytes: usize) -> Self {
        Self {
            max_bytes,
            inner: Mutex::new(Requests {
                by_signing_hash: HashMap::new(),
                by_expiry: BTreeSet::new(),
                bytes: 0,
            }),
        }
    }

    /// Admits the request signed over `signing_hash`, valid until `expires`, at Unix time
    /// `now`. Requests whose signature expired before `now` are forgotten first.
    pub fn admit(
        &self,
        signing_hash: B256,
        expires: u64,
        now: u64,
    ) -> Admission<A> {
        let mut requests =
            self.inner.lock().unwrap_or_else(|err| err.into_inner());
        while let Some(&(oldest, hash)) = requests.by_expiry.first() {
            if oldest >= now {
                break;
            }
            requests.by_expiry.pop_first();
            if let Some(request) = requests.by_signing_hash.remove(&hash) {
                requests.bytes -= request.bytes;
            }
        }
        if let Some(request) = requests.by_signing_hash.get(&signing_hash) {
            return Admission::Repeat(request.answer.clone());
        }
        if requests.bytes + ENTRY_OVERHEAD_BYTES > self.max_bytes {
            return Admission::Full;
        }
        let (sender, answer) = watch::channel(None);
        requests.by_signing_hash.insert(
            signing_hash,
            Request {
                answer,
                bytes: ENTRY_OVERHEAD_BYTES,
            },
        );
        requests.by_expiry.insert((expires, signing_hash));
        requests.bytes += ENTRY_OVERHEAD_BYTES;
        Admission::First(sender)
    }

    /// Counts the `answer_bytes` the answer to `signing_hash` holds.
    pub fn settle(&self, signing_hash: &B256, answer_bytes: usize) {
        let mut requests =
            self.inner.lock().unwrap_or_else(|err| err.into_inner());
        if let Some(request) = requests.by_signing_hash.get_mut(signing_hash) {
            request.bytes += answer_bytes;
            requests.bytes += answer_bytes;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NOW: u64 = 1_790_950_000;

    fn first(admission: Admission<u8>) -> watch::Sender<Option<u8>> {
        match admission {
            Admission::First(sender) => sender,
            _ => panic!("expected the first copy"),
        }
    }

    #[tokio::test]
    async fn every_copy_until_expiry_gets_the_first_answer() {
        let cache = AnswerCache::new(1 << 20);
        let sender = first(cache.admit(B256::repeat_byte(1), NOW + 30, NOW));
        let Admission::Repeat(mut early) =
            cache.admit(B256::repeat_byte(1), NOW + 30, NOW)
        else {
            panic!("a copy sent before the answer is a repeat");
        };
        sender.send_replace(Some(7));
        assert_eq!(*early.wait_for(Option::is_some).await.unwrap(), Some(7));
        let Admission::Repeat(late) =
            cache.admit(B256::repeat_byte(1), NOW + 30, NOW + 30)
        else {
            panic!("a copy sent at the expiry is a repeat");
        };
        assert_eq!(*late.borrow(), Some(7));
        first(cache.admit(B256::repeat_byte(1), NOW + 61, NOW + 31));
    }

    #[test]
    fn a_full_cache_refuses_new_requests_until_old_ones_expire() {
        let cache = AnswerCache::new(2 * ENTRY_OVERHEAD_BYTES + 10);
        first(cache.admit(B256::repeat_byte(1), NOW, NOW));
        first(cache.admit(B256::repeat_byte(2), NOW + 30, NOW));
        cache.settle(&B256::repeat_byte(2), 10);
        assert!(matches!(
            cache.admit(B256::repeat_byte(3), NOW + 30, NOW),
            Admission::Full
        ));
        assert!(matches!(
            cache.admit(B256::repeat_byte(2), NOW + 30, NOW),
            Admission::Repeat(_)
        ));
        // The first request expired: its bytes make room for the third.
        first(cache.admit(B256::repeat_byte(3), NOW + 30, NOW + 1));
    }
}
