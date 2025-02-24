// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.28;

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
    /// @notice The maximum number of ciphertext handles that can be requested at once.
    uint8 internal constant _MAX_CONTRACTS_INPUT = 10;

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

    // TODO: Revisit the delegation storage structures; maybe use the delegationDigest as the mapping index.
    // prettier-ignore
    /// @dev The mapping of the already delegated accounts.
    mapping(address delegator => mapping(address delegatee =>
        mapping(uint256 chainId => mapping(bytes32 delegationDigest => bool isDelegated))))
            internal _delegatedAccounts;
    // prettier-ignore
    /// @dev The counter used for the account delegation consensus.
    mapping(address delegator => mapping(address delegatee =>
        mapping(uint256 chainId => mapping(bytes32 delegationDigest => uint8 counter))))
            internal _delegateAccountCounters;
    // prettier-ignore
    /// @dev The mapping of the Coprocessors that have already delegated the account.
    mapping(address delegator => mapping(address delegatee =>
        mapping(uint256 chainId => mapping(bytes32 delegationDigest =>
            mapping(address coprocessorAddress => bool hasDelegated)))))
                internal _delegateAccountAuthorizers;

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
        bool isCoprocessor = _HTTPZ.isCoprocessor(msg.sender);
        if (!isCoprocessor) {
            revert InvalidCoprocessorSender(msg.sender);
        }
        _;
    }

    /// @notice Checks if the given ciphertext handle is associated to the given chain ID.
    modifier isHandleOnNetwork(uint256 ctHandle, uint256 chainId) {
        bool isOnNetwork = _CIPHERTEXT_STORAGE.isOnNetwork(ctHandle, chainId);
        if (!isOnNetwork) {
            revert CiphertextHandleNotOnNetwork(ctHandle, chainId);
        }
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
        address[] calldata allowedContracts
    ) public virtual override onlyCoprocessor {
        if (allowedContracts.length > _MAX_CONTRACTS_INPUT) {
            revert TooManyContractsRequested(_MAX_CONTRACTS_INPUT, allowedContracts.length);
        }
        /// @dev The delegation digest is the hash of the allowedContracts list.
        /// @dev This digest is used to track the delegation consensus over the whole allowedContracts list,
        /// @dev and assumes that the Coprocessors will delegate the same list of contracts and keep the same order.
        bytes32 delegationDigest = keccak256(abi.encode(allowedContracts));

        /// @dev Declare storage variables as they are used multiple times during delegation request processing.
        mapping(bytes32 => bool) storage delegatedAccounts = _delegatedAccounts[delegator][delegatee][chainId];
        mapping(bytes32 => uint8) storage delegateAccountCounters = _delegateAccountCounters[delegator][delegatee][
            chainId
        ];
        mapping(address => bool) storage delegateAccountAuthorizers = _delegateAccountAuthorizers[delegator][delegatee][
            chainId
        ][delegationDigest];

        if (delegateAccountAuthorizers[msg.sender]) {
            revert CoprocessorHasAlreadyDelegated(msg.sender);
        }

        delegateAccountCounters[delegationDigest]++;
        delegateAccountAuthorizers[msg.sender] = true;

        /// @dev Only send the event if consensus has not been reached in a previous response call
        /// @dev and the consensus is reached in the current response call.
        /// @dev This means a "late" delegation will not be reverted, just ignored
        if (!delegatedAccounts[delegationDigest] && _isConsensusReached(delegateAccountCounters[delegationDigest])) {
            for (uint256 i = 0; i < allowedContracts.length; i++) {
                emit DelegateAccount(chainId, delegator, delegatee, allowedContracts[i]);
            }
            delegatedAccounts[delegationDigest] = true;
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
    function isAccountDelegated(
        uint256 chainId,
        address delegator,
        address delegatee,
        address[] calldata allowedContracts
    ) public view virtual returns (bool) {
        bytes32 delegationDigest = keccak256(abi.encode(allowedContracts));
        return _delegatedAccounts[delegator][delegatee][chainId][delegationDigest];
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
