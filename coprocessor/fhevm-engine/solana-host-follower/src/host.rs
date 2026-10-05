//! The zama-host instructions a followed transaction carries, decoded with the program's own
//! types (`zama_host::decode`), so there is no bespoke decoder to drift from the on-chain layout.
//!
//! [`host_operations`] pairs each `fhe_execute` with the `FheExecutedEvent` it emitted and
//! resolves the encrypted-store writes of a transaction: every store an execution writes, and
//! every `make_store_handle_public`. A write names the store, its leaf count before the write,
//! the handle it installs, the keys it allows and whether it makes the handle public: the
//! input of the leaf record of the Solana access control RFC.

use anchor_lang::prelude::Pubkey;
use anyhow::{bail, Context, Result};
use solana_client::nonblocking::rpc_client::RpcClient;
use zama_host::decode::{
    decode_instruction, is_fhe_execute_instruction, ZamaHostInstruction,
};
use zama_host::state::{FheExecuteArgs, FheExecuteEffect, FheExecuteStep};
use zama_host::FheExecutedEvent;

/// The account index of the encrypted store in `make_store_handle_public`.
const MAKE_PUBLIC_STORE_ACCOUNT_INDEX: usize = 2;

/// One host-program instruction of a transaction, top level or inner, with its accounts
/// resolved.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DecodedInstruction {
    pub data: Vec<u8>,
    pub accounts: Vec<[u8; 32]>,
}

/// One write to an encrypted store, with its account resolved.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EncryptedStoreWrite {
    pub encrypted_store: [u8; 32],
    pub previous_leaf_count: u64,
    pub handle: [u8; 32],
    pub allowed_keys: Vec<[u8; 32]>,
    pub make_public: bool,
}

/// A host operation of one transaction, in instruction order.
pub enum HostOperation {
    FheExecute {
        args: FheExecuteArgs,
        /// Holds one result per step and one seed per random step, in step order.
        event: FheExecutedEvent,
        store_writes: Vec<EncryptedStoreWrite>,
    },
    MakeStoreHandlePublic(EncryptedStoreWrite),
}

impl HostOperation {
    pub fn store_writes(&self) -> &[EncryptedStoreWrite] {
        match self {
            Self::FheExecute { store_writes, .. } => store_writes,
            Self::MakeStoreHandlePublic(write) => std::slice::from_ref(write),
        }
    }
}

/// Decodes a `fhe_execute` instruction's data into the program's own `FheExecuteArgs`.
pub fn decode_fhe_execute_args(
    instruction_data: &[u8],
) -> Option<FheExecuteArgs> {
    match decode_instruction(instruction_data) {
        Ok(Some(ZamaHostInstruction::FheExecute(args))) => Some(args),
        _ => None,
    }
}

pub fn decode_fhe_executed_event(
    instruction_data: &[u8],
) -> Option<FheExecutedEvent> {
    let event: FheExecutedEvent =
        zama_host::decode::decode_event_cpi(instruction_data)?;
    (event.version == zama_host::EVENT_VERSION).then_some(event)
}

/// The chain id of `program_id`'s deployment, from its confirmed `HostConfig` account.
pub async fn host_chain_id(
    rpc: &RpcClient,
    program_id: &Pubkey,
) -> Result<u64> {
    let (host_config, _) = Pubkey::find_program_address(
        &[zama_host::constants::HOST_CONFIG_SEED],
        program_id,
    );
    let account = rpc
        .get_account(&host_config)
        .await
        .with_context(|| format!("fetch HostConfig {host_config}"))?;
    parse_host_config(&account.data)
}

/// Reads the chain id from `HostConfig` account data, with the program's own type so the layout
/// cannot drift.
fn parse_host_config(account_data: &[u8]) -> Result<u64> {
    use anchor_lang::AccountDeserialize;
    let config =
        zama_host::state::HostConfig::try_deserialize(&mut &account_data[..])
            .map_err(|e| anyhow::anyhow!("decode HostConfig account: {e}"))?;
    Ok(config.chain_id)
}

fn fhe_execute_dynamic_account(
    accounts: &[[u8; 32]],
    remaining_index: u8,
) -> Option<[u8; 32]> {
    accounts
        .get(
            zama_host::FHE_EXECUTE_FIXED_ACCOUNTS
                + usize::from(remaining_index),
        )
        .copied()
}

/// The host operations of one transaction's host instructions, in order. Empty when the
/// transaction executes nothing and makes nothing public. Errors when the chain accepted
/// something this crate cannot decode, since every sink would then miss host work.
pub fn host_operations(
    instructions: &[DecodedInstruction],
    slot: u64,
) -> Result<Vec<HostOperation>> {
    let mut operations = Vec::new();
    for (instruction_index, ix) in instructions.iter().enumerate() {
        match decode_instruction(&ix.data) {
            Ok(Some(ZamaHostInstruction::FheExecute(args))) => {
                // The execution's own event CPI follows it, before the next execution. Only
                // the host can sign its event authority, so a host instruction carrying the
                // event tag was emitted by the host.
                let mut events = instructions[instruction_index + 1..]
                    .iter()
                    .take_while(|later| {
                        !is_fhe_execute_instruction(&later.data)
                    })
                    .filter_map(|later| decode_fhe_executed_event(&later.data));
                let (Some(event), None) = (events.next(), events.next()) else {
                    bail!(
                        "fhe_execute in slot {slot} is not followed by exactly one \
                         FheExecutedEvent of version {}",
                        zama_host::EVENT_VERSION
                    );
                };
                if !event_matches_steps(&args, &event) {
                    bail!(
                        "fhe_execute in slot {slot} and its FheExecutedEvent do not describe \
                         the same steps"
                    );
                }
                let store_writes =
                    fhe_execute_store_writes(ix, &args, &event, slot)?;
                operations.push(HostOperation::FheExecute {
                    args,
                    event,
                    store_writes,
                });
            }
            Ok(Some(ZamaHostInstruction::MakeStoreHandlePublic {
                handle,
                previous_leaf_count,
                ..
            })) => {
                let Some(encrypted_store) =
                    ix.accounts.get(MAKE_PUBLIC_STORE_ACCOUNT_INDEX).copied()
                else {
                    bail!(
                        "make_store_handle_public account index {MAKE_PUBLIC_STORE_ACCOUNT_INDEX} \
                         out of range in slot {slot}; accounts={}",
                        ix.accounts.len()
                    );
                };
                operations.push(HostOperation::MakeStoreHandlePublic(
                    EncryptedStoreWrite {
                        encrypted_store,
                        previous_leaf_count,
                        handle,
                        allowed_keys: Vec::new(),
                        make_public: true,
                    },
                ));
            }
            Ok(_) => {}
            Err(error) => bail!(
                "{} in slot {slot} has undecodable arguments: {}",
                error.instruction,
                error.message
            ),
        }
    }
    Ok(operations)
}

/// The event has one result per step and one seed per random step, in step order.
fn event_matches_steps(
    args: &FheExecuteArgs,
    event: &FheExecutedEvent,
) -> bool {
    let random_steps = args
        .steps
        .iter()
        .enumerate()
        .filter(|(_, step)| {
            matches!(
                step,
                FheExecuteStep::Rand { .. }
                    | FheExecuteStep::RandBounded { .. }
            )
        })
        .map(|(index, _)| index);
    event.results.len() == args.steps.len()
        && random_steps.map(Some).eq(event
            .seeds
            .iter()
            .map(|seed| Some(usize::from(seed.step_index))))
}

/// The stores one execution writes. A computation-only grant persists nothing, so it is
/// not a write.
fn fhe_execute_store_writes(
    ix: &DecodedInstruction,
    args: &FheExecuteArgs,
    event: &FheExecutedEvent,
    slot: u64,
) -> Result<Vec<EncryptedStoreWrite>> {
    let mut writes = Vec::new();
    for effect in &args.effects {
        let handle = (effect.result.output_index == 0)
            .then(|| event.results.get(usize::from(effect.result.step_index)))
            .flatten();
        let Some(handle) = handle.copied() else {
            bail!("fhe_execute in slot {slot} writes the output of a step it does not have");
        };
        if effect.slot.is_none()
            && effect.allow_indexes.is_empty()
            && !effect.make_public
        {
            continue;
        }
        let Some(allowed_keys) = allowed_keys(effect, &args.dictionary) else {
            bail!("fhe_execute in slot {slot} allows a key outside its dictionary");
        };
        let Some(encrypted_store) =
            fhe_execute_dynamic_account(&ix.accounts, effect.store_index)
        else {
            bail!(
                "fhe_execute state output out of range in slot {slot}; remaining_index={}, \
                 accounts={}, handle={}",
                effect.store_index,
                ix.accounts.len(),
                bs58::encode(handle).into_string()
            );
        };
        writes.push(EncryptedStoreWrite {
            encrypted_store,
            previous_leaf_count: effect.previous_leaf_count,
            handle,
            allowed_keys,
            make_public: effect.make_public,
        });
    }
    Ok(writes)
}

fn allowed_keys(
    effect: &FheExecuteEffect,
    dictionary: &[[u8; 32]],
) -> Option<Vec<[u8; 32]>> {
    effect
        .allow_indexes
        .iter()
        .map(|index| dictionary.get(usize::from(*index)).copied())
        .collect()
}

#[cfg(test)]
mod tests {
    use anchor_lang::InstructionData;
    use zama_host::instruction::FheExecute;
    use zama_host::state::{
        ExecutionResultRef, FheBinaryOpCode, FheExecuteOperand, FheExecuteStep,
    };

    use super::*;
    use crate::follower::wire_fixtures::event_cpi_data;

    const FIXED_ACCOUNTS: usize = zama_host::FHE_EXECUTE_FIXED_ACCOUNTS;

    /// One trivial encryption with `effects`, followed by its event, whose result is `[7; 32]`.
    fn execution(
        effects: Vec<FheExecuteEffect>,
        dictionary: Vec<[u8; 32]>,
        accounts: Vec<[u8; 32]>,
    ) -> Vec<DecodedInstruction> {
        let args = FheExecuteArgs {
            execution_store_index: 0,
            account_count: u8::from(!effects.is_empty()),
            effects,
            returned_results: vec![],
            dictionary,
            steps: vec![FheExecuteStep::TrivialEncrypt {
                plaintext: [7; 32],
                fhe_type: 5,
            }],
        };
        vec![
            DecodedInstruction {
                data: FheExecute { args }.data(),
                accounts,
            },
            DecodedInstruction {
                data: event_cpi_data(vec![[7; 32]]),
                accounts: vec![],
            },
        ]
    }

    fn effect(
        store_index: u8,
        allow_indexes: Vec<u8>,
        make_public: bool,
    ) -> FheExecuteEffect {
        FheExecuteEffect {
            result: ExecutionResultRef {
                step_index: 0,
                output_index: 0,
            },
            store_index,
            previous_leaf_count: 9,
            slot: None,
            allow_indexes,
            make_public,
            grants: vec![],
        }
    }

    fn store_writes(
        instructions: &[DecodedInstruction],
    ) -> Result<Vec<EncryptedStoreWrite>> {
        Ok(host_operations(instructions, 42)?
            .iter()
            .flat_map(|operation| operation.store_writes().to_vec())
            .collect())
    }

    #[test]
    fn making_a_handle_public_writes_its_store() {
        let data = zama_host::instruction::MakeStoreHandlePublic {
            key: [0x11; 32],
            handle: [0x22; 32],
            previous_leaf_count: 8,
        }
        .data();
        let mut accounts = vec![[0; 32]; 6];
        accounts[MAKE_PUBLIC_STORE_ACCOUNT_INDEX] = [0x33; 32];
        assert_eq!(
            store_writes(&[DecodedInstruction {
                data: data.clone(),
                accounts
            }])
            .unwrap(),
            vec![EncryptedStoreWrite {
                encrypted_store: [0x33; 32],
                previous_leaf_count: 8,
                handle: [0x22; 32],
                allowed_keys: vec![],
                make_public: true,
            }]
        );

        let mut truncated = data;
        truncated.pop();
        let error = store_writes(&[DecodedInstruction {
            data: truncated,
            accounts: vec![[0; 32]; 6],
        }])
        .unwrap_err();
        assert!(
            error.to_string().contains("make_store_handle_public"),
            "{error}"
        );
    }

    /// A create-store payload starts with enough fixed-width bytes to deserialize as the
    /// make-public arguments if the discriminator were ignored. It must never fabricate a write.
    #[test]
    fn other_host_instructions_write_nothing() {
        let data = zama_host::instruction::CreateEncryptedStore {
            args: zama_host::instructions::CreateEncryptedStoreArgs {
                program: Pubkey::new_unique(),
                authority_seeds: vec![vec![12; 40]],
            },
        }
        .data();
        assert!(host_operations(
            &[DecodedInstruction {
                data,
                accounts: vec![[0; 32]; 6],
            }],
            42
        )
        .unwrap()
        .is_empty());
    }

    #[test]
    fn fhe_execute_batch_round_trips_via_program_type() {
        let execution = FheExecuteArgs {
            execution_store_index: 0,
            effects: vec![],
            returned_results: Vec::new(),
            account_count: 0,
            dictionary: vec![[2u8; 32]],
            steps: vec![
                FheExecuteStep::TrivialEncrypt {
                    plaintext: [7u8; 32],
                    fhe_type: 5,
                },
                FheExecuteStep::Binary {
                    op: FheBinaryOpCode::Add,
                    lhs: FheExecuteOperand::EarlierStep { producer_index: 0 },
                    rhs: FheExecuteOperand::Scalar { value_index: 0 },
                    output_fhe_type: 5,
                },
            ],
        };
        // Serialized like the on-chain instruction: discriminator + borsh args.
        let mut bytes = FheExecute {
            args: execution.clone(),
        }
        .data();
        let decoded =
            decode_fhe_execute_args(&bytes).expect("decode execution");
        assert_eq!(decoded, execution);
        assert_eq!(decoded.steps.len(), 2);
        // Wrong/missing discriminator -> None.
        assert!(decode_fhe_execute_args(&bytes[1..]).is_none());

        bytes.extend_from_slice(&[0xAA, 0xBB]);
        assert_eq!(decode_fhe_execute_args(&bytes), Some(execution));
    }

    #[test]
    fn decodes_the_executed_event_and_rejects_other_versions() {
        let decoded = decode_fhe_executed_event(&event_cpi_data(vec![[5; 32]]))
            .expect("decode event");
        assert_eq!(decoded.results, vec![[5; 32]]);

        let mut other_version = event_cpi_data(vec![[5; 32]]);
        // The version is the first byte after the 8-byte event tag and 8-byte discriminator.
        other_version[16] = zama_host::EVENT_VERSION.wrapping_add(1);
        assert!(decode_fhe_executed_event(&other_version).is_none());
    }

    /// A store output is written with the handle the event emitted, the store's account, its
    /// prior leaf count and its allowed keys in order; slot and grant policy do not enter.
    #[test]
    fn an_execution_writes_each_store_it_outputs_to() {
        let mut accounts = vec![[0; 32]; FIXED_ACCOUNTS + 4];
        accounts[FIXED_ACCOUNTS + 3] = [0x22; 32];
        let writes = store_writes(&execution(
            vec![effect(3, vec![0, 1], true)],
            vec![[0xB1; 32], [0xB2; 32]],
            accounts,
        ))
        .unwrap();
        assert_eq!(
            writes,
            vec![EncryptedStoreWrite {
                encrypted_store: [0x22; 32],
                previous_leaf_count: 9,
                handle: [7; 32],
                allowed_keys: vec![[0xB1; 32], [0xB2; 32]],
                make_public: true,
            }]
        );
    }

    #[test]
    fn dynamic_account_index_is_relative_to_remaining_accounts() {
        let accounts: Vec<[u8; 32]> = (0..13).map(|n| [n; 32]).collect();
        assert_eq!(fhe_execute_dynamic_account(&accounts, 0), Some([11; 32]));
        assert_eq!(fhe_execute_dynamic_account(&accounts, 1), Some([12; 32]));
        assert_eq!(fhe_execute_dynamic_account(&accounts[..11], 0), None);
    }

    /// A grant lets another store compute on the result but persists nothing in this one.
    #[test]
    fn a_computation_only_grant_writes_no_store() {
        let mut grant = effect(0, vec![], false);
        grant.grants = vec![zama_host::ResultGrant {
            consumer_store_index: 0,
        }];
        let writes = store_writes(&execution(
            vec![grant],
            vec![],
            vec![[0; 32]; FIXED_ACCOUNTS + 1],
        ))
        .unwrap();
        assert!(writes.is_empty());
    }

    #[test]
    fn a_transient_execution_writes_no_store() {
        let instructions = execution(vec![], vec![], vec![]);
        let operations = host_operations(&instructions, 42).unwrap();
        assert_eq!(operations.len(), 1);
        assert!(operations[0].store_writes().is_empty());
    }

    #[test]
    fn a_store_output_must_resolve_its_keys_and_account() {
        let overflow = store_writes(&execution(
            vec![effect(0, vec![1], false)],
            vec![[0xA1; 32]],
            vec![[0; 32]; FIXED_ACCOUNTS + 1],
        ))
        .unwrap_err();
        assert!(
            overflow.to_string().contains("outside its dictionary"),
            "{overflow}"
        );

        let missing = store_writes(&execution(
            vec![effect(0, vec![], true)],
            vec![],
            vec![[0; 32]; FIXED_ACCOUNTS],
        ))
        .unwrap_err();
        assert!(
            missing.to_string().contains("state output out of range"),
            "{missing}"
        );
    }

    #[test]
    fn an_execution_needs_exactly_one_event() {
        let paired = execution(vec![], vec![], vec![]);
        let twice = [paired.clone(), vec![paired[1].clone()]].concat();
        for unpaired in [vec![paired[0].clone()], twice] {
            let error = host_operations(&unpaired, 42)
                .err()
                .expect("an unpaired execution is refused");
            assert!(
                error.to_string().contains("exactly one FheExecutedEvent"),
                "{error}"
            );
        }
    }

    /// The event must carry one result per step and one seed per random step, in step order:
    /// a sink that skipped this check would record a handle the chain never installed.
    #[test]
    fn an_execution_and_its_event_describe_the_same_steps() {
        let rand =
            |steps: Vec<FheExecuteStep>, results: usize, seeds: Vec<u16>| {
                let args = FheExecuteArgs {
                    execution_store_index: 0,
                    account_count: 0,
                    effects: vec![],
                    returned_results: vec![],
                    dictionary: vec![],
                    steps,
                };
                let event = FheExecutedEvent {
                    version: zama_host::EVENT_VERSION,
                    previous_bank_hash: [0x44; 32],
                    unix_timestamp: 0,
                    results: vec![[7; 32]; results],
                    seeds: seeds
                        .into_iter()
                        .map(|step_index| zama_host::FheExecuteRandomSeed {
                            step_index,
                            seed: [1; 16],
                        })
                        .collect(),
                };
                let event_data = anchor_lang::event::EVENT_IX_TAG_LE
                    .iter()
                    .copied()
                    .chain(anchor_lang::Event::data(&event))
                    .collect();
                host_operations(
                    &[
                        DecodedInstruction {
                            data: FheExecute { args }.data(),
                            accounts: vec![],
                        },
                        DecodedInstruction {
                            data: event_data,
                            accounts: vec![],
                        },
                    ],
                    42,
                )
            };
        let trivial = FheExecuteStep::TrivialEncrypt {
            plaintext: [7; 32],
            fhe_type: 5,
        };
        let random = FheExecuteStep::Rand { fhe_type: 5 };

        assert!(rand(vec![trivial.clone(), random.clone()], 2, vec![1]).is_ok());
        for (steps, results, seeds) in [
            (vec![trivial.clone(), random.clone()], 1, vec![1]),
            (vec![trivial.clone(), random.clone()], 2, vec![0]),
            (vec![trivial.clone(), random.clone()], 2, vec![]),
            (vec![trivial, random], 2, vec![1, 1]),
        ] {
            let error = rand(steps, results, seeds)
                .err()
                .expect("a mismatched event is refused");
            assert!(
                error.to_string().contains("do not describe the same steps"),
                "{error}"
            );
        }
    }

    #[test]
    fn a_write_must_name_a_step_of_its_execution() {
        let mut missing = effect(0, vec![], true);
        missing.result.step_index = 1;
        let error = store_writes(&execution(
            vec![missing],
            vec![],
            vec![[0; 32]; FIXED_ACCOUNTS + 1],
        ))
        .unwrap_err();
        assert!(
            error.to_string().contains("a step it does not have"),
            "{error}"
        );
    }
}
