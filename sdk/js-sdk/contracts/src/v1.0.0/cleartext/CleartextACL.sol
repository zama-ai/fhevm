// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {ACL} from "../host-contracts/contracts/ACL.sol";

/**
 * @title CleartextACL
 * @notice The ACL of a cleartext stack, carrying the markers that identify one.
 * @dev Behaviourally identical to `ACL`: it adds two constants and overrides nothing. A real deployment
 *      has neither selector, so a consumer can probe either one to tell a cleartext stack from a
 *      production deployment, and `CLEARTEXT_PROTOCOL_VERSION` says which generation it is talking to
 *      without parsing the version strings `getVersion()` returns.
 *
 *      Constants rather than storage: these run behind proxies, where an initialized state variable is
 *      only ever written to the implementation's own storage and reads back as `false` or zero through
 *      the proxy.
 */
/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
contract CleartextACL is ACL {
    /// @notice Marks a cleartext (mock) implementation. Real host contracts have no such selector, so a
    ///         consumer can probe it to tell a cleartext stack from a production deployment.
    bool public constant IS_CLEARTEXT = true;

    /// @notice The host-contracts generation this cleartext stack implements. Below 1.0 it was the minor
    ///         (13, 14); from 1.0 on it is `major * 1_000_000 + minor * 1_000 + patch`. Compare generations,
    ///         not values: a patch that leaves this contract alone does not move it.
    uint256 public constant CLEARTEXT_PROTOCOL_VERSION = 1_000_000;
}
