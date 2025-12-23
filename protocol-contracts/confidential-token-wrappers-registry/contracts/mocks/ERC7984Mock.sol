// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {ERC165} from "@openzeppelin/contracts/utils/introspection/ERC165.sol";
import {IERC7984} from "@openzeppelin/confidential-contracts/interfaces/IERC7984.sol";

/**
 * @title ERC7984Mock
 * @notice Minimal mock contract that implements ERC7984 interface for testing purposes.
 */
contract ERC7984Mock is ERC165 {
    /**
     * @dev See {IERC165-supportsInterface}.
     */
    function supportsInterface(bytes4 interfaceId) public view virtual override returns (bool) {
        return interfaceId == type(IERC7984).interfaceId || super.supportsInterface(interfaceId);
    }
}
