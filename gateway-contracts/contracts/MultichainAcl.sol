// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { gatewayConfigAddress } from "../addresses/GatewayAddresses.sol";
import { Ownable2StepUpgradeable } from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import "./interfaces/IMultichainAcl.sol";
import "./interfaces/ICiphertextCommits.sol";
import "./interfaces/IGatewayConfig.sol";
import "./shared/UUPSUpgradeableEmptyProxy.sol";
import "./shared/GatewayConfigChecks.sol";
import "./shared/Pausable.sol";

/// @title MultichainAcl smart contract
/// @dev See {IMultichainAcl}
contract MultichainAcl is
    IMultichainAcl,
    Ownable2StepUpgradeable,
    UUPSUpgradeableEmptyProxy,
    GatewayConfigChecks,
    Pausable
{
    /// @notice The address of the GatewayConfig contract for protocol state calls.
    IGatewayConfig private constant GATEWAY_CONFIG = IGatewayConfig(gatewayConfigAddress);

    /// @notice The maximum number of contracts that can be requested for delegation.
    uint8 internal constant MAX_CONTRACT_ADDRESSES = 10;

    /// @dev The following constants are used for versioning the contract. They are made private
    /// @dev in order to force derived contracts to consider a different version. Note that
    /// @dev they can still define their own private constants with the same name.
    string private constant CONTRACT_NAME = "MultichainAcl";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    /// Constant used for making sure the version number using in the `reinitializer` modifier is
    /// identical between `initializeFromEmptyProxy` and the reinitializeVX` method
    uint64 private constant REINITIALIZER_VERSION = 3;

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
        // Transaction sender addresses from consensus state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The coprocessor transaction senders involved in a consensus for allowing a public decryption.
        mapping(bytes32 ctHandle => address[] coprocessorTxSenderAddresses) allowPublicDecryptConsensusTxSenders;
        // prettier-ignore
        /// @notice The coprocessor transaction senders involved in a consensus for allowing an account.
        mapping(bytes32 ctHandle => mapping(address accountAddress =>
            address[] coprocessorTxSenderAddresses))
               allowAccountConsensusTxSenders;
        // @notice The coprocessor transaction senders involved in a consensus for delegating an account.
        mapping(bytes32 delegateAccountHash => address[] coprocessorTxSenderAddresses) delegateAccountConsensusTxSenders;
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
    function initializeFromEmptyProxy() public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        __Ownable_init(owner());
        __Pausable_init();
    }

    /**
     * @notice Re-initializes the contract from V1.
     */
    function reinitializeV2() public virtual reinitializer(REINITIALIZER_VERSION) {}

    /// @dev See {IMultichainAcl-allowPublicDecrypt}.
    function allowPublicDecrypt(
        bytes32 ctHandle,
        bytes calldata /* extraData */
    ) external virtual onlyCoprocessorTxSender onlyHandleFromRegisteredHostChain(ctHandle) {
        MultichainAclStorage storage $ = _getMultichainAclStorage();

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

        // Store the coprocessor transaction sender address for the public decryption response
        // It is important to consider the same mapping fields used for the consensus
        // A "late" valid coprocessor transaction sender address will still be added in the list.
        $.allowPublicDecryptConsensusTxSenders[ctHandle].push(msg.sender);

        // Send the event if and only if the consensus is reached in the current response call.
        // This means a "late" response will not be reverted, just ignored and no event will be emitted
        if (!$.allowedPublicDecrypts[ctHandle] && _isConsensusReached($._allowPublicDecryptCounters[ctHandle])) {
            $.allowedPublicDecrypts[ctHandle] = true;
            emit AllowPublicDecrypt(ctHandle);
        }
    }

    /// @dev See {IMultichainAcl-allowAccount}.
    function allowAccount(
        bytes32 ctHandle,
        address accountAddress,
        bytes calldata /* extraData */
    ) external virtual onlyCoprocessorTxSender onlyHandleFromRegisteredHostChain(ctHandle) {
        MultichainAclStorage storage $ = _getMultichainAclStorage();

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

        // Store the coprocessor transaction sender address for the allow account response
        // It is important to consider the same mapping fields used for the consensus
        // A "late" valid coprocessor transaction sender address will still be added in the list.
        $.allowAccountConsensusTxSenders[ctHandle][accountAddress].push(msg.sender);

        // Send the event if and only if the consensus is reached in the current response call.
        // This means a "late" response will not be reverted, just ignored and no event will be emitted
        if (
            !$.allowedAccounts[ctHandle][accountAddress] &&
            _isConsensusReached($._allowAccountCounters[ctHandle][accountAddress])
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
    ) external virtual onlyCoprocessorTxSender {
        if (contractAddresses.length == 0) {
            revert EmptyContractAddresses();
        }
        if (contractAddresses.length > MAX_CONTRACT_ADDRESSES) {
            revert ContractsMaxLengthExceeded(MAX_CONTRACT_ADDRESSES, contractAddresses.length);
        }

        MultichainAclStorage storage $ = _getMultichainAclStorage();

        /// @dev The delegateAccountHash is the hash of all input arguments.
        /// @dev This hash is used to track the delegation consensus over the whole contractAddresses list,
        /// @dev and assumes that the Coprocessors will delegate the same list of contracts and keep the same order.
        bytes32 delegateAccountHash = _getDelegateAccountHash(chainId, delegationAccounts, contractAddresses);

        mapping(address => bool) storage alreadyDelegatedCoprocessors = $._alreadyDelegatedCoprocessors[
            delegateAccountHash
        ];

        /// @dev Check if the coprocessor has already delegated the account.
        if (alreadyDelegatedCoprocessors[msg.sender]) {
            revert CoprocessorAlreadyDelegated(chainId, delegationAccounts, contractAddresses, msg.sender);
        }

        $._delegateAccountHashCounters[delegateAccountHash]++;
        alreadyDelegatedCoprocessors[msg.sender] = true;

        // Store the coprocessor transaction sender address for the delegate account response
        // It is important to consider the same mapping fields used for the consensus
        // A "late" valid coprocessor transaction sender address will still be added in the list.
        $.delegateAccountConsensusTxSenders[delegateAccountHash].push(msg.sender);

        // Send the event if and only if the consensus is reached in the current response call.
        // This means a "late" response will not be reverted, just ignored and no event will be emitted
        if (
            !$._delegatedAccountHashes[delegateAccountHash] &&
            _isConsensusReached($._delegateAccountHashCounters[delegateAccountHash])
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

    /// @dev See {IMultichainAcl-getAllowPublicDecryptConsensusTxSenders}.
    function getAllowPublicDecryptConsensusTxSenders(
        bytes32 ctHandle
    ) external view virtual returns (address[] memory) {
        MultichainAclStorage storage $ = _getMultichainAclStorage();

        return $.allowPublicDecryptConsensusTxSenders[ctHandle];
    }

    /// @dev See {IMultichainAcl-getAllowAccountConsensusTxSenders}.
    function getAllowAccountConsensusTxSenders(
        bytes32 ctHandle,
        address accountAddress
    ) external view virtual returns (address[] memory) {
        MultichainAclStorage storage $ = _getMultichainAclStorage();

        return $.allowAccountConsensusTxSenders[ctHandle][accountAddress];
    }

    /**
     * @dev See {IMultichainAcl-getDelegateAccountConsensusTxSenders}.
     * The contract address list needs to be provided in the same order as when the consensus was reached
     * in order to be able to retrieve the coprocessor transaction senders associated to it.
     */
    function getDelegateAccountConsensusTxSenders(
        uint256 chainId,
        DelegationAccounts calldata delegationAccounts,
        address[] calldata contractAddresses
    ) external view virtual returns (address[] memory) {
        MultichainAclStorage storage $ = _getMultichainAclStorage();

        // Get the hash of the delegate account's inputs used to track the consensus.
        bytes32 delegateAccountHash = _getDelegateAccountHash(chainId, delegationAccounts, contractAddresses);

        return $.delegateAccountConsensusTxSenders[delegateAccountHash];
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
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyGatewayOwner {}

    /// @notice Checks if the consensus is reached among the Coprocessors.
    /// @param coprocessorCounter The number of coprocessors that agreed
    /// @return Whether the consensus is reached
    function _isConsensusReached(uint8 coprocessorCounter) internal view virtual returns (bool) {
        uint256 consensusThreshold = GATEWAY_CONFIG.getCoprocessorMajorityThreshold();
        return coprocessorCounter >= consensusThreshold;
    }

    /// @dev Returns the hash of a delegate account's inputs.
    function _getDelegateAccountHash(
        uint256 chainId,
        DelegationAccounts calldata delegationAccounts,
        address[] calldata contractAddresses
    ) internal pure virtual returns (bytes32) {
        return keccak256(abi.encode(chainId, delegationAccounts, contractAddresses));
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
