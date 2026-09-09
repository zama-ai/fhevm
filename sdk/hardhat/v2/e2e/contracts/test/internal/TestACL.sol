// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {FHE, euint32, externalEuint32, FheType} from "@fhevm/solidity/lib/FHE.sol";
import {CoprocessorConfig, IFHEVMExecutor, IACL} from "@fhevm/solidity/lib/Impl.sol";
import {ZamaEthereumConfig} from "@fhevm/solidity/config/ZamaConfig.sol";

contract TestACL is ZamaEthereumConfig {
    euint32 private _count;

    /// keccak256(abi.encode(uint256(keccak256("confidential.storage.config")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant CoprocessorConfigLocation =
        0x9e7b61f58c47dc699ac88507c4f5bb9f121c03808c5676a8078fe583e4649700;

    function getCount() external view returns (euint32) {
        return _count;
    }

    function getCoprocessorConfig2() internal pure returns (CoprocessorConfig storage $) {
        assembly {
            $.slot := CoprocessorConfigLocation
        }
    }

    function verify2(address callerAddress, bytes32 inputHandle, bytes memory inputProof, FheType toType)
        internal
        returns (bytes32 result)
    {
        CoprocessorConfig storage $ = getCoprocessorConfig2();
        result = IFHEVMExecutor($.CoprocessorAddress).verifyInput(inputHandle, callerAddress, inputProof, toType);
        IACL($.ACLAddress).allowTransient(result, msg.sender);
    }

    function fromExternal2(address callerAddress, externalEuint32 inputEuint32, bytes memory inputProof)
        internal
        returns (euint32)
    {
        return euint32.wrap(verify2(callerAddress, externalEuint32.unwrap(inputEuint32), inputProof, FheType.Uint32));
    }

    function increment1(externalEuint32 inputEuint32, bytes calldata inputProof) external {
        euint32 encryptedEuint32 = FHE.fromExternal(inputEuint32, inputProof);

        _count = FHE.add(_count, encryptedEuint32);

        FHE.allowThis(_count);
        FHE.allow(_count, msg.sender);
    }

    function increment2(address userAddress, externalEuint32 inputEuint32, bytes calldata inputProof) external {
        euint32 encryptedEuint32 = fromExternal2(userAddress, inputEuint32, inputProof);

        _count = FHE.add(_count, encryptedEuint32);

        FHE.allowThis(_count);
        FHE.allow(_count, msg.sender);
        FHE.allow(_count, userAddress);
    }
}
