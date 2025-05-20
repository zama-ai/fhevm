// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/// @title MultichainAcl smart contract
/// @dev source: github.com/zama-ai/fhevm-gateway/blob/main/contracts/MultichainAcl.sol
/// @notice This contract is a mock of the MultichainAcl contract from L2.
contract MultichainAcl {
    error CoprocessorAlreadyAllowed(address coprocessor, bytes32 ctHandle);

    event AllowAccount(bytes32 indexed ctHandle, address accountAddress);
    event AllowPublicDecrypt(bytes32 indexed ctHandle);

    bool alreadyAllowedRevert;

    constructor(bool _alreadyAllowedRevert) {
        alreadyAllowedRevert = _alreadyAllowedRevert;
    }

    function allowAccount(bytes32 ctHandle, address accountAddress) public {
        if (alreadyAllowedRevert) {
            revert CoprocessorAlreadyAllowed(msg.sender, ctHandle);
        }
        emit AllowAccount(ctHandle, accountAddress);
    }

    function allowPublicDecrypt(bytes32 ctHandle) public {
        if (alreadyAllowedRevert) {
            revert CoprocessorAlreadyAllowed(msg.sender, ctHandle);
        }
        emit AllowPublicDecrypt(ctHandle);
    }
}
