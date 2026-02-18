// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {ACL} from "../../../../contracts/ACL.sol";
import {fhevmExecutorAdd} from "../../../../addresses/FHEVMHostAddresses.sol";

contract ACLPermissionsHandler is Test {
    uint256 internal constant MAX_TRACKED_PAIRS = 24;

    ACL internal immutable acl;
    address internal immutable owner;
    address internal immutable pauser;
    address internal immutable actor1;
    address internal immutable actor2;
    address internal immutable actor3;

    bool public privilegeViolation;
    bool public deniedBypassViolation;
    bool public pausedBypassViolation;
    bool public senderPermissionBypassViolation;
    bool public transientVisibilityViolation;

    bytes32[MAX_TRACKED_PAIRS] internal successfulAllowHandles;
    address[MAX_TRACKED_PAIRS] internal successfulAllowAccounts;
    uint256 public successfulAllowCount;

    bytes32[MAX_TRACKED_PAIRS] internal successfulDecryptHandles;
    uint256 public successfulDecryptHandleCount;

    constructor(ACL _acl, address _owner, address _pauser, address _actor1, address _actor2, address _actor3) {
        acl = _acl;
        owner = _owner;
        pauser = _pauser;
        actor1 = _actor1;
        actor2 = _actor2;
        actor3 = _actor3;
    }

    function allow(bytes32 handle, uint8 accountSeed, bool prepareSender) external {
        address account = _seededAddress(accountSeed, 220);
        bool wasPaused = acl.paused();

        if (prepareSender && !wasPaused) {
            _prepareSenderHandle(handle, msg.sender);
        }
        bool senderAllowed = acl.isAllowed(handle, msg.sender);

        bool wasDenied = acl.isAccountDenied(msg.sender);
        vm.prank(msg.sender);
        (bool ok, ) = address(acl).call(abi.encodeCall(ACL.allow, (handle, account)));
        if (ok && wasDenied) {
            deniedBypassViolation = true;
        }
        if (ok && wasPaused) {
            pausedBypassViolation = true;
        }
        if (ok && !senderAllowed) {
            senderPermissionBypassViolation = true;
        }
        if (ok) {
            _trackSuccessfulAllow(handle, account);
        }
    }

    function allowTransient(bytes32 handle, uint8 accountSeed, bool prepareSender) external {
        address account = _seededAddress(accountSeed, 250);
        bool wasPaused = acl.paused();

        if (prepareSender && !wasPaused) {
            _prepareSenderHandle(handle, msg.sender);
        }
        bool senderAllowed = acl.isAllowed(handle, msg.sender);

        bool wasDenied = acl.isAccountDenied(msg.sender);
        vm.prank(msg.sender);
        (bool ok, ) = address(acl).call(abi.encodeCall(ACL.allowTransient, (handle, account)));
        if (ok && wasDenied) {
            deniedBypassViolation = true;
        }
        if (ok && wasPaused) {
            pausedBypassViolation = true;
        }
        if (ok && !senderAllowed) {
            senderPermissionBypassViolation = true;
        }
        if (ok && !acl.isAllowed(handle, account)) {
            transientVisibilityViolation = true;
        }
    }

    function allowForDecryption(uint8 handleCountSeed, uint256 handleSeed, bool prepareSender) external {
        uint256 len = bound(uint256(handleCountSeed), 1, 3);
        bool wasPaused = acl.paused();
        bytes32[] memory handles = new bytes32[](len);
        bool senderAllowedAll = true;
        for (uint256 i = 0; i < len; i++) {
            handles[i] = keccak256(abi.encode(handleSeed, i));
            if (prepareSender && !wasPaused) {
                _prepareSenderHandle(handles[i], msg.sender);
            }
            if (!acl.isAllowed(handles[i], msg.sender)) {
                senderAllowedAll = false;
            }
        }

        bool wasDenied = acl.isAccountDenied(msg.sender);
        vm.prank(msg.sender);
        (bool ok, ) = address(acl).call(abi.encodeCall(ACL.allowForDecryption, (handles)));
        if (ok && wasDenied) {
            deniedBypassViolation = true;
        }
        if (ok && wasPaused) {
            pausedBypassViolation = true;
        }
        if (ok && !senderAllowedAll) {
            senderPermissionBypassViolation = true;
        }
        if (ok) {
            for (uint256 i = 0; i < handles.length; i++) {
                _trackSuccessfulDecryptHandle(handles[i]);
            }
        }
    }

    function pause() external {
        vm.prank(msg.sender);
        (bool ok, ) = address(acl).call(abi.encodeCall(ACL.pause, ()));
        if (ok && !acl.isPauser(msg.sender)) {
            privilegeViolation = true;
        }
    }

    function unpause() external {
        vm.prank(msg.sender);
        (bool ok, ) = address(acl).call(abi.encodeCall(ACL.unpause, ()));
        if (ok && msg.sender != owner) {
            privilegeViolation = true;
        }
    }

    function blockAccount(uint8 accountSeed) external {
        address account = _actorFromSeed(accountSeed);
        vm.prank(msg.sender);
        (bool ok, ) = address(acl).call(abi.encodeCall(ACL.blockAccount, (account)));
        if (ok && msg.sender != owner) {
            privilegeViolation = true;
        }
    }

    function unblockAccount(uint8 accountSeed) external {
        address account = _actorFromSeed(accountSeed);
        vm.prank(msg.sender);
        (bool ok, ) = address(acl).call(abi.encodeCall(ACL.unblockAccount, (account)));
        if (ok && msg.sender != owner) {
            privilegeViolation = true;
        }
    }

    function cleanTransientStorage() external {
        vm.prank(msg.sender);
        acl.cleanTransientStorage();
    }

    function getSuccessfulAllowPair(uint256 index) external view returns (bytes32 handle, address account) {
        return (successfulAllowHandles[index], successfulAllowAccounts[index]);
    }

    function getSuccessfulDecryptHandle(uint256 index) external view returns (bytes32 handle) {
        return successfulDecryptHandles[index];
    }

    function _prepareSenderHandle(bytes32 handle, address sender) internal {
        vm.prank(fhevmExecutorAdd);
        acl.allowTransient(handle, sender);
        vm.prank(sender);
        (bool ok, ) = address(acl).call(abi.encodeCall(ACL.allow, (handle, sender)));
        if (ok) {
            vm.prank(sender);
            acl.cleanTransientStorage();
        }
    }

    function _trackSuccessfulAllow(bytes32 handle, address account) internal {
        for (uint256 i = 0; i < successfulAllowCount; i++) {
            if (successfulAllowHandles[i] == handle && successfulAllowAccounts[i] == account) {
                return;
            }
        }

        if (successfulAllowCount < MAX_TRACKED_PAIRS) {
            successfulAllowHandles[successfulAllowCount] = handle;
            successfulAllowAccounts[successfulAllowCount] = account;
            successfulAllowCount++;
        }
    }

    function _trackSuccessfulDecryptHandle(bytes32 handle) internal {
        for (uint256 i = 0; i < successfulDecryptHandleCount; i++) {
            if (successfulDecryptHandles[i] == handle) {
                return;
            }
        }

        if (successfulDecryptHandleCount < MAX_TRACKED_PAIRS) {
            successfulDecryptHandles[successfulDecryptHandleCount] = handle;
            successfulDecryptHandleCount++;
        }
    }

    function _seededAddress(uint8 seed, uint160 offset) internal pure returns (address) {
        return address(offset + uint160(seed) + 1);
    }

    function _actorFromSeed(uint8 seed) internal view returns (address) {
        uint8 role = seed % 5;
        if (role == 0) {
            return owner;
        }
        if (role == 1) {
            return pauser;
        }
        if (role == 2) {
            return actor1;
        }
        if (role == 3) {
            return actor2;
        }
        return actor3;
    }
}
