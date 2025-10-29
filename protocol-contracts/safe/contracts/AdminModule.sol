// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.0;
import "@safe-global/safe-contracts/contracts/common/Enum.sol";
import "@safe-global/safe-contracts/contracts/SafeL2.sol";

contract AdminModule {
    address immutable public ADMIN_ACCOUNT;
    address payable immutable public SAFE_PROXY;

    error SenderIsNotAdmin();
    error CouldNotExecuteSafeTx(bytes errorData);

    constructor(address _adminAccount, address payable safeProxy){
        ADMIN_ACCOUNT = _adminAccount;
        SAFE_PROXY = safeProxy;
    }

    function execTransactionFromModuleReturnData(
        address target,
        uint256 value,
        bytes memory data,
        Enum.Operation operation
    ) external {
        if(msg.sender != ADMIN_ACCOUNT) revert SenderIsNotAdmin();

        (bool success, bytes memory returnData) = SafeL2(SAFE_PROXY).execTransactionFromModuleReturnData(
                target,
                value,
                data,
                operation
            );
        if(!success) revert CouldNotExecuteSafeTx(returnData);
    }
}
