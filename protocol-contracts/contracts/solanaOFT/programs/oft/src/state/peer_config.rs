use crate::*;

pub const ENFORCED_OPTIONS_SEND_MAX_LEN: usize = 512;
pub const ENFORCED_OPTIONS_SEND_AND_CALL_MAX_LEN: usize = 1024;

#[account]
#[derive(InitSpace)]
pub struct PeerConfig {
    pub peer_address: [u8; 32],
    pub enforced_options: EnforcedOptions,
    pub outbound_rate_limiter: Option<RateLimiter>,
    pub inbound_rate_limiter: Option<RateLimiter>,
    pub fee_bps: Option<u16>,
    pub bump: u8,
}

#[derive(Clone, Default, AnchorSerialize, AnchorDeserialize, InitSpace)]
pub struct RateLimiter {
    pub capacity: u64,
    pub tokens: u64,
    pub refill_per_second: u64,
    pub last_refill_time: u64,
}

impl RateLimiter {
    pub fn set_rate(&mut self, refill_per_second: u64) -> Result<()> {
        self.refill(0)?;
        self.refill_per_second = refill_per_second;
        Ok(())
    }

    pub fn set_capacity(&mut self, capacity: u64) -> Result<()> {
        self.capacity = capacity;
        self.tokens = capacity;
        self.last_refill_time = Clock::get()?.unix_timestamp.try_into().unwrap();
        Ok(())
    }

    pub fn refill(&mut self, extra_tokens: u64) -> Result<()> {
        let mut new_tokens = extra_tokens;
        let current_time: u64 = Clock::get()?.unix_timestamp.try_into().unwrap();
        if current_time > self.last_refill_time {
            let time_elapsed_in_seconds = current_time - self.last_refill_time;
            new_tokens = new_tokens
                .saturating_add(time_elapsed_in_seconds.saturating_mul(self.refill_per_second));
        }
        self.tokens = std::cmp::min(self.capacity, self.tokens.saturating_add(new_tokens));

        self.last_refill_time = current_time;
        Ok(())
    }

    pub fn try_consume(&mut self, amount: u64) -> Result<()> {
        self.refill(0)?;
        match self.tokens.checked_sub(amount) {
            Some(new_tokens) => {
                self.tokens = new_tokens;
                Ok(())
            },
            None => Err(error!(OFTError::RateLimitExceeded)),
        }
    }
}

#[derive(Clone, Default, AnchorSerialize, AnchorDeserialize, InitSpace)]
pub struct EnforcedOptions {
    #[max_len(ENFORCED_OPTIONS_SEND_MAX_LEN)]
    pub send: Vec<u8>,
    #[max_len(ENFORCED_OPTIONS_SEND_AND_CALL_MAX_LEN)]
    pub send_and_call: Vec<u8>,
}

impl EnforcedOptions {
    pub fn get_enforced_options(&self, composed_msg: &Option<Vec<u8>>) -> Vec<u8> {
        if composed_msg.is_none() {
            self.send.clone()
        } else {
            self.send_and_call.clone()
        }
    }

    pub fn combine_options(
        &self,
        compose_msg: &Option<Vec<u8>>,
        extra_options: &Vec<u8>,
    ) -> Result<Vec<u8>> {
        let enforced_options = self.get_enforced_options(compose_msg);
        oapp::options::combine_options(enforced_options, extra_options)
    }
}

utils::generate_account_size_test!(EnforcedOptions, enforced_options_test);
