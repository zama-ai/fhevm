// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.28;
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import "./interfaces/ICiphertextStorage.sol";
import "./interfaces/IHTTPZ.sol";
import "./interfaces/IKeyManager.sol";

/// @title CiphertextStorage smart contract
/// @dev See {ICiphertextStorage}.
contract CiphertextStorage is ICiphertextStorage {
    /// @notice The address of the HTTPZ contract, used for fetching information about coprocessors.
    IHTTPZ internal immutable _HTTPZ;

    /// @notice The address of the KeyManager contract, used for fetching information about the current key.
    IKeyManager internal immutable _KEY_MANAGER;

    /// @notice The regular ciphertexts tied to the ciphertext handle.
    mapping(uint256 ctHandle => bytes ciphertext) internal _ciphertexts;
    /// @notice The SNS ciphertexts tied to the ciphertext handle.
    mapping(uint256 ctHandle => bytes snsCiphertext) internal _snsCiphertexts;
    /// @notice The key IDs used for generating the ciphertext.
    /// @dev It's necessary in case new keys are generated: we need to know what key to use for using a ciphertext.
    mapping(uint256 ctHandle => uint256 keyId) internal _keyIds;
    /// @notice The chain IDs associated to the ciphertext handle.
    mapping(uint256 ctHandle => uint256 chainId) internal _chainIds;
    /// @notice The mapping of the stored ciphertext of the given handle.
    mapping(uint256 ctHandle => bool isStored) internal _isStored;
    /// @notice The counter of the Coprocessors that have added the ciphertext.
    mapping(uint256 ctHandle => uint8 ctHandleCounter) internal _ctHandleCounters;
    /// @notice The mapping of the Coprocessors that have already sent the ciphertext.
    mapping(uint256 ctHandle => mapping(address coprocessorAddress => bool hasSent)) internal _ctHandleSenders;

    string private constant CONTRACT_NAME = "CiphertextStorage";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    constructor(IHTTPZ httpz, IKeyManager keyManager) {
        _HTTPZ = httpz;
        _KEY_MANAGER = keyManager;
    }

    /// @notice See {ICiphertextStorage-hasCiphertext}.
    function hasCiphertext(uint256 ctHandle) public view returns (bool) {
        return _isStored[ctHandle];
    }

    /// @notice See {ICiphertextStorage-isOnNetwork}.
    function isOnNetwork(uint256 ctHandle, uint256 chainId) public view returns (bool) {
        return _chainIds[ctHandle] == chainId;
    }

    /// @notice See {ICiphertextStorage-getCiphertextMaterials}.
    function getCiphertextMaterials(
        uint256[] calldata ctHandles
    ) public view returns (CiphertextMaterial[] memory ctMaterials) {
        ctMaterials = new CiphertextMaterial[](ctHandles.length);

        for (uint256 i = 0; i < ctHandles.length; i++) {
            if (!hasCiphertext(ctHandles[i])) {
                revert CiphertextNotFound(ctHandles[i]);
            }

            ctMaterials[i] = CiphertextMaterial(ctHandles[i], _keyIds[ctHandles[i]], _ciphertexts[ctHandles[i]]);
        }

        return ctMaterials;
    }

    /// @notice See {ICiphertextStorage-getSnsCiphertextMaterials}.
    function getSnsCiphertextMaterials(
        uint256[] calldata ctHandles
    ) public view returns (SnsCiphertextMaterial[] memory snsCtMaterials) {
        snsCtMaterials = new SnsCiphertextMaterial[](ctHandles.length);

        for (uint256 i = 0; i < ctHandles.length; i++) {
            if (!hasCiphertext(ctHandles[i])) {
                revert CiphertextNotFound(ctHandles[i]);
            }

            snsCtMaterials[i] = SnsCiphertextMaterial(
                ctHandles[i],
                _keyIds[ctHandles[i]],
                _snsCiphertexts[ctHandles[i]]
            );
        }

        return snsCtMaterials;
    }

    /// @notice See {ICiphertextStorage-addCiphertext}.
    /// @dev This function calls the HTTPZ contract to check that the sender address is a Coprocessor.
    function addCiphertext(
        uint256 ctHandle,
        uint256 keyId,
        uint256 chainId,
        bytes calldata ciphertext,
        bytes calldata snsCiphertext
    ) public {
        bool isCoprocessor = _HTTPZ.isCoprocessor(msg.sender);
        if (!isCoprocessor) {
            revert InvalidCoprocessorSender(msg.sender);
        }

        /// @dev Check if the Coprocessor has already added the ciphertext.
        if (_ctHandleSenders[ctHandle][msg.sender]) {
            revert CoprocessorHasAlreadyAdded(msg.sender);
        }

        /// @dev Check if the received key ID is the latest activated.
        // TODO: Revisit the following line accordingly with key lifecycles issue
        // See: https://github.com/zama-ai/gateway-l2/issues/90
        bool isCurrentKeyId = _KEY_MANAGER.isCurrentKeyId(keyId);
        if (!isCurrentKeyId) {
            revert InvalidCurrentKeyId(keyId);
        }

        _ctHandleCounters[ctHandle]++;
        _ctHandleSenders[ctHandle][msg.sender] = true;

        /// @dev Only send the event if consensus has not been reached in a previous call
        /// @dev and the consensus is reached in the current call.
        /// @dev This means a "late" allow will not be reverted, just ignored
        if (!hasCiphertext(ctHandle) && _isConsensusReached(_ctHandleCounters[ctHandle])) {
            _ciphertexts[ctHandle] = ciphertext;
            _snsCiphertexts[ctHandle] = snsCiphertext;
            _keyIds[ctHandle] = keyId;
            _chainIds[ctHandle] = chainId;
            _isStored[ctHandle] = true;

            emit AddCiphertext(ctHandle);
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

    /// @notice Checks if the consensus is reached among the Coprocessors.
    /// @dev This function calls the HTTPZ contract to retrieve the consensus threshold.
    /// @param coprocessorCounter The number of coprocessors that agreed
    /// @return Whether the consensus is reached
    function _isConsensusReached(uint256 coprocessorCounter) internal view virtual returns (bool) {
        uint256 consensusThreshold = _HTTPZ.getCoprocessorMajorityThreshold();
        return coprocessorCounter >= consensusThreshold;
    }
}
