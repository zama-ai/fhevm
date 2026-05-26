//! App-facing helpers for building ZamaHost execution frames.
//!
//! Most applications should use [`execute`]. The [`frame`] module exposes the
//! structured builder for custom SDKs, and [`protocol`] re-exports the Anchor
//! ABI types used by `zama-host::execute_frame`.

pub mod frame;

pub mod protocol {
    pub use zama_host::{AclSubjectEntry, FheFrameAction, FheFrameStep, FheOpcode, FheOperand};
}

pub use frame::{execute, AuthorizedAppAccount, Context, DurableAllow, EncryptedValue};
