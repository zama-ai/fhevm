// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {ERC1967Proxy} from "../../src/erc1967/ERC1967Proxy.sol";

/**
 * A uniquely-named ERC-1967 proxy for the test harness.
 *
 * `deployCodeTo` resolves an artifact by `"File.sol:Name"`, and both this package and OpenZeppelin ship an
 * `ERC1967Proxy.sol:ERC1967Proxy` — so the plain name is ambiguous ("multiple matching artifacts found").
 * This subclass gives the harness an unambiguous one; the behaviour is the package's own proxy.
 */
contract HarnessProxy is ERC1967Proxy {
    constructor(address implementation, bytes memory data) payable ERC1967Proxy(implementation, data) {}
}
