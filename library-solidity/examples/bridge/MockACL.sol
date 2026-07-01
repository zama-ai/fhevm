// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/**
 * @title   MockACL
 * @notice  Minimal stand-in for the host ACL exposing only the surface `FHE.bridge` reads:
 *          {getConfidentialBridgeAddress}. Lets the wrapper resolve a test bridge without
 *          spinning up the full FHEVM host stack.
 * @dev     Test-only; not part of the published library. The real ACL returns a fixed bridge
 *          address, but this mock makes it settable so a test can point the wrapper at a
 *          {MockConfidentialBridge} — or at address(0) to check the "no bridge configured"
 *          (`BridgeNotConfigured`) revert. It also stubs {isAllowed} (settable per handle/account)
 *          so the safe `ConfidentialOAppSender._bridgeFrom` path (`FHE.isSenderAllowed`) is testable.
 */
contract MockACL {
    address public confidentialBridge;

    mapping(bytes32 handle => mapping(address account => bool)) private _allowed;

    function setConfidentialBridgeAddress(address bridge) external {
        confidentialBridge = bridge;
    }

    function getConfidentialBridgeAddress() external view returns (address) {
        return confidentialBridge;
    }

    /// @notice Grant or revoke `account`'s allowance on `handle` (drives `FHE.isSenderAllowed`).
    function setAllowed(bytes32 handle, address account, bool value) external {
        _allowed[handle][account] = value;
    }

    function isAllowed(bytes32 handle, address account) external view returns (bool) {
        return _allowed[handle][account];
    }
}
