//! Recognize handles whose executor preimage predates operand-origin bits.
//!
//! There is no version bit distinguishing these handles. Reconstruct the old
//! preimage instead of guessing from the binary version or an activation date:
//! historical replay can run on a fresh database and cross a contract upgrade.

use alloy::primitives::{keccak256, Address, FixedBytes, U256};

use crate::cmd::block_history::BlockSummary;
use crate::contracts::TfheContract::TfheContractEvents as E;

type Handle = FixedBytes<32>;

/// Only operations with encrypted operands changed their preimage. Trivial
/// encryption and randomness have no operand-origin ambiguity.
pub(super) fn has_legacy_operand_origin(
    event: &E,
    acl: Address,
    chain_id: u64,
    block: &BlockSummary,
) -> bool {
    // Match FHEVMExecutor's pre-v0.15 abi.encodePacked layouts. The caller is
    // deliberately absent. Before the boolean check, _binaryOp normalized the
    // scalar byte's low bit before hashing but emitted the original byte.
    let (op, args, result) = match event {
        E::FheAdd(e) => binary(0, e.lhs, e.rhs, e.scalarByte, e.result),
        E::FheSub(e) => binary(1, e.lhs, e.rhs, e.scalarByte, e.result),
        E::FheMul(e) => binary(2, e.lhs, e.rhs, e.scalarByte, e.result),
        E::FheDiv(e) => binary(3, e.lhs, e.rhs, e.scalarByte, e.result),
        E::FheRem(e) => binary(4, e.lhs, e.rhs, e.scalarByte, e.result),
        E::FheBitAnd(e) => binary(5, e.lhs, e.rhs, e.scalarByte, e.result),
        E::FheBitOr(e) => binary(6, e.lhs, e.rhs, e.scalarByte, e.result),
        E::FheBitXor(e) => binary(7, e.lhs, e.rhs, e.scalarByte, e.result),
        E::FheShl(e) => binary(8, e.lhs, e.rhs, e.scalarByte, e.result),
        E::FheShr(e) => binary(9, e.lhs, e.rhs, e.scalarByte, e.result),
        E::FheRotl(e) => binary(10, e.lhs, e.rhs, e.scalarByte, e.result),
        E::FheRotr(e) => binary(11, e.lhs, e.rhs, e.scalarByte, e.result),
        E::FheEq(e) => binary(12, e.lhs, e.rhs, e.scalarByte, e.result),
        E::FheNe(e) => binary(13, e.lhs, e.rhs, e.scalarByte, e.result),
        E::FheGe(e) => binary(14, e.lhs, e.rhs, e.scalarByte, e.result),
        E::FheGt(e) => binary(15, e.lhs, e.rhs, e.scalarByte, e.result),
        E::FheLe(e) => binary(16, e.lhs, e.rhs, e.scalarByte, e.result),
        E::FheLt(e) => binary(17, e.lhs, e.rhs, e.scalarByte, e.result),
        E::FheMin(e) => binary(18, e.lhs, e.rhs, e.scalarByte, e.result),
        E::FheMax(e) => binary(19, e.lhs, e.rhs, e.scalarByte, e.result),
        E::FheNeg(e) => (20, e.ct.to_vec(), e.result),
        E::FheNot(e) => (21, e.ct.to_vec(), e.result),
        E::Cast(e) => {
            let mut args = e.ct.to_vec();
            args.push(e.toType);
            (23, args, e.result)
        }
        E::FheIfThenElse(e) => (
            25,
            [
                e.control.as_slice(),
                e.ifTrue.as_slice(),
                e.ifFalse.as_slice(),
            ]
            .concat(),
            e.result,
        ),
        E::FheSum(e) => {
            let mut args =
                U256::from(e.values.len()).to_be_bytes::<32>().to_vec();
            args.extend(e.values.iter().flat_map(|v| v.0));
            (28, args, e.result)
        }
        E::FheIsIn(e) => {
            let mut args =
                U256::from(e.values.len()).to_be_bytes::<32>().to_vec();
            args.extend_from_slice(e.value.as_slice());
            args.extend(e.values.iter().flat_map(|v| v.0));
            (29, args, e.result)
        }
        E::FheMulDiv(e) => {
            let mut args = [
                e.factor1.as_slice(),
                e.factor2.as_slice(),
                e.divisor.as_slice(),
            ]
            .concat();
            args.extend_from_slice(e.scalarByte.as_slice());
            (30, args, e.result)
        }
        E::Initialized(_)
        | E::Upgraded(_)
        | E::VerifyInput(_)
        | E::TrivialEncrypt(_)
        | E::FheRand(_)
        | E::FheRandBounded(_) => return false,
    };
    let mut operands_and_scope = vec![op];
    operands_and_scope.extend(args);
    operands_and_scope.extend_from_slice(acl.as_slice());
    operands_and_scope
        .extend_from_slice(&U256::from(chain_id).to_be_bytes::<32>());
    // Before 5cd51da13 (March 2026), the preimage also omitted the domain and
    // block context. Collection/MulDiv operators were introduced afterwards.
    if op < 28 && keccak256(&operands_and_scope)[..21] == result[..21] {
        return true;
    }
    let mut preimage = b"FHE_comp".to_vec();
    preimage.extend(operands_and_scope);
    preimage.extend_from_slice(block.parent_hash.as_slice());
    preimage
        .extend_from_slice(&U256::from(block.timestamp).to_be_bytes::<32>());
    // _appendMetadataToPrehandle replaces bytes 21..32. Their chain/type/version
    // validation is performed by evm_computation before this recognition.
    keccak256(preimage)[..21] == result[..21]
}

fn binary(
    op: u8,
    lhs: Handle,
    rhs: Handle,
    scalar: FixedBytes<1>,
    result: Handle,
) -> (u8, Vec<u8>, Handle) {
    let mut args = [lhs.as_slice(), rhs.as_slice()].concat();
    args.push(scalar.0[0] & 1);
    (op, args, result)
}
