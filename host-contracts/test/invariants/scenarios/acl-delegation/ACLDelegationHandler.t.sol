// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {ACL} from "../../../../contracts/ACL.sol";
import {fhevmExecutorAdd} from "../../../../addresses/FHEVMHostAddresses.sol";

contract ACLDelegationHandler is Test {
    uint256 internal constant MAX_TRACKED_TUPLES = 24;
    uint8 internal constant STATUS_ACTIVE = 1;
    uint8 internal constant STATUS_REVOKED = 2;

    ACL internal immutable acl;
    address internal immutable owner;
    address internal immutable pauser;
    address internal immutable actor1;
    address internal immutable actor2;
    address internal immutable actor3;

    bool public pausedBypassViolation;
    bool public privilegeViolation;

    address[MAX_TRACKED_TUPLES] internal trackedDelegators;
    address[MAX_TRACKED_TUPLES] internal trackedDelegates;
    address[MAX_TRACKED_TUPLES] internal trackedContractAddresses;
    bytes32[MAX_TRACKED_TUPLES] internal trackedDelegationHandles;
    uint8[MAX_TRACKED_TUPLES] internal trackedDelegationStatuses;
    uint256 public trackedDelegationCount;
    uint256 internal trackedDelegationWriteIndex;
    uint256 public successfulDelegationCount;
    uint256 public successfulRevocationCount;

    constructor(ACL _acl, address _owner, address _pauser, address _actor1, address _actor2, address _actor3) {
        acl = _acl;
        owner = _owner;
        pauser = _pauser;
        actor1 = _actor1;
        actor2 = _actor2;
        actor3 = _actor3;
    }

    function delegateForUserDecryption(
        uint8 delegateSeed,
        uint8 contractSeed,
        uint8 hoursAheadSeed,
        uint256 handleSeed
    ) external {
        address delegate = _actorFromSeed(delegateSeed);
        address contractAddress = _actorFromSeed(uint8(uint256(contractSeed) + 11));

        if (delegate == msg.sender) {
            delegate = address(uint160(delegate) + 1000);
        }
        if (contractAddress == msg.sender || contractAddress == delegate) {
            contractAddress = address(uint160(contractAddress) + 2000);
        }
        bytes32 handle = keccak256(abi.encode(handleSeed));

        uint64 expirationDate = uint64(block.timestamp + bound(uint256(hoursAheadSeed), 1, 72) * 1 hours);
        bool wasPaused = acl.paused();
        if (!wasPaused) {
            _prepareSenderHandle(handle, msg.sender);
            _prepareSenderHandle(handle, contractAddress);
        }
        vm.prank(msg.sender);
        (bool ok, ) = address(acl).call(
            abi.encodeCall(ACL.delegateForUserDecryption, (delegate, contractAddress, expirationDate))
        );
        if (ok) {
            if (wasPaused) {
                pausedBypassViolation = true;
            } else {
                _upsertDelegation(msg.sender, delegate, contractAddress, handle, STATUS_ACTIVE);
                successfulDelegationCount++;
                vm.roll(block.number + 1);
            }
        }
    }

    function revokeTrackedDelegationAsDelegator(uint8 trackedIndexSeed) external {
        if (trackedDelegationCount == 0) {
            return;
        }
        uint256 index = bound(uint256(trackedIndexSeed), 0, trackedDelegationCount - 1);
        address delegator = trackedDelegators[index];
        address delegate = trackedDelegates[index];
        address contractAddress = trackedContractAddresses[index];

        bool wasPaused = acl.paused();
        vm.prank(delegator);
        (bool ok, ) = address(acl).call(
            abi.encodeCall(ACL.revokeDelegationForUserDecryption, (delegate, contractAddress))
        );
        if (ok) {
            if (wasPaused) {
                pausedBypassViolation = true;
            } else {
                if (_markRevoked(delegator, delegate, contractAddress)) {
                    successfulRevocationCount++;
                    vm.roll(block.number + 1);
                }
            }
        }
    }

    function revokeTrackedDelegationAsCaller(uint8 trackedIndexSeed) external {
        if (trackedDelegationCount == 0) {
            return;
        }
        uint256 index = bound(uint256(trackedIndexSeed), 0, trackedDelegationCount - 1);
        address selectedDelegator = trackedDelegators[index];
        address delegate = trackedDelegates[index];
        address contractAddress = trackedContractAddresses[index];
        uint64 selectedBefore = acl.getUserDecryptionDelegationExpirationDate(selectedDelegator, delegate, contractAddress);
        uint64 callerBefore = acl.getUserDecryptionDelegationExpirationDate(msg.sender, delegate, contractAddress);

        bool wasPaused = acl.paused();
        vm.prank(msg.sender);
        (bool ok, ) = address(acl).call(
            abi.encodeCall(ACL.revokeDelegationForUserDecryption, (delegate, contractAddress))
        );
        if (ok) {
            if (wasPaused) {
                pausedBypassViolation = true;
            } else {
                // Successful revocation by a non-selected delegator must not mutate selected tuple state.
                if (msg.sender != selectedDelegator) {
                    if (callerBefore == 0) {
                        privilegeViolation = true;
                    }
                    uint64 selectedAfter = acl.getUserDecryptionDelegationExpirationDate(
                        selectedDelegator,
                        delegate,
                        contractAddress
                    );
                    if (selectedBefore != 0 && selectedAfter == 0) {
                        privilegeViolation = true;
                    }
                }
                // This call path is auth-pressure only: caller can revoke only their own tuple.
                // We update the model when it has the tuple and otherwise ignore.
                if (_markRevoked(msg.sender, delegate, contractAddress)) {
                    successfulRevocationCount++;
                    vm.roll(block.number + 1);
                }
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

    function getTrackedDelegation(
        uint256 index
    ) external view returns (address delegator, address delegate, address contractAddress, bytes32 handle, uint8 status) {
        return (
            trackedDelegators[index],
            trackedDelegates[index],
            trackedContractAddresses[index],
            trackedDelegationHandles[index],
            trackedDelegationStatuses[index]
        );
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

    function _upsertDelegation(
        address delegator,
        address delegate,
        address contractAddress,
        bytes32 handle,
        uint8 status
    ) internal {
        (bool found, uint256 index) = _findTrackedIndex(delegator, delegate, contractAddress);
        if (found) {
            trackedDelegationHandles[index] = handle;
            trackedDelegationStatuses[index] = status;
            return;
        }

        if (trackedDelegationCount < MAX_TRACKED_TUPLES) {
            trackedDelegators[trackedDelegationCount] = delegator;
            trackedDelegates[trackedDelegationCount] = delegate;
            trackedContractAddresses[trackedDelegationCount] = contractAddress;
            trackedDelegationHandles[trackedDelegationCount] = handle;
            trackedDelegationStatuses[trackedDelegationCount] = status;
            trackedDelegationCount++;
        } else {
            uint256 slot = trackedDelegationWriteIndex % MAX_TRACKED_TUPLES;
            trackedDelegators[slot] = delegator;
            trackedDelegates[slot] = delegate;
            trackedContractAddresses[slot] = contractAddress;
            trackedDelegationHandles[slot] = handle;
            trackedDelegationStatuses[slot] = status;
            trackedDelegationWriteIndex++;
        }
    }

    function _markRevoked(address delegator, address delegate, address contractAddress) internal returns (bool) {
        (bool found, uint256 index) = _findTrackedIndex(delegator, delegate, contractAddress);
        if (!found) {
            return false;
        }
        trackedDelegationStatuses[index] = STATUS_REVOKED;
        return true;
    }

    function _findTrackedIndex(
        address delegator,
        address delegate,
        address contractAddress
    ) internal view returns (bool found, uint256 index) {
        for (uint256 i = 0; i < trackedDelegationCount; i++) {
            if (
                trackedDelegators[i] == delegator &&
                trackedDelegates[i] == delegate &&
                trackedContractAddresses[i] == contractAddress
            ) {
                return (true, i);
            }
        }
        return (false, 0);
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
