// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../../lib/FHE.sol";
import {CoprocessorSetup} from "../CoprocessorSetup.sol";

interface IMultiSig {
    function getOwners() external view returns (address[] memory);
}

contract MultiSigHelper {
    IMultiSig public immutable multiSig;

    constructor(address _multiSig) {
        FHE.setCoprocessor(CoprocessorSetup.defaultConfig());
        multiSig = IMultiSig(_multiSig);
    }

    function allowForMultiSig(externalEuint64 inputHandle, bytes memory inputProof) external {
        euint64 handle = FHE.fromExternal(inputHandle, inputProof);
        FHE.allow(handle, address(multiSig));
        address[] memory owners = getMultiSigOwners();
        uint256 numOwners = owners.length;
        for (uint256 i; i < numOwners; i++) {
            FHE.allow(handle, owners[i]);
        }
    }

    function getMultiSigOwners() internal view returns (address[] memory) {
        return multiSig.getOwners();
    }
}
