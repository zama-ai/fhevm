// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.27;

import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

/// @dev Local proxy artifact used by FhevmTest.deployCodeTo() so consumers do not
/// need to pull OpenZeppelin's ERC1967Proxy into their own compile graph.
contract CleartextTestProxy is ERC1967Proxy {
    constructor(address implementation, bytes memory data) ERC1967Proxy(implementation, data) {}
}
