// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import {E2ECoprocessorConfig} from "../E2ECoprocessorConfigLocal.sol";

/// @notice Dapp exposing one function per kind of HCU-charged operation, plus
/// functions that only verify inputs and manage ACL permissions.
contract HCUAllowlistDapp is E2ECoprocessorConfig {
    euint64 public result;
    euint32 public result32;
    ebool public resultBool;
    euint64 public stored;

    function _publish(euint64 value) private {
        result = value;
        FHE.allowThis(value);
        FHE.makePubliclyDecryptable(value);
    }

    // --- HCU-charged operations ---

    function trivialEncrypt(uint64 value) external {
        _publish(FHE.asEuint64(value));
    }

    function addTrivialValues(uint64 a, uint64 b) external {
        _publish(FHE.add(FHE.asEuint64(a), FHE.asEuint64(b)));
    }

    function addToInput(externalEuint64 inputHandle, bytes calldata inputProof, uint64 value) external {
        euint64 sum = FHE.add(FHE.fromExternal(inputHandle, inputProof), FHE.asEuint64(value));
        FHE.allow(sum, msg.sender);
        _publish(sum);
    }

    function addScalarToStored(uint64 value) external {
        _publish(FHE.add(stored, value));
    }

    function notStored() external {
        _publish(FHE.not(stored));
    }

    function castStored() external {
        result32 = FHE.asEuint32(stored);
        FHE.allowThis(result32);
    }

    function selectStored() external {
        _publish(FHE.select(FHE.asEbool(true), stored, stored));
    }

    function random() external {
        _publish(FHE.randEuint64());
    }

    function randomBounded(uint64 upperBound) external {
        _publish(FHE.randEuint64(upperBound));
    }

    function sumStored() external {
        euint64[] memory values = new euint64[](2);
        values[0] = stored;
        values[1] = stored;
        _publish(FHE.sum(values));
    }

    function isInStored() external {
        euint64[] memory values = new euint64[](1);
        values[0] = stored;
        resultBool = FHE.isIn(stored, values);
        FHE.allowThis(resultBool);
    }

    function mulDivStored(uint64 multiplier, uint64 divisor) external {
        _publish(FHE.mulDiv(stored, multiplier, divisor));
    }

    // --- Operations that are not HCU-charged ---

    function storeInput(externalEuint64 inputHandle, bytes calldata inputProof) external {
        stored = FHE.fromExternal(inputHandle, inputProof);
        FHE.allowThis(stored);
        FHE.allow(stored, msg.sender);
    }

    function publishStored() external {
        FHE.makePubliclyDecryptable(stored);
    }
}

/// @notice Dapp that computes in its constructor.
contract HCUAllowlistConstructorDapp is E2ECoprocessorConfig {
    euint64 public value;

    constructor(uint64 initialValue) {
        value = FHE.asEuint64(initialValue);
        FHE.allowThis(value);
        FHE.makePubliclyDecryptable(value);
    }
}

/// @notice Factory that computes itself and deploys dapps.
contract HCUAllowlistFactory is E2ECoprocessorConfig {
    euint64 public result;

    event DappDeployed(address dapp);

    function compute(uint64 value) external {
        result = FHE.asEuint64(value);
        FHE.allowThis(result);
        FHE.makePubliclyDecryptable(result);
    }

    function deployDapp() external {
        emit DappDeployed(address(new HCUAllowlistDapp()));
    }

    function deployConstructorDapp(bytes32 salt, uint64 initialValue) external {
        emit DappDeployed(address(new HCUAllowlistConstructorDapp{salt: salt}(initialValue)));
    }

    function predictConstructorDapp(bytes32 salt, uint64 initialValue) external view returns (address) {
        bytes32 initCodeHash = keccak256(
            abi.encodePacked(type(HCUAllowlistConstructorDapp).creationCode, abi.encode(initialValue))
        );
        return address(uint160(uint256(keccak256(abi.encodePacked(bytes1(0xff), address(this), salt, initCodeHash)))));
    }
}
