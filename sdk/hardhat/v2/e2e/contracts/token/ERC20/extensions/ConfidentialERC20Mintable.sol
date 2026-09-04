// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Ownable2Step, Ownable} from "@openzeppelin/contracts/access/Ownable2Step.sol";
import {ConfidentialERC20} from "../ConfidentialERC20.sol";

/**
 * @title   ConfidentialERC20Mintable.
 * @notice  This contract inherits ConfidentialERC20.
 * @dev     It allows an owner to mint tokens. Mint amounts are not encrypted.
 */
abstract contract ConfidentialERC20Mintable is Ownable2Step, ConfidentialERC20 {
    /**
     * @notice Emitted when `amount` tokens are minted to one account (`to`).
     */
    event Mint(address indexed to, uint64 amount);

    /**
     * @param name_     Name of the token.
     * @param symbol_   Symbol.
     * @param owner_    Owner address.
     */
    constructor(string memory name_, string memory symbol_, address owner_)
        Ownable(owner_)
        ConfidentialERC20(name_, symbol_)
    {}

    /**
     * @notice          Mint tokens.
     * @param to        Address to mint tokens to.
     * @param amount    Amount of tokens to mint.
     */
    function mint(address to, uint64 amount) public virtual onlyOwner {
        _unsafeMint(to, amount);
        /// @dev Since _totalSupply is not encrypted and we ensure there is no underflow/overflow of encrypted balances
        /// during transfers, making _totalSupply invariant during transfers, we know _totalSupply is greater than
        /// all individual balances. Hence, the next line forbids any overflow to happen in the _unsafeMint above.
        _totalSupply = _totalSupply + amount;
        emit Mint(to, amount);
    }
}
