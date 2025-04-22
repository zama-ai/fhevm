// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { gatewayConfigAddress } from "../addresses/GatewayConfigAddress.sol";
import { ciphertextCommitsAddress } from "../addresses/CiphertextCommitsAddress.sol";
import { Ownable2StepUpgradeable } from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import { UUPSUpgradeable } from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import "./interfaces/IMultichainAcl.sol";
import "./interfaces/ICiphertextCommits.sol";
import "./interfaces/IGatewayConfig.sol";
import "./shared/GatewayConfigChecks.sol";

/// @title MultichainAcl smart contract
/// @dev See {IMultichainAcl}
contract MultichainAcl is IMultichainAcl, Ownable2StepUpgradeable, UUPSUpgradeable, GatewayConfigChecks {
    /// @notice The address of the GatewayConfig contract for protocol state calls.
    IGatewayConfig private constant _GATEWAY_CONFIG = IGatewayConfig(gatewayConfigAddress);
    /// @notice The address of the CiphertextCommits contract for checking ciphertext materials.
    ICiphertextCommits private constant _CIPHERTEXT_COMMITS = ICiphertextCommits(ciphertextCommitsAddress);
    /// @notice The maximum number of contracts that can be requested for delegation.
    uint8 internal constant _MAX_CONTRACT_ADDRESSES = 10;

    string private constant CONTRACT_NAME = "MultichainAcl";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    /// @notice The contract's variable storage struct (@dev see ERC-7201)
    /// @custom:storage-location erc7201:fhevm_gateway.storage.MultichainAcl
    struct MultichainAclStorage {
        /// @notice Accounts allowed to use the ciphertext handle.
        mapping(bytes32 ctHandle => mapping(address accountAddress => bool isAllowed)) allowedAccounts;
        /// @notice The counter used for the allowAccount consensus.
        mapping(bytes32 ctHandle => mapping(address accountAddress => uint8 counter)) _allowAccountCounters;
        // prettier-ignore
        /// @notice Coprocessors that have already allowed an account to use the ciphertext handle.
        mapping(bytes32 ctHandle => mapping(address accountAddress =>
            mapping(address coprocessorTxSenderAddress => bool hasAllowed)))
                _allowAccountCoprocessors;
        /// @notice Allowed public decryptions.
        mapping(bytes32 ctHandle => bool isAllowed) allowedPublicDecrypts;
        /// @notice The counter used for the public decryption consensus.
        mapping(bytes32 ctHandle => uint8 counter) _allowPublicDecryptCounters;
        // prettier-ignore
        /// @notice Coprocessors that have already allowed a public decryption.
        mapping(bytes32 ctHandle => mapping(address coprocessorTxSenderAddress => bool hasAllowed)) 
            _allowPublicDecryptCoprocessors;
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
    function initialize() public virtual reinitializer(2) {
        __Ownable_init(owner());
    }

    /// @dev See {IMultichainAcl-allowPublicDecrypt}.
    function allowPublicDecrypt(
        bytes32 ctHandle
    ) public virtual override onlyCoprocessorTxSender onlyHandleFromRegisteredNetwork(ctHandle) {
        MultichainAclStorage storage $ = _getMultichainAclStorage();

        /**
         * @dev Check if the coprocessor has already allowed the ciphertext handle for public decryption.
         * A Coprocessor can only allow once for a given ctHandle, so it's not possible for it to allow
         * the same ctHandle for different chainIds, hence the chainId is not included in the mapping.
         */
        if ($._allowPublicDecryptCoprocessors[ctHandle][msg.sender]) {
            revert CoprocessorAlreadyAllowed(msg.sender, ctHandle);
        }
        $._allowPublicDecryptCounters[ctHandle]++;
        $._allowPublicDecryptCoprocessors[ctHandle][msg.sender] = true;

        /// @dev Only send the event if consensus has not been reached in a previous call
        /// @dev and the consensus is reached in the current call.
        /// @dev This means a "late" allow will not be reverted, just ignored
        if (!$.allowedPublicDecrypts[ctHandle] && _isConsensusReached($._allowPublicDecryptCounters[ctHandle])) {
            $.allowedPublicDecrypts[ctHandle] = true;
            emit AllowPublicDecrypt(ctHandle);
        }
    }

    /// @dev See {IMultichainAcl-allowAccount}.
    function allowAccount(
        bytes32 ctHandle,
        address accountAddress
    ) public virtual override onlyCoprocessorTxSender onlyHandleFromRegisteredNetwork(ctHandle) {
        MultichainAclStorage storage $ = _getMultichainAclStorage();

        /**
         * @dev Check if the coprocessor has already allowed the account to use the ciphertext handle.
         * A Coprocessor can only allow once for a given ctHandle, so it's not possible for it to allow
         * the same ctHandle for different chainIds, hence the chainId is not included in the mapping.
         */
        if ($._allowAccountCoprocessors[ctHandle][accountAddress][msg.sender]) {
            revert CoprocessorAlreadyAllowed(msg.sender, ctHandle);
        }
        $._allowAccountCounters[ctHandle][accountAddress]++;
        $._allowAccountCoprocessors[ctHandle][accountAddress][msg.sender] = true;

        /// @dev Only send the event if consensus has not been reached in a previous call
        /// @dev and the consensus is reached in the current call.
        /// @dev This means a "late" allow will not be reverted, just ignored
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
    ) public virtual override onlyCoprocessorTxSender {
        if (contractAddresses.length == 0) {
            revert EmptyContractAddresses();
        }
        if (contractAddresses.length > _MAX_CONTRACT_ADDRESSES) {
            revert ContractsMaxLengthExceeded(_MAX_CONTRACT_ADDRESSES, contractAddresses.length);
        }

        MultichainAclStorage storage $ = _getMultichainAclStorage();

        /// @dev The delegateAccountHash is the hash of all input arguments.
        /// @dev This hash is used to track the delegation consensus over the whole contractAddresses list,
        /// @dev and assumes that the Coprocessors will delegate the same list of contracts and keep the same order.
        bytes32 delegateAccountHash = keccak256(abi.encode(chainId, delegationAccounts, contractAddresses));

        mapping(address => bool) storage alreadyDelegatedCoprocessors = $._alreadyDelegatedCoprocessors[
            delegateAccountHash
        ];

        if (alreadyDelegatedCoprocessors[msg.sender]) {
            revert CoprocessorAlreadyDelegated(msg.sender, chainId, delegationAccounts, contractAddresses);
        }

        $._delegateAccountHashCounters[delegateAccountHash]++;
        alreadyDelegatedCoprocessors[msg.sender] = true;

        /// @dev Send the event if and only if the consensus is reached in the current response call.
        /// @dev This means a "late" response will not be reverted, just ignored
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
    function checkPublicDecryptAllowed(bytes32 ctHandle) public view virtual {
        MultichainAclStorage storage $ = _getMultichainAclStorage();

        if (!$.allowedPublicDecrypts[ctHandle]) {
            revert PublicDecryptNotAllowed(ctHandle);
        }
    }

    /// @dev See {IMultichainAcl-checkAccountAllowed}.
    function checkAccountAllowed(bytes32 ctHandle, address accountAddress) public view virtual {
        MultichainAclStorage storage $ = _getMultichainAclStorage();

        /// @dev Check that the account address is allowed to use this ciphertext.
        if (!$.allowedAccounts[ctHandle][accountAddress]) {
            revert AccountNotAllowedToUseCiphertext(ctHandle, accountAddress);
        }
    }

    /// @dev See {IMultichainAcl-isAccountDelegated}.
    function checkAccountDelegated(
        uint256 chainId,
        DelegationAccounts calldata delegationAccounts,
        address[] calldata contractAddresses
    ) public view virtual {
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

    /// @notice Returns the versions of the MultichainAcl contract in SemVer format.
    /// @dev This is conventionally used for upgrade features.
    function getVersion() public pure virtual returns (string memory) {
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

    /// @notice Checks if the consensus is reached among the Coprocessors.
    /// @dev This function calls the GatewayConfig contract to retrieve the consensus threshold.
    /// @param coprocessorCounter The number of coprocessors that agreed
    /// @return Whether the consensus is reached
    function _isConsensusReached(uint8 coprocessorCounter) internal view virtual returns (bool) {
        uint256 consensusThreshold = _GATEWAY_CONFIG.getCoprocessorMajorityThreshold();
        return coprocessorCounter >= consensusThreshold;
    }

    /**
     * @dev Returns the MultichainAcl storage location.
     */
    function _getMultichainAclStorage() internal pure returns (MultichainAclStorage storage $) {
        // solhint-disable-next-line no-inline-assembly
        assembly {
            $.slot := MULTICHAIN_ACL_STORAGE_LOCATION
        }
    }

    /**
     * @dev Should revert when `msg.sender` is not authorized to upgrade the contract.
     */
    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyOwner {}
}
