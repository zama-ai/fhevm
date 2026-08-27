// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {ERC1967Proxy as OpenZeppelinERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract ERC1967Proxy is OpenZeppelinERC1967Proxy {
    constructor(address implementation, bytes memory data) payable OpenZeppelinERC1967Proxy(implementation, data) {}
}
