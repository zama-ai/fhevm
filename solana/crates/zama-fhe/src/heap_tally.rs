//! The crate's model of the SBF bump allocator, and the types that pay into it.
//!
//! Crate-private on purpose: app programs budget against the *numbers* in [`crate::cost`] —
//! the ceiling constants and [`crate::cost::FheExecutionCost`] — and never against the growth
//! model that produces them. Everything here exists so the builder can tally, byte for byte,
//! what one execution requests from the program's fixed, never-freeing 32 KB heap (DD-046).
//! Both models are validated against a counting global allocator in `heap_budget`, which is
//! what keeps them honest if `Vec`'s growth strategy or Anchor's codegen ever changes.
//!
//! [`HeapBudget`] is the one running total. Intern tables grow only through
//! [`TalliedVec::try_push`] — there is no `DerefMut` to the `Vec`, so a forgotten `.push`
//! does not compile. Exact-size sites go through [`TalliedVec::try_with_capacity`] (and
//! [`TalliedVec::try_filled`] for the finish bitmaps), never a raw `Vec`.

use crate::{FheExecutionBuildError, Result};

/// `RawVec`'s first non-zero capacity for an element size — the other half of the growth model
/// [`pushes_request`] and [`invoke_table_heap_bytes`] share.
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

/// Bytes `new_elements` further pushes into `vec` will request from the allocator.
fn pushes_request<T>(vec: &Vec<T>, new_elements: usize) -> usize {
    let elem_size = std::mem::size_of::<T>();
    let mut capacity = vec.capacity();
    let mut requested = 0;
    for length in vec.len()..vec.len() + new_elements {
        if length == capacity {
            capacity = grown_capacity(capacity, elem_size);
            requested += capacity * elem_size;
        }
    }
    requested
}

/// Running total of every byte this build has admitted against
/// [`crate::cost::BUILD_HEAP_BUDGET_BYTES`]. Owned by the builder; borrowed by lowering for
/// the duration of a step. Packet and invoke costs are not charged here — they have not
/// allocated yet at `finish` — they are tested with [`fits_with`](Self::fits_with).
#[derive(Debug)]
pub(crate) struct HeapBudget {
    total: usize,
}

impl HeapBudget {
    pub(crate) fn new() -> Self {
        Self { total: 0 }
    }

    pub(crate) fn total(&self) -> usize {
        self.total
    }

    /// Admits `upcoming` bytes against the budget and charges them. Zero is always admitted.
    pub(crate) fn admit(&mut self, upcoming: usize) -> Result<()> {
        let next = self.total.saturating_add(upcoming);
        if next > crate::cost::BUILD_HEAP_BUDGET_BYTES {
            return Err(FheExecutionBuildError::ExceedsBuildHeapBudget);
        }
        self.total = next;
        Ok(())
    }

    /// Whether `extra` bytes that have not allocated yet still fit with the charged total.
    pub(crate) fn fits_with(&self, extra: usize) -> bool {
        self.total.saturating_add(extra) <= crate::cost::BUILD_HEAP_BUDGET_BYTES
    }
}

/// A `Vec` that cannot forget to tally: growth goes through [`try_push`](Self::try_push) so it
/// is admitted against a [`HeapBudget`] before the allocator serves it.
///
/// Reads go through `Deref<Target = [T]>`. There is deliberately no `DerefMut` to the `Vec`:
/// the only mutation paths are the ones that keep the tally honest. `truncate` keeps the
/// charged bytes — on the bump region a rolled-back request is spent all the same.
#[derive(Debug, Clone)]
pub(crate) struct TalliedVec<T> {
    vec: Vec<T>,
}

impl<T> TalliedVec<T> {
    /// An empty table that has requested nothing yet.
    pub(crate) fn new() -> Self {
        Self { vec: Vec::new() }
    }

    /// A table with its bound reserved up front — the reservation is charged to `budget`.
    pub(crate) fn try_with_capacity(budget: &mut HeapBudget, capacity: usize) -> Result<Self> {
        budget.admit(capacity * std::mem::size_of::<T>())?;
        Ok(Self {
            vec: Vec::with_capacity(capacity),
        })
    }

    /// Admits an exact-size buffer and fills it. Length stays within the reserved capacity, so
    /// filling does not allocate again.
    pub(crate) fn try_filled(budget: &mut HeapBudget, len: usize, fill: T) -> Result<Self>
    where
        T: Clone,
    {
        let mut this = Self::try_with_capacity(budget, len)?;
        this.vec.resize(len, fill);
        Ok(this)
    }

    /// Admits the next push against `budget` before the allocator serves it.
    pub(crate) fn try_push(&mut self, budget: &mut HeapBudget, value: T) -> Result<()> {
        let upcoming = pushes_request(&self.vec, 1);
        if upcoming > 0 {
            budget.admit(upcoming)?;
        }
        self.vec.push(value);
        Ok(())
    }

    /// Test-only: grow the table without a [`HeapBudget`]. Fixture builders inject illegal
    /// states to exercise `finish` validation; those paths are not production admission.
    #[cfg(test)]
    pub(crate) fn push(&mut self, value: T) {
        self.vec.push(value);
    }

    /// Shortens the table, keeping the charged bytes: the allocator already served them.
    pub(crate) fn truncate(&mut self, len: usize) {
        self.vec.truncate(len);
    }

    pub(crate) fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.vec.get_mut(index)
    }

    pub(crate) fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.vec
    }

    /// Hands the underlying `Vec` over (to the wire args, or the finished execution). The
    /// request has already been charged to the [`HeapBudget`] and does not travel with the
    /// `Vec`.
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
/// program). The invoke-measurement test in `heap_budget` pins this against the real Anchor
/// struct, so it cannot drift silently when the host's account list changes.
pub(crate) const FHE_EXECUTE_FIXED_CPI_ACCOUNTS: usize = 9;

/// Heap bytes the crate's own invoke path requests *after* `build()`: the CPI account-meta and
/// account-info tables (`invoke`), and `resolve_accounts`'s three right-sized tables. An exact
/// function of the account counts the builder already knows, charged against
/// [`crate::cost::BUILD_HEAP_BUDGET_BYTES`] at `finish` — an execution admitted by `build()`
/// fits the whole instruction, not just its own construction. Kept honest by the invoke
/// measurement in `heap_budget`, which runs the real assembly under a counting allocator.
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
