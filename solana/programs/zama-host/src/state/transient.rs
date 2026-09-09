use super::*;

pub const TRANSIENT_SEED: &[u8] = b"transient";
pub const MAX_TRANSIENT_GRANTS: usize = 32;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct TransientGrant {
    pub handle: [u8; 32],
    pub consumer_state: Pubkey,
}

#[account]
pub struct TransientState {
    pub initiating_state: Pubkey,
    pub refund: Pubkey,
    pub grants: Vec<TransientGrant>,
    pub bump: u8,
}

impl TransientState {
    pub const SPACE: usize = 8 + 32 + 32 + 4 + 64 * MAX_TRANSIENT_GRANTS + 1;

    pub fn allow(&mut self, handle: [u8; 32], consumer_state: Pubkey) -> Result<()> {
        let grant = TransientGrant {
            handle,
            consumer_state,
        };
        if self.grants.contains(&grant) {
            return Ok(());
        }
        require!(
            self.grants.len() < MAX_TRANSIENT_GRANTS,
            ZamaHostError::TransientCapacityExceeded
        );
        self.grants.push(grant);
        Ok(())
    }

    pub fn allows(&self, handle: [u8; 32], consumer_state: Pubkey) -> bool {
        self.grants.contains(&TransientGrant {
            handle,
            consumer_state,
        })
    }
}

pub fn transient_address(initiating_state: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[TRANSIENT_SEED, initiating_state.as_ref()], &crate::ID)
}
