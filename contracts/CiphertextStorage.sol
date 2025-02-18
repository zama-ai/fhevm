// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.28;
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import "./interfaces/ICiphertextStorage.sol";
import "./interfaces/IHTTPZ.sol";

/// @title CiphertextStorage smart contract
/// @dev See {ICiphertextStorage}.
contract CiphertextStorage is ICiphertextStorage {
    /// @notice The address of the HTTPZ contract for protocol state calls.
    IHTTPZ internal immutable _HTTPZ;

    /// @notice The normal (64-bit) ciphertexts tied to the ciphertext handle.
    mapping(uint256 ctHandle => bytes ciphertext64) internal _ciphertext64s;
    /// @notice The PBS (128-bit) ciphertexts tied to the ciphertext handle.
    mapping(uint256 ctHandle => bytes ciphertext128) internal _ciphertext128s;
    /// @notice The key IDs used for generating the ciphertext.
    /// @dev It's necessary in case new keys are generated: we need to know what key to use for using a ciphertext.
    mapping(uint256 ctHandle => uint256 keyId) internal _keyIds;
    /// @notice The chain IDs associated to the ciphertext handle.
    mapping(uint256 ctHandle => uint256 chainId) internal _chainIds;
    /// @notice The mapping of the stored ciphertext of the given handle.
    mapping(uint256 ctHandle => bool isStored) internal _storedCiphertexts;
    /// @notice The counter of the Coprocessors that have added the ciphertext.
    mapping(uint256 ctHandle => uint8 ctHandleCounter) internal _ctHandleCounters;
    /// @notice The mapping of the Coprocessors that have already sent the ciphertext.
    mapping(uint256 ctHandle => mapping(address coprocessorAddress => bool hasSent)) internal _ctHandleSenders;

    string private constant CONTRACT_NAME = "CiphertextStorage";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    constructor(IHTTPZ httpz) {
        _HTTPZ = httpz;
    }

    /// @notice See {ICiphertextStorage-hasCiphertext}.
    function hasCiphertext(uint256 ctHandle) public view returns (bool) {
        return _storedCiphertexts[ctHandle];
    }

    /// @notice See {ICiphertextStorage-isOnNetwork}.
    function isOnNetwork(uint256 ctHandle, uint256 chainId) public view returns (bool) {
        return _chainIds[ctHandle] == chainId;
    }

    /// @notice See {ICiphertextStorage-getCiphertexts}.
    function getCiphertexts(
        uint256[] calldata ctHandles
    ) public view returns (CiphertextMaterial[] memory ctMaterials) {
        ctMaterials = new CiphertextMaterial[](ctHandles.length);
        for (uint256 i = 0; i < ctHandles.length; i++) {
            ctMaterials[i] = CiphertextMaterial(ctHandles[i], _keyIds[ctHandles[i]], _ciphertext128s[ctHandles[i]]);
        }
        return ctMaterials;
    }

    /// @notice See {ICiphertextStorage-addCiphertext}.
    /// @dev This function calls the HTTPZ contract to check that the sender address is a Coprocessor.
    function addCiphertext(
        uint256 ctHandle,
        uint256 keyId,
        uint256 chainId,
        bytes calldata ciphertext64,
        bytes calldata ciphertext128
    ) public {
        // TODO: Implement the HTTPZ.isCoprocessor(msg.sender) contract call
        bool isCoprocessor = true;
        if (!isCoprocessor) {
            revert InvalidCoprocessorSender(msg.sender);
        }
        // TODO: Implement the HTTPZ.isCurrentKeyId(keyId) contract call
        bool isCurrentKeyId = true;
        if (!isCurrentKeyId) {
            revert InvalidCurrentKeyId(keyId);
        }
        if (_ctHandleSenders[ctHandle][msg.sender]) {
            revert CoprocessorHasAlreadyAdded(msg.sender);
        }
        _ctHandleCounters[ctHandle]++;
        _ctHandleSenders[ctHandle][msg.sender] = true;
        if (!hasCiphertext(ctHandle) && _isConsensusReached(ctHandle)) {
            _ciphertext64s[ctHandle] = ciphertext64;
            _ciphertext128s[ctHandle] = ciphertext128;
            _keyIds[ctHandle] = keyId;
            _chainIds[ctHandle] = chainId;
            _storedCiphertexts[ctHandle] = true;
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

    /// @notice Checks if the Ciphertext storing consensus is reached among the Coprocessors.
    /// @dev This function calls the HTTPZ contract to retrieve the current Coprocessors.
    /// @dev The consensus threshold is calculated as the simple majority of the total Coprocessors.
    function _isConsensusReached(uint256 ctHandle) internal view returns (bool) {
        // TODO: Implement the HTTPZ.getCoprocessorsCount() contract call
        uint256 coprocessorsCount = 4;
        uint256 consensusThreshold = coprocessorsCount / 2 + 1;
        return _ctHandleCounters[ctHandle] >= consensusThreshold;
    }
}
