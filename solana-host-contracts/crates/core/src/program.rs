use crate::acl::{AclSession, AclState};
use crate::error::{HostContractError, Result};
use crate::events::HostEvent;
use crate::executor::ExecutorState;
use crate::hcu::{HcuLimitState, TransactionMeter};
use crate::input_verifier::{InputProofVerifier, InputVerifierSession, InputVerifierState};
use crate::instructions::{HostInstruction, HostProgramConfig, ProgramContext};
use crate::kms_verifier::{KmsProofVerifier, KmsVerifierState};
use crate::types::{Handle, Pubkey};
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct HostProgramSession {
    pub acl_session: AclSession,
    pub input_verifier_session: InputVerifierSession,
    pub tx_meter: TransactionMeter,
}

impl HostProgramSession {
    pub fn reset(&mut self) {
        self.acl_session.clear();
        self.input_verifier_session.clear();
        self.tx_meter = TransactionMeter::default();
    }

    pub fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct InstructionResult {
    pub events: Vec<HostEvent>,
    pub returned_handle: Option<Handle>,
    pub verified: Option<bool>,
}

impl InstructionResult {
    pub fn from_event(event: HostEvent) -> Self {
        Self {
            events: vec![event],
            returned_handle: None,
            verified: None,
        }
    }

    pub fn with_handle(handle: Handle, event: HostEvent) -> Self {
        Self {
            events: vec![event],
            returned_handle: Some(handle),
            verified: None,
        }
    }

    pub fn with_verification(verified: bool) -> Self {
        Self {
            events: Vec::new(),
            returned_handle: None,
            verified: Some(verified),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct HostProgramState {
    owner: Pubkey,
    upgrade_authority: Pubkey,
    host_chain_id: u64,
    state_version: u32,
    acl: AclState,
    executor: ExecutorState,
    input_verifier: InputVerifierState,
    kms_verifier: KmsVerifierState,
    hcu_limit: HcuLimitState,
}

impl HostProgramState {
    pub const CURRENT_STATE_VERSION: u32 = 1;

    pub fn new(config: HostProgramConfig) -> Result<Self> {
        Ok(Self {
            owner: config.owner,
            upgrade_authority: config.upgrade_authority,
            host_chain_id: config.host_chain_id,
            state_version: Self::CURRENT_STATE_VERSION,
            acl: AclState::new(config.owner, config.acl_program),
            executor: ExecutorState::new(config.acl_program),
            input_verifier: InputVerifierState::new(
                config.input_verifier.source_contract,
                config.input_verifier.source_chain_id,
                config.input_verifier.signers,
                config.input_verifier.threshold,
            )?,
            kms_verifier: KmsVerifierState::new(
                config.kms_verifier.source_contract,
                config.kms_verifier.source_chain_id,
                config.kms_verifier.signers,
                config.kms_verifier.threshold,
            )?,
            hcu_limit: HcuLimitState::new(
                config.hcu.hcu_cap_per_block,
                config.hcu.max_hcu_depth_per_tx,
                config.hcu.max_hcu_per_tx,
            )?,
        })
    }

    pub fn owner(&self) -> Pubkey {
        self.owner
    }

    pub fn upgrade_authority(&self) -> Pubkey {
        self.upgrade_authority
    }

    pub fn state_version(&self) -> u32 {
        self.state_version
    }

    pub fn host_chain_id(&self) -> u64 {
        self.host_chain_id
    }

    pub fn acl(&self) -> &AclState {
        &self.acl
    }

    pub fn input_verifier(&self) -> &InputVerifierState {
        &self.input_verifier
    }

    pub fn kms_verifier(&self) -> &KmsVerifierState {
        &self.kms_verifier
    }

    pub fn hcu_limit(&self) -> &HcuLimitState {
        &self.hcu_limit
    }

    pub fn executor(&self) -> &ExecutorState {
        &self.executor
    }

    pub fn process_instruction<IV, KV>(
        &mut self,
        instruction: &HostInstruction,
        context: ProgramContext,
        session: &mut HostProgramSession,
        input_proof_verifier: &IV,
        kms_proof_verifier: &KV,
    ) -> Result<InstructionResult>
    where
        IV: InputProofVerifier,
        KV: KmsProofVerifier,
    {
        match instruction {
            HostInstruction::AddPauser { account } => {
                self.ensure_owner(context.caller)?;
                self.acl.add_pauser(context.caller, *account)?;
                Ok(InstructionResult::default())
            }
            HostInstruction::ResetAclState => {
                self.ensure_owner(context.caller)?;
                self.acl.reset_runtime_state();
                Ok(InstructionResult::default())
            }
            HostInstruction::Pause => {
                self.acl.pause(context.caller)?;
                Ok(InstructionResult::default())
            }
            HostInstruction::Unpause => {
                self.ensure_owner(context.caller)?;
                self.acl.unpause(context.caller)?;
                Ok(InstructionResult::default())
            }
            HostInstruction::Allow { handle, account } => {
                let event =
                    self.acl
                        .allow(context.caller, *handle, *account, &session.acl_session)?;
                Ok(InstructionResult::from_event(event))
            }
            HostInstruction::AllowMany { handles, account } => {
                let event =
                    self.acl
                        .allow_many(context.caller, handles, *account, &session.acl_session)?;
                Ok(InstructionResult::from_event(event))
            }
            HostInstruction::AllowForDecryption { handles } => {
                let event =
                    self.acl
                        .allow_for_decryption(context.caller, handles, &session.acl_session)?;
                Ok(InstructionResult::from_event(event))
            }
            HostInstruction::DelegateForUserDecryption {
                delegate,
                contract_address,
                expiration_date,
            } => {
                let event = self.acl.delegate_for_user_decryption(
                    context.caller,
                    *delegate,
                    *contract_address,
                    *expiration_date,
                    context.timestamp as u64,
                    context.slot,
                )?;
                Ok(InstructionResult::from_event(event))
            }
            HostInstruction::RevokeDelegationForUserDecryption {
                delegate,
                contract_address,
            } => {
                let event = self.acl.revoke_delegation_for_user_decryption(
                    context.caller,
                    *delegate,
                    *contract_address,
                    context.slot,
                )?;
                Ok(InstructionResult::from_event(event))
            }
            HostInstruction::BlockAccount { account } => {
                self.ensure_owner(context.caller)?;
                let event = self.acl.block_account(context.caller, *account)?;
                Ok(InstructionResult::from_event(event))
            }
            HostInstruction::UnblockAccount { account } => {
                self.ensure_owner(context.caller)?;
                let event = self.acl.unblock_account(context.caller, *account)?;
                Ok(InstructionResult::from_event(event))
            }
            HostInstruction::DefineInputVerifierContext { signers, threshold } => {
                self.ensure_owner(context.caller)?;
                let event = self
                    .input_verifier
                    .define_new_context(signers, *threshold)?;
                Ok(InstructionResult::from_event(event))
            }
            HostInstruction::DefineKmsContext { signers, threshold } => {
                self.ensure_owner(context.caller)?;
                let event = self.kms_verifier.define_new_context(signers, *threshold)?;
                Ok(InstructionResult::from_event(event))
            }
            HostInstruction::DestroyKmsContext { kms_context_id } => {
                self.ensure_owner(context.caller)?;
                let event = self.kms_verifier.destroy_kms_context(*kms_context_id)?;
                Ok(InstructionResult::from_event(event))
            }
            HostInstruction::SetHcuPerBlock { hcu_per_block } => {
                self.ensure_owner(context.caller)?;
                let event = self.hcu_limit.set_hcu_per_block(*hcu_per_block)?;
                Ok(InstructionResult::from_event(event))
            }
            HostInstruction::SetMaxHcuDepthPerTx {
                max_hcu_depth_per_tx,
            } => {
                self.ensure_owner(context.caller)?;
                let event = self
                    .hcu_limit
                    .set_max_hcu_depth_per_tx(*max_hcu_depth_per_tx)?;
                Ok(InstructionResult::from_event(event))
            }
            HostInstruction::SetMaxHcuPerTx { max_hcu_per_tx } => {
                self.ensure_owner(context.caller)?;
                let event = self.hcu_limit.set_max_hcu_per_tx(*max_hcu_per_tx)?;
                Ok(InstructionResult::from_event(event))
            }
            HostInstruction::AddToBlockHcuWhitelist { account } => {
                self.ensure_owner(context.caller)?;
                let event = self.hcu_limit.add_to_block_hcu_whitelist(*account)?;
                Ok(InstructionResult::from_event(event))
            }
            HostInstruction::RemoveFromBlockHcuWhitelist { account } => {
                self.ensure_owner(context.caller)?;
                let event = self.hcu_limit.remove_from_block_hcu_whitelist(*account)?;
                Ok(InstructionResult::from_event(event))
            }
            HostInstruction::UnaryOp { op, ct, .. } => {
                self.process_unary_op(*op, *ct, context, session)
            }
            HostInstruction::BinaryOp {
                op,
                lhs,
                rhs,
                result_type,
                ..
            } => self.process_binary_op(*op, *lhs, *rhs, *result_type, context, session),
            HostInstruction::TernaryOp {
                op,
                control,
                if_true,
                if_false,
                ..
            } => self.process_ternary_op(*op, *control, *if_true, *if_false, context, session),
            HostInstruction::Cast { ct, to_type, .. } => {
                self.process_cast(*ct, *to_type, context, session)
            }
            HostInstruction::TrivialEncrypt {
                plaintext, to_type, ..
            } => self.process_trivial_encrypt(*plaintext, *to_type, context, session),
            HostInstruction::FheRand { rand_type, .. } => {
                self.process_fhe_rand(*rand_type, context, session)
            }
            HostInstruction::FheRandBounded {
                upper_bound,
                rand_type,
                ..
            } => self.process_fhe_rand_bounded(*upper_bound, *rand_type, context, session),
            HostInstruction::VerifyInput {
                context: user_context,
                input_handle,
                input_proof,
            } => self.process_verify_input(
                *user_context,
                *input_handle,
                input_proof,
                context,
                session,
                input_proof_verifier,
            ),
            HostInstruction::VerifyDecryptionSignatures {
                handles_list,
                decrypted_result,
                decryption_proof,
            } => self.process_verify_decryption_signatures(
                handles_list,
                decrypted_result,
                decryption_proof,
                kms_proof_verifier,
            ),
            HostInstruction::CleanTransientStorage => {
                session.reset();
                Ok(InstructionResult::default())
            }
            HostInstruction::Migrate { new_state_version } => {
                self.ensure_upgrade_authority(context.caller)?;
                if *new_state_version <= self.state_version {
                    return Err(HostContractError::InvalidStateVersionTransition {
                        current: self.state_version,
                        requested: *new_state_version,
                    });
                }
                self.state_version = *new_state_version;
                Ok(InstructionResult::default())
            }
        }
    }

    fn ensure_owner(&self, caller: Pubkey) -> Result<()> {
        if caller != self.owner {
            return Err(HostContractError::SenderNotAllowed);
        }
        Ok(())
    }

    fn ensure_upgrade_authority(&self, caller: Pubkey) -> Result<()> {
        if caller != self.upgrade_authority {
            return Err(HostContractError::NotUpgradeAuthority);
        }
        Ok(())
    }

    #[inline(never)]
    fn process_unary_op(
        &mut self,
        op: crate::types::Operator,
        ct: Handle,
        context: ProgramContext,
        session: &mut HostProgramSession,
    ) -> Result<InstructionResult> {
        let (handle, event) = self.executor.unary_op(
            op,
            ct,
            context.execution_meta(),
            &self.acl,
            &mut session.acl_session,
            Some((&mut self.hcu_limit, &mut session.tx_meter)),
        )?;
        Ok(InstructionResult::with_handle(handle, event))
    }

    #[inline(never)]
    fn process_binary_op(
        &mut self,
        op: crate::types::Operator,
        lhs: Handle,
        rhs: crate::executor::BinaryOperand,
        result_type: crate::types::FheType,
        context: ProgramContext,
        session: &mut HostProgramSession,
    ) -> Result<InstructionResult> {
        let (handle, event) = self.executor.binary_op(
            op,
            lhs,
            rhs,
            context.execution_meta(),
            &self.acl,
            &mut session.acl_session,
            result_type,
            Some((&mut self.hcu_limit, &mut session.tx_meter)),
        )?;
        Ok(InstructionResult::with_handle(handle, event))
    }

    #[inline(never)]
    fn process_ternary_op(
        &mut self,
        op: crate::types::Operator,
        control: Handle,
        if_true: Handle,
        if_false: Handle,
        context: ProgramContext,
        session: &mut HostProgramSession,
    ) -> Result<InstructionResult> {
        let (handle, event) = self.executor.ternary_op(
            op,
            control,
            if_true,
            if_false,
            context.execution_meta(),
            &self.acl,
            &mut session.acl_session,
            Some((&mut self.hcu_limit, &mut session.tx_meter)),
        )?;
        Ok(InstructionResult::with_handle(handle, event))
    }

    #[inline(never)]
    fn process_cast(
        &mut self,
        ct: Handle,
        to_type: crate::types::FheType,
        context: ProgramContext,
        session: &mut HostProgramSession,
    ) -> Result<InstructionResult> {
        let (handle, event) = self.executor.cast(
            ct,
            to_type,
            context.execution_meta(),
            &self.acl,
            &mut session.acl_session,
            Some((&mut self.hcu_limit, &mut session.tx_meter)),
        )?;
        Ok(InstructionResult::with_handle(handle, event))
    }

    #[inline(never)]
    fn process_trivial_encrypt(
        &mut self,
        plaintext: [u8; 32],
        to_type: crate::types::FheType,
        context: ProgramContext,
        session: &mut HostProgramSession,
    ) -> Result<InstructionResult> {
        let (handle, event) = self.executor.trivial_encrypt(
            plaintext,
            to_type,
            context.execution_meta(),
            &self.acl,
            &mut session.acl_session,
            Some((&mut self.hcu_limit, &mut session.tx_meter)),
        )?;
        Ok(InstructionResult::with_handle(handle, event))
    }

    #[inline(never)]
    fn process_fhe_rand(
        &mut self,
        rand_type: crate::types::FheType,
        context: ProgramContext,
        session: &mut HostProgramSession,
    ) -> Result<InstructionResult> {
        let (handle, event) = self.executor.fhe_rand(
            rand_type,
            context.execution_meta(),
            &self.acl,
            &mut session.acl_session,
            Some((&mut self.hcu_limit, &mut session.tx_meter)),
        )?;
        Ok(InstructionResult::with_handle(handle, event))
    }

    #[inline(never)]
    fn process_fhe_rand_bounded(
        &mut self,
        upper_bound: [u8; 32],
        rand_type: crate::types::FheType,
        context: ProgramContext,
        session: &mut HostProgramSession,
    ) -> Result<InstructionResult> {
        let (handle, event) = self.executor.fhe_rand_bounded(
            upper_bound,
            rand_type,
            context.execution_meta(),
            &self.acl,
            &mut session.acl_session,
            Some((&mut self.hcu_limit, &mut session.tx_meter)),
        )?;
        Ok(InstructionResult::with_handle(handle, event))
    }

    #[inline(never)]
    fn process_verify_input<IV: InputProofVerifier>(
        &mut self,
        user_context: crate::types::ContextUserInputs,
        input_handle: Handle,
        input_proof: &[u8],
        context: ProgramContext,
        session: &mut HostProgramSession,
        input_proof_verifier: &IV,
    ) -> Result<InstructionResult> {
        let (handle, event) = self.executor.verify_input(
            &mut self.input_verifier,
            &mut session.input_verifier_session,
            input_proof_verifier,
            user_context,
            input_handle,
            input_proof,
            context.execution_meta(),
            &self.acl,
            &mut session.acl_session,
        )?;
        Ok(InstructionResult::with_handle(handle, event))
    }

    #[inline(never)]
    fn process_verify_decryption_signatures<KV: KmsProofVerifier>(
        &self,
        handles_list: &[Handle],
        decrypted_result: &[u8],
        decryption_proof: &[u8],
        kms_proof_verifier: &KV,
    ) -> Result<InstructionResult> {
        Ok(InstructionResult::with_verification(
            self.kms_verifier.verify_decryption_signatures(
                handles_list,
                decrypted_result,
                decryption_proof,
                kms_proof_verifier,
            )?,
        ))
    }
}
