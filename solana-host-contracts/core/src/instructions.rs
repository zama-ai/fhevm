use crate::executor::{BinaryOperand, ExecutionMeta};
use crate::types::{
    ContextUserInputs, EvmAddress, FheType, Handle, KmsContextId, Operator, Pubkey,
    SignatureThreshold,
};
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProgramContext {
    pub caller: Pubkey,
    pub chain_id: u64,
    pub slot: u64,
    pub timestamp: i64,
    pub recent_blockhash: [u8; 32],
}

impl ProgramContext {
    pub fn execution_meta(self) -> ExecutionMeta {
        ExecutionMeta {
            chain_id: self.chain_id,
            slot: self.slot,
            timestamp: self.timestamp,
            recent_blockhash: self.recent_blockhash,
            caller: self.caller,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct VerifierContextConfig {
    pub source_contract: EvmAddress,
    pub source_chain_id: u64,
    pub signers: Vec<EvmAddress>,
    pub threshold: SignatureThreshold,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct HcuConfig {
    pub hcu_cap_per_block: u64,
    pub max_hcu_depth_per_tx: u64,
    pub max_hcu_per_tx: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct HostProgramConfig {
    pub owner: Pubkey,
    pub upgrade_authority: Pubkey,
    pub acl_program: Pubkey,
    pub host_chain_id: u64,
    pub input_verifier: VerifierContextConfig,
    pub kms_verifier: VerifierContextConfig,
    pub hcu: HcuConfig,
}

#[derive(Clone, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum HostInstruction {
    AddPauser {
        account: Pubkey,
    },
    Pause,
    Unpause,
    Allow {
        handle: Handle,
        account: Pubkey,
    },
    AllowForDecryption {
        handles: Vec<Handle>,
    },
    DelegateForUserDecryption {
        delegate: Pubkey,
        contract_address: Pubkey,
        expiration_date: u64,
    },
    RevokeDelegationForUserDecryption {
        delegate: Pubkey,
        contract_address: Pubkey,
    },
    BlockAccount {
        account: Pubkey,
    },
    UnblockAccount {
        account: Pubkey,
    },
    DefineInputVerifierContext {
        signers: Vec<EvmAddress>,
        threshold: SignatureThreshold,
    },
    DefineKmsContext {
        signers: Vec<EvmAddress>,
        threshold: SignatureThreshold,
    },
    DestroyKmsContext {
        kms_context_id: KmsContextId,
    },
    SetHcuPerBlock {
        hcu_per_block: u64,
    },
    SetMaxHcuDepthPerTx {
        max_hcu_depth_per_tx: u64,
    },
    SetMaxHcuPerTx {
        max_hcu_per_tx: u64,
    },
    AddToBlockHcuWhitelist {
        account: Pubkey,
    },
    RemoveFromBlockHcuWhitelist {
        account: Pubkey,
    },
    UnaryOp {
        op: Operator,
        ct: Handle,
        charge_hcu: bool,
    },
    BinaryOp {
        op: Operator,
        lhs: Handle,
        rhs: BinaryOperand,
        result_type: FheType,
        charge_hcu: bool,
    },
    TernaryOp {
        op: Operator,
        control: Handle,
        if_true: Handle,
        if_false: Handle,
        charge_hcu: bool,
    },
    Cast {
        ct: Handle,
        to_type: FheType,
        charge_hcu: bool,
    },
    TrivialEncrypt {
        plaintext: [u8; 32],
        to_type: FheType,
        charge_hcu: bool,
    },
    FheRand {
        rand_type: FheType,
        charge_hcu: bool,
    },
    FheRandBounded {
        upper_bound: [u8; 32],
        rand_type: FheType,
        charge_hcu: bool,
    },
    VerifyInput {
        context: ContextUserInputs,
        input_handle: Handle,
        input_proof: Vec<u8>,
    },
    VerifyDecryptionSignatures {
        handles_list: Vec<Handle>,
        decrypted_result: Vec<u8>,
        decryption_proof: Vec<u8>,
    },
    CleanTransientStorage,
    Migrate {
        new_state_version: u32,
    },
}
