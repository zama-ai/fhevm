// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import "../shared/Structs.sol";
import "../shared/Enums.sol";

contract MultichainACLMock {
    event AllowAccount(bytes32 indexed ctHandle, address accountAddress);

    event AllowPublicDecrypt(bytes32 indexed ctHandle);

    function allowPublicDecrypt(bytes32 ctHandle, bytes calldata /* unusedVariable */) external {
        emit AllowPublicDecrypt(ctHandle);
    }

    function allowAccount(bytes32 ctHandle, address accountAddress, bytes calldata /* unusedVariable */) external {
        emit AllowAccount(ctHandle, accountAddress);
    }
}
