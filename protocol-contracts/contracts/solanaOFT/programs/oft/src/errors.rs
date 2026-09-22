use anchor_lang::prelude::error_code;

#[error_code]
pub enum OFTError {
    Unauthorized,
    InvalidSender,
    InvalidDecimals,
    SlippageExceeded,
    InvalidTokenDest,
    RateLimitExceeded,
    InvalidFee,
    InvalidMintAuthority,
    Paused,
}
