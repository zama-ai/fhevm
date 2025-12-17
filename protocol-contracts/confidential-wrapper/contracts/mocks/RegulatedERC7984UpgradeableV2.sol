// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {RegulatedERC7984Upgradeable} from "../token/RegulatedERC7984Upgradeable.sol";

contract RegulatedERC7984UpgradeableV2Mock is RegulatedERC7984Upgradeable {
    /// @custom:storage-location erc7201:zaiffer.storage.RegulatedERC7984V2
    struct RegulatedERC7984V2Storage {
        uint64 counter;
    }

    // keccak256(abi.encode(uint256(keccak256("zaiffer.storage.RegulatedERC7984V2")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant RegulatedERC7984V2StorageLocation =
        0x190df7ff214060257e3b73924d834d06c896c5f08a7237d4c441916b0d675500;

    function _getRegulatedERC7984StorageV2() internal pure returns (RegulatedERC7984V2Storage storage $) {
        assembly {
            $.slot := RegulatedERC7984V2StorageLocation
        }
    }

    function incrementCounter() public returns (uint64) {
        RegulatedERC7984V2Storage storage $ = _getRegulatedERC7984StorageV2();
        $.counter += 1;
        return $.counter;
    }

    function counter() public view returns (uint64) {
        RegulatedERC7984V2Storage storage $ = _getRegulatedERC7984StorageV2();
        return $.counter;
    }
}
