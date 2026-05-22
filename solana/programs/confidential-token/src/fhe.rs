//! Build symbolic `execute_frame` programs for the host.
//!
//! Computed handles are opaque and assigned only inside `zama-host::execute_frame`.
//! This module records operands and step indices — never precomputes result handles.

use anchor_lang::prelude::*;
use zama_host::{
    cpi, cpi::accounts::ExecuteFrame, program::ZamaHost, AclSubjectEntry, FheFrameAction,
    FheFrameStep, FheOpcode, FheOperand,
};

use crate::ConfidentialTokenError;

#[derive(Clone)]
pub struct EncryptedValue<'info> {
    pub handle: [u8; 32],
    pub acl_record: AccountInfo<'info>,
}

/// Symbolic frame value: operand reference only. Frame results have no handle until the host runs.
#[derive(Clone)]
pub struct FheValue {
    operand: FheOperand,
}

pub struct Context<'a, 'info> {
    pub payer: &'a Signer<'info>,
    pub event_authority: &'a UncheckedAccount<'info>,
    pub zama_program: &'a Program<'info, ZamaHost>,
    pub compute_signer: &'a UncheckedAccount<'info>,
    pub acl_domain_key: Pubkey,
    pub compute_signer_bump: u8,
    pub system_program: &'a Program<'info, System>,
}

pub struct DurableAllow<'info> {
    pub acl_record: AccountInfo<'info>,
    pub app_account: Pubkey,
    pub nonce_key: [u8; 32],
    pub nonce_sequence: u64,
    pub encrypted_value_label: [u8; 32],
    pub subjects: Vec<AclSubjectEntry>,
    pub public_decrypt: bool,
}

pub fn execute<'info, R>(
    ctx: Context<'_, 'info>,
    f: impl FnOnce(&mut Builder<'_, 'info>) -> Result<R>,
) -> Result<R> {
    let mut builder = Builder::new(ctx.acl_domain_key);
    let output = f(&mut builder)?;
    builder.submit(ctx)?;
    Ok(output)
}

pub struct Builder<'a, 'info> {
    steps: Vec<FheFrameStep>,
    actions: Vec<FheFrameAction>,
    remaining_accounts: Vec<AccountInfo<'info>>,
    authorized_app_accounts: Vec<Pubkey>,
    acl_domain_key: Pubkey,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a, 'info> Builder<'a, 'info> {
    fn new(acl_domain_key: Pubkey) -> Self {
        Self {
            steps: Vec::new(),
            actions: Vec::new(),
            remaining_accounts: Vec::new(),
            authorized_app_accounts: Vec::new(),
            acl_domain_key,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn encrypted(&mut self, value: EncryptedValue<'info>) -> Result<FheValue> {
        let acl_record = self.push_account(value.acl_record)?;
        Ok(FheValue {
            operand: FheOperand::AclRecord {
                handle: value.handle,
                acl_record,
            },
        })
    }

    pub fn trivial_encrypt_u64(&mut self, value: u64, fhe_type: u8) -> Result<FheValue> {
        let index = self.steps.len();
        self.steps.push(FheFrameStep::TrivialEncrypt {
            plaintext: scalar_plaintext(value),
            fhe_type,
        });
        Ok(FheValue {
            operand: FheOperand::PreviousResult { index: index as u8 },
        })
    }

    pub fn add(&mut self, lhs: FheValue, rhs: FheValue, output_fhe_type: u8) -> Result<FheValue> {
        self.binary_op(FheOpcode::Add, lhs, rhs, output_fhe_type)
    }

    pub fn sub(&mut self, lhs: FheValue, rhs: FheValue, output_fhe_type: u8) -> Result<FheValue> {
        self.binary_op(FheOpcode::Sub, lhs, rhs, output_fhe_type)
    }

    pub fn allow(&mut self, value: &FheValue, allow: DurableAllow<'info>) -> Result<()> {
        self.authorize_app_account(allow.app_account);
        let output_acl_record = self.push_account(allow.acl_record)?;
        self.actions.push(FheFrameAction::Allow {
            source: value.operand.clone(),
            output_acl_record,
            nonce_key: allow.nonce_key,
            nonce_sequence: allow.nonce_sequence,
            acl_domain_key: self.acl_domain_key,
            app_account: allow.app_account,
            encrypted_value_label: allow.encrypted_value_label,
            subjects: allow.subjects,
            public_decrypt: allow.public_decrypt,
        });
        Ok(())
    }

    fn authorize_app_account(&mut self, app_account: Pubkey) {
        if !self.authorized_app_accounts.contains(&app_account) {
            self.authorized_app_accounts.push(app_account);
        }
    }

    fn binary_op(
        &mut self,
        opcode: FheOpcode,
        lhs: FheValue,
        rhs: FheValue,
        output_fhe_type: u8,
    ) -> Result<FheValue> {
        let scalar_byte = match rhs.operand {
            FheOperand::Scalar { .. } => 1,
            _ => 0,
        };
        let index = self.steps.len();
        self.steps.push(FheFrameStep::Operation {
            opcode,
            operands: vec![lhs.operand, rhs.operand],
            scalar_byte,
            output_fhe_type,
        });
        Ok(FheValue {
            operand: FheOperand::PreviousResult { index: index as u8 },
        })
    }

    fn push_account(&mut self, account: AccountInfo<'info>) -> Result<Pubkey> {
        let key = account.key();
        if !self
            .remaining_accounts
            .iter()
            .any(|existing| existing.key() == key)
        {
            require!(
                self.remaining_accounts.len() < 256,
                ConfidentialTokenError::TooManyFrameAccounts
            );
            self.remaining_accounts.push(account);
        }
        Ok(key)
    }

    fn submit(self, ctx: Context<'_, 'info>) -> Result<()> {
        let compute_bump = [ctx.compute_signer_bump];
        let compute_signer_seeds: &[&[u8]] =
            &[b"fhe-compute", ctx.acl_domain_key.as_ref(), &compute_bump];
        let signer_seeds: &[&[&[u8]]] = &[compute_signer_seeds];

        cpi::execute_frame(
            CpiContext::new_with_signer(
                ctx.zama_program.key(),
                ExecuteFrame {
                    payer: ctx.payer.to_account_info(),
                    compute_subject: ctx.compute_signer.to_account_info(),
                    system_program: ctx.system_program.to_account_info(),
                    event_authority: ctx.event_authority.to_account_info(),
                    program: ctx.zama_program.to_account_info(),
                },
                signer_seeds,
            )
            .with_remaining_accounts(self.remaining_accounts),
            self.authorized_app_accounts,
            self.steps,
            self.actions,
        )
    }
}

fn scalar_plaintext(value: u64) -> [u8; 32] {
    let mut plaintext = [0_u8; 32];
    plaintext[24..].copy_from_slice(&value.to_be_bytes());
    plaintext
}
