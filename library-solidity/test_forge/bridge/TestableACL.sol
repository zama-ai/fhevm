// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {ACL} from "@fhevm-host-contracts/contracts/ACL.sol";

/**
 * @title  TestableACL
 * @notice Test-only ACL double that makes the ConfidentialBridge address runtime-settable.
 * @dev    Production `ACL` hardcodes the bridge as a `private constant` (`confidentialBridgeAdd`),
 *         which defaults to `address(0)` in the generated `host-contracts/addresses/FHEVMHostAddresses.sol`
 *         (only a real ConfidentialBridge deployment sets it non-zero). That constant is used in two places:
 *         `getConfidentialBridgeAddress()` (read by the cOApp base to resolve/authenticate the
 *         bridge) and the privileged `allowTransient` bypass (used by the bridge's `lzCompose`).
 * @dev    The bridge address is stored at a dedicated keccak slot to avoid colliding with the
 *         ACL's ERC-7201 namespaced storage.
 */
contract TestableACL is ACL {
    /// @dev Dedicated storage slot for the test bridge address (collision-free vs. ACL storage).
    bytes32 private constant _BRIDGE_SLOT = keccak256("fhevm.test.confidential.bridge.address");

    /// @notice Set the address treated as the trusted/privileged ConfidentialBridge.
    function setConfidentialBridgeAddressForTest(address bridge) external {
        bytes32 slot = _BRIDGE_SLOT;
        assembly {
            sstore(slot, bridge)
        }
    }

    /// @inheritdoc ACL
    function getConfidentialBridgeAddress() public view override returns (address bridge) {
        bytes32 slot = _BRIDGE_SLOT;
        assembly {
            bridge := sload(slot)
        }
    }

    /// @inheritdoc ACL
    /// @dev Mirrors {ACL-allowTransient} but grants the privileged bypass to the runtime-settable
    ///      bridge (in addition to the FHEVMExecutor path handled by `super`).
    function allowTransient(bytes32 handle, address account) public override {
        bytes32 slot = _BRIDGE_SLOT;
        address bridge;
        assembly {
            bridge := sload(slot)
        }
        if (msg.sender == bridge) {
            // Same transient-storage write as ACL.allowTransient's privileged branch.
            bytes32 key = keccak256(abi.encodePacked(handle, account));
            assembly {
                tstore(key, 1)
                let length := tload(0)
                let lengthPlusOne := add(length, 1)
                tstore(lengthPlusOne, key)
                tstore(0, lengthPlusOne)
            }
        } else {
            super.allowTransient(handle, account);
        }
    }
}
