use super::*;

pub(super) fn accept_state_output(
    table: &mut ExecutionAccountTable<'_, '_>,
    dictionary: &[[u8; 32]],
    state_index: u8,
    previous_leaf_count: u64,
    slot: &Option<SlotWrite>,
    allow_indexes: &[u8],
    make_public: bool,
    grants: &[ResultGrant],
    result: [u8; 32],
) -> Result<Pubkey> {
    let state_key = table.account(state_index.into())?.key();
    require!(
        table.state(state_index.into())?.leaf_count == previous_leaf_count,
        ZamaHostError::PreviousStateMismatch
    );
    let allows = resolve_dictionary_keys(dictionary, allow_indexes)?;
    assert_allow_keys(&allows)?;
    if slot.is_some() || !allows.is_empty() || make_public {
        let state = table.state_mut(state_index.into())?;
        if let Some(slot) = slot {
            let key = dictionary_bytes(dictionary, slot.key_index)?;
            let expected = slot
                .previous_handle_index
                .map(|i| dictionary_bytes(dictionary, i))
                .transpose()?;
            state.set(key, expected, result)?;
        }
        for key in allows {
            let leaf = zama_solana_acl::historical_access_leaf_commitment(
                state_key.to_bytes(),
                state.leaf_count,
                result,
                key.to_bytes(),
            );
            zama_solana_acl::mmr_append(&mut state.peaks, &mut state.leaf_count, leaf)
                .map_err(|_| error!(ZamaHostError::InvalidFheExecuteAccount))?;
        }
        if make_public {
            let leaf = zama_solana_acl::public_decrypt_leaf_commitment(
                state_key.to_bytes(),
                state.leaf_count,
                result,
            );
            zama_solana_acl::mmr_append(&mut state.peaks, &mut state.leaf_count, leaf)
                .map_err(|_| error!(ZamaHostError::InvalidFheExecuteAccount))?;
        }
    }
    for grant in grants {
        let initiating_state = table.account(grant.initiating_state_index.into())?.key();
        let consumer = table.account(grant.consumer_state_index.into())?.key();
        let scratch = table.scratch_mut(grant.scratch_index.into())?;
        require_keys_eq!(
            scratch.initiating_state,
            initiating_state,
            ZamaHostError::TransientAccountInvalid
        );
        scratch.allow(result, consumer)?;
    }
    Ok(state_key)
}
