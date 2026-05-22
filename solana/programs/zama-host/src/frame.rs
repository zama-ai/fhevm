use anchor_lang::prelude::*;

use crate::acl::{
    assert_canonical_acl_record_data, assert_output_acl_metadata, assert_record_allows_handle,
    create_acl_record_account, deserialize_acl_record, serialize_acl_record,
    write_acl_record_data,
};
use crate::handles;
use crate::{
    AclAllowedEvent, FheBinaryOpCode, FheBinaryOpEvent, FheFrameAction, FheFrameStep, FheOperand,
    FheOpcode, TrivialEncryptEvent, EVENT_VERSION, MAX_FRAME_RESULTS, MAX_FRAME_TRANSIENT_ALLOWS,
    SOLANA_POC_CHAIN_ID, ZamaHostError,
};

#[derive(Clone, Copy)]
struct FrameResult {
    handle: [u8; 32],
}

struct ExecutionFrame {
    subject: Pubkey,
    results: Vec<FrameResult>,
    transient_allows: Vec<([u8; 32], Pubkey)>,
    events: Vec<FrameEvent>,
}

pub enum FrameEvent {
    BinaryOp(FheBinaryOpEvent),
    TrivialEncrypt(TrivialEncryptEvent),
    AclAllowed(AclAllowedEvent),
}

#[derive(Clone, Copy)]
struct ResolvedOperand {
    value: [u8; 32],
    is_scalar: bool,
}

impl ExecutionFrame {
    fn new(subject: Pubkey) -> Self {
        Self {
            subject,
            results: Vec::new(),
            transient_allows: Vec::new(),
            events: Vec::new(),
        }
    }

    fn push_result(&mut self, result: FrameResult) -> Result<()> {
        require!(
            self.results.len() < MAX_FRAME_RESULTS,
            ZamaHostError::FrameLimitExceeded
        );
        self.results.push(result);
        self.allow_transient(result.handle)
    }

    fn allow_transient(&mut self, handle: [u8; 32]) -> Result<()> {
        require!(
            self.transient_allows.len() < MAX_FRAME_TRANSIENT_ALLOWS,
            ZamaHostError::FrameLimitExceeded
        );
        self.transient_allows.push((handle, self.subject));
        Ok(())
    }

    fn transient_allows(&self, handle: [u8; 32], subject: Pubkey) -> bool {
        self.transient_allows
            .iter()
            .any(|(allowed_handle, allowed_subject)| {
                *allowed_handle == handle && *allowed_subject == subject
            })
    }
}

pub fn execute<'info>(
    subject: Pubkey,
    authorized_app_accounts: &[Pubkey],
    remaining_accounts: &[AccountInfo<'info>],
    payer: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
    steps: Vec<FheFrameStep>,
    actions: Vec<FheFrameAction>,
    previous_bank_hash: [u8; 32],
    unix_timestamp: i64,
) -> Result<Vec<FrameEvent>> {
    let mut frame = ExecutionFrame::new(subject);

    for step in steps {
        execute_frame_step(
            &mut frame,
            remaining_accounts,
            step,
            previous_bank_hash,
            unix_timestamp,
        )?;
    }

    for action in actions {
        apply_frame_action(
            &mut frame,
            authorized_app_accounts,
            payer.clone(),
            system_program.clone(),
            remaining_accounts,
            action,
        )?;
    }

    Ok(frame.events)
}

fn execute_frame_step<'info>(
    frame: &mut ExecutionFrame,
    remaining_accounts: &[AccountInfo<'info>],
    step: FheFrameStep,
    previous_bank_hash: [u8; 32],
    unix_timestamp: i64,
) -> Result<()> {
    match step {
        FheFrameStep::Operation {
            opcode,
            operands,
            scalar_byte,
            output_fhe_type,
        } => execute_operation_step(
            frame,
            remaining_accounts,
            opcode,
            operands,
            scalar_byte,
            output_fhe_type,
            previous_bank_hash,
            unix_timestamp,
        ),
        FheFrameStep::TrivialEncrypt {
            plaintext,
            fhe_type,
        } => {
            let result = handles::computed_trivial_handle(
                plaintext,
                fhe_type,
                SOLANA_POC_CHAIN_ID,
                previous_bank_hash,
                unix_timestamp,
            );
            frame.push_result(FrameResult { handle: result })?;
            frame
                .events
                .push(FrameEvent::TrivialEncrypt(TrivialEncryptEvent {
                    version: EVENT_VERSION,
                    subject: frame.subject.to_bytes(),
                    plaintext,
                    fhe_type,
                    result,
                }));
            Ok(())
        }
    }
}

fn execute_operation_step<'info>(
    frame: &mut ExecutionFrame,
    remaining_accounts: &[AccountInfo<'info>],
    opcode: FheOpcode,
    operands: Vec<FheOperand>,
    scalar_byte: u8,
    output_fhe_type: u8,
    previous_bank_hash: [u8; 32],
    unix_timestamp: i64,
) -> Result<()> {
    let binary_op = FheBinaryOpCode::try_from(opcode)?;
    require!(operands.len() == 2, ZamaHostError::InvalidFrameOperands);
    let lhs = resolve_operand(frame, remaining_accounts, &operands[0])?;
    let rhs = resolve_operand(frame, remaining_accounts, &operands[1])?;
    require!(!lhs.is_scalar, ZamaHostError::InvalidFrameOperands);
    let scalar = rhs.is_scalar;
    require!(
        scalar_byte == u8::from(scalar),
        ZamaHostError::InvalidFrameOperands
    );

    let result = handles::computed_binary_handle(
        binary_op,
        lhs.value,
        rhs.value,
        scalar,
        output_fhe_type,
        SOLANA_POC_CHAIN_ID,
        previous_bank_hash,
        unix_timestamp,
    );
    frame.push_result(FrameResult { handle: result })?;
    frame.events.push(FrameEvent::BinaryOp(FheBinaryOpEvent {
        version: EVENT_VERSION,
        op: binary_op,
        subject: frame.subject.to_bytes(),
        lhs: lhs.value,
        rhs: rhs.value,
        scalar,
        result,
    }));
    Ok(())
}

fn apply_frame_action<'info>(
    frame: &mut ExecutionFrame,
    authorized_app_accounts: &[Pubkey],
    payer: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
    remaining_accounts: &[AccountInfo<'info>],
    action: FheFrameAction,
) -> Result<()> {
    match action {
        FheFrameAction::Allow {
            source,
            output_acl_record,
            nonce_key,
            nonce_sequence,
            acl_domain_key,
            app_account,
            encrypted_value_label,
            subjects,
            public_decrypt,
        } => {
            assert_app_account_authorized(app_account, authorized_app_accounts)?;
            let source = resolve_operand(frame, remaining_accounts, &source)?;
            require!(!source.is_scalar, ZamaHostError::InvalidFrameOperands);
            assert_output_acl_metadata(
                nonce_key,
                acl_domain_key,
                app_account,
                encrypted_value_label,
                &subjects,
            )?;
            let output_acl_record = account_by_pubkey(remaining_accounts, output_acl_record)?;
            create_acl_record_account(
                &payer,
                &output_acl_record,
                &system_program,
                nonce_key,
                nonce_sequence,
            )?;
            write_acl_record_data(
                &output_acl_record,
                nonce_key,
                nonce_sequence,
                acl_domain_key,
                app_account,
                encrypted_value_label,
                source.value,
                &subjects,
                public_decrypt,
            )?;
            for subject in subjects {
                frame.events.push(FrameEvent::AclAllowed(AclAllowedEvent {
                    version: EVENT_VERSION,
                    handle: source.value,
                    subject: subject.pubkey.to_bytes(),
                }));
            }
            Ok(())
        }
        FheFrameAction::AllowForDecryption {
            source,
            acl_record,
        } => {
            let source = resolve_operand(frame, remaining_accounts, &source)?;
            require!(!source.is_scalar, ZamaHostError::InvalidFrameOperands);
            let acl_record_info = account_by_pubkey(remaining_accounts, acl_record)?;
            let mut record = deserialize_acl_record(&acl_record_info)?;
            assert_canonical_acl_record_data(acl_record_info.key(), &record)?;
            require!(
                record.handle == source.value,
                ZamaHostError::AclHandleMismatch
            );
            record.public_decrypt = true;
            serialize_acl_record(&acl_record_info, &record)?;
            frame.events.push(FrameEvent::AclAllowed(AclAllowedEvent {
                version: EVENT_VERSION,
                handle: source.value,
                subject: frame.subject.to_bytes(),
            }));
            Ok(())
        }
    }
}

fn resolve_operand<'info>(
    frame: &ExecutionFrame,
    remaining_accounts: &[AccountInfo<'info>],
    operand: &FheOperand,
) -> Result<ResolvedOperand> {
    match operand {
        FheOperand::AclRecord {
            handle,
            acl_record,
        } => {
            let record_info = account_by_pubkey(remaining_accounts, *acl_record)?;
            let record = deserialize_acl_record(&record_info)?;
            assert_canonical_acl_record_data(record_info.key(), &record)?;
            assert_record_allows_handle(&record, *handle, frame.subject)?;
            Ok(ResolvedOperand {
                value: *handle,
                is_scalar: false,
            })
        }
        FheOperand::PreviousResult { index } => {
            let result = frame
                .results
                .get(*index as usize)
                .ok_or_else(|| error!(ZamaHostError::FrameResultIndexOutOfRange))?;
            require!(
                frame.transient_allows(result.handle, frame.subject),
                ZamaHostError::AclSubjectMismatch
            );
            Ok(ResolvedOperand {
                value: result.handle,
                is_scalar: false,
            })
        }
        FheOperand::Scalar { value, .. } => Ok(ResolvedOperand {
            value: *value,
            is_scalar: true,
        }),
    }
}

fn account_by_pubkey<'info>(
    remaining_accounts: &[AccountInfo<'info>],
    pubkey: Pubkey,
) -> Result<AccountInfo<'info>> {
    remaining_accounts
        .iter()
        .find(|account| account.key() == pubkey)
        .cloned()
        .ok_or_else(|| error!(ZamaHostError::FrameAccountMissing))
}

fn assert_app_account_authorized(app_account: Pubkey, authorized: &[Pubkey]) -> Result<()> {
    require!(
        authorized.contains(&app_account),
        ZamaHostError::UnauthorizedAppAccount
    );
    Ok(())
}
