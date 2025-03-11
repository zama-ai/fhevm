// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import "./interfaces/IACLManager.sol";
import "./interfaces/ICiphertextStorage.sol";
import "./interfaces/IHTTPZ.sol";

/// @title ACLManager smart contract
/// @dev See {IACLManager}
contract ACLManager is IACLManager {
    /// @notice The address of the HTTPZ contract for protocol state calls.
    IHTTPZ internal immutable _HTTPZ;
    /// @notice The address of the CiphertextStorage contract from which ciphertexts are retrieve.
    ICiphertextStorage internal immutable _CIPHERTEXT_STORAGE;
    /// @notice The maximum number of contracts that can be requested for delegation.
    uint8 internal constant _MAX_CONTRACT_ADDRESSES = 10;

    /// @dev The mapping of the already allowed user decryptions.
    mapping(uint256 ctHandle => mapping(address userAddress => bool isAllowed)) public allowedUserDecrypts;
    /// @dev The counter used for the user decryption consensus.
    mapping(uint256 ctHandle => mapping(address userAddress => uint8 counter)) internal _allowUserDecryptCounters;
    /// @notice The mapping of the Coprocessors that have already allowed the user decryption.
    mapping(uint256 ctHandle => mapping(address userAddress => mapping(address coprocessorAddress => bool hasAllowed)))
        internal _allowUserDecryptAuthorizers;

    /// @dev The mapping of the already allowed public decryptions.
    mapping(uint256 ctHandle => bool isAllowed) public allowedPublicDecrypts;
    /// @dev The counter used for the public decryption consensus.
    mapping(uint256 ctHandle => uint8 counter) internal _allowPublicDecryptCounters;
    /// @notice The mapping of the Coprocessors that have already allowed the user decryption.
    mapping(uint256 ctHandle => mapping(address coprocessorAddress => bool hasAllowed))
        internal _allowPublicDecryptAuthorizers;

    /// @dev Tracks the computed delegateAccountHash that has already been delegated.
    mapping(bytes32 delegateAccountHash => bool isDelegated) internal _delegatedAccountHashes;
    /// @dev Tracks the number of times a delegateAccountHash has received confirmations.
    mapping(bytes32 delegateAccountHash => uint8 counter) internal _delegateAccountHashCounters;
    /// @dev Tracks the Coprocessors that has already delegated an account for a given delegateAccountHash.
    mapping(bytes32 delegateAccountHash => mapping(address coprocessorAddress => bool hasDelegated))
        internal _alreadyDelegatedCoprocessors;
    // prettier-ignore
    /// @dev Tracks the account delegations for a given contract after reaching consensus.
    mapping(address delegator => mapping(address delegatee =>
        mapping(uint256 chainId => mapping(address contractAddress => bool isDelegated))))
            internal _delegatedContracts;

    string private constant CONTRACT_NAME = "ACLManager";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    constructor(IHTTPZ httpz, ICiphertextStorage ciphertextStorage) {
        _HTTPZ = httpz;
        _CIPHERTEXT_STORAGE = ciphertextStorage;
    }

    /// @notice Checks if the sender is a Coprocessor.
    modifier onlyCoprocessor() {
        _HTTPZ.checkIsCoprocessor(msg.sender);
        _;
    }

    /// @notice Checks if the given ciphertext handle is associated to the given chain ID.
    /// @dev TODO: Remove chainId check and replace with pending allow logic in allow calls
    /// @dev https://github.com/zama-ai/gateway-l2/issues/171
    modifier isHandleOnNetwork(uint256 ctHandle, uint256 chainId) {
        // _CIPHERTEXT_STORAGE.checkIsOnNetwork(ctHandle, chainId);
        _;
    }

    /// @dev See {IACLManager-allowUserDecrypt}.
    function allowUserDecrypt(
        uint256 chainId,
        uint256 ctHandle,
        address allowedAddress
    ) public virtual override onlyCoprocessor isHandleOnNetwork(ctHandle, chainId) {
        /// @dev Check if the Coprocessor has already allowed the decryption.
        if (_allowUserDecryptAuthorizers[ctHandle][allowedAddress][msg.sender]) {
            revert CoprocessorHasAlreadyAllowed(msg.sender, ctHandle);
        }
        _allowUserDecryptCounters[ctHandle][allowedAddress]++;
        _allowUserDecryptAuthorizers[ctHandle][allowedAddress][msg.sender] = true;

        /// @dev Only send the event if consensus has not been reached in a previous call
        /// @dev and the consensus is reached in the current call.
        /// @dev This means a "late" allow will not be reverted, just ignored
        if (
            !allowedUserDecrypts[ctHandle][allowedAddress] &&
            _isConsensusReached(_allowUserDecryptCounters[ctHandle][allowedAddress])
        ) {
            emit AllowUserDecrypt(ctHandle, allowedAddress);
            allowedUserDecrypts[ctHandle][allowedAddress] = true;
        }
    }

    /// @dev See {IACLManager-allowPublicDecrypt}.
    function allowPublicDecrypt(
        uint256 chainId,
        uint256 ctHandle
    ) public virtual override onlyCoprocessor isHandleOnNetwork(ctHandle, chainId) {
        if (_allowPublicDecryptAuthorizers[ctHandle][msg.sender]) {
            revert CoprocessorHasAlreadyAllowed(msg.sender, ctHandle);
        }
        _allowPublicDecryptCounters[ctHandle]++;
        _allowPublicDecryptAuthorizers[ctHandle][msg.sender] = true;

        /// @dev Only send the event if consensus has not been reached in a previous call
        /// @dev and the consensus is reached in the current call.
        /// @dev This means a "late" allow will not be reverted, just ignored
        if (!allowedPublicDecrypts[ctHandle] && _isConsensusReached(_allowPublicDecryptCounters[ctHandle])) {
            emit AllowPublicDecrypt(ctHandle);
            allowedPublicDecrypts[ctHandle] = true;
        }
    }

    /// @dev See {IACLManager-delegateAccount}.
    function delegateAccount(
        uint256 chainId,
        address delegator,
        address delegatee,
        address[] calldata contractAddresses
    ) public virtual override onlyCoprocessor {
        if (contractAddresses.length > _MAX_CONTRACT_ADDRESSES) {
            revert ContractsMaxLengthExceeded(_MAX_CONTRACT_ADDRESSES, contractAddresses.length);
        }
        /// @dev The delegateAccountHash is the hash of all input arguments.
        /// @dev This hash is used to track the delegation consensus over the whole contractAddresses list,
        /// @dev and assumes that the Coprocessors will delegate the same list of contracts and keep the same order.
        bytes32 delegateAccountHash = keccak256(abi.encode(chainId, delegator, delegatee, contractAddresses));

        mapping(address => bool) storage alreadyDelegatedCoprocessors = _alreadyDelegatedCoprocessors[
            delegateAccountHash
        ];

        if (alreadyDelegatedCoprocessors[msg.sender]) {
            revert CoprocessorHasAlreadyDelegated(msg.sender, chainId, delegator, delegatee, contractAddresses);
        }

        _delegateAccountHashCounters[delegateAccountHash]++;
        alreadyDelegatedCoprocessors[msg.sender] = true;

        /// @dev Send the event if and only if the consensus is reached in the current response call.
        /// @dev This means a "late" response will not be reverted, just ignored
        if (
            !_delegatedAccountHashes[delegateAccountHash] &&
            _isConsensusReached(_delegateAccountHashCounters[delegateAccountHash])
        ) {
            mapping(address => bool) storage delegatedContracts = _delegatedContracts[delegator][delegatee][chainId];
            for (uint256 i = 0; i < contractAddresses.length; i++) {
                delegatedContracts[contractAddresses[i]] = true;
            }
            _delegatedAccountHashes[delegateAccountHash] = true;
            emit DelegateAccount(chainId, delegator, delegatee, contractAddresses);
        }
    }

    /// @dev See {IACLManager-checkUserDecryptAllowed}.
    function checkUserDecryptAllowed(
        address userAddress,
        IDecryptionManager.CtHandleContractPair[] calldata ctHandleContractPairs
    ) public view virtual {
        for (uint256 i = 0; i < ctHandleContractPairs.length; i++) {
            uint256 ctHandle = ctHandleContractPairs[i].ctHandle;
            address contractAddress = ctHandleContractPairs[i].contractAddress;

            /// @dev Check that the contract address is difference from the user address
            if (userAddress == contractAddress) {
                revert UserAddressInContractAddresses(userAddress);
            }

            /// @dev Check that the user is allowed to decrypt this ciphertext.
            if (!allowedUserDecrypts[ctHandle][userAddress]) {
                revert UserNotAllowedToUserDecrypt(ctHandle, userAddress);
            }

            /// @dev Check that the contract is allowed to decrypt this ciphertext.
            if (!allowedUserDecrypts[ctHandle][contractAddress]) {
                revert ContractNotAllowedToUserDecrypt(ctHandle, contractAddress);
            }
        }
    }

    /// @dev See {IACLManager-checkPublicDecryptAllowed}.
    function checkPublicDecryptAllowed(uint256[] calldata ctHandles) public view virtual {
        /// @dev Iterate over the ctHandles to check if the public decryption is allowed.
        for (uint256 i = 0; i < ctHandles.length; i++) {
            if (!allowedPublicDecrypts[ctHandles[i]]) {
                revert PublicDecryptNotAllowed(ctHandles[i]);
            }
        }
    }

    /// @notice Returns the versions of the CiphertextStorage contract in SemVer format.
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

    /// @dev See {IACLManager-isAccountDelegated}.
    function checkAccountDelegated(
        uint256 chainId,
        address delegator,
        address delegatee,
        address[] calldata contractAddresses
    ) public view virtual {
        for (uint256 i = 0; i < contractAddresses.length; i++) {
            if (!_delegatedContracts[delegator][delegatee][chainId][contractAddresses[i]]) {
                revert AccountNotDelegated(chainId, delegator, delegatee, contractAddresses[i]);
            }
        }
    }

    /// @notice Checks if the consensus is reached among the Coprocessors.
    /// @dev This function calls the HTTPZ contract to retrieve the consensus threshold.
    /// @param coprocessorCounter The number of coprocessors that agreed
    /// @return Whether the consensus is reached
    function _isConsensusReached(uint8 coprocessorCounter) internal view virtual returns (bool) {
        uint256 consensusThreshold = _HTTPZ.getCoprocessorMajorityThreshold();
        return coprocessorCounter >= consensusThreshold;
    }
}
