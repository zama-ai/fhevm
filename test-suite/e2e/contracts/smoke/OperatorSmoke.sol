// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import {E2ECoprocessorConfig} from "../E2ECoprocessorConfigLocal.sol";

contract OperatorSmoke is E2ECoprocessorConfig {
    ebool public boolResult;
    euint32 public uint32Result;
    euint64 public uint64Result;

    function add(
        externalEuint64 lhs,
        externalEuint64 rhs,
        bytes calldata inputProof
    ) external {
        uint64Result = FHE.add(FHE.fromExternal(lhs, inputProof), FHE.fromExternal(rhs, inputProof));
        FHE.makePubliclyDecryptable(uint64Result);
    }

    function div(externalEuint64 lhs, uint64 rhs, bytes calldata inputProof) external {
        uint64Result = FHE.div(FHE.fromExternal(lhs, inputProof), rhs);
        FHE.makePubliclyDecryptable(uint64Result);
    }

    function lt(
        externalEuint64 lhs,
        externalEuint64 rhs,
        bytes calldata inputProof
    ) external {
        boolResult = FHE.lt(FHE.fromExternal(lhs, inputProof), FHE.fromExternal(rhs, inputProof));
        FHE.makePubliclyDecryptable(boolResult);
    }

    function neg(externalEuint32 value, bytes calldata inputProof) external {
        uint32Result = FHE.neg(FHE.fromExternal(value, inputProof));
        FHE.makePubliclyDecryptable(uint32Result);
    }
}
