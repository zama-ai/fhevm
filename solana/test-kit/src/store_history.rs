//! The ACL history of each `EncryptedStore` a test touches, rebuilt from the host instructions an
//! instruction issued. A store keeps only its MMR peaks, so a public-decrypt inclusion proof needs
//! the leaves: this is the in-test counterpart of the SDK's `reconstructSolanaStoreHistory`.
//!
//! Values are not tracked here: the cleartext host build records them in the accounts
//! ([`crate::cleartext`]).

use std::collections::HashMap;

use anchor_lang::{AnchorDeserialize, Discriminator};
use mollusk_svm::result::InstructionResult;
use solana_sdk::pubkey::Pubkey;

use crate::{decode_fhe_execute_args, Ctx};

/// What one instruction's recorded host calls covered.
pub struct RecordedExecutions {
    /// Distinct `fhe_execute` CPIs decoded from the inner instructions.
    pub executions: usize,
    /// Effects that wrote a store slot.
    pub persistent_outputs: usize,
}

/// Leaf commitments per `EncryptedStore`, in history order.
#[derive(Default)]
pub struct StoreHistory {
    leaves: HashMap<Pubkey, Vec<[u8; 32]>>,
}

impl StoreHistory {
    /// Records the allow leaf a fixture store starts with, as if an earlier execution wrote it.
    pub fn seed_state_allow(&mut self, state: Pubkey, handle: [u8; 32], key: Pubkey) {
        let leaves = self.leaves.entry(state).or_default();
        let commitment = zama_solana_acl::historical_access_leaf_commitment(
            state.to_bytes(),
            0,
            handle,
            key.to_bytes(),
        );
        if leaves.is_empty() {
            leaves.push(commitment);
        } else {
            assert_eq!(leaves[0], commitment, "different initial state history");
        }
    }

    /// Appends the leaves of every `fhe_execute` and `make_store_handle_public` the instruction
    /// issued, in order, and checks each execution's event against the runtime sysvars.
    pub fn record(&mut self, context: &Ctx, result: &InstructionResult) -> RecordedExecutions {
        let message = result
            .message
            .as_ref()
            .expect("Mollusk result must include its compiled message");
        let mut executions = 0;
        let mut persistent_outputs = 0;
        for (index, inner) in result.inner_instructions.iter().enumerate() {
            let program = message
                .account_keys()
                .get(inner.instruction.program_id_index as usize)
                .copied();
            if program != Some(zama_host::id()) {
                continue;
            }
            let accounts = inner.instruction.accounts.as_slice();
            if let Some(args) = decode_fhe_execute_args(&inner.instruction.data) {
                let mut events = result.inner_instructions[index + 1..]
                    .iter()
                    .take_while(|child| child.stack_height > inner.stack_height)
                    .filter(|child| {
                        message.account_keys()[child.instruction.program_id_index as usize]
                            == zama_host::ID
                    })
                    .filter_map(|child| {
                        crate::decode_anchor_event::<zama_host::FheExecutedEvent>(
                            &child.instruction.data,
                        )
                    });
                let event = events
                    .next()
                    .expect("every execution emits FheExecutedEvent");
                assert!(events.next().is_none(), "one executed event per execution");
                assert_event_matches_runtime(context, &args, &event);
                executions += 1;
                for effect in &args.effects {
                    let handle = event.results[usize::from(effect.result.step_index)];
                    let account_index = accounts
                        [zama_host::FHE_EXECUTE_FIXED_ACCOUNTS + usize::from(effect.store_index)]
                        as usize;
                    let address = message.account_keys()[account_index];
                    let leaves = self.leaves.entry(address).or_default();
                    assert_eq!(
                        leaves.len() as u64,
                        effect.previous_leaf_count,
                        "history missed EncryptedStore leaves before {address}: {effect:?}"
                    );
                    for allow_index in &effect.allow_indexes {
                        let key = args
                            .dictionary_bytes(*allow_index)
                            .expect("valid allow dictionary index");
                        leaves.push(zama_solana_acl::historical_access_leaf_commitment(
                            address.to_bytes(),
                            leaves.len() as u64,
                            handle,
                            key,
                        ));
                    }
                    if effect.make_public {
                        leaves.push(zama_solana_acl::public_decrypt_leaf_commitment(
                            address.to_bytes(),
                            leaves.len() as u64,
                            handle,
                        ));
                    }
                    persistent_outputs += usize::from(effect.slot.is_some());
                }
                continue;
            }
            let Some(payload) = inner
                .instruction
                .data
                .strip_prefix(zama_host::instruction::MakeStoreHandlePublic::DISCRIMINATOR)
            else {
                continue;
            };
            let args = zama_host::instruction::MakeStoreHandlePublic::deserialize(&mut &*payload)
                .expect("make_store_handle_public args");
            let address = message.account_keys()[accounts[2] as usize];
            let leaves = self.leaves.entry(address).or_default();
            assert_eq!(
                leaves.len() as u64,
                args.previous_leaf_count,
                "history missed EncryptedStore leaves before public sealing {address}"
            );
            leaves.push(zama_solana_acl::public_decrypt_leaf_commitment(
                address.to_bytes(),
                leaves.len() as u64,
                args.handle,
            ));
        }
        RecordedExecutions {
            executions,
            persistent_outputs,
        }
    }

    /// The inclusion proof of the latest public-decrypt leaf for `handle` in `state`'s history.
    pub fn public_decrypt_proof(
        &self,
        state: Pubkey,
        handle: [u8; 32],
    ) -> zama_host::instructions::MmrInclusionProof {
        let leaves = self
            .leaves
            .get(&state)
            .expect("recorded history for encrypted store");
        let leaf_index = leaves
            .iter()
            .enumerate()
            .rev()
            .find_map(|(index, commitment)| {
                (*commitment
                    == zama_solana_acl::public_decrypt_leaf_commitment(
                        state.to_bytes(),
                        index as u64,
                        handle,
                    ))
                .then_some(index as u64)
            })
            .expect("public-decrypt leaf for handle");
        let proof = zama_solana_acl::mmr_build_proof(leaves, leaf_index)
            .expect("public-decrypt inclusion proof");
        zama_host::instructions::MmrInclusionProof {
            leaf_index: proof.leaf_index,
            siblings: proof.siblings,
        }
    }
}

fn assert_event_matches_runtime(
    context: &Ctx,
    args: &zama_host::FheExecuteArgs,
    event: &zama_host::FheExecutedEvent,
) {
    assert_eq!(event.version, zama_host::EVENT_VERSION);
    let slot = context.mollusk.sysvars.clock.slot;
    let previous_bank_hash = context
        .mollusk
        .sysvars
        .slot_hashes
        .iter()
        .find(|(candidate, _)| *candidate < slot)
        .map(|(_, hash)| hash.to_bytes())
        .expect("test runtime must contain a previous bank hash");
    assert_eq!(event.previous_bank_hash, previous_bank_hash);
    assert_eq!(
        event.unix_timestamp,
        context.mollusk.sysvars.clock.unix_timestamp
    );
    assert_eq!(event.results.len(), args.steps.len(), "one result per step");
}
