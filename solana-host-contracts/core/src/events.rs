use crate::types::{
    EvmAddress, FheType, Handle, KmsContextId, Operator, Pubkey, SignatureThreshold,
};
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Clone, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum HostEvent {
    Operation {
        caller: Pubkey,
        op: Operator,
        operands: Vec<[u8; 32]>,
        scalar_flag: Option<u8>,
        result_type: FheType,
        result: Handle,
    },
    VerifyInput {
        caller: Pubkey,
        input_handle: Handle,
        user_address: EvmAddress,
        input_proof_len: u32,
        input_type: FheType,
        result: Handle,
    },
    Allowed {
        caller: Pubkey,
        account: Pubkey,
        handle: Handle,
    },
    AllowedForDecryption {
        caller: Pubkey,
        handles: Vec<Handle>,
    },
    DelegatedForUserDecryption {
        delegator: Pubkey,
        delegate: Pubkey,
        contract_address: Pubkey,
        delegation_counter: u64,
        old_expiration_date: u64,
        new_expiration_date: u64,
    },
    RevokedDelegationForUserDecryption {
        delegator: Pubkey,
        delegate: Pubkey,
        contract_address: Pubkey,
        delegation_counter: u64,
        old_expiration_date: u64,
    },
    BlockedAccount {
        account: Pubkey,
    },
    UnblockedAccount {
        account: Pubkey,
    },
    InputVerifierContextUpdated {
        signers: Vec<EvmAddress>,
        threshold: SignatureThreshold,
    },
    KmsContextUpdated {
        kms_context_id: KmsContextId,
        signers: Vec<EvmAddress>,
        threshold: SignatureThreshold,
    },
    KmsContextDestroyed {
        kms_context_id: KmsContextId,
    },
    HcuPerBlockSet {
        hcu_per_block: u64,
    },
    MaxHcuDepthPerTxSet {
        max_hcu_depth_per_tx: u64,
    },
    MaxHcuPerTxSet {
        max_hcu_per_tx: u64,
    },
    BlockHcuWhitelistAdded {
        account: Pubkey,
    },
    BlockHcuWhitelistRemoved {
        account: Pubkey,
    },
}
