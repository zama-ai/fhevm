//! Test stand-ins for the RFC-038 PRF endpoint spike. Not production code: the pad-worker writes a
//! keccak pad in place of the FHE PRF output, and the connector stands in for the kms-connector
//! retrieval client.

pub mod connector;
pub mod faults;
pub mod load;
pub mod pad_worker;
