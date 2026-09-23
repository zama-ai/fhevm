// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import {E2ECoprocessorConfig} from "../E2ECoprocessorConfigLocal.sol";

/// @notice A deliberately small, deterministic graph for byte-consensus tests
/// of transaction-boundary ciphertext materialization.
///
/// The caller queues `stageInputA`, `deriveFromAAndB`, and `runIndependent` in
/// one L1 block.  `deriveFromAAndB` consumes the output of `stageInputA`
/// through its persisted (compressed) form — the canonical cross-transaction
/// representation — while its own graph fans `selected` out into `sum` and
/// `difference`, an intra-transaction edge that workers forward in memory and
/// never materialize.  `runIndependent` is a competing independent
/// transaction in the same block, and a later block calls `consumeFanout`,
/// which reads both fan-out outputs from their persisted form.
///
/// This contract intentionally exposes every computation result used by the
/// fixture.  The E2E oracle checks their persisted bytes only between
/// coprocessors running the same software and the same backend/hardware
/// class.  CPU and GPU are never put in the same byte-equality comparison;
/// their common oracle is the plaintext returned by user decryption.
contract MaterializationFixture is E2ECoprocessorConfig {
    // First transaction: 2 TFHE computations.
    euint64 public stageZero;
    euint64 public inputA;

    // Second transaction (same block, consumes `inputA`): 5 computations.
    euint64 public inputB;
    euint64 public trivialOne;
    ebool public inputAIsZero;
    euint64 public selected;
    euint64 public sum;
    euint64 public difference;

    // Independent same-block transaction: 2 TFHE computations.
    euint64 public independentInput;
    euint64 public independentBias;
    euint64 public independent;

    // Next-block transaction: 1 computation output.
    euint64 public terminal;

    /// @notice First transaction in the first block.
    function stageInputA(externalEuint64 inputHandle, bytes calldata inputProof) external {
        euint64 externalInputA = FHE.fromExternal(inputHandle, inputProof);
        stageZero = FHE.asEuint64(0);

        // VerifyInput has no computation row.  Materialising A + 0 here gives
        // the next transaction a real same-block producer to consume, while
        // preserving the intended plaintext A -> compare -> ITE graph.
        // Without this bridge, the external input would be only a persisted
        // boundary and the two transactions would be disconnected.
        inputA = FHE.add(externalInputA, stageZero);
        _expose(stageZero);
        _expose(inputA);
    }

    /// @notice Second transaction in the first block; it consumes `inputA`.
    function deriveFromAAndB(externalEuint64 inputHandle, bytes calldata inputProof) external {
        inputB = FHE.fromExternal(inputHandle, inputProof);
        // `FHE.asEuint64` has a deterministic result handle for a given
        // plaintext/type.  The first transaction already materialises zero,
        // so use one here rather than emitting a second producer for the same
        // output handle.  This keeps the fixture's output list one-to-one
        // with the ciphertext/digest/Gateway oracle while still exercising a
        // TrivialEncrypt in each transaction.
        trivialOne = FHE.asEuint64(1);
        inputAIsZero = FHE.eq(inputA, stageZero);

        // `inputA` reaches this transaction through the comparison; the
        // comparison and both ITE inputs connect the whole graph.  The
        // selected value then fans out into add and sub — intra-transaction
        // edges that the worker forwards in memory rather than materializing.
        selected = FHE.select(inputAIsZero, inputB, trivialOne);
        sum = FHE.add(selected, inputB);
        difference = FHE.sub(selected, inputB);

        _expose(inputB);
        _expose(trivialOne);
        _expose(inputAIsZero);
        _expose(selected);
        _expose(sum);
        _expose(difference);
    }

    /// @notice Third transaction in the first block, intentionally disconnected.
    function runIndependent(externalEuint64 inputHandle, bytes calldata inputProof) external {
        independentInput = FHE.fromExternal(inputHandle, inputProof);
        independentBias = FHE.asEuint64(7);
        independent = FHE.add(independentInput, independentBias);

        _expose(independentInput);
        _expose(independentBias);
        _expose(independent);
    }

    /// @notice A later-block consumer of both fan-out outputs.
    function consumeFanout() external {
        terminal = FHE.add(sum, difference);
        _expose(terminal);
    }

    // The fixture needs both the contract and initiating user to be able to
    // decrypt every intermediate output.  Public decryption is only a
    // convenience for diagnosis; consensus itself is established from the
    // homogeneous persisted-byte oracle outside this contract.
    function _expose(ebool value) private {
        FHE.allowThis(value);
        FHE.allow(value, msg.sender);
        FHE.makePubliclyDecryptable(value);
    }

    function _expose(euint64 value) private {
        FHE.allowThis(value);
        FHE.allow(value, msg.sender);
        FHE.makePubliclyDecryptable(value);
    }
}
