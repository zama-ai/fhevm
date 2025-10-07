// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { gatewayConfigAddress } from "../addresses/GatewayAddresses.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import { IMultichainACL } from "./interfaces/IMultichainACL.sol";
import { ICiphertextCommits } from "./interfaces/ICiphertextCommits.sol";
import { IGatewayConfig } from "./interfaces/IGatewayConfig.sol";
import { UUPSUpgradeableEmptyProxy } from "./shared/UUPSUpgradeableEmptyProxy.sol";
import { GatewayConfigChecks } from "./shared/GatewayConfigChecks.sol";
import { GatewayOwnable } from "./shared/GatewayOwnable.sol";
import { DelegationAccounts } from "./shared/Structs.sol";

/**
 * @title MultichainACL smart contract
 * @notice See {IMultichainACL}
 */
contract MultichainACL is IMultichainACL, UUPSUpgradeableEmptyProxy, GatewayOwnable, GatewayConfigChecks {
    /**
     * @notice The address of the GatewayConfig contract for protocol state calls.
     */
    IGatewayConfig private constant GATEWAY_CONFIG = IGatewayConfig(gatewayConfigAddress);

    /**
     * @notice The maximum number of contracts that can be requested for delegation.
     */
    uint8 internal constant MAX_CONTRACT_ADDRESSES = 10;

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
    uint64 private constant REINITIALIZER_VERSION = 2;

    /**
     * @notice The contract's variable storage struct (@dev see ERC-7201)
     */
    /// @custom:storage-location erc7201:fhevm_gateway.storage.MultichainACL
    struct MultichainACLStorage {
        // ----------------------------------------------------------------------------------------------
        // Common consensus state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The coprocessor transaction senders involved in a consensus for allowing a public decryption.
        mapping(bytes32 ctHandle => address[] coprocessorTxSenderAddresses) allowPublicDecryptConsensusTxSenders;
        // prettier-ignore
        /// @notice The coprocessor transaction senders involved in a consensus for allowing an account.
        mapping(bytes32 ctHandle => mapping(address accountAddress =>
            address[] coprocessorTxSenderAddresses))
               allowAccountConsensusTxSenders;
        /// @notice The coprocessor transaction senders involved in a consensus for delegating an account.
        mapping(bytes32 delegateAccountHash => address[] coprocessorTxSenderAddresses) delegateAccountConsensusTxSenders;
        // ----------------------------------------------------------------------------------------------
        // Allow account state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice Accounts allowed to use the ciphertext handle.
        mapping(bytes32 ctHandle => mapping(address accountAddress => bool isAllowed)) allowedAccounts;
        /// @notice The counter used for the allowAccount consensus.
        mapping(bytes32 ctHandle => mapping(address accountAddress => uint256 counter)) allowAccountCounters;
        // prettier-ignore
        /// @notice The coprocessors that have already allowed an account to use the ciphertext handle.
        mapping(bytes32 ctHandle => mapping(address accountAddress =>
            mapping(address coprocessorTxSenderAddress => bool hasAllowed)))
                allowAccountCoprocessors;
        // ----------------------------------------------------------------------------------------------
        // Allow public decryption state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice Allowed public decryptions.
        mapping(bytes32 ctHandle => bool isAllowed) allowedPublicDecrypts;
        /// @notice The counter used for the public decryption consensus.
        mapping(bytes32 ctHandle => uint256 counter) allowPublicDecryptCounters;
        // prettier-ignore
        /// @notice The coprocessors that have already allowed a public decryption.
        mapping(bytes32 ctHandle => mapping(address coprocessorTxSenderAddress => bool hasAllowed)) 
            allowPublicDecryptCoprocessors;
        // ----------------------------------------------------------------------------------------------
        // Delegate account state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The computed delegateAccountHash that has already been delegated.
        mapping(bytes32 delegateAccountHash => bool isDelegated) delegatedAccountHashes;
        /// @notice The number of times a delegateAccountHash has received confirmations.
        mapping(bytes32 delegateAccountHash => uint256 counter) delegateAccountHashCounters;
        // prettier-ignore
        /// @notice The coprocessors that have already delegated an account for a given delegateAccountHash.
        mapping(bytes32 delegateAccountHash =>
            mapping(address coprocessorTxSenderAddress => bool hasDelegated))
                alreadyDelegatedCoprocessors;
        // prettier-ignore
        /// @notice The account delegations for a given contract after reaching consensus.
        mapping(address delegator => mapping(address delegated =>
            mapping(uint256 chainId => mapping(address contractAddress => bool isDelegated))))
                delegatedContracts;
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
    // function reinitializeV2() public virtual reinitializer(REINITIALIZER_VERSION) {}

    /**
     * @notice See {IMultichainACL-allowPublicDecrypt}.
     */
    function allowPublicDecrypt(
        bytes32 ctHandle,
        bytes calldata /* extraData */
    ) external virtual onlyCoprocessorTxSender onlyHandleFromRegisteredHostChain(ctHandle) {
        MultichainACLStorage storage $ = _getMultichainACLStorage();

        // Check if the coprocessor has already allowed the ciphertext handle for public decryption.
        // A Coprocessor can only allow once for a given ctHandle, so it's not possible for it to allow
        // the same ctHandle for different host chains, hence the chain ID is not included in the mapping.
        if ($.allowPublicDecryptCoprocessors[ctHandle][msg.sender]) {
            revert CoprocessorAlreadyAllowedPublicDecrypt(ctHandle, msg.sender);
        }
        $.allowPublicDecryptCounters[ctHandle]++;
        $.allowPublicDecryptCoprocessors[ctHandle][msg.sender] = true;

        // Store the coprocessor transaction sender address for the public decryption response
        // It is important to consider the same mapping fields used for the consensus
        // A "late" valid coprocessor transaction sender address will still be added in the list.
        $.allowPublicDecryptConsensusTxSenders[ctHandle].push(msg.sender);

        // Send the event if and only if the consensus is reached in the current response call.
        // This means a "late" response will not be reverted, just ignored and no event will be emitted
        if (!$.allowedPublicDecrypts[ctHandle] && _isConsensusReached($.allowPublicDecryptCounters[ctHandle])) {
            $.allowedPublicDecrypts[ctHandle] = true;
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

        // Check if the coprocessor has already allowed the account to use the ciphertext handle.
        // A Coprocessor can only allow once for a given ctHandle, so it's not possible for it to allow
        // the same ctHandle for different host chains, hence the chain ID is not included in the mapping.
        if ($.allowAccountCoprocessors[ctHandle][accountAddress][msg.sender]) {
            revert CoprocessorAlreadyAllowedAccount(ctHandle, accountAddress, msg.sender);
        }
        $.allowAccountCounters[ctHandle][accountAddress]++;
        $.allowAccountCoprocessors[ctHandle][accountAddress][msg.sender] = true;

        // Store the coprocessor transaction sender address for the allow account response
        // It is important to consider the same mapping fields used for the consensus
        // A "late" valid coprocessor transaction sender address will still be added in the list.
        $.allowAccountConsensusTxSenders[ctHandle][accountAddress].push(msg.sender);

        // Send the event if and only if the consensus is reached in the current response call.
        // This means a "late" response will not be reverted, just ignored and no event will be emitted
        if (
            !$.allowedAccounts[ctHandle][accountAddress] &&
            _isConsensusReached($.allowAccountCounters[ctHandle][accountAddress])
        ) {
            $.allowedAccounts[ctHandle][accountAddress] = true;
            emit AllowAccount(ctHandle, accountAddress);
        }
    }

    /**
     * @notice See {IMultichainACL-delegateAccount}.
     */
    function delegateAccount(
        uint256 chainId,
        DelegationAccounts calldata delegationAccounts,
        address[] calldata contractAddresses
    ) external virtual onlyCoprocessorTxSender {
        if (contractAddresses.length == 0) {
            revert EmptyContractAddresses();
        }
        if (contractAddresses.length > MAX_CONTRACT_ADDRESSES) {
            revert ContractsMaxLengthExceeded(MAX_CONTRACT_ADDRESSES, contractAddresses.length);
        }

        MultichainACLStorage storage $ = _getMultichainACLStorage();

        // The delegateAccountHash is the hash of all input arguments.
        // This hash is used to track the delegation consensus over the whole contractAddresses list,
        // and assumes that the Coprocessors will delegate the same list of contracts and keep the same order.
        bytes32 delegateAccountHash = _getDelegateAccountHash(chainId, delegationAccounts, contractAddresses);

        mapping(address => bool) storage alreadyDelegatedCoprocessors = $.alreadyDelegatedCoprocessors[
            delegateAccountHash
        ];

        // Check if the coprocessor has already delegated the account.
        if (alreadyDelegatedCoprocessors[msg.sender]) {
            revert CoprocessorAlreadyDelegated(chainId, delegationAccounts, contractAddresses, msg.sender);
        }

        $.delegateAccountHashCounters[delegateAccountHash]++;
        alreadyDelegatedCoprocessors[msg.sender] = true;

        // Store the coprocessor transaction sender address for the delegate account response
        // It is important to consider the same mapping fields used for the consensus
        // A "late" valid coprocessor transaction sender address will still be added in the list.
        $.delegateAccountConsensusTxSenders[delegateAccountHash].push(msg.sender);

        // Send the event if and only if the consensus is reached in the current response call.
        // This means a "late" response will not be reverted, just ignored and no event will be emitted
        if (
            !$.delegatedAccountHashes[delegateAccountHash] &&
            _isConsensusReached($.delegateAccountHashCounters[delegateAccountHash])
        ) {
            mapping(address => bool) storage delegatedContracts = $.delegatedContracts[
                delegationAccounts.delegatorAddress
            ][delegationAccounts.delegatedAddress][chainId];
            for (uint256 i = 0; i < contractAddresses.length; i++) {
                delegatedContracts[contractAddresses[i]] = true;
            }
            $.delegatedAccountHashes[delegateAccountHash] = true;
            emit DelegateAccount(chainId, delegationAccounts, contractAddresses);
        }
    }

    /**
     * @notice See {IMultichainACL-isPublicDecryptAllowed}.
     */
    function isPublicDecryptAllowed(bytes32 ctHandle) external view virtual returns (bool) {
        MultichainACLStorage storage $ = _getMultichainACLStorage();
        return $.allowedPublicDecrypts[ctHandle];
    }

    /**
     * @notice See {IMultichainACL-isAccountAllowed}.
     */
    function isAccountAllowed(bytes32 ctHandle, address accountAddress) external view virtual returns (bool) {
        MultichainACLStorage storage $ = _getMultichainACLStorage();
        return $.allowedAccounts[ctHandle][accountAddress];
    }

    /**
     * @notice See {IMultichainACL-isAccountDelegated}.
     */
    function isAccountDelegated(
        uint256 chainId,
        DelegationAccounts calldata delegationAccounts,
        address[] calldata contractAddresses
    ) external view virtual returns (bool) {
        // An account cannot be delegated to an empty list of contracts.
        if (contractAddresses.length == 0) {
            return false;
        }

        // Check if each contract address is delegated.
        MultichainACLStorage storage $ = _getMultichainACLStorage();
        for (uint256 i = 0; i < contractAddresses.length; i++) {
            if (
                !$.delegatedContracts[delegationAccounts.delegatorAddress][delegationAccounts.delegatedAddress][
                    chainId
                ][contractAddresses[i]]
            ) {
                return false;
            }
        }
        return true;
    }

    /**
     * @notice See {IMultichainACL-getAllowPublicDecryptConsensusTxSenders}.
     */
    function getAllowPublicDecryptConsensusTxSenders(
        bytes32 ctHandle
    ) external view virtual returns (address[] memory) {
        MultichainACLStorage storage $ = _getMultichainACLStorage();

        return $.allowPublicDecryptConsensusTxSenders[ctHandle];
    }

    /**
     * @notice See {IMultichainACL-getAllowAccountConsensusTxSenders}.
     */
    function getAllowAccountConsensusTxSenders(
        bytes32 ctHandle,
        address accountAddress
    ) external view virtual returns (address[] memory) {
        MultichainACLStorage storage $ = _getMultichainACLStorage();

        return $.allowAccountConsensusTxSenders[ctHandle][accountAddress];
    }

    /**
     * @notice See {IMultichainACL-getDelegateAccountConsensusTxSenders}.
     * @dev The contract address list needs to be provided in the same order as when the consensus
     * was reached in order to be able to retrieve the coprocessor transaction senders associated to it.
     */
    function getDelegateAccountConsensusTxSenders(
        uint256 chainId,
        DelegationAccounts calldata delegationAccounts,
        address[] calldata contractAddresses
    ) external view virtual returns (address[] memory) {
        MultichainACLStorage storage $ = _getMultichainACLStorage();

        // Get the hash of the delegate account's inputs used to track the consensus.
        bytes32 delegateAccountHash = _getDelegateAccountHash(chainId, delegationAccounts, contractAddresses);

        return $.delegateAccountConsensusTxSenders[delegateAccountHash];
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
     * @notice Returns the hash of a delegate account's inputs.
     */
    function _getDelegateAccountHash(
        uint256 chainId,
        DelegationAccounts calldata delegationAccounts,
        address[] calldata contractAddresses
    ) internal pure virtual returns (bytes32) {
        return keccak256(abi.encode(chainId, delegationAccounts, contractAddresses));
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
