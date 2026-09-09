// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FHE, euint8, externalEuint8} from "@fhevm/solidity/lib/FHE.sol";
import {ZamaEthereumConfig} from "@fhevm/solidity/config/ZamaConfig.sol";

contract APlusB is ZamaEthereumConfig {
    euint8 private _a;
    euint8 private _b;
    euint8 private _aplusb;

    constructor() {}

    function setA(externalEuint8 inputA, bytes calldata inputProof) external {
        _a = FHE.fromExternal(inputA, inputProof);
        FHE.allowThis(_a);
    }

    function setB(externalEuint8 inputB, bytes calldata inputProof) external {
        _b = FHE.fromExternal(inputB, inputProof);
        FHE.allowThis(_b);
    }

    function computeAPlusB() external {
        _aplusb = FHE.add(_a, _b);
        FHE.allowThis(_aplusb);
        FHE.allow(_aplusb, msg.sender);
    }

    function aplusb() public view returns (euint8) {
        return _aplusb;
    }
}
