// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.27;

import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

/**
 * A named ERC1967Proxy so Foundry's `deployCodeTo` can find an artifact for it. `deployCodeTo` resolves a
 * contract by `"File.sol:Name"`, which needs a concrete contract compiled into THIS project's `out/`.
 */
contract DeployableERC1967Proxy is ERC1967Proxy {
    constructor(address implementation, bytes memory data) ERC1967Proxy(implementation, data) {}
}
