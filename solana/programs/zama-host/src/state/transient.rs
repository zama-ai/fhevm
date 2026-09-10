use super::*;

pub const TRANSIENT_SEED: &[u8] = b"transient";
pub const MAX_TRANSIENT_GRANTS: usize = 32;
// Leave room for the header and grants within one System Program allocation.
pub const MAX_TRANSIENT_RESULTS: usize = 112;

#[zero_copy]
#[derive(Debug, PartialEq, Eq)]
pub struct TransientGrant {
    pub handle: [u8; 32],
    pub consumer_state: Pubkey,
}

#[zero_copy]
#[derive(Debug, PartialEq, Eq)]
pub struct TransientResult {
    pub handle: [u8; 32],
    pub producer_state: Pubkey,
    pub depth: u64,
}

/// Host-owned transaction context. Borrowed in place so nested executions do
/// not deserialize the growing result history onto Solana's bump heap.
#[account(zero_copy)]
pub struct TransientState {
    pub payer: Pubkey,
    pub total_hcu: u64,
    result_count: u16,
    grant_count: u16,
    pub bump: u8,
    reserved: [u8; 3],
    results: [TransientResult; MAX_TRANSIENT_RESULTS],
    grants: [TransientGrant; MAX_TRANSIENT_GRANTS],
}

impl TransientState {
    pub const SPACE: usize = 8 + std::mem::size_of::<Self>();

    pub fn validate(&self, address: Pubkey) -> Result<()> {
        let (expected, bump) = transient_address(self.payer);
        require_keys_eq!(address, expected, ZamaHostError::TransientAccountInvalid);
        require!(self.bump == bump, ZamaHostError::TransientAccountInvalid);
        require!(
            usize::from(self.result_count) <= MAX_TRANSIENT_RESULTS
                && usize::from(self.grant_count) <= MAX_TRANSIENT_GRANTS,
            ZamaHostError::TransientAccountInvalid
        );
        Ok(())
    }

    pub fn len(&self) -> usize {
        usize::from(self.result_count)
    }

    pub fn is_empty(&self) -> bool {
        self.result_count == 0
    }

    pub fn result(&self, index: usize) -> Option<&TransientResult> {
        self.results.get(..self.len())?.get(index)
    }

    /// None means the handle predates this transaction. Repeated occurrences
    /// retain the longest dependency path, regardless of which witness is used.
    pub fn origin_depth(&self, handle: [u8; 32]) -> Option<u64> {
        self.results[..self.len()]
            .iter()
            .filter(|result| result.handle == handle)
            .map(|result| result.depth)
            .max()
    }

    pub fn authorized_depth(&self, handle: [u8; 32], consumer_state: Pubkey) -> Option<u64> {
        let mut depth = None;
        let mut allowed = false;
        for result in self.results[..self.len()]
            .iter()
            .filter(|result| result.handle == handle)
        {
            depth = Some(depth.unwrap_or(0).max(result.depth));
            allowed |= result.producer_state == consumer_state;
        }
        allowed |= self.grants[..usize::from(self.grant_count)].contains(&TransientGrant {
            handle,
            consumer_state,
        });
        depth.filter(|_| allowed)
    }

    pub fn record(&mut self, handle: [u8; 32], producer_state: Pubkey, depth: u64) -> Result<()> {
        let index = self.len();
        require!(
            index < MAX_TRANSIENT_RESULTS,
            ZamaHostError::TransientCapacityExceeded
        );
        // Keep occurrences: call-local result indexes must survive a repeated handle.
        self.results[index] = TransientResult {
            handle,
            producer_state,
            depth,
        };
        self.result_count += 1;
        Ok(())
    }

    pub fn allow(&mut self, handle: [u8; 32], consumer_state: Pubkey) -> Result<()> {
        if self.authorized_depth(handle, consumer_state).is_some() {
            return Ok(());
        }
        let index = usize::from(self.grant_count);
        require!(
            index < MAX_TRANSIENT_GRANTS,
            ZamaHostError::TransientCapacityExceeded
        );
        self.grants[index] = TransientGrant {
            handle,
            consumer_state,
        };
        self.grant_count += 1;
        Ok(())
    }
}

pub fn transient_address(payer: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[TRANSIENT_SEED, payer.as_ref()], &crate::ID)
}

const _: () = assert!(TransientState::SPACE <= 10_240);
