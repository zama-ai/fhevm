//! Deterministic synthetic FHE work injected by the **GCS (green) listener only**, so a
//! quiet host chain still produces a block that can anchor cross-operator consensus.
//!
//! # Why
//!
//! A host chain's dry-run window can only anchor on a block carrying at least one
//! *non-trivial* successful FHE op — `trivialEncrypt` outputs are deterministic across
//! operators, so agreeing on them proves nothing. If no chain in the proposal sees real
//! work, no track ever anchors and the upgrade times out and rolls back.
//!
//! Rather than depend on external traffic generators, the green listener synthesizes one
//! transaction into a single block of its own window:
//!
//! ```text
//! A = trivialEncrypt(0xA)     (deterministic, just an operand)
//! B = trivialEncrypt(0xB)     (deterministic, just an operand)
//! C = A + B                   (the consensus probe)
//! Allowed(C)                  (only the published result, as a real tx would)
//! ```
//!
//! `C` is a real FHE addition under the incoming stack's keys and parameters, so if green's
//! key material, serialization or scheduling differs from the other operators', `C`'s bytes
//! differ and the state hashes diverge. That is exactly the signal the dry-run exists to
//! produce.
//!
//! # Why raw logs, and not rows
//!
//! These are injected as **ABI-encoded logs**, appended to the block's log list before the
//! ingest loop decodes it. Everything downstream is then the production path: the events are
//! decoded normally, they bump `fhe_event_count` (so the state-hash worker does not treat the
//! block as empty and emit the `SHA-256("")` sentinel), `Allowed(C)` puts `C` into the
//! `is_allowed` set, and dependence chains, `schedule_order` and `transaction_id` are all
//! derived by the existing logic instead of hand-written.
//!
//! Only `C` is allowed, as a real transaction would: a contract allows the result it
//! publishes, not its intermediate operands. That is sufficient — the worker's acquisition
//! query selects transaction ids off `is_allowed` rows and then fetches *every* computation in
//! those transactions, without consulting `is_completed`, so A and B are computed as C's
//! dependencies.
//!
//! Note the side effect: `insert_computation*` binds `is_completed` to `!is_allowed`, so A and
//! B land as `is_completed = true` before anything computes them. That reads oddly but is the
//! production shape — there, `is_completed` means "not queued", not "computed".
//!
//! # Determinism
//!
//! Every input is on-chain-derived and therefore identical at every operator: the proposal
//! id, the chain id, and the block number (`start_block + 1`). Handles and the transaction
//! hash are keccak-derived from those, under dedicated domain separators so they can never
//! collide with a contract-produced handle. Re-ingesting the same block after a reorg or
//! during catch-up regenerates byte-identical logs, so the inserts dedupe on conflict.
//!
//! # Cleanup
//!
//! The synthetic transaction hash is the marker: `computations.transaction_id` is the log's
//! transaction hash, so cutover can find and delete this work — and the ciphertexts reached
//! through it — before merging `gcs.*` into `public`. Without that, synthetic handles would
//! land in production tables, and after cutover the now-live green `transaction-sender`
//! would try to publish digests for handles that exist on no chain.

use alloy::primitives::{
    keccak256, Address, FixedBytes, Log as PrimitiveLog, B256,
};
use alloy::rpc::types::Log;
use alloy::sol_types::SolEvent;
use fhevm_engine_common::types::{
    COMPUTED_HANDLE_INDEX_MARKER, HANDLE_VERSION,
};

use crate::contracts::{AclContract, TfheContract};
use crate::database::tfhe_event_propagate::{ClearConst, Handle};

/// Domain separator for synthetic handle derivation. Distinct from any contract-side
/// derivation, so a synthetic handle cannot collide with a real one.
const SYNTHETIC_HANDLE_DOMAIN: &[u8] = b"FHEVM_BLUE_GREEN_SYNTHETIC_HANDLE_V1";

/// Domain separator for the synthetic transaction hash.
const SYNTHETIC_TXN_DOMAIN: &[u8] = b"FHEVM_BLUE_GREEN_SYNTHETIC_TXN_V1";

/// `FheType` written into byte 30 of every synthetic handle. `FheUint64` (type id 5) is a
/// plain arithmetic type, so `A + B` is a genuine non-trivial op.
const SYNTHETIC_FHE_TYPE: u8 = 5;

/// Plaintext operands. Any fixed values work; these are recognisable in logs.
const SYNTHETIC_PLAINTEXT_A: u64 = 0xA;
const SYNTHETIC_PLAINTEXT_B: u64 = 0xB;

/// Caller recorded on the synthetic events. Not a real account: nothing authenticates it,
/// and it exists only because the event shape requires one.
const SYNTHETIC_CALLER: [u8; 20] = [0x5Au8; 20];

/// Offset from a chain's `start_block` at which the synthetic transaction is injected.
///
/// Not `start_block` itself: that block is the boundary the readiness check settles blue up
/// to, and keeping one block of clearance avoids overlapping the prune of pre-window rows.
pub const SYNTHETIC_BLOCK_OFFSET: i64 = 1;

/// Everything the synthetic work is derived from. Every field is an on-chain value, so all
/// operators build byte-identical work from the same proposal.
#[derive(Debug, Clone, Copy)]
pub struct SyntheticContext<'a> {
    /// `CoprocessorUpgradeProposed.proposalId`.
    pub proposal_id: &'a [u8],
    /// `CoprocessorUpgradeProposed.softwareVersion`: the version green upgrades *to*, read
    /// from `upgrade_state.version` rather than the local compiled-in constant.
    pub target_version: &'a str,
    pub chain_id: u64,
    /// The designated block, `start_block + SYNTHETIC_BLOCK_OFFSET`.
    pub block_number: i64,
    /// Binds the work to one fork branch, so a reorg produces different work.
    pub block_hash: B256,
}

/// The three synthetic handles for one [`SyntheticContext`], in dependency order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyntheticHandles {
    /// `trivialEncrypt` operand.
    pub a: Handle,
    /// `trivialEncrypt` operand.
    pub b: Handle,
    /// `a + b`, the non-trivial op that can anchor consensus.
    pub c: Handle,
}

/// Deterministic transaction hash shared by all four synthetic logs.
///
/// Doubles as the cleanup marker: it becomes `computations.transaction_id`.
///
/// `block_hash` is part of the preimage so the transaction is bound to one fork branch. A
/// reorg that replaces this block yields a different transaction hash, so the replacement's
/// work stays distinguishable from the orphaned branch's instead of colliding with it on the
/// legacy `computations` key `(output_handle, transaction_id)`.
///
/// `target_version` is the proposal's `softwareVersion` - the version green upgrades *to* -
/// read from `upgrade_state.version` rather than the local compiled-in constant, so it is the
/// same on-chain value at every operator.
pub fn synthetic_transaction_hash(ctx: &SyntheticContext<'_>) -> B256 {
    let SyntheticContext {
        proposal_id,
        target_version,
        chain_id,
        block_number,
        block_hash,
    } = *ctx;
    let mut hasher_input = Vec::with_capacity(
        SYNTHETIC_TXN_DOMAIN.len()
            + proposal_id.len()
            + target_version.len()
            + 16
            + B256::len_bytes(),
    );
    hasher_input.extend_from_slice(SYNTHETIC_TXN_DOMAIN);
    hasher_input.extend_from_slice(proposal_id);
    hasher_input.extend_from_slice(target_version.as_bytes());
    hasher_input.extend_from_slice(&chain_id.to_be_bytes());
    hasher_input.extend_from_slice(&block_number.to_be_bytes());
    hasher_input.extend_from_slice(block_hash.as_slice());
    keccak256(hasher_input)
}

/// Derive one synthetic handle, laid out exactly like a contract-produced computed handle
/// (see `fhevm_engine_common::types`): keccak bits, then byte 21 the computed marker, bytes
/// 22..30 the chain id, byte 30 the `FheType`, byte 31 the handle version.
///
/// Keeping the layout means `get_ct_type` and `chain_id_from_handle` behave normally; only
/// the preimage differs from a real handle.
/// The preimage covers the whole [`SyntheticContext`], so handles are specific to the
/// proposal, the target version, the chain, the block *and* the fork branch — the same scoping
/// as [`synthetic_transaction_hash`]. Two attempts, or two branches of a reorged block, never
/// produce the same handle.
fn synthetic_handle(ctx: &SyntheticContext<'_>, index: u8) -> Handle {
    let SyntheticContext {
        proposal_id,
        target_version,
        chain_id,
        block_number,
        block_hash,
    } = *ctx;
    let mut hasher_input = Vec::with_capacity(
        SYNTHETIC_HANDLE_DOMAIN.len()
            + proposal_id.len()
            + target_version.len()
            + 17
            + B256::len_bytes(),
    );
    hasher_input.extend_from_slice(SYNTHETIC_HANDLE_DOMAIN);
    hasher_input.extend_from_slice(proposal_id);
    hasher_input.extend_from_slice(target_version.as_bytes());
    hasher_input.extend_from_slice(&chain_id.to_be_bytes());
    hasher_input.extend_from_slice(&block_number.to_be_bytes());
    hasher_input.extend_from_slice(block_hash.as_slice());
    hasher_input.push(index);

    let mut handle: [u8; 32] = keccak256(hasher_input).into();
    handle[21] = COMPUTED_HANDLE_INDEX_MARKER;
    handle[22..30].copy_from_slice(&chain_id.to_be_bytes());
    handle[30] = SYNTHETIC_FHE_TYPE;
    handle[31] = HANDLE_VERSION;
    FixedBytes::from(handle)
}

/// The three handles for one [`SyntheticContext`].
pub fn synthetic_handles(ctx: &SyntheticContext<'_>) -> SyntheticHandles {
    SyntheticHandles {
        a: synthetic_handle(ctx, 0),
        b: synthetic_handle(ctx, 1),
        c: synthetic_handle(ctx, 2),
    }
}

/// Wrap an ABI-encoded event as an `alloy` RPC log, carrying the block and transaction
/// identity the ingest path reads.
fn rpc_log(
    address: Address,
    data: alloy::primitives::LogData,
    block_number: i64,
    block_hash: B256,
    transaction_hash: B256,
    log_index: u64,
) -> Log {
    Log {
        inner: PrimitiveLog { address, data },
        block_hash: Some(block_hash),
        block_number: Some(block_number as u64),
        block_timestamp: None,
        transaction_hash: Some(transaction_hash),
        transaction_index: None,
        log_index: Some(log_index),
        removed: false,
    }
}

/// Build the four logs for one block: `trivialEncrypt` A, `trivialEncrypt` B, `C = A + B`,
/// and `Allowed(C)`.
///
/// Returns an empty vector when either contract address is unset, since the ingest loop
/// gates decoding on the log's address matching the configured contract.
///
/// `log_index_base` should be past the block's real logs, so the synthetic ones do not
/// collide with a real `(transaction_hash, log_index)` pair.
pub fn synthetic_logs(
    ctx: &SyntheticContext<'_>,
    tfhe_contract_address: Option<Address>,
    acl_contract_address: Option<Address>,
    log_index_base: u64,
) -> Vec<Log> {
    let (block_number, block_hash) = (ctx.block_number, ctx.block_hash);
    let (Some(tfhe_address), Some(acl_address)) =
        (tfhe_contract_address, acl_contract_address)
    else {
        return vec![];
    };

    let handles = synthetic_handles(ctx);
    let txn_hash = synthetic_transaction_hash(ctx);
    let caller = Address::from(SYNTHETIC_CALLER);

    let trivial = |pt: u64, result: Handle| {
        TfheContract::TrivialEncrypt {
            caller,
            pt: ClearConst::from(pt),
            toType: SYNTHETIC_FHE_TYPE,
            result,
        }
        .encode_log_data()
    };

    let add = TfheContract::FheAdd {
        caller,
        lhs: handles.a,
        rhs: handles.b,
        // Zero means "not a scalar op": both operands are ciphertexts.
        scalarByte: FixedBytes::from_slice(&[0]),
        result: handles.c,
    }
    .encode_log_data();

    // Only `C` is allowed, matching a real transaction: a contract allows the result it
    // publishes, not the intermediate operands. The worker still computes A and B — the
    // acquisition query selects transaction ids off `is_allowed` rows and then fetches *every*
    // computation in those transactions without consulting `is_completed`.
    let allowed = AclContract::Allowed {
        caller,
        account: caller,
        handle: handles.c,
    }
    .encode_log_data();

    vec![
        rpc_log(
            tfhe_address,
            trivial(SYNTHETIC_PLAINTEXT_A, handles.a),
            block_number,
            block_hash,
            txn_hash,
            log_index_base,
        ),
        rpc_log(
            tfhe_address,
            trivial(SYNTHETIC_PLAINTEXT_B, handles.b),
            block_number,
            block_hash,
            txn_hash,
            log_index_base + 1,
        ),
        rpc_log(
            tfhe_address,
            add,
            block_number,
            block_hash,
            txn_hash,
            log_index_base + 2,
        ),
        rpc_log(
            acl_address,
            allowed,
            block_number,
            block_hash,
            txn_hash,
            log_index_base + 3,
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::sol_types::SolEventInterface;
    use fhevm_engine_common::bridge::chain_id_from_handle;
    use fhevm_engine_common::types::get_ct_type;

    const PROPOSAL: &[u8] = &[0xABu8; 32];
    const VERSION: &str = "v0.15.0";
    const CHAIN: u64 = 12345;
    const BLOCK: i64 = 101;

    fn ctx() -> SyntheticContext<'static> {
        SyntheticContext {
            proposal_id: PROPOSAL,
            target_version: VERSION,
            chain_id: CHAIN,
            block_number: BLOCK,
            block_hash: B256::ZERO,
        }
    }

    fn addresses() -> (Option<Address>, Option<Address>) {
        (
            Some(Address::from([0x11u8; 20])),
            Some(Address::from([0x22u8; 20])),
        )
    }

    /// Every operator derives the same work from the same on-chain inputs. This is the
    /// property the whole approach rests on: differing handles would make the state hashes
    /// differ for a reason unrelated to the upgrade.
    #[test]
    fn derivation_is_deterministic() {
        assert_eq!(synthetic_handles(&ctx()), synthetic_handles(&ctx()));
        assert_eq!(
            synthetic_transaction_hash(&ctx()),
            synthetic_transaction_hash(&ctx())
        );
    }

    /// Changing any part of the context must change both the handles and the transaction
    /// hash, so no two attempts, chains, blocks or fork branches can be confused.
    #[test]
    fn derivation_is_scoped_to_the_whole_context() {
        let base_handles = synthetic_handles(&ctx());
        let base_txn = synthetic_transaction_hash(&ctx());

        let variants = [
            (
                "proposal",
                SyntheticContext {
                    proposal_id: &[0xCDu8; 32],
                    ..ctx()
                },
            ),
            (
                "target version",
                SyntheticContext {
                    target_version: "v0.16.0",
                    ..ctx()
                },
            ),
            (
                "chain",
                SyntheticContext {
                    chain_id: CHAIN + 1,
                    ..ctx()
                },
            ),
            (
                "block number",
                SyntheticContext {
                    block_number: BLOCK + 1,
                    ..ctx()
                },
            ),
            (
                "block hash (reorg)",
                SyntheticContext {
                    block_hash: B256::from([0x77u8; 32]),
                    ..ctx()
                },
            ),
        ];

        for (what, variant) in variants {
            assert_ne!(
                base_handles,
                synthetic_handles(&variant),
                "handles must change with the {what}"
            );
            assert_ne!(
                base_txn,
                synthetic_transaction_hash(&variant),
                "transaction hash must change with the {what}"
            );
        }
    }

    /// The three handles must be distinct, or `C = A + B` would not be a two-operand op.
    #[test]
    fn handles_are_distinct() {
        let h = synthetic_handles(&ctx());
        assert_ne!(h.a, h.b);
        assert_ne!(h.b, h.c);
        assert_ne!(h.a, h.c);
    }

    /// Synthetic handles must be indistinguishable in *layout* from contract-produced ones,
    /// so downstream decoding treats them normally.
    #[test]
    fn handle_layout_matches_the_contract_scheme() {
        let h = synthetic_handles(&ctx());
        for handle in [h.a, h.b, h.c] {
            let bytes = handle.as_slice();
            assert_eq!(bytes.len(), 32);
            assert_eq!(bytes[21], COMPUTED_HANDLE_INDEX_MARKER);
            assert_eq!(bytes[31], HANDLE_VERSION);
            assert_eq!(get_ct_type(bytes).unwrap(), SYNTHETIC_FHE_TYPE as i16);
            assert_eq!(
                chain_id_from_handle(&handle.0),
                CHAIN,
                "bytes 22..30 must carry the chain id"
            );
        }
    }

    /// The logs must decode through the same path `ingest_block_logs` uses, land on the
    /// configured contract addresses, and share one transaction hash.
    #[test]
    fn logs_decode_as_the_expected_events() {
        let (tfhe, acl) = addresses();
        let logs = synthetic_logs(&ctx(), tfhe, acl, 7);
        assert_eq!(logs.len(), 4);

        let handles = synthetic_handles(&ctx());
        let txn = synthetic_transaction_hash(&ctx());
        for log in &logs {
            assert_eq!(log.transaction_hash, Some(txn));
            assert_eq!(log.block_number, Some(BLOCK as u64));
        }
        // Contiguous from the base, so they cannot collide with the block's real logs.
        let indices: Vec<u64> =
            logs.iter().filter_map(|l| l.log_index).collect();
        assert_eq!(indices, vec![7, 8, 9, 10]);

        // Two trivial encrypts then the add, all on the TFHE address.
        assert_eq!(logs[0].inner.address, tfhe.unwrap());
        match TfheContract::TfheContractEvents::decode_log(&logs[0].inner)
            .expect("A decodes")
            .data
        {
            TfheContract::TfheContractEvents::TrivialEncrypt(e) => {
                assert_eq!(e.result, handles.a);
                assert_eq!(e.pt, ClearConst::from(SYNTHETIC_PLAINTEXT_A));
            }
            other => panic!("expected TrivialEncrypt, got {other:?}"),
        }
        match TfheContract::TfheContractEvents::decode_log(&logs[1].inner)
            .expect("B decodes")
            .data
        {
            TfheContract::TfheContractEvents::TrivialEncrypt(e) => {
                assert_eq!(e.result, handles.b);
                assert_eq!(e.pt, ClearConst::from(SYNTHETIC_PLAINTEXT_B));
            }
            other => panic!("expected TrivialEncrypt, got {other:?}"),
        }
        match TfheContract::TfheContractEvents::decode_log(&logs[2].inner)
            .expect("C decodes")
            .data
        {
            TfheContract::TfheContractEvents::FheAdd(e) => {
                assert_eq!(e.lhs, handles.a);
                assert_eq!(e.rhs, handles.b);
                assert_eq!(e.result, handles.c);
            }
            other => panic!("expected FheAdd, got {other:?}"),
        }

        // Last is the single ACL allowance, for C only, on the ACL address.
        assert_eq!(logs[3].inner.address, acl.unwrap());
        match AclContract::AclContractEvents::decode_log(&logs[3].inner)
            .expect("Allowed decodes")
            .data
        {
            AclContract::AclContractEvents::Allowed(e) => {
                assert_eq!(e.handle, handles.c)
            }
            other => panic!("expected Allowed, got {other:?}"),
        }
    }

    /// Only `C` is allowed, matching a real transaction. Sufficient because the acquisition
    /// query fetches every computation of an eligible transaction without consulting
    /// `is_completed`, so A and B are computed as C's dependencies.
    #[test]
    fn only_the_published_result_is_allowed() {
        let (tfhe, acl) = addresses();
        let logs = synthetic_logs(&ctx(), tfhe, acl, 0);
        let handles = synthetic_handles(&ctx());

        let allowed: Vec<Handle> = logs
            .iter()
            .filter_map(|log| {
                AclContract::AclContractEvents::decode_log(&log.inner).ok()
            })
            .flat_map(|event| {
                crate::database::tfhe_event_propagate::acl_result_handles(
                    &event,
                )
            })
            .collect();
        assert_eq!(allowed, vec![handles.c]);
    }

    /// Without both contract addresses the ingest loop would never decode these, so emitting
    /// them would be pointless.
    #[test]
    fn no_logs_without_contract_addresses() {
        let (tfhe, acl) = addresses();
        assert!(synthetic_logs(&ctx(), None, acl, 0).is_empty());
        assert!(synthetic_logs(&ctx(), tfhe, None, 0).is_empty());
        assert!(synthetic_logs(&ctx(), None, None, 0).is_empty());
    }
}
