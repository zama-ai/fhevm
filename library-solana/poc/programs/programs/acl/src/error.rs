use anchor_lang::prelude::*;

#[error_code]
pub enum AclError {
    #[msg("Provided account does not match the handle")]
    HandleMismatch,
    #[msg("Unauthorized program tried to change ACL registry")]
    UnauthorizedAccess,
    #[msg("Handler is not set in ACL record")]
    HandleNotReady,
    #[msg("Handler has reached maximum amount of handles")]
    HandleOverflow,
    #[msg("Handle authorization request failed")]
    HandleAuthorizationFailed,
    #[msg("Can't allow default key")]
    DefaultKeyAllow,
    #[msg("Default key is not allowed")]
    DefaultKeyNotAllowed,
}
