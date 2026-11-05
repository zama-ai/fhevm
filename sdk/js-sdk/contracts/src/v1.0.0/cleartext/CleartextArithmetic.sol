// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {UUPSUpgradeableEmptyProxy} from "../host-contracts/contracts/shared/UUPSUpgradeableEmptyProxy.sol";
import {ACLOwnable} from "../host-contracts/contracts/shared/ACLOwnable.sol";
import {CleartextArithmeticBase} from "./shared/CleartextArithmeticBase.sol";
import {cleartextDbAdd} from "../host-contracts/addresses/FHEVMHostAddresses.sol";

/**
 * @title CleartextArithmetic
 * @notice Cleartext computation + persistence layer for the execution mocks. Deployed as a
 *         standalone upgradeable contract (rather than an inlined library) so its bytecode does not
 *         count against `CleartextFHEVMExecutor`'s EIP-170 size limit. It reads operand cleartexts
 *         from `CleartextDB`, computes results mirroring FHE bit-width semantics, and writes them
 *         back — the executor never touches the DB.
 * @dev The math is `pure` (no local storage); the `record*` entry points perform external DB
 *      reads/writes. This contract is the sole writer registered in `CleartextDB`.
 */
/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
contract CleartextArithmetic is CleartextArithmeticBase, UUPSUpgradeableEmptyProxy, ACLOwnable {
    /// @dev Name of the contract, used in `getVersion`.
    string private constant CONTRACT_NAME = "CleartextArithmetic";

    /// @dev Major version of the contract.
    uint256 private constant MAJOR_VERSION = 0;

    /// @dev Minor version of the contract.
    uint256 private constant MINOR_VERSION = 6;

    /// @dev Patch version of the contract.
    uint256 private constant PATCH_VERSION = 0;

    /**
     * @dev Constant used for making sure the version number used in the `reinitializer` modifier is
     * identical between `initializeFromEmptyProxy` and any future `reinitializeVX` method.
     */
    uint64 private constant REINITIALIZER_VERSION = 5;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /// @notice Initializes the contract from an empty proxy. No state to set (stateless/pure math).
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy() public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {}

    /// @notice Re-initializer for the V(N-1)→V(N) upgrade, which adds the RFC-030 pad-based input unmasking to
    ///         `recordVerifyInput`. Stateless (pure math), so it only marks the freshly deployed
    ///         implementation initialized.
    function reinitializeV4() public virtual onlyACLOwner reinitializer(REINITIALIZER_VERSION) {}

    /// @notice Getter for the name and version of the contract.
    /// @return string Name and the version of the contract.
    function getVersion() external pure virtual returns (string memory) {
        return string(
            abi.encodePacked(
                CONTRACT_NAME,
                " v",
                Strings.toString(MAJOR_VERSION),
                ".",
                Strings.toString(MINOR_VERSION),
                ".",
                Strings.toString(PATCH_VERSION)
            )
        );
    }

    /// @dev Should revert when `msg.sender` is not authorized to upgrade the contract.
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyACLOwner {}

    /// @inheritdoc CleartextArithmeticBase
    /// @dev The host's shared store, at its well-known address. This is the only thing the base needs
    ///      from a deployment, which is why the address constant is imported HERE and not there.
    function getCleartextDBAddress() public view virtual override returns (address) {
        return address(cleartextDbAdd);
    }
}
