use anchor_lang::prelude::*;

#[error_code]
pub enum AclError {
    #[msg("Provided account does not match the handle")]
    HandleMismatch,
    #[msg("Unauthorized program tried to change ACL registry")]
    UnauthorizedAccess,
}
