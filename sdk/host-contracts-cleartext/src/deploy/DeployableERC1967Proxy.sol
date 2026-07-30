// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

/**
 * A UNIQUELY named ERC1967Proxy so Foundry's `deployCodeTo` can resolve an artifact for it. `deployCodeTo`
 * looks a contract up by `"File.sol:Name"` in the consuming project's `out/` — and `ERC1967Proxy.sol:ERC1967Proxy`
 * is ambiguous there, because OpenZeppelin's own proxy (and this package's `erc1967/ERC1967Proxy.sol`) carry
 * exactly that name.
 */
contract DeployableERC1967Proxy is ERC1967Proxy {
    constructor(address implementation, bytes memory data) ERC1967Proxy(implementation, data) {}
}
