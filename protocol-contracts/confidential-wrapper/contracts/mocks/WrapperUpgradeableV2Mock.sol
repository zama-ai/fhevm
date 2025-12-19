// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {WrapperUpgradeable} from "../wrapper/WrapperUpgradeable.sol";

contract WrapperUpgradeableV2Mock is WrapperUpgradeable {
    /// @custom:storage-location erc7201:zaiffer.storage.WrapperV2
    struct WrapperV2Storage {
        uint64 counter;
    }

    // keccak256(abi.encode(uint256(keccak256("zaiffer.storage.WrapperV2")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant WrapperV2StorageLocation =
        0x8c8b7b1e3e3e3e3e3e3e3e3e3e3e3e3e3e3e3e3e3e3e3e3e3e3e3e3e3e3e3e00;

    function _getWrapperStorageV2() internal pure returns (WrapperV2Storage storage $) {
        assembly {
            $.slot := WrapperV2StorageLocation
        }
    }

    function incrementCounter() public returns (uint64) {
        WrapperV2Storage storage $ = _getWrapperStorageV2();
        $.counter += 1;
        return $.counter;
    }

    function counter() public view returns (uint64) {
        WrapperV2Storage storage $ = _getWrapperStorageV2();
        return $.counter;
    }
}
