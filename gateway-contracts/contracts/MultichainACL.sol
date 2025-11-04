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
 * @dev This contract implements MulticallUpgradeable to allow batching multiple calls in a single
 * transaction, e.g., for bundling user decryption delegations of multiple contracts.
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
     * @notice The domain separator for the allow public decrypt hash.
     */
    bytes32 private constant ALLOW_PUBLIC_DECRYPT_DOMAIN_SEPARATOR_HASH =
        keccak256(bytes("MultichainACL.allowPublicDecrypt"));

    /**
     * @notice The domain separator for the allow account hash.
     */
    bytes32 private constant ALLOW_ACCOUNT_DOMAIN_SEPARATOR_HASH = keccak256(bytes("MultichainACL.allowAccount"));

    /**
     * @notice The domain separator for the delegate user decryption hash.
     */
    bytes32 private constant DELEGATE_USER_DECRYPTION_DOMAIN_SEPARATOR_HASH =
        keccak256(bytes("MultichainACL.delegateUserDecryption"));

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
        /// @notice DEPRECATED: to remove in a state reset.
        mapping(bytes32 delegateAccountHash => address[] coprocessorTxSenderAddresses) delegateAccountConsensusTxSenders;
        // ----------------------------------------------------------------------------------------------
        // Coprocessor context state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The context ID for the allow consensus.
        mapping(bytes32 allowHash => uint256 contextId) allowContextId;
        // ----------------------------------------------------------------------------------------------
        // Delegation state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The computed delegate user decryption hash that has already reached consensus for delegation or revocation.
        mapping(bytes32 delegateUserDecryptionHash => bool isDelegatedOrRevoked) delegatedOrRevokedUserDecryptionHashes;
        /// @notice The coprocessor transaction senders involved in a consensus for delegating user decryption.
        mapping(bytes32 delegateUserDecryptionHash => address[] coprocessorTxSenderAddresses) delegateUserDecryptionTxSenders;
        /// @notice The coprocessor transaction senders involved in a consensus for revoking user decryption.
        mapping(bytes32 delegateUserDecryptionHash => address[] coprocessorTxSenderAddresses) revokeUserDecryptionTxSenders;
        // prettier-ignore
        /// @notice The coprocessors that have already delegated for user decryption.
        mapping(bytes32 delegateUserDecryptionHash =>
            mapping(address coprocessorTxSenderAddress => bool hasDelegated))
                alreadyDelegatedUserDecryptionCoprocessors;
        // prettier-ignore
        /// @notice The coprocessors that have already revoked delegation for user decryption.
        mapping(bytes32 delegateUserDecryptionHash =>
            mapping(address coprocessorTxSenderAddress => bool hasRevoked))
                alreadyRevokedUserDecryptionCoprocessors;
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
        bytes calldata extraData
    ) external virtual onlyCoprocessorTxSender onlyHandleFromRegisteredHostChain(ctHandle) {
        MultichainACLStorage storage $ = _getMultichainACLStorage();

        // Compute the hash of the allow call, unique across all types of allow calls.
        bytes32 allowHash = _getAllowPublicDecryptHash(ctHandle);

        // Associate the ctHandle to coprocessor context ID 1 to anticipate their introduction in V2.
        // Only set the context ID if it hasn't been set yet to avoid multiple identical SSTOREs.
        if ($.allowContextId[allowHash] == 0) {
            $.allowContextId[allowHash] = 1;
        }

        // Check if the coprocessor has already allowed the ciphertext handle for public decryption.
        // A Coprocessor can only allow once for a given ctHandle, so it's not possible for it to allow
        // the same ctHandle for different host chains, hence the chain ID is not included in the mapping.
        if ($.allowCoprocessors[allowHash][msg.sender]) {
            revert CoprocessorAlreadyAllowedPublicDecrypt(ctHandle, msg.sender);
        }
        $.allowCounters[allowHash]++;
        $.allowCoprocessors[allowHash][msg.sender] = true;

        // Store the coprocessor transaction sender address for the public decryption response
        // It is important to consider the same mapping fields used for the consensus
        // A "late" valid coprocessor transaction sender address will still be added in the list.
        $.allowConsensusTxSenders[allowHash].push(msg.sender);

        // Emit the event at each call for monitoring purposes.
        emit AllowPublicDecrypt(ctHandle, msg.sender, extraData);

        // Send the event if and only if the consensus is reached in the current response call.
        // This means a "late" response will not be reverted, just ignored and no event will be emitted
        if (!$.isAllowed[allowHash] && _isConsensusReached($.allowCounters[allowHash])) {
            $.isAllowed[allowHash] = true;
            emit AllowPublicDecryptConsensus(ctHandle, extraData);
        }
    }

    /**
     * @notice See {IMultichainACL-allowAccount}.
     */
    function allowAccount(
        bytes32 ctHandle,
        address accountAddress,
        bytes calldata extraData
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

        // Emit the event at each call for monitoring purposes.
        emit AllowAccount(ctHandle, accountAddress, msg.sender, extraData);

        // Send the event if and only if the consensus is reached in the current response call.
        // This means a "late" response will not be reverted, just ignored and no event will be emitted
        if (!$.isAllowed[allowHash] && _isConsensusReached($.allowCounters[allowHash])) {
            $.isAllowed[allowHash] = true;
            emit AllowAccountConsensus(ctHandle, accountAddress, extraData);
        }
    }

    /// @dev See {IMultichainACL-delegateUserDecryption}.
    function delegateUserDecryption(
        uint256 chainId,
        address delegator,
        address delegate,
        address contractAddress,
        uint64 delegationCounter,
        uint64 expirationDate
    ) external virtual onlyCoprocessorTxSender {
        MultichainACLStorage storage $ = _getMultichainACLStorage();
        bytes32 delegateUserDecryptionHash = _getDelegateUserDecryptionHash(
            chainId,
            delegator,
            delegate,
            contractAddress,
            delegationCounter,
            expirationDate
        );

        _checkAlreadyDelegatedOrRevokedUserDecryptionDelegation(
            chainId,
            delegator,
            delegate,
            contractAddress,
            delegationCounter,
            expirationDate,
            delegateUserDecryptionHash
        );

        mapping(address => bool) storage alreadyDelegatedUserDecryptionCoprocessors = $
            .alreadyDelegatedUserDecryptionCoprocessors[delegateUserDecryptionHash];

        // Mark the coprocessor has already delegated for the user decryption hash.
        alreadyDelegatedUserDecryptionCoprocessors[msg.sender] = true;

        // Store the coprocessor transaction sender address for the delegate user decryption hash.
        // A "late" valid coprocessor transaction sender address will still be added in the list.
        $.delegateUserDecryptionTxSenders[delegateUserDecryptionHash].push(msg.sender);

        emit DelegateUserDecryption(chainId, delegator, delegate, contractAddress, delegationCounter, expirationDate);

        // Send the consensus reached event only if the consensus is reached in the current call.
        // This means a "late" response will not be reverted, just ignored and no event will be emitted.
        if (
            !$.delegatedOrRevokedUserDecryptionHashes[delegateUserDecryptionHash] &&
            _isConsensusReached($.delegateUserDecryptionTxSenders[delegateUserDecryptionHash].length)
        ) {
            UserDecryptionDelegation storage userDecryptionDelegation = $.userDecryptionDelegations[chainId][delegator][
                delegate
            ][contractAddress];

            // Check that the delegation counter is strictly greater than the previous one.
            // This check must be performed only after consensus is reached to guarantee correct ordering
            // of delegation operations among multiple coprocessors. Performing this check outside of the
            // consensus branch could lead to incorrect reverts in cases where multiple delegation or
            // revocation transactions are processed in parallel.
            if (delegationCounter <= userDecryptionDelegation.delegationCounter) {
                revert UserDecryptionDelegationCounterTooLow(delegationCounter);
            }

            // Update the user decryption delegation information.
            uint64 oldExpirationDate = userDecryptionDelegation.expirationDate;
            userDecryptionDelegation.delegationCounter = delegationCounter;
            userDecryptionDelegation.expirationDate = expirationDate;

            // Mark the delegate user decryption hash as having reached consensus for delegation.
            $.delegatedOrRevokedUserDecryptionHashes[delegateUserDecryptionHash] = true;

            emit DelegateUserDecryptionConsensus(
                chainId,
                delegator,
                delegate,
                contractAddress,
                delegationCounter,
                oldExpirationDate,
                expirationDate
            );
        }
    }

    /// @dev See {IMultichainACL-revokeUserDecryptionDelegation}.
    function revokeUserDecryptionDelegation(
        uint256 chainId,
        address delegator,
        address delegate,
        address contractAddress,
        uint64 delegationCounter,
        uint64 expirationDate
    ) external virtual onlyCoprocessorTxSender {
        MultichainACLStorage storage $ = _getMultichainACLStorage();
        bytes32 delegateUserDecryptionHash = _getDelegateUserDecryptionHash(
            chainId,
            delegator,
            delegate,
            contractAddress,
            delegationCounter,
            expirationDate
        );

        _checkAlreadyDelegatedOrRevokedUserDecryptionDelegation(
            chainId,
            delegator,
            delegate,
            contractAddress,
            delegationCounter,
            expirationDate,
            delegateUserDecryptionHash
        );

        mapping(address => bool) storage alreadyRevokedUserDecryptionCoprocessors = $
            .alreadyRevokedUserDecryptionCoprocessors[delegateUserDecryptionHash];

        // Mark the coprocessor has already revoked for the delegate user decryption hash.
        alreadyRevokedUserDecryptionCoprocessors[msg.sender] = true;

        // Store the revoking coprocessor transaction sender address for the delegate user decryption hash.
        // A "late" valid coprocessor transaction sender address will still be added in the list.
        $.revokeUserDecryptionTxSenders[delegateUserDecryptionHash].push(msg.sender);

        emit RevokeUserDecryptionDelegation(chainId, delegator, delegate, contractAddress, delegationCounter);

        // Send the consensus reached event only if the consensus is reached in the current call.
        // This means a "late" response will not be reverted, just ignored and no event will be emitted.
        if (
            !$.delegatedOrRevokedUserDecryptionHashes[delegateUserDecryptionHash] &&
            _isConsensusReached($.revokeUserDecryptionTxSenders[delegateUserDecryptionHash].length)
        ) {
            UserDecryptionDelegation storage userDecryptionDelegation = $.userDecryptionDelegations[chainId][delegator][
                delegate
            ][contractAddress];

            // Check that the delegation counter is strictly greater than the previous one.
            // This check must be performed only after consensus is reached to guarantee correct ordering
            // of revocation operations among multiple coprocessors. Performing this check outside of the
            // consensus branch could lead to incorrect reverts in cases where multiple delegation or
            // revocation transactions are processed in parallel.
            if (delegationCounter <= userDecryptionDelegation.delegationCounter) {
                revert UserDecryptionDelegationCounterTooLow(delegationCounter);
            }

            // Update the user decryption delegation information.
            uint64 oldExpirationDate = userDecryptionDelegation.expirationDate;
            userDecryptionDelegation.delegationCounter = delegationCounter;
            userDecryptionDelegation.expirationDate = 0;

            // Mark the delegate user decryption hash as having reached consensus for revocation.
            $.delegatedOrRevokedUserDecryptionHashes[delegateUserDecryptionHash] = true;

            emit RevokeUserDecryptionDelegationConsensusReached(
                chainId,
                delegator,
                delegate,
                contractAddress,
                delegationCounter,
                oldExpirationDate
            );
        }
    }

    /**
     * @notice See {IMultichainACL-isPublicDecryptAllowed}.
     */
    function isPublicDecryptAllowed(bytes32 ctHandle) external view virtual returns (bool) {
        MultichainACLStorage storage $ = _getMultichainACLStorage();

        bytes32 allowHash = _getAllowPublicDecryptHash(ctHandle);
        return $.isAllowed[allowHash];
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

        return userDecryptionDelegation.expirationDate >= block.timestamp;
    }

    /**
     * @notice See {IMultichainACL-getAllowPublicDecryptConsensusTxSenders}.
     */
    function getAllowPublicDecryptConsensusTxSenders(
        bytes32 ctHandle
    ) external view virtual returns (address[] memory) {
        MultichainACLStorage storage $ = _getMultichainACLStorage();

        bytes32 allowHash = _getAllowPublicDecryptHash(ctHandle);
        return $.allowConsensusTxSenders[allowHash];
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
     * @notice See {IMultichainACL-getDelegateUserDecryptionConsensusTxSenders}.
     */
    function getDelegateUserDecryptionConsensusTxSenders(
        uint256 chainId,
        address delegator,
        address delegate,
        address contractAddress,
        uint64 delegationCounter,
        uint64 expirationDate
    ) external view returns (address[] memory) {
        MultichainACLStorage storage $ = _getMultichainACLStorage();

        bytes32 delegateUserDecryptionHash = _getDelegateUserDecryptionHash(
            chainId,
            delegator,
            delegate,
            contractAddress,
            delegationCounter,
            expirationDate
        );

        return $.delegateUserDecryptionTxSenders[delegateUserDecryptionHash];
    }

    /**
     * @notice See {IMultichainACL-getRevokeUserDecryptionDelegationConsensusTxSenders}.
     */
    function getRevokeUserDecryptionDelegationConsensusTxSenders(
        uint256 chainId,
        address delegator,
        address delegate,
        address contractAddress,
        uint64 delegationCounter,
        uint64 expirationDate
    ) external view returns (address[] memory) {
        MultichainACLStorage storage $ = _getMultichainACLStorage();

        bytes32 delegateUserDecryptionHash = _getDelegateUserDecryptionHash(
            chainId,
            delegator,
            delegate,
            contractAddress,
            delegationCounter,
            expirationDate
        );

        return $.revokeUserDecryptionTxSenders[delegateUserDecryptionHash];
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
     * @notice Returns the hash of a allow public decrypt call.
     */
    function _getAllowPublicDecryptHash(bytes32 ctHandle) internal pure virtual returns (bytes32) {
        return keccak256(abi.encode(ALLOW_PUBLIC_DECRYPT_DOMAIN_SEPARATOR_HASH, ctHandle));
    }

    /**
     * @notice Returns the hash of a allow account call.
     */
    function _getAllowAccountHash(bytes32 ctHandle, address accountAddress) internal pure virtual returns (bytes32) {
        return keccak256(abi.encode(ALLOW_ACCOUNT_DOMAIN_SEPARATOR_HASH, ctHandle, accountAddress));
    }

    /**
     * @notice Returns the hash of a delegate user decryption information.
     */
    function _getDelegateUserDecryptionHash(
        uint256 chainId,
        address delegator,
        address delegate,
        address contractAddress,
        uint64 delegationCounter,
        uint64 expirationDate
    ) internal pure virtual returns (bytes32) {
        return
            keccak256(
                abi.encode(
                    DELEGATE_USER_DECRYPTION_DOMAIN_SEPARATOR_HASH,
                    chainId,
                    delegator,
                    delegate,
                    contractAddress,
                    delegationCounter,
                    expirationDate
                )
            );
    }

    /**
     * @notice Checks if the coprocessor has already delegated or revoked the user decryption delegation.
     */
    function _checkAlreadyDelegatedOrRevokedUserDecryptionDelegation(
        uint256 chainId,
        address delegator,
        address delegate,
        address contractAddress,
        uint64 delegationCounter,
        uint64 expirationDate,
        bytes32 delegateUserDecryptionHash
    ) internal view virtual {
        MultichainACLStorage storage $ = _getMultichainACLStorage();

        mapping(address => bool) storage alreadyDelegatedUserDecryptionCoprocessors = $
            .alreadyDelegatedUserDecryptionCoprocessors[delegateUserDecryptionHash];

        // Check if the coprocessor has already delegated the user decryption.
        if (alreadyDelegatedUserDecryptionCoprocessors[msg.sender]) {
            revert CoprocessorAlreadyDelegatedUserDecryption(
                chainId,
                delegator,
                delegate,
                contractAddress,
                delegationCounter,
                expirationDate,
                msg.sender
            );
        }

        mapping(address => bool) storage alreadyRevokedUserDecryptionCoprocessors = $
            .alreadyRevokedUserDecryptionCoprocessors[delegateUserDecryptionHash];

        // Check if the coprocessor has already revoked the user decryption delegation.
        if (alreadyRevokedUserDecryptionCoprocessors[msg.sender]) {
            revert CoprocessorAlreadyRevokedUserDecryption(
                chainId,
                delegator,
                delegate,
                contractAddress,
                delegationCounter,
                expirationDate,
                msg.sender
            );
        }
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
