//! The answer the Merkle proof server gives each signed request, by the signing hash of its
//! authorization, until the server stops accepting the signature.
//!
//! A signed request can reach its server more than once until it expires: resent by anyone who
//! reads the traffic, or by the connector itself. The first copy to reach a server is
//! [`Admission::First`]: the server charges the signer and reads the record once. Every other
//! copy, sent at the same time or later, is an [`Admission::Repeat`] and waits for that answer,
//! including a refusal the answer ended in. A server therefore charges and reads at most once per
//! signed request (DD-067). The signing hash names no signer, so two KMS nodes that sign the same
//! body in the same second share one answer.
//!
//! Each signer may hold [`AnswerCache::new`]'s bytes of requests. Past that, its new requests are
//! refused ([`Admission::Full`]) rather than answered unremembered, so a faulty signer refuses
//! only itself. Only a charged request is remembered.

use std::{
    collections::{BTreeSet, HashMap},
    sync::Mutex,
};

use alloy::primitives::{Address, B256};
use tokio::sync::watch;

/// Bytes a request holds besides its answer: its two map entries, its expiry entry and its
/// channel, rounded up.
pub const ENTRY_OVERHEAD_BYTES: usize = 768;

pub struct AnswerCache<A> {
    max_bytes_per_signer: usize,
    inner: Mutex<Requests<A>>,
}

struct Requests<A> {
    by_signing_hash: HashMap<B256, Request<A>>,
    by_expiry: BTreeSet<(u64, B256)>,
    bytes_by_signer: HashMap<Address, usize>,
    /// Every request whose signature expired before this Unix time is forgotten. A copy of one
    /// that arrives later is refused rather than taken for a first copy. It follows the requests
    /// forgotten, not the clock, so a clock that steps forward refuses no request still valid.
    forgotten_before: u64,
}

struct Request<A> {
    signer: Address,
    answer: watch::Receiver<Option<A>>,
    bytes: usize,
}

/// How a server treats a signed request.
pub enum Admission<A> {
    /// The first copy, charged: send its answer here, then count its size with
    /// [`AnswerCache::settle`].
    First(watch::Sender<Option<A>>),
    /// A copy of a request already admitted: its answer, once the first copy has one.
    Repeat(watch::Receiver<Option<A>>),
    /// The signer holds as many requests as it may. Nothing was charged.
    Full,
    /// The charge refused the request. Nothing was remembered.
    OverRate,
    /// The signature expired before the requests last forgotten, so this may be a copy of one.
    Expired,
}

impl<A> AnswerCache<A> {
    pub fn new(max_bytes_per_signer: usize) -> Self {
        Self {
            max_bytes_per_signer,
            inner: Mutex::new(Requests {
                by_signing_hash: HashMap::new(),
                by_expiry: BTreeSet::new(),
                bytes_by_signer: HashMap::new(),
                forgotten_before: 0,
            }),
        }
    }

    /// Admits the request `signer` signed over `signing_hash`, accepted until `expires`, at Unix
    /// time `now`. Requests no longer accepted at `now` are forgotten first. A new request with
    /// room is admitted when `charge` accepts it; a copy is never charged.
    pub fn admit(
        &self,
        signing_hash: B256,
        expires: u64,
        signer: Address,
        now: u64,
        charge: impl FnOnce() -> bool,
    ) -> Admission<A> {
        let mut requests =
            self.inner.lock().unwrap_or_else(|err| err.into_inner());
        requests.forget_expired_before(now);
        if let Some(request) = requests.by_signing_hash.get(&signing_hash) {
            return Admission::Repeat(request.answer.clone());
        }
        if expires < requests.forgotten_before {
            return Admission::Expired;
        }
        let held = requests.bytes_by_signer.get(&signer).copied().unwrap_or(0);
        if held + ENTRY_OVERHEAD_BYTES > self.max_bytes_per_signer {
            return Admission::Full;
        }
        if !charge() {
            return Admission::OverRate;
        }
        let (sender, answer) = watch::channel(None);
        requests.by_signing_hash.insert(
            signing_hash,
            Request {
                signer,
                answer,
                bytes: ENTRY_OVERHEAD_BYTES,
            },
        );
        requests.by_expiry.insert((expires, signing_hash));
        *requests.bytes_by_signer.entry(signer).or_default() +=
            ENTRY_OVERHEAD_BYTES;
        Admission::First(sender)
    }

    /// Counts the `answer_bytes` the answer to `signing_hash` holds. Answers are counted once
    /// built, so a signer can pass its bytes by the answers it has in flight.
    pub fn settle(&self, signing_hash: &B256, answer_bytes: usize) {
        let mut requests =
            self.inner.lock().unwrap_or_else(|err| err.into_inner());
        let Some(request) = requests.by_signing_hash.get_mut(signing_hash)
        else {
            return;
        };
        request.bytes += answer_bytes;
        let signer = request.signer;
        *requests.bytes_by_signer.entry(signer).or_default() += answer_bytes;
    }
}

impl<A> Requests<A> {
    fn forget_expired_before(&mut self, now: u64) {
        while let Some(&(expires, hash)) = self.by_expiry.first() {
            if expires >= now {
                break;
            }
            self.by_expiry.pop_first();
            self.forgotten_before = self.forgotten_before.max(expires + 1);
            let request = self
                .by_signing_hash
                .remove(&hash)
                .expect("every expiry entry has its request");
            let held = self
                .bytes_by_signer
                .get_mut(&request.signer)
                .expect("every request counts against its signer");
            *held -= request.bytes;
            if *held == 0 {
                self.bytes_by_signer.remove(&request.signer);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NOW: u64 = 1_790_950_000;
    const ALICE: Address = Address::repeat_byte(0xA1);
    const BOB: Address = Address::repeat_byte(0xB0);

    fn hash(byte: u8) -> B256 {
        B256::repeat_byte(byte)
    }

    fn charged() -> bool {
        true
    }

    fn not_charged() -> bool {
        panic!("this request must not be charged")
    }

    fn first(admission: Admission<u8>) -> watch::Sender<Option<u8>> {
        match admission {
            Admission::First(sender) => sender,
            _ => panic!("expected the first copy"),
        }
    }

    #[tokio::test]
    async fn every_copy_until_expiry_gets_the_first_answer_uncharged() {
        let cache = AnswerCache::new(1 << 20);
        let sender = first(cache.admit(hash(1), NOW + 30, ALICE, NOW, charged));
        let Admission::Repeat(mut early) =
            cache.admit(hash(1), NOW + 30, ALICE, NOW, not_charged)
        else {
            panic!("a copy sent before the answer is a repeat");
        };
        sender.send_replace(Some(7));
        assert_eq!(*early.wait_for(Option::is_some).await.unwrap(), Some(7));
        let Admission::Repeat(late) =
            cache.admit(hash(1), NOW + 30, ALICE, NOW + 30, not_charged)
        else {
            panic!("a copy sent at the expiry is a repeat");
        };
        assert_eq!(*late.borrow(), Some(7));
    }

    #[test]
    fn a_signer_at_its_bytes_refuses_only_itself() {
        // Room for three requests' overhead, less the answer counted below.
        let cache = AnswerCache::new(3 * ENTRY_OVERHEAD_BYTES);
        first(cache.admit(hash(1), NOW, ALICE, NOW, charged));
        cache.settle(&hash(1), 1);
        first(cache.admit(hash(2), NOW + 30, ALICE, NOW, charged));
        assert!(matches!(
            cache.admit(hash(3), NOW + 30, ALICE, NOW, not_charged),
            Admission::Full
        ));
        first(cache.admit(hash(4), NOW + 30, BOB, NOW, charged));
        assert!(matches!(
            cache.admit(hash(2), NOW + 30, ALICE, NOW, not_charged),
            Admission::Repeat(_)
        ));
        // The first request expired with its answer: its bytes make room for two more.
        first(cache.admit(hash(3), NOW + 30, ALICE, NOW + 1, charged));
        first(cache.admit(hash(5), NOW + 30, ALICE, NOW + 1, charged));
    }

    #[test]
    fn a_request_the_charge_refuses_is_not_remembered() {
        let cache = AnswerCache::<u8>::new(1 << 20);
        assert!(matches!(
            cache.admit(hash(1), NOW + 30, ALICE, NOW, || false),
            Admission::OverRate
        ));
        first(cache.admit(hash(1), NOW + 30, ALICE, NOW, charged));
    }

    #[test]
    fn a_copy_that_arrives_after_its_request_was_forgotten_is_refused() {
        let cache = AnswerCache::<u8>::new(1 << 20);
        first(cache.admit(hash(1), NOW, ALICE, NOW, charged));
        // Another request, read one second later, forgets the first.
        first(cache.admit(hash(2), NOW + 30, ALICE, NOW + 1, charged));
        // A copy of the first whose handler read the clock before that.
        assert!(matches!(
            cache.admit(hash(1), NOW, ALICE, NOW, not_charged),
            Admission::Expired
        ));
    }
}
