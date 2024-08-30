// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../lib/FHEPaymentAddress.sol";

interface IFHEPayment {
    function depositETH(address account) external payable;
    function withdrawETH(uint256 amount, address receiver) external;
    function getAvailableDepositsETH(address account) external view returns (uint256);
}

library Payment {
    function depositForAccount(address account, uint256 amount) internal {
        IFHEPayment(fhePaymentAdd).depositETH{value: amount}(account);
    }

    function depositForThis(uint256 amount) internal {
        IFHEPayment(fhePaymentAdd).depositETH{value: amount}(address(this));
    }

    function withdrawToAccount(address account, uint256 amount) internal {
        IFHEPayment(fhePaymentAdd).withdrawETH(amount, account);
    }

    function withdrawToThis(uint256 amount) internal {
        IFHEPayment(fhePaymentAdd).withdrawETH(amount, address(this));
    }

    function getDepositedBalanceOfAccount(address account) internal view returns (uint256) {
        return IFHEPayment(fhePaymentAdd).getAvailableDepositsETH(account);
    }

    function getDepositedBalanceOfThis() internal view returns (uint256) {
        return IFHEPayment(fhePaymentAdd).getAvailableDepositsETH(address(this));
    }
}
