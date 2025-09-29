// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/// @title MultichainACL smart contract
/// @dev source: github.com/zama-ai/fhevm-gateway/blob/main/contracts/MultichainACL.sol
/// @notice This contract is a mock of the MultichainACL contract from L2.
contract MultichainACL {
    error CoprocessorAlreadyAllowedAccount(bytes32 ctHandle, address account, address coprocessor);
    error CoprocessorAlreadyAllowedPublicDecrypt(bytes32 ctHandle, address txSender);

    event AllowAccount(bytes32 indexed ctHandle, address accountAddress);
    event AllowPublicDecrypt(bytes32 indexed ctHandle);

    bool alreadyAllowedRevert;

    constructor(bool _alreadyAllowedRevert) {
        alreadyAllowedRevert = _alreadyAllowedRevert;
    }

    function allowAccount(bytes32 ctHandle, address accountAddress, bytes calldata /* extraData */) public {
        if (alreadyAllowedRevert) {
            revert CoprocessorAlreadyAllowedAccount(ctHandle, accountAddress, msg.sender);
        }
        emit AllowAccount(ctHandle, accountAddress);
    }

    function allowPublicDecrypt(bytes32 ctHandle, bytes calldata /* extraData */) public {
        if (alreadyAllowedRevert) {
            revert CoprocessorAlreadyAllowedPublicDecrypt(ctHandle, msg.sender);
        }
        emit AllowPublicDecrypt(ctHandle);
    }
}
