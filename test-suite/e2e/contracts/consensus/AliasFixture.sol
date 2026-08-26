// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import {E2ECoprocessorConfig} from "../E2ECoprocessorConfigLocal.sol";

/// @notice Deterministic aliased-handle graph for byte-consensus tests.
///
/// FHE handles are content-derived (operation, operand handles, boundary
/// bits, chain id and per-block context), so two transactions in one block
/// context that emit the same operation on the same operands WITH THE SAME
/// SOURCING produce the SAME output handle — an alias. Under the
/// minted-in-transaction discriminant, sourcing is part of the handle:
/// `combineFromStorage` and `combineFromStorageAgain` both consume persisted
/// cross-transaction boundaries and alias each other, while `combineLocal`
/// recomputes the same inputs inside its own transaction and therefore mints
/// a DIFFERENT output handle (its operands fold zero boundary bits). Its
/// trivial encrypts, however, still alias `produceInputs`' — trivial
/// encrypts consume no operands, so their handles carry no boundary bits.
///
/// The byte obligations this pins: alias instances with identical sourcing
/// must persist identical bytes on every coprocessor, and the
/// representation-mixing recompute must be pinned to its own handle instead
/// of colliding.
contract AliasFixture is E2ECoprocessorConfig {
    euint64 public inputB;
    euint64 public inputC;
    euint64 public combined;
    euint64 public combinedSecond;
    euint64 public combinedLocal;

    /// @notice Produce and expose the two inputs (their handles become
    /// persisted, allowed boundaries).
    function produceInputs() external {
        inputB = FHE.asEuint64(7);
        inputC = FHE.asEuint64(5);
        _expose(inputB);
        _expose(inputC);
    }

    /// @notice Combine the two persisted inputs read from storage. Only the
    /// add is emitted; the inputs are cross-transaction boundaries.
    function combineFromStorage() external {
        combined = FHE.add(inputB, inputC);
        _expose(combined);
    }

    /// @notice Same operation, same boundary sourcing, different
    /// transaction: in the same block context this aliases
    /// `combineFromStorage`'s output — both instances (and every
    /// coprocessor) must persist identical bytes.
    function combineFromStorageAgain() external {
        combinedSecond = FHE.add(inputB, inputC);
        _expose(combinedSecond);
    }

    /// @notice Recompute the same inputs locally and combine, in one
    /// transaction. The local trivial encrypts land on `produceInputs`'
    /// handles (an input alias), but the add consumes operands minted in its
    /// own transaction, folds zero boundary bits, and mints a handle
    /// DISTINCT from `combined` — mixed sourcing can no longer collide. The
    /// local inputs are intentionally not exposed.
    function combineLocal() external {
        euint64 localB = FHE.asEuint64(7);
        euint64 localC = FHE.asEuint64(5);
        combinedLocal = FHE.add(localB, localC);
        _expose(combinedLocal);
    }

    function _expose(euint64 value) private {
        FHE.allowThis(value);
        FHE.allow(value, msg.sender);
        FHE.makePubliclyDecryptable(value);
    }
}
