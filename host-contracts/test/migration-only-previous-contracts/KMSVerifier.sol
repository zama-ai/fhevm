// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {ACLOwnable} from "../../contracts/shared/ACLOwnable.sol";
import {EIP712UpgradeableCrossChain} from "../../contracts/shared/EIP712UpgradeableCrossChain.sol";
import {KMS_CONTEXT_COUNTER_BASE} from "../../contracts/shared/Constants.sol";
import {UUPSUpgradeableEmptyProxy} from "../../contracts/shared/UUPSUpgradeableEmptyProxy.sol";

/// @dev Historical on-disk snapshot of KMSVerifier v0.2.0 kept as a migration-only fixture
///      for `test/tasks/migration.ts`.
contract KMSVerifier is UUPSUpgradeableEmptyProxy, EIP712UpgradeableCrossChain, ACLOwnable {
    error KMSAlreadySigner();
    error KMSSignerNull();
    error SignersSetIsEmpty();
    error ThresholdIsNull();
    error ThresholdIsAboveNumberOfSigners();

    event NewContextSet(uint256 indexed kmsContextId, address[] newKmsSignersSet, uint256 newThreshold);

    string private constant CONTRACT_NAME = "KMSVerifier";
    string private constant CONTRACT_NAME_SOURCE = "Decryption";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 2;
    uint256 private constant PATCH_VERSION = 0;

    uint64 private constant REINITIALIZER_VERSION = 3;

    /// @custom:storage-location erc7201:fhevm.storage.KMSVerifier
    struct KMSVerifierStorage {
        mapping(address => bool) isSigner;
        address[] signers;
        uint256 threshold;
        uint256 currentKmsContextId;
        mapping(uint256 => address[]) contextSigners;
        mapping(uint256 => mapping(address => bool)) contextIsSigner;
        mapping(uint256 => uint256) contextThreshold;
        mapping(uint256 => bool) destroyedContexts;
    }

    /// @dev keccak256(abi.encode(uint256(keccak256("fhevm.storage.KMSVerifier")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant KMSVerifierStorageLocation =
        0x7e81a744be86773af8644dd7304fa1dc9350ccabf16cfcaa614ddb78b4ce8900;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy(
        address verifyingContractSource,
        uint64 chainIDSource,
        address[] calldata initialSigners,
        uint256 initialThreshold
    ) public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        __EIP712_init(CONTRACT_NAME_SOURCE, "1", verifyingContractSource, chainIDSource);
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        $.currentKmsContextId = KMS_CONTEXT_COUNTER_BASE;
        _defineContext(initialSigners, initialThreshold);
    }

    function defineNewContext(address[] memory newSignersSet, uint256 newThreshold) public virtual onlyACLOwner {
        uint256 newContextId = _defineContext(newSignersSet, newThreshold);
        emit NewContextSet(newContextId, newSignersSet, newThreshold);
    }

    function getKmsSigners() public view virtual returns (address[] memory) {
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        return $.contextSigners[$.currentKmsContextId];
    }

    function getThreshold() public view virtual returns (uint256) {
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        return $.contextThreshold[$.currentKmsContextId];
    }

    function isSigner(address account) public view virtual returns (bool) {
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        return $.contextIsSigner[$.currentKmsContextId][account];
    }

    function getCurrentKmsContextId() public view virtual returns (uint256) {
        return _getKMSVerifierStorage().currentKmsContextId;
    }

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

    function _defineContext(
        address[] memory newSignersSet,
        uint256 newThreshold
    ) internal virtual returns (uint256 newContextId) {
        if (newSignersSet.length == 0) {
            revert SignersSetIsEmpty();
        }

        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        newContextId = ++$.currentKmsContextId;
        _setContextSigners(newContextId, newSignersSet);
        _setContextThreshold(newContextId, newThreshold);
    }

    function _setContextThreshold(uint256 contextId, uint256 threshold_) internal virtual {
        if (threshold_ == 0) {
            revert ThresholdIsNull();
        }

        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        if (threshold_ > $.contextSigners[contextId].length) {
            revert ThresholdIsAboveNumberOfSigners();
        }
        $.contextThreshold[contextId] = threshold_;
    }

    function _setContextSigners(uint256 contextId, address[] memory signersList) internal virtual {
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        for (uint256 i = 0; i < signersList.length; i++) {
            address signer = signersList[i];
            if (signer == address(0)) {
                revert KMSSignerNull();
            }
            if ($.contextIsSigner[contextId][signer]) {
                revert KMSAlreadySigner();
            }
            $.contextSigners[contextId].push(signer);
            $.contextIsSigner[contextId][signer] = true;
        }
    }

    function _getKMSVerifierStorage() internal pure returns (KMSVerifierStorage storage $) {
        assembly {
            $.slot := KMSVerifierStorageLocation
        }
    }

    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyACLOwner {}
}
