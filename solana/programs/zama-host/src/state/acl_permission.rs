//! On-chain account data for `AclPermission`.

use super::*;

/// Overflow subject witness for an [`AclRecord`].
///
/// The canonical address is `PDA("acl-permission", acl_record, subject)`.
/// KMS/Gateway requests that rely on overflow membership must carry this
/// account as an explicit witness.
#[account]
pub struct AclPermission {
    /// ACL record this permission extends.
    pub acl_record: Pubkey,
    /// Subject granted by this overflow permission.
    pub subject: Pubkey,
    /// Bitset of `ACL_ROLE_*` flags granted to `subject`.
    pub role_flags: u8,
    /// PDA bump for this permission account.
    pub bump: u8,
}

impl AclPermission {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = 32 + 32 + 1 + 1;
}
