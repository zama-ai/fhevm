// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.27;

import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {ERC7984ERC20WrapperUpgradeable} from "./extensions/ERC7984ERC20WrapperUpgradeable.sol";
import {ZamaEthereumConfigUpgradeable} from "./fhevm/ZamaEthereumConfigUpgradeable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";
import {Ownable2StepUpgradeable} from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";

/**
 * @title ConfidentialWrapper
 * @dev An upgradeable wrapper contract built on top of {ERC7984Upgradeable} that allows wrapping an `ERC20` token
 * into an `ERC7984` token. The wrapper contract implements the `IERC1363Receiver` interface
 * which allows users to transfer `ERC1363` tokens directly to the wrapper with a callback to wrap the tokens.
 *
 * WARNING: Minting assumes the full amount of the underlying token transfer has been received, hence some non-standard
 * tokens such as fee-on-transfer or other deflationary-type tokens are not supported by this wrapper.
 */
contract ConfidentialWrapper is
    ERC7984ERC20WrapperUpgradeable,
    ZamaEthereumConfigUpgradeable,
    UUPSUpgradeable,
    Ownable2StepUpgradeable
{
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function initialize(
        string memory name_,
        string memory symbol_,
        string memory contractURI_,
        IERC20 underlying_,
        address owner_
    ) public initializer {
        __ERC7984_init(name_, symbol_, contractURI_);
        __ERC7984ERC20Wrapper_init(underlying_);
        __ZamaEthereumConfig_init();
        __Ownable_init(owner_);
        __Ownable2Step_init();
    }

    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {}
}
