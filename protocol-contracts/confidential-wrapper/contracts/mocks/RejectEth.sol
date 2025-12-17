// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

contract RejectEth {
    receive() external payable {
        revert("ETH transfers rejected");
    }
    
    fallback() external payable {
        revert("ETH transfers rejected");
    }
}