//! The crate's model of the SBF bump allocator, and the vector type that pays into it.
//!
//! Crate-private on purpose: app programs budget against the *numbers* in [`crate::cost`] —
//! the ceiling constants and [`crate::cost::FheExecutionCost`] — and never against the growth
//! model that produces them. Everything here exists so the builder can tally, byte for byte,
//! what one execution requests from the program's fixed, never-freeing 32 KB heap (DD-046).
//! Both models are validated against a counting global allocator in `heap_budget.rs`, which is
//! what keeps them honest if `Vec`'s growth strategy or Anchor's codegen ever changes.

/// `RawVec`'s first non-zero capacity for an element size — the other half of the growth model
/// [`tally_push`] and [`invoke_table_heap_bytes`] share.
fn minimum_nonzero_capacity(elem_size: usize) -> usize {
    if elem_size == 1 {
        8
    } else if elem_size <= 1024 {
        4
    } else {
        1
    }
}

/// The capacity a full `Vec` grows to on the next push: double, or `RawVec`'s minimum first
/// capacity.
fn grown_capacity(capacity: usize, elem_size: usize) -> usize {
    std::cmp::max(2 * capacity, minimum_nonzero_capacity(elem_size))
}

/// Tallies the bytes the next `push` will request from the allocator, modeling `Vec` growth
/// the way the never-freeing bump region pays for it: a full vector reallocates to double its
/// capacity (or to `RawVec`'s minimum first capacity), and the outgrown buffer is never
/// reclaimed. Call immediately before the push.
pub(crate) fn tally_push<T>(vec: &Vec<T>, tally: &mut usize) {
    if vec.len() < vec.capacity() {
        return;
    }
    let elem_size = std::mem::size_of::<T>();
    *tally += grown_capacity(vec.capacity(), elem_size) * elem_size;
}

/// A `Vec` that cannot forget to tally: every allocation it makes — the up-front reservation
/// and every growth past it — is recorded on the vector itself, and
/// [`requested_bytes`](Self::requested_bytes) reports the total. The builder's tables are all
/// `TalliedVec`s, so a new push site is paid for by construction instead of by remembering to
/// call [`tally_push`] next to it; the exact-size allocations that are not a growing table
/// (attestation embeds, subject index lists, purpose tables) stay on the explicit counter in
/// `StepTables`.
///
/// Reads go through `Deref<Target = [T]>`. There is deliberately no `DerefMut` to the `Vec`:
/// the only mutation paths are the ones that keep the tally honest. `truncate` keeps the
/// requested bytes — on the bump region a rolled-back request is spent all the same.
#[derive(Debug, Clone)]
pub(crate) struct TalliedVec<T> {
    vec: Vec<T>,
    requested: usize,
}

impl<T> TalliedVec<T> {
    /// An empty table that has requested nothing yet.
    pub(crate) fn new() -> Self {
        Self {
            vec: Vec::new(),
            requested: 0,
        }
    }

    /// A table with its bound reserved up front — the reservation is the request.
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            vec: Vec::with_capacity(capacity),
            requested: capacity * std::mem::size_of::<T>(),
        }
    }

    pub(crate) fn push(&mut self, value: T) {
        tally_push(&self.vec, &mut self.requested);
        self.vec.push(value);
    }

    /// Shortens the table, keeping the requested bytes: the allocator already served them.
    pub(crate) fn truncate(&mut self, len: usize) {
        self.vec.truncate(len);
    }

    pub(crate) fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.vec.get_mut(index)
    }

    /// Every byte this table has requested from the allocator so far.
    pub(crate) fn requested_bytes(&self) -> usize {
        self.requested
    }

    /// Hands the underlying `Vec` over (to the wire args, or the finished execution). The
    /// caller must have folded [`requested_bytes`](Self::requested_bytes) into its total first —
    /// the request does not travel with the `Vec`.
    pub(crate) fn into_inner(self) -> Vec<T> {
        self.vec
    }
}

impl<T> std::ops::Deref for TalliedVec<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        &self.vec
    }
}

impl<T: PartialEq> PartialEq for TalliedVec<T> {
    /// Contents only: two tables that hold the same entries are the same table, however their
    /// reservations happened to grow.
    fn eq(&self, other: &Self) -> bool {
        self.vec == other.vec
    }
}

impl<T: Eq> Eq for TalliedVec<T> {}

/// Bytes `count` pushes into an empty `Vec` request from the allocator, and the capacity the
/// vector ends at — the growth pattern Anchor's generated `to_account_metas`/`to_account_infos`
/// pay for the fixed CPI accounts, which start from `vec![]`.
fn pushes_from_empty(count: usize, elem_size: usize) -> (usize, usize) {
    let mut capacity = 0usize;
    let mut requested = 0usize;
    for length in 0..count {
        if length == capacity {
            capacity = grown_capacity(capacity, elem_size);
            requested += capacity * elem_size;
        }
    }
    (requested, capacity)
}

/// Fixed accounts on the host's `fhe_execute` CPI account struct (payer through the event CPI
/// program). The invoke-measurement test in `heap_budget.rs` pins this against the real Anchor
/// struct, so it cannot drift silently when the host's account list changes.
pub(crate) const FHE_EXECUTE_FIXED_CPI_ACCOUNTS: usize = 9;

/// Heap bytes the crate's own invoke path requests *after* `build()`: the CPI account-meta and
/// account-info tables (`invoke`), and `resolve_accounts`'s three right-sized tables. An exact
/// function of the account counts the builder already knows, charged against
/// [`crate::cost::BUILD_HEAP_BUDGET_BYTES`] at `finish` — an execution admitted by `build()`
/// fits the whole instruction, not just its own construction. Kept honest by the invoke
/// measurement in `heap_budget.rs`, which runs the real assembly under a counting allocator.
///
/// Assumes the minimal invoke: zero per-transaction deny-subject witnesses (each deny record an
/// app's transaction adds costs one more `AccountMeta` plus one more `AccountInfo` slot,
/// ~90 bytes, out of the reserve), and every fixed account present — an absent optional HCU
/// witness only makes the real cost smaller than charged.
pub(crate) fn invoke_table_heap_bytes(
    remaining_accounts: usize,
    dynamic_accounts: usize,
    output_authorities: usize,
) -> usize {
    let account_info_size = std::mem::size_of::<anchor_lang::prelude::AccountInfo<'static>>();
    let account_meta_size =
        std::mem::size_of::<anchor_lang::solana_program::instruction::AccountMeta>();
    // Anchor's generated accessors grow the fixed accounts from `vec![]`; `invoke` then makes
    // one exact reservation for the dynamic tail (no allocation when it still fits the grown
    // capacity).
    let table = |elem_size: usize| {
        let (fixed_bytes, fixed_capacity) =
            pushes_from_empty(FHE_EXECUTE_FIXED_CPI_ACCOUNTS, elem_size);
        let final_length = FHE_EXECUTE_FIXED_CPI_ACCOUNTS + remaining_accounts;
        let tail_bytes = if final_length > fixed_capacity {
            final_length * elem_size
        } else {
            0
        };
        fixed_bytes + tail_bytes
    };
    // Anchor's generated `to_account_infos` builds each fixed account's contribution as a
    // one-element `vec![clone]` before extending the table with it — one exact-size temporary
    // per present field.
    let fixed_info_temporaries = FHE_EXECUTE_FIXED_CPI_ACCOUNTS * account_info_size;
    // `resolve_accounts` sizes its dynamic-account and authority-witness tables from the counts
    // the execution itself requires, and its resolved table holds every remaining account.
    let resolve_bytes =
        (dynamic_accounts + output_authorities + remaining_accounts) * account_info_size;
    table(account_meta_size) + table(account_info_size) + fixed_info_temporaries + resolve_bytes
}
