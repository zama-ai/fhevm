//! Stack-local manifest consensus_epoch selection.
//!
//! The selector resolves through the service search path: Blue reads the
//! public singleton and Green reads its GCS copy. All manifest data tables are
//! shared in `public` and isolate their rows with this selected consensus_epoch.

pub(crate) mod active;
