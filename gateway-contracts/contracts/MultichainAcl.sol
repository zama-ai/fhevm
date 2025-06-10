// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { coprocessorContextsAddress } from "../addresses/CoprocessorContextsAddress.sol";
import { Ownable2StepUpgradeable } from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import "./interfaces/IMultichainAcl.sol";
import "./interfaces/ICiphertextCommits.sol";
import { ICoprocessorContexts } from "./interfaces/ICoprocessorContexts.sol";
import "./shared/UUPSUpgradeableEmptyProxy.sol";
import "./shared/GatewayConfigChecks.sol";
import "./shared/Pausable.sol";
import { ContextChecks } from "./shared/ContextChecks.sol";

/// @title MultichainAcl smart contract
/// @dev See {IMultichainAcl}
contract MultichainAcl is
    IMultichainAcl,
    Ownable2StepUpgradeable,
    UUPSUpgradeableEmptyProxy,
    GatewayConfigChecks,
    Pausable,
    ContextChecks
{
    /// @notice The address of the CoprocessorContexts contract, used for fetching information about coprocessors.
    ICoprocessorContexts private constant COPROCESSOR_CONTEXTS = ICoprocessorContexts(coprocessorContextsAddress);

    /// @notice The maximum number of contracts that can be requested for delegation.
    uint8 internal constant MAX_CONTRACT_ADDRESSES = 10;

    /// @dev The following constants are used for versioning the contract. They are made private
    /// @dev in order to force derived contracts to consider a different version. Note that
    /// @dev they can still define their own private constants with the same name.
    string private constant CONTRACT_NAME = "MultichainAcl";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    /// @notice The contract's variable storage struct (@dev see ERC-7201)
    /// @custom:storage-location erc7201:fhevm_gateway.storage.MultichainAcl
    struct MultichainAclStorage {
        // ----------------------------------------------------------------------------------------------
        // Allow account state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice Accounts allowed to use the ciphertext handle.
        mapping(bytes32 ctHandle => mapping(address accountAddress => bool isAllowed)) allowedAccounts;
        /// @notice The counter used for the allowAccount consensus.
        mapping(bytes32 ctHandle => mapping(address accountAddress => uint8 counter)) _allowAccountCounters;
        // prettier-ignore
        /// @notice Coprocessors that have already allowed an account to use the ciphertext handle.
        mapping(bytes32 ctHandle => mapping(address accountAddress =>
            mapping(address coprocessorTxSenderAddress => bool hasAllowed)))
                _allowAccountCoprocessors;
        // ----------------------------------------------------------------------------------------------
        // Allow public decryption state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice Allowed public decryptions.
        mapping(bytes32 ctHandle => bool isAllowed) allowedPublicDecrypts;
        /// @notice The counter used for the public decryption consensus.
        mapping(bytes32 ctHandle => uint8 counter) _allowPublicDecryptCounters;
        // prettier-ignore
        /// @notice Coprocessors that have already allowed a public decryption.
        mapping(bytes32 ctHandle => mapping(address coprocessorTxSenderAddress => bool hasAllowed)) 
            _allowPublicDecryptCoprocessors;
        // ----------------------------------------------------------------------------------------------
        // Delegate account state variables:
        // ----------------------------------------------------------------------------------------------
        /// @dev Tracks the computed delegateAccountHash that has already been delegated.
        mapping(bytes32 delegateAccountHash => bool isDelegated) _delegatedAccountHashes;
        /// @dev Tracks the number of times a delegateAccountHash has received confirmations.
        mapping(bytes32 delegateAccountHash => uint8 counter) _delegateAccountHashCounters;
        // prettier-ignore
        /// @dev Tracks the Coprocessors that has already delegated an account for a given delegateAccountHash.
        mapping(bytes32 delegateAccountHash =>
            mapping(address coprocessorTxSenderAddress => bool hasDelegated))
                _alreadyDelegatedCoprocessors;
        // prettier-ignore
        /// @dev Tracks the account delegations for a given contract after reaching consensus.
        mapping(address delegator => mapping(address delegated =>
            mapping(uint256 chainId => mapping(address contractAddress => bool isDelegated))))
                _delegatedContracts;
        // ----------------------------------------------------------------------------------------------
        // Coprocessor context state variables:
        // ----------------------------------------------------------------------------------------------
        mapping(bytes32 delegateAccountHash => uint256 contextId) delegateAccountContextId;
        /// @notice The context ID for the allowAccount consensus.
        mapping(bytes32 ctHandle => mapping(address accountAddress => uint256 contextId)) allowAccountContextId;
        mapping(bytes32 ctHandle => uint256 contextId) allowPublicDecryptContextId;
    }

    /// @dev Storage location has been computed using the following command:
    /// @dev keccak256(abi.encode(uint256(keccak256("fhevm_gateway.storage.MultichainAcl")) - 1))
    /// @dev & ~bytes32(uint256(0xff))
    bytes32 private constant MULTICHAIN_ACL_STORAGE_LOCATION =
        0xc6e55c773d840671d532b9f3847a71edf30a8cc021a5cb4790841cc1251d0700;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /// @notice Initializes the contract.
    /// @dev This function needs to be public in order to be called by the UUPS proxy.
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy() public virtual onlyFromEmptyProxy reinitializer(3) {
        __Ownable_init(owner());
        __Pausable_init();
    }

    /// @notice Reinitializes the contract.
    function reinitializeV2() external reinitializer(3) {}

    /// @dev See {IMultichainAcl-allowPublicDecrypt}.
    function allowPublicDecrypt(
        bytes32 ctHandle
    ) external virtual onlyHandleFromRegisteredHostChain(ctHandle) whenNotPaused refreshCoprocessorContextStatuses {
        MultichainAclStorage storage $ = _getMultichainAclStorage();

        // Get the context ID from the allow public decryption context ID mapping
        uint256 contextId = $.allowPublicDecryptContextId[ctHandle];

        // If the context ID is not set, get the active coprocessor context's ID and associate it to
        // this public decryption allow
        if (contextId == 0) {
            contextId = COPROCESSOR_CONTEXTS.getActiveCoprocessorContextId();
            $.allowPublicDecryptContextId[ctHandle] = contextId;

            // Else, that means a coprocessor already started to allow the public decryption and we need
            // to check that the context is active or suspended
            // If it is not, that means the context is no longer valid for this operation and we revert
        } else if (!COPROCESSOR_CONTEXTS.isCoprocessorContextActiveOrSuspended(contextId)) {
            ContextStatus contextStatus = COPROCESSOR_CONTEXTS.getCoprocessorContextStatus(contextId);
            revert InvalidCoprocessorContextAllowPublicDecrypt(ctHandle, contextId, contextStatus);
        }

        // Only accept coprocessor transaction senders from the same context
        COPROCESSOR_CONTEXTS.checkIsCoprocessorTxSenderFromContext(contextId, msg.sender);

        /**
         * @dev Check if the coprocessor has already allowed the ciphertext handle for public decryption.
         * A Coprocessor can only allow once for a given ctHandle, so it's not possible for it to allow
         * the same ctHandle for different host chains, hence the chain ID is not included in the mapping.
         */
        if ($._allowPublicDecryptCoprocessors[ctHandle][msg.sender]) {
            revert CoprocessorAlreadyAllowedPublicDecrypt(ctHandle, msg.sender);
        }
        $._allowPublicDecryptCounters[ctHandle]++;
        $._allowPublicDecryptCoprocessors[ctHandle][msg.sender] = true;

        // Only send the event if consensus has not been reached in a previous call and the consensus
        // is reached in the current call. This means a "late" addition will not be reverted, just ignored
        // Besides, consensus only considers the coprocessors of the same context
        if (
            !$.allowedPublicDecrypts[ctHandle] &&
            _isConsensusReached(contextId, $._allowPublicDecryptCounters[ctHandle])
        ) {
            $.allowedPublicDecrypts[ctHandle] = true;
            emit AllowPublicDecrypt(ctHandle);
        }
    }

    /// @dev See {IMultichainAcl-allowAccount}.
    function allowAccount(
        bytes32 ctHandle,
        address accountAddress
    ) external virtual onlyHandleFromRegisteredHostChain(ctHandle) whenNotPaused refreshCoprocessorContextStatuses {
        MultichainAclStorage storage $ = _getMultichainAclStorage();

        // Get the context ID from the allow account context ID mapping
        uint256 contextId = $.allowAccountContextId[ctHandle][accountAddress];

        // If the context ID is not set, get the active coprocessor context's ID and associate it to
        // this account allow
        if (contextId == 0) {
            contextId = COPROCESSOR_CONTEXTS.getActiveCoprocessorContextId();
            $.allowAccountContextId[ctHandle][accountAddress] = contextId;

            // Else, that means a coprocessor already started to allow the account and we need to check
            // that the context is active or suspended
            // If it is not, that means the context is no longer valid for this operation and we revert
        } else if (!COPROCESSOR_CONTEXTS.isCoprocessorContextActiveOrSuspended(contextId)) {
            ContextStatus contextStatus = COPROCESSOR_CONTEXTS.getCoprocessorContextStatus(contextId);
            revert InvalidCoprocessorContextAllowAccount(ctHandle, accountAddress, contextId, contextStatus);
        }

        // Only accept coprocessor transaction senders from the same context
        COPROCESSOR_CONTEXTS.checkIsCoprocessorTxSenderFromContext(contextId, msg.sender);

        /**
         * @dev Check if the coprocessor has already allowed the account to use the ciphertext handle.
         * A Coprocessor can only allow once for a given ctHandle, so it's not possible for it to allow
         * the same ctHandle for different host chains, hence the chain ID is not included in the mapping.
         */
        if ($._allowAccountCoprocessors[ctHandle][accountAddress][msg.sender]) {
            revert CoprocessorAlreadyAllowedAccount(ctHandle, accountAddress, msg.sender);
        }
        $._allowAccountCounters[ctHandle][accountAddress]++;
        $._allowAccountCoprocessors[ctHandle][accountAddress][msg.sender] = true;

        // Only send the event if consensus has not been reached in a previous call and the consensus
        // is reached in the current call. This means a "late" addition will not be reverted, just ignored
        // Besides, consensus only considers the coprocessors of the same context
        if (
            !$.allowedAccounts[ctHandle][accountAddress] &&
            _isConsensusReached(contextId, $._allowAccountCounters[ctHandle][accountAddress])
        ) {
            $.allowedAccounts[ctHandle][accountAddress] = true;
            emit AllowAccount(ctHandle, accountAddress);
        }
    }

    /// @dev See {IMultichainAcl-delegateAccount}.
    function delegateAccount(
        uint256 chainId,
        DelegationAccounts calldata delegationAccounts,
        address[] calldata contractAddresses
    ) external virtual whenNotPaused refreshCoprocessorContextStatuses {
        if (contractAddresses.length == 0) {
            revert EmptyContractAddresses();
        }
        if (contractAddresses.length > MAX_CONTRACT_ADDRESSES) {
            revert ContractsMaxLengthExceeded(MAX_CONTRACT_ADDRESSES, contractAddresses.length);
        }

        MultichainAclStorage storage $ = _getMultichainAclStorage();

        // Compute the hash of the ciphertext material to differentiate different ciphertext
        // material additions
        bytes32 delegateAccountHash = keccak256(abi.encode(chainId, delegationAccounts, contractAddresses));

        // Get the context ID from the delegate account context ID mapping
        uint256 contextId = $.delegateAccountContextId[delegateAccountHash];

        // If the context ID is not set, get the active coprocessor context's ID and associate it to
        // this account delegation
        if (contextId == 0) {
            contextId = COPROCESSOR_CONTEXTS.getActiveCoprocessorContextId();
            $.delegateAccountContextId[delegateAccountHash] = contextId;

            // Else, that means a coprocessor already started to delegate the account and we need to check
            // that the context is active or suspended
            // If it is not, that means the context is no longer valid for this operation and we revert
        } else if (!COPROCESSOR_CONTEXTS.isCoprocessorContextActiveOrSuspended(contextId)) {
            ContextStatus contextStatus = COPROCESSOR_CONTEXTS.getCoprocessorContextStatus(contextId);
            revert InvalidCoprocessorContextDelegateAccount(
                chainId,
                delegationAccounts,
                contractAddresses,
                contextId,
                contextStatus
            );
        }

        // Only accept coprocessor transaction senders from the same context
        COPROCESSOR_CONTEXTS.checkIsCoprocessorTxSenderFromContext(contextId, msg.sender);

        // The delegateAccountHash is the hash of all input arguments.
        // This hash is used to track the delegation consensus over the whole contractAddresses list.
        // It assumes that the coprocessors will delegate the same list of contracts and keep the same order.
        mapping(address => bool) storage alreadyDelegatedCoprocessors = $._alreadyDelegatedCoprocessors[
            delegateAccountHash
        ];

        /// @dev Check if the coprocessor has already delegated the account.
        if (alreadyDelegatedCoprocessors[msg.sender]) {
            revert CoprocessorAlreadyDelegated(chainId, delegationAccounts, contractAddresses, msg.sender);
        }

        $._delegateAccountHashCounters[delegateAccountHash]++;
        alreadyDelegatedCoprocessors[msg.sender] = true;

        // Only send the event if consensus has not been reached in a previous call and the consensus
        // is reached in the current call. This means a "late" addition will not be reverted, just ignored
        // Besides, consensus only considers the coprocessors of the same context
        if (
            !$._delegatedAccountHashes[delegateAccountHash] &&
            _isConsensusReached(contextId, $._delegateAccountHashCounters[delegateAccountHash])
        ) {
            mapping(address => bool) storage delegatedContracts = $._delegatedContracts[
                delegationAccounts.delegatorAddress
            ][delegationAccounts.delegatedAddress][chainId];
            for (uint256 i = 0; i < contractAddresses.length; i++) {
                delegatedContracts[contractAddresses[i]] = true;
            }
            $._delegatedAccountHashes[delegateAccountHash] = true;
            emit DelegateAccount(chainId, delegationAccounts, contractAddresses);
        }
    }

    /// @dev See {IMultichainAcl-checkPublicDecryptAllowed}.
    function checkPublicDecryptAllowed(bytes32 ctHandle) external view virtual {
        MultichainAclStorage storage $ = _getMultichainAclStorage();

        if (!$.allowedPublicDecrypts[ctHandle]) {
            revert PublicDecryptNotAllowed(ctHandle);
        }
    }

    /// @dev See {IMultichainAcl-checkAccountAllowed}.
    function checkAccountAllowed(bytes32 ctHandle, address accountAddress) external view virtual {
        MultichainAclStorage storage $ = _getMultichainAclStorage();

        /// @dev Check that the account address is allowed to use this ciphertext.
        if (!$.allowedAccounts[ctHandle][accountAddress]) {
            revert AccountNotAllowedToUseCiphertext(ctHandle, accountAddress);
        }
    }

    /// @dev See {IMultichainAcl-checkAccountDelegated}.
    function checkAccountDelegated(
        uint256 chainId,
        DelegationAccounts calldata delegationAccounts,
        address[] calldata contractAddresses
    ) external view virtual {
        if (contractAddresses.length == 0) {
            revert EmptyContractAddresses();
        }

        MultichainAclStorage storage $ = _getMultichainAclStorage();
        for (uint256 i = 0; i < contractAddresses.length; i++) {
            if (
                !$._delegatedContracts[delegationAccounts.delegatorAddress][delegationAccounts.delegatedAddress][
                    chainId
                ][contractAddresses[i]]
            ) {
                revert AccountNotDelegated(chainId, delegationAccounts, contractAddresses[i]);
            }
        }
    }

    /// @dev See {IMultichainAcl-getVersion}.
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
     * @dev Should revert when `msg.sender` is not authorized to upgrade the contract.
     */
    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyOwner {}

    /**
     * @notice Checks if the consensus is reached among the coprocessors from the same context.
     * @param contextId The coprocessor context ID
     * @param coprocessorCounter The number of coprocessors that agreed
     * @return Whether the consensus is reached
     */
    function _isConsensusReached(uint256 contextId, uint8 coprocessorCounter) internal view virtual returns (bool) {
        uint256 consensusThreshold = COPROCESSOR_CONTEXTS.getCoprocessorMajorityThresholdFromContext(contextId);
        return coprocessorCounter >= consensusThreshold;
    }

    /**
     * @dev Returns the MultichainAcl storage location.
     * Note that this function is internal but not virtual: derived contracts should be able to
     * access it, but if the underlying storage struct version changes, we force them to define a new
     * getter function and use that one instead in order to avoid overriding the storage location.
     */
    function _getMultichainAclStorage() internal pure returns (MultichainAclStorage storage $) {
        // solhint-disable-next-line no-inline-assembly
        assembly {
            $.slot := MULTICHAIN_ACL_STORAGE_LOCATION
        }
    }
}
