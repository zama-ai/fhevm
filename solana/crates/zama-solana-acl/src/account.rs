//! An account as an RPC node returned it, for the verdicts off-chain readers share.

/// `11111111111111111111111111111111`, the System program.
pub const SYSTEM_PROGRAM_ID: [u8; 32] = [0; 32];

/// One fetched account: its owner program and its data.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AccountView<'a> {
    pub owner: &'a [u8; 32],
    pub data: &'a [u8],
}

impl AccountView<'_> {
    /// System-owned and empty: what a bare transfer to a derivable address leaves. Anyone can fund
    /// an address before the host program creates an account there, so such an account says
    /// nothing about host state and reads as absent.
    pub fn is_uninitialized(&self) -> bool {
        *self.owner == SYSTEM_PROGRAM_ID && self.data.is_empty()
    }
}

/// The account at an address, `None` when it is absent or [uninitialized](AccountView::is_uninitialized).
pub(crate) fn initialized(account: Option<AccountView<'_>>) -> Option<AccountView<'_>> {
    account.filter(|account| !account.is_uninitialized())
}
