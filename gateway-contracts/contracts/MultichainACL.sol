// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { gatewayConfigAddress } from "../addresses/GatewayAddresses.sol";
import { MulticallUpgradeable } from "@openzeppelin/contracts-upgradeable/utils/MulticallUpgradeable.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import { IMultichainACL } from "./interfaces/IMultichainACL.sol";
import { ICiphertextCommits } from "./interfaces/ICiphertextCommits.sol";
import { IGatewayConfig } from "./interfaces/IGatewayConfig.sol";
import { UUPSUpgradeableEmptyProxy } from "./shared/UUPSUpgradeableEmptyProxy.sol";
import { GatewayConfigChecks } from "./shared/GatewayConfigChecks.sol";
import { GatewayOwnable } from "./shared/GatewayOwnable.sol";
import { UserDecryptionDelegation } from "./shared/Structs.sol";

/**
 * @title MultichainACL smart contract
 * @notice See {IMultichainACL}
 */
contract MultichainACL is
    IMultichainACL,
    UUPSUpgradeableEmptyProxy,
    GatewayOwnable,
    GatewayConfigChecks,
    MulticallUpgradeable
{
    /**
     * @notice The address of the GatewayConfig contract for protocol state calls.
     */
    IGatewayConfig private constant GATEWAY_CONFIG = IGatewayConfig(gatewayConfigAddress);

    /**
     * @dev The following constants are used for versioning the contract. They are made private
     * in order to force derived contracts to consider a different version. Note that
     * they can still define their own private constants with the same name.
     */
    string private constant CONTRACT_NAME = "MultichainACL";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    /**
     * @dev Constant used for making sure the version number using in the `reinitializer` modifier is
     * identical between `initializeFromEmptyProxy` and the reinitializeVX` method
     * This constant does not represent the number of time a specific contract have been upgraded,
     * as a contract deployed from version VX will have a REINITIALIZER_VERSION > 2.
     */
    uint64 private constant REINITIALIZER_VERSION = 3;

    /**
     * @notice The contract's variable storage struct (@dev see ERC-7201)
     */
    /// @custom:storage-location erc7201:fhevm_gateway.storage.MultichainACL
    struct MultichainACLStorage {
        // ----------------------------------------------------------------------------------------------
        // Common state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The coprocessors that have already made an allow call.
        mapping(bytes32 allowHash => mapping(address coprocessorTxSenderAddress => bool hasAllowed)) allowCoprocessors;
        /// @notice The counter used for an allow consensus.
        mapping(bytes32 allowHash => uint256 counter) allowCounters;
        /// @notice The coprocessor transaction senders involved in a consensus for an allow call.
        mapping(bytes32 allowHash => address[] coprocessorTxSenderAddresses) allowConsensusTxSenders;
        /// @notice Allowed public decryptions.
        mapping(bytes32 allowHash => bool isAllowed) isAllowed;
        /// @notice The coprocessor transaction senders involved in a consensus for delegating or revoking user decryption.
        mapping(bytes32 delegateUserDecryptionHash => address[] coprocessorTxSenderAddresses) delegateOrRevokeUserDecryptionTxSenders;
        // ----------------------------------------------------------------------------------------------
        // Coprocessor context state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The context ID for the allow consensus.
        mapping(bytes32 allowHash => uint256 contextId) allowContextId;
        // ----------------------------------------------------------------------------------------------
        // Delegation state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The computed delegate user decryption hash that has already reached consensus for delegation or revocation.
        mapping(bytes32 delegateUserDecryptionHash => bool isDelegated) delegatedOrRevokedUserDecryptionHashes;
        // prettier-ignore
        /// @notice The coprocessors that have already delegated or revoked delegation for user decryption.
        mapping(bytes32 delegateUserDecryptionHash =>
            mapping(address coprocessorTxSenderAddress => bool hasDelegated))
                alreadyDelegatedOrRevokedUserDecryptionCoprocessors;
        // prettier-ignore
        /// @notice The user decryption delegation info after reaching consensus for delegation or revocation.
        mapping(uint256 chainId => mapping(address delegator =>
            mapping(address delegate => mapping(address contractAddress => UserDecryptionDelegation delegation))))
                userDecryptionDelegations;
    }

    /**
     * @dev Storage location has been computed using the following command:
     * keccak256(abi.encode(uint256(keccak256("fhevm_gateway.storage.MultichainACL")) - 1))
     * & ~bytes32(uint256(0xff))
     */
    bytes32 private constant MULTICHAIN_ACL_STORAGE_LOCATION =
        0x7f733a54a70114addd729bcd827932a6c402ccf3920960665917bc2e6640f400;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice Initializes the contract.
     * @dev This function needs to be public in order to be called by the UUPS proxy.
     */
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy() public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {}

    /**
     * @notice Re-initializes the contract from V1.
     * @dev Define a `reinitializeVX` function once the contract needs to be upgraded.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function reinitializeV2() public virtual reinitializer(REINITIALIZER_VERSION) {}

    /**
     * @notice See {IMultichainACL-allowPublicDecrypt}.
     */
    function allowPublicDecrypt(
        bytes32 ctHandle,
        bytes calldata /* extraData */
    ) external virtual onlyCoprocessorTxSender onlyHandleFromRegisteredHostChain(ctHandle) {
        MultichainACLStorage storage $ = _getMultichainACLStorage();

        // Associate the ctHandle to coprocessor context ID 1 to anticipate their introduction in V2.
        // Only set the context ID if it hasn't been set yet to avoid multiple identical SSTOREs.
        if ($.allowContextId[ctHandle] == 0) {
            $.allowContextId[ctHandle] = 1;
        }

        // Check if the coprocessor has already allowed the ciphertext handle for public decryption.
        // A Coprocessor can only allow once for a given ctHandle, so it's not possible for it to allow
        // the same ctHandle for different host chains, hence the chain ID is not included in the mapping.
        if ($.allowCoprocessors[ctHandle][msg.sender]) {
            revert CoprocessorAlreadyAllowedPublicDecrypt(ctHandle, msg.sender);
        }
        $.allowCounters[ctHandle]++;
        $.allowCoprocessors[ctHandle][msg.sender] = true;

        // Store the coprocessor transaction sender address for the public decryption response
        // It is important to consider the same mapping fields used for the consensus
        // A "late" valid coprocessor transaction sender address will still be added in the list.
        $.allowConsensusTxSenders[ctHandle].push(msg.sender);

        // Send the event if and only if the consensus is reached in the current response call.
        // This means a "late" response will not be reverted, just ignored and no event will be emitted
        if (!$.isAllowed[ctHandle] && _isConsensusReached($.allowCounters[ctHandle])) {
            $.isAllowed[ctHandle] = true;
            emit AllowPublicDecrypt(ctHandle);
        }
    }

    /**
     * @notice See {IMultichainACL-allowAccount}.
     */
    function allowAccount(
        bytes32 ctHandle,
        address accountAddress,
        bytes calldata /* extraData */
    ) external virtual onlyCoprocessorTxSender onlyHandleFromRegisteredHostChain(ctHandle) {
        MultichainACLStorage storage $ = _getMultichainACLStorage();

        // Compute the hash of the allow call, unique across all types of allow calls.
        bytes32 allowHash = _getAllowAccountHash(ctHandle, accountAddress);

        // Associate the allowHash to coprocessor context ID 1 to anticipate their introduction in V2.
        // Only set the context ID if it hasn't been set yet to avoid multiple identical SSTOREs.
        if ($.allowContextId[allowHash] == 0) {
            $.allowContextId[allowHash] = 1;
        }

        // Check if the coprocessor has already allowed the account to use the ciphertext handle.
        // A Coprocessor can only allow once for a given ctHandle, so it's not possible for it to allow
        // the same ctHandle for different host chains, hence the chain ID is not included in the mapping.
        if ($.allowCoprocessors[allowHash][msg.sender]) {
            revert CoprocessorAlreadyAllowedAccount(ctHandle, accountAddress, msg.sender);
        }
        $.allowCounters[allowHash]++;
        $.allowCoprocessors[allowHash][msg.sender] = true;

        // Store the coprocessor transaction sender address for the allow account response
        // It is important to consider the same mapping fields used for the consensus
        // A "late" valid coprocessor transaction sender address will still be added in the list.
        $.allowConsensusTxSenders[allowHash].push(msg.sender);

        // Send the event if and only if the consensus is reached in the current response call.
        // This means a "late" response will not be reverted, just ignored and no event will be emitted
        if (!$.isAllowed[allowHash] && _isConsensusReached($.allowCounters[allowHash])) {
            $.isAllowed[allowHash] = true;
            emit AllowAccount(ctHandle, accountAddress);
        }
    }

    /// @dev See {IMultichainACL-delegateUserDecryption}.
    function delegateUserDecryption(
        uint256 chainId,
        address delegator,
        address delegate,
        address contractAddress,
        uint64 expiryDate,
        uint64 delegationCounter
    ) external virtual onlyCoprocessorTxSender {
        MultichainACLStorage storage $ = _getMultichainACLStorage();
        bytes32 delegateUserDecryptionHash = _getDelegateUserDecryptionHash(
            chainId,
            delegator,
            delegate,
            contractAddress,
            expiryDate,
            delegationCounter
        );

        mapping(address => bool) storage alreadyDelegatedUserDecryptionCoprocessors = $
            .alreadyDelegatedOrRevokedUserDecryptionCoprocessors[delegateUserDecryptionHash];

        /// @dev Check if the coprocessor has already delegated the user decryption.
        if (alreadyDelegatedUserDecryptionCoprocessors[msg.sender]) {
            revert CoprocessorAlreadyDelegatedOrRevokedUserDecryption(
                chainId,
                delegator,
                delegate,
                contractAddress,
                expiryDate,
                delegationCounter,
                msg.sender
            );
        }

        // Mark the coprocessor has already delegated for the user decryption hash.
        alreadyDelegatedUserDecryptionCoprocessors[msg.sender] = true;

        // Store the coprocessor transaction sender address for the delegate user decryption hash.
        // A "late" valid coprocessor transaction sender address will still be added in the list.
        $.delegateOrRevokeUserDecryptionTxSenders[delegateUserDecryptionHash].push(msg.sender);

        // Send the event only if the consensus is reached in the current call.
        // This means a "late" response will not be reverted, just ignored and no event will be emitted.
        if (
            !$.delegatedOrRevokedUserDecryptionHashes[delegateUserDecryptionHash] &&
            _isConsensusReached($.delegateOrRevokeUserDecryptionTxSenders[delegateUserDecryptionHash].length)
        ) {
            UserDecryptionDelegation storage userDecryptionDelegation = $.userDecryptionDelegations[chainId][delegator][
                delegate
            ][contractAddress];

            // Check that the delegation counter is greater than a previous one.
            if (delegationCounter <= userDecryptionDelegation.delegationCounter) {
                revert UserDecryptionDelegationCounterTooLow(delegationCounter);
            }

            // Update the user decryption delegation information.
            userDecryptionDelegation.delegationCounter = delegationCounter;
            userDecryptionDelegation.expiryDate = expiryDate;

            // Mark the delegate user decryption hash as having reached consensus for delegation.
            $.delegatedOrRevokedUserDecryptionHashes[delegateUserDecryptionHash] = true;

            emit DelegateUserDecryption(chainId, delegator, delegate, contractAddress);
        }
    }

    /// @dev See {IMultichainACL-revokeUserDecryption}.
    function revokeUserDecryption(
        uint256 chainId,
        address delegator,
        address delegate,
        address contractAddress,
        uint64 expiryDate,
        uint64 delegationCounter
    ) external virtual onlyCoprocessorTxSender {
        MultichainACLStorage storage $ = _getMultichainACLStorage();
        bytes32 delegateUserDecryptionHash = _getDelegateUserDecryptionHash(
            chainId,
            delegator,
            delegate,
            contractAddress,
            expiryDate,
            delegationCounter
        );

        mapping(address => bool) storage alreadyRevokedUserDecryptionCoprocessors = $
            .alreadyDelegatedOrRevokedUserDecryptionCoprocessors[delegateUserDecryptionHash];

        /// @dev Check if the coprocessor has already revoked the user decryption delegation.
        if (alreadyRevokedUserDecryptionCoprocessors[msg.sender]) {
            revert CoprocessorAlreadyDelegatedOrRevokedUserDecryption(
                chainId,
                delegator,
                delegate,
                contractAddress,
                expiryDate,
                delegationCounter,
                msg.sender
            );
        }

        // Mark the coprocessor has already revoked for the delegate user decryption hash.
        alreadyRevokedUserDecryptionCoprocessors[msg.sender] = true;

        // Store the coprocessor transaction sender address for the delegate user decryption hash.
        // A "late" valid coprocessor transaction sender address will still be added in the list.
        $.delegateOrRevokeUserDecryptionTxSenders[delegateUserDecryptionHash].push(msg.sender);

        // Send the event if and only if the consensus is reached in the current call.
        // This means a "late" response will not be reverted, just ignored and no event will be emitted.
        if (
            !$.delegatedOrRevokedUserDecryptionHashes[delegateUserDecryptionHash] &&
            _isConsensusReached($.delegateOrRevokeUserDecryptionTxSenders[delegateUserDecryptionHash].length)
        ) {
            UserDecryptionDelegation storage userDecryptionDelegation = $.userDecryptionDelegations[chainId][delegator][
                delegate
            ][contractAddress];

            // Check that the delegation counter is greater than a previous one.
            if (delegationCounter <= userDecryptionDelegation.delegationCounter) {
                revert UserDecryptionDelegationCounterTooLow(delegationCounter);
            }

            // Update the user decryption delegation information.
            userDecryptionDelegation.delegationCounter = delegationCounter;
            userDecryptionDelegation.expiryDate = 0;

            // Mark the delegate user decryption hash as having reached consensus for revocation.
            $.delegatedOrRevokedUserDecryptionHashes[delegateUserDecryptionHash] = true;

            emit RevokeUserDecryption(chainId, delegator, delegate, contractAddress);
        }
    }

    /**
     * @notice See {IMultichainACL-isPublicDecryptAllowed}.
     */
    function isPublicDecryptAllowed(bytes32 ctHandle) external view virtual returns (bool) {
        MultichainACLStorage storage $ = _getMultichainACLStorage();

        return $.isAllowed[ctHandle];
    }

    /**
     * @notice See {IMultichainACL-isAccountAllowed}.
     */
    function isAccountAllowed(bytes32 ctHandle, address accountAddress) external view virtual returns (bool) {
        MultichainACLStorage storage $ = _getMultichainACLStorage();

        bytes32 allowHash = _getAllowAccountHash(ctHandle, accountAddress);
        return $.isAllowed[allowHash];
    }

    /**
     * @notice See {IMultichainACL-isUserDecryptionDelegated}.
     */
    function isUserDecryptionDelegated(
        uint256 chainId,
        address delegator,
        address delegate,
        address contractAddress
    ) external view returns (bool) {
        MultichainACLStorage storage $ = _getMultichainACLStorage();

        UserDecryptionDelegation storage userDecryptionDelegation = $.userDecryptionDelegations[chainId][delegator][
            delegate
        ][contractAddress];

        return userDecryptionDelegation.expiryDate >= block.timestamp;
    }

    /**
     * @notice See {IMultichainACL-getAllowPublicDecryptConsensusTxSenders}.
     */
    function getAllowPublicDecryptConsensusTxSenders(
        bytes32 ctHandle
    ) external view virtual returns (address[] memory) {
        MultichainACLStorage storage $ = _getMultichainACLStorage();

        return $.allowConsensusTxSenders[ctHandle];
    }

    /**
     * @notice See {IMultichainACL-getAllowAccountConsensusTxSenders}.
     */
    function getAllowAccountConsensusTxSenders(
        bytes32 ctHandle,
        address accountAddress
    ) external view virtual returns (address[] memory) {
        MultichainACLStorage storage $ = _getMultichainACLStorage();

        bytes32 allowHash = _getAllowAccountHash(ctHandle, accountAddress);
        return $.allowConsensusTxSenders[allowHash];
    }

    /**
     * @notice See {IMultichainACL-getVersion}.
     */
    function getVersion() external pure virtual returns (string memory) {
        return
            string(
                abi.encodePacked(
                    CONTRACT_NAME,
                    " v",
                    Strings.toString(MAJOR_VERSION),
                    ".",
                    Strings.toString(MINOR_VERSION),
                    ".",
                    Strings.toString(PATCH_VERSION)
                )
            );
    }

    /**
     * @notice Checks if the sender is authorized to upgrade the contract and reverts otherwise.
     */
    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyGatewayOwner {}

    /**
     * @notice Checks if the consensus is reached among the Coprocessors.
     * @param coprocessorCounter The number of coprocessors that agreed
     * @return Whether the consensus is reached
     */
    function _isConsensusReached(uint256 coprocessorCounter) internal view virtual returns (bool) {
        uint256 consensusThreshold = GATEWAY_CONFIG.getCoprocessorMajorityThreshold();
        return coprocessorCounter >= consensusThreshold;
    }

    /**
     * @notice Returns the hash of a allow account call.
     */
    function _getAllowAccountHash(bytes32 ctHandle, address accountAddress) internal pure virtual returns (bytes32) {
        return keccak256(abi.encode(ctHandle, accountAddress));
    }

    /**
     * @notice Returns the hash of a delegate user decryption information.
     */
    function _getDelegateUserDecryptionHash(
        uint256 chainId,
        address delegator,
        address delegate,
        address contractAddress,
        uint64 expiryDate,
        uint64 delegationCounter
    ) internal pure virtual returns (bytes32) {
        return keccak256(abi.encode(chainId, delegator, delegate, contractAddress, expiryDate, delegationCounter));
    }

    /**
     * @notice Returns the MultichainACL storage location.
     * @dev Note that this function is internal but not virtual: derived contracts should be able to
     * access it, but if the underlying storage struct version changes, we force them to define a new
     * getter function and use that one instead in order to avoid overriding the storage location.
     */
    function _getMultichainACLStorage() internal pure returns (MultichainACLStorage storage $) {
        // solhint-disable-next-line no-inline-assembly
        assembly {
            $.slot := MULTICHAIN_ACL_STORAGE_LOCATION
        }
    }
}
