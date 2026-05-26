//! Structured frame builder for `zama-host::execute_frame`.

use anchor_lang::prelude::*;
use zama_host::{
    cpi, cpi::accounts::ExecuteFrame, program::ZamaHost, AclSubjectEntry, FheFrameAction,
    FheFrameStep, FheOpcode, FheOperand,
};

#[derive(Clone)]
pub struct EncryptedValue<'info> {
    pub handle: [u8; 32],
    pub acl_record: AccountInfo<'info>,
}

#[derive(Clone)]
pub struct AuthorizedAppAccount<'info> {
    account: AccountInfo<'info>,
}

impl<'info> AuthorizedAppAccount<'info> {
    pub fn new(account: AccountInfo<'info>) -> Self {
        Self { account }
    }

    fn key(&self) -> Pubkey {
        self.account.key()
    }
}

/// Symbolic frame value: operand reference only. Frame results have no handle until the host runs.
#[derive(Clone)]
pub struct FrameValue {
    operand: FheOperand,
}

pub struct Context<'a, 'info> {
    pub payer: &'a Signer<'info>,
    pub event_authority: &'a UncheckedAccount<'info>,
    pub zama_program: &'a Program<'info, ZamaHost>,
    pub compute_signer: &'a UncheckedAccount<'info>,
    pub rand_counter: &'a UncheckedAccount<'info>,
    pub acl_domain_key: Pubkey,
    pub compute_signer_bump: u8,
    pub system_program: &'a Program<'info, System>,
}

pub struct DurableAllow<'info> {
    pub acl_record: AccountInfo<'info>,
    pub app_account: AuthorizedAppAccount<'info>,
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

    pub fn encrypted(&mut self, value: EncryptedValue<'info>) -> Result<FrameValue> {
        let acl_record = self.push_account(value.acl_record)?;
        Ok(FrameValue {
            operand: FheOperand::AclRecord {
                handle: value.handle,
                acl_record,
            },
        })
    }

    pub fn trivial_encrypt_u64(&mut self, value: u64, fhe_type: u8) -> Result<FrameValue> {
        let index = self.steps.len();
        self.steps.push(FheFrameStep::TrivialEncrypt {
            plaintext: scalar_plaintext(value),
            fhe_type,
        });
        Ok(FrameValue {
            operand: FheOperand::PreviousResult { index: index as u8 },
        })
    }

    pub fn rand_u64(&mut self) -> Result<FrameValue> {
        const UINT64_FHE_TYPE: u8 = 5;
        let index = self.steps.len();
        self.steps.push(FheFrameStep::Rand {
            fhe_type: UINT64_FHE_TYPE,
        });
        Ok(FrameValue {
            operand: FheOperand::PreviousResult { index: index as u8 },
        })
    }

    pub fn add(
        &mut self,
        lhs: FrameValue,
        rhs: FrameValue,
        output_fhe_type: u8,
    ) -> Result<FrameValue> {
        self.binary_op(FheOpcode::Add, lhs, rhs, output_fhe_type)
    }

    pub fn sub(
        &mut self,
        lhs: FrameValue,
        rhs: FrameValue,
        output_fhe_type: u8,
    ) -> Result<FrameValue> {
        self.binary_op(FheOpcode::Sub, lhs, rhs, output_fhe_type)
    }

    pub fn allow(&mut self, value: &FrameValue, allow: DurableAllow<'info>) -> Result<()> {
        let app_account = allow.app_account.key();
        self.authorize_app_account(allow.app_account)?;
        let output_acl_record = self.push_account(allow.acl_record)?;
        self.actions.push(FheFrameAction::Allow {
            source: value.operand.clone(),
            output_acl_record,
            nonce_key: allow.nonce_key,
            nonce_sequence: allow.nonce_sequence,
            acl_domain_key: self.acl_domain_key,
            app_account,
            encrypted_value_label: allow.encrypted_value_label,
            subjects: allow.subjects,
            public_decrypt: allow.public_decrypt,
        });
        Ok(())
    }

    fn authorize_app_account(&mut self, app_account: AuthorizedAppAccount<'info>) -> Result<()> {
        let key = app_account.key();
        if !self.authorized_app_accounts.contains(&key) {
            self.authorized_app_accounts.push(key);
        }
        self.push_account(app_account.account)?;
        Ok(())
    }

    fn binary_op(
        &mut self,
        opcode: FheOpcode,
        lhs: FrameValue,
        rhs: FrameValue,
        output_fhe_type: u8,
    ) -> Result<FrameValue> {
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
        Ok(FrameValue {
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
                ZamaFheError::TooManyFrameAccounts
            );
            self.remaining_accounts.push(account);
        }
        Ok(key)
    }

    fn submit(self, ctx: Context<'_, 'info>) -> Result<()> {
        let signer_seed_bytes = [vec![
            b"fhe-compute".to_vec(),
            ctx.acl_domain_key.to_bytes().to_vec(),
            vec![ctx.compute_signer_bump],
        ]];
        let signer_seed_slices = signer_seed_bytes
            .iter()
            .map(|seed_set| seed_set.iter().map(Vec::as_slice).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let signer_seeds = signer_seed_slices
            .iter()
            .map(Vec::as_slice)
            .collect::<Vec<_>>();

        cpi::execute_frame(
            CpiContext::new_with_signer(
                ctx.zama_program.key(),
                ExecuteFrame {
                    payer: ctx.payer.to_account_info(),
                    compute_subject: ctx.compute_signer.to_account_info(),
                    system_program: ctx.system_program.to_account_info(),
                    rand_counter: ctx.rand_counter.to_account_info(),
                    event_authority: ctx.event_authority.to_account_info(),
                    program: ctx.zama_program.to_account_info(),
                },
                &signer_seeds,
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

#[error_code]
pub enum ZamaFheError {
    #[msg("Too many accounts in execute_frame remaining_accounts (max 256)")]
    TooManyFrameAccounts,
}
