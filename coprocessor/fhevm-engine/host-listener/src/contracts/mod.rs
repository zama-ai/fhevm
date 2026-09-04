//! Host-contract ABIs used by the listener.
//!
//! These names (`AclContract`, `TfheContract`, `BridgeContract`) are the
//! historical aliases from `sol!(AclContract, "ACL.json")`. The ABI now
//! comes from `fhevm_host_bindings`; this module only preserves the old
//! paths so call sites do not rename `AclContractEvents` → `ACLEvents`.
#![allow(non_snake_case)]

pub use fhevm_host_bindings::kms_generation::KMSGeneration;
pub use fhevm_host_bindings::protocol_config::ProtocolConfig;

pub mod AclContract {
    pub use fhevm_host_bindings::acl::ACL::ACLEvents as AclContractEvents;
    pub use fhevm_host_bindings::acl::ACL::*;
}

pub mod TfheContract {
    pub use fhevm_host_bindings::fhevm_executor::FHEVMExecutor::FHEVMExecutorEvents as TfheContractEvents;
    pub use fhevm_host_bindings::fhevm_executor::FHEVMExecutor::*;
}

pub mod BridgeContract {
    pub use fhevm_host_bindings::bridge_events::BridgeEvents::BridgeEventsEvents as BridgeContractEvents;
    pub use fhevm_host_bindings::bridge_events::BridgeEvents::*;
}
