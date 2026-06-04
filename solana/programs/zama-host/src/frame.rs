use anchor_lang::prelude::*;

use crate::acl::{
    assert_canonical_acl_record_data, assert_output_acl_metadata, assert_record_allows_handle,
    create_acl_record_account, deserialize_acl_record, serialize_acl_record, write_acl_record,
};
use crate::handles::{FHE_TYPE_BOOL, computed_ternary_handle};
use crate::{FheTernaryOpCode, FheTernaryOpEvent, handles};
use crate::{
    AclAllowedEvent, AclPublicDecryptAllowedEvent, FheBinaryOpCode, FheBinaryOpEvent,
    FheFrameAction, FheFrameStep, FheOpcode, FheOperand, FheRandEvent, InputVerifiedEvent,
    RandCounter, TrivialEncryptEvent, ZamaHostError, EVENT_VERSION, MAX_FRAME_RESULTS,
    MAX_FRAME_TRANSIENT_ALLOWS, SOLANA_POC_CHAIN_ID,
};

struct ExecutionFrame {
    subject: Pubkey,
    results: Vec<[u8; 32]>,
    transient_allows: Vec<[u8; 32]>,
    events: Vec<FrameEvent>,
}

pub enum FrameEvent {
    BinaryOp(FheBinaryOpEvent),
    TrivialEncrypt(TrivialEncryptEvent),
    Rand(FheRandEvent),
    AclAllowed(AclAllowedEvent),
    AclPublicDecryptAllowed(AclPublicDecryptAllowedEvent),
    InputVerified(InputVerifiedEvent),
    TernaryOp(FheTernaryOpEvent)
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

    fn push_result(&mut self, handle: [u8; 32]) -> Result<()> {
        require!(
            self.results.len() < MAX_FRAME_RESULTS,
            ZamaHostError::FrameLimitExceeded
        );
        self.results.push(handle);
        self.allow_transient(handle)
    }

    fn allow_transient(&mut self, handle: [u8; 32]) -> Result<()> {
        require!(
            self.transient_allows.len() < MAX_FRAME_TRANSIENT_ALLOWS,
            ZamaHostError::FrameLimitExceeded
        );
        // A frame has a single compute subject, so every transient allow is for
        // `self.subject`; we only track which result handles are frame-local.
        self.transient_allows.push(handle);
        Ok(())
    }

    fn is_transiently_allowed(&self, handle: [u8; 32]) -> bool {
        self.transient_allows.contains(&handle)
    }
}

pub fn execute<'info>(
    subject: Pubkey,
    authorized_app_accounts: &[Pubkey],
    remaining_accounts: &[AccountInfo<'info>],
    payer: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
    rand_counter: &mut RandCounter,
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
            rand_counter,
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
    rand_counter: &mut RandCounter,
    step: FheFrameStep,
    previous_bank_hash: [u8; 32],
    unix_timestamp: i64,
) -> Result<()> {
    match step {
        FheFrameStep::Input {
            input_handle,
            user,
            app_account,
            acl_domain_key,
            proof,
            fhe_type,
        } => {
            let expected = handles::poc_external_input_proof(
                input_handle,
                user,
                app_account,
                acl_domain_key,
                fhe_type,
                SOLANA_POC_CHAIN_ID,
            );
            require!(proof == expected, ZamaHostError::InvalidInputProof);
            frame.push_result(input_handle)?;
            frame
                .events
                .push(FrameEvent::InputVerified(InputVerifiedEvent {
                    version: EVENT_VERSION,
                    input_handle,
                    result_handle: input_handle,
                    user: user.to_bytes(),
                    acl_domain_key: acl_domain_key.to_bytes(),
                    app_account: app_account.to_bytes(),
                    fhe_type,
                }));
            Ok(())
        }
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
            frame.push_result(result)?;
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
        FheFrameStep::Rand { fhe_type } => {
            handles::ensure_supported_rand_fhe_type(fhe_type)?;
            let seed = handles::computed_rand_seed(
                rand_counter.counter,
                previous_bank_hash,
                SOLANA_POC_CHAIN_ID,
                unix_timestamp,
            );
            rand_counter.counter = rand_counter
                .counter
                .checked_add(1)
                .ok_or(ZamaHostError::RandCounterOverflow)?;
            let result = handles::computed_rand_handle(fhe_type, seed, SOLANA_POC_CHAIN_ID);
            frame.push_result(result)?;
            frame.events.push(FrameEvent::Rand(FheRandEvent {
                version: EVENT_VERSION,
                subject: frame.subject.to_bytes(),
                seed,
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
    if let Ok(ternary_op) = FheTernaryOpCode::try_from(opcode) {
        require!(operands.len() == 3, ZamaHostError::InvalidFrameOperands);
        let ls = resolve_operand(frame, remaining_accounts, &operands[0])?;
        let ms = resolve_operand(frame, remaining_accounts, &operands[1])?;
        let rs = resolve_operand(frame, remaining_accounts, &operands[2])?;

        require!(!ls.is_scalar, ZamaHostError::InvalidFrameOperands);
        require!(!ms.is_scalar, ZamaHostError::InvalidFrameOperands);
        require!(!rs.is_scalar, ZamaHostError::InvalidFrameOperands);

        require!(scalar_byte == 0, ZamaHostError::InvalidFrameOperands);

        require!(ls.value[30] == FHE_TYPE_BOOL, ZamaHostError::UnsupportedFheType);
  
        require!(ms.value[30] == rs.value[30], ZamaHostError::InvalidFrameOperands);
        require!(ms.value[30] == output_fhe_type, ZamaHostError::InvalidFrameOperands);

        let result = computed_ternary_handle(
            ternary_op, 
            ls.value, 
            ms.value, 
            rs.value, 
            output_fhe_type, 
            SOLANA_POC_CHAIN_ID, 
            previous_bank_hash, 
            unix_timestamp
        );

        frame.push_result(result)?;
        frame.events.push(FrameEvent::TernaryOp(FheTernaryOpEvent {
            version: EVENT_VERSION,
            op: ternary_op,
            subject: frame.subject.to_bytes(),
            ls: ls.value,
            ms: ms.value,
            rs: rs.value,
            result,
        }));

        return Ok(());
    }
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
    // TODO: validate the operand FHE type against the opcode (e.g. forbid
    // arithmetic add/sub on bools / non-integer types) once the PoC settles on
    // the supported type matrix.

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
    frame.push_result(result)?;
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
            assert_app_account_authorized(
                app_account,
                authorized_app_accounts,
                frame.subject,
                acl_domain_key,
                &payer,
                remaining_accounts,
            )?;
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
            let bump = create_acl_record_account(
                &payer,
                &output_acl_record,
                &system_program,
                nonce_key,
                nonce_sequence,
            )?;
            write_acl_record(
                &output_acl_record,
                bump,
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
        FheFrameAction::AllowForDecryption { source, acl_record } => {
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
            frame.events.push(FrameEvent::AclPublicDecryptAllowed(
                AclPublicDecryptAllowedEvent {
                    version: EVENT_VERSION,
                    handle: source.value,
                    subject: frame.subject.to_bytes(),
                },
            ));
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
        FheOperand::AclRecord { handle, acl_record } => {
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
            let handle = *frame
                .results
                .get(*index as usize)
                .ok_or_else(|| error!(ZamaHostError::FrameResultIndexOutOfRange))?;
            require!(
                frame.is_transiently_allowed(handle),
                ZamaHostError::AclSubjectMismatch
            );
            Ok(ResolvedOperand {
                value: handle,
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

fn assert_app_account_authorized(
    app_account: Pubkey,
    authorized: &[Pubkey],
    subject: Pubkey,
    acl_domain_key: Pubkey,
    payer: &AccountInfo<'_>,
    remaining_accounts: &[AccountInfo<'_>],
) -> Result<()> {
    require!(
        authorized.contains(&app_account),
        ZamaHostError::UnauthorizedAppAccount
    );

    if payer.key() == app_account && payer.is_signer {
        return Ok(());
    }

    let app_account_info = account_by_pubkey(remaining_accounts, app_account)?;
    if app_account_info.is_signer {
        return Ok(());
    }

    let (expected_subject, _) = Pubkey::find_program_address(
        &[b"fhe-compute", acl_domain_key.as_ref()],
        app_account_info.owner,
    );
    require!(
        subject == expected_subject,
        ZamaHostError::UnauthorizedAppAccount
    );
    Ok(())
}
