// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.22;

import { ERC20Burnable } from "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";

/**
 * @title ProtocolFeesBurner Contract
 * @dev ProtocolFeesBurner is a contract that should be deployed on Ethereum chain.
 * @dev Most of the ZAMA fees are sent to this contract via LayerZero from FeesSenderToBurner contract.
 * @dev Anyone can call burnFees() function.
 */
contract ProtocolFeesBurner {
    ERC20Burnable public immutable ZAMA_ERC20;

    event FeesBurned(uint256 amount);

    constructor(address _token) {
        ZAMA_ERC20 = ERC20Burnable(_token);
    }

    /**
     * @notice burn all the ZAMA tokens currently owned by the contract.
     **/
    function burnFees() external {
        uint256 feesToBurn = ZAMA_ERC20.balanceOf(address(this));
        ZAMA_ERC20.burn(feesToBurn);
        emit FeesBurned(feesToBurn);
    }
}
