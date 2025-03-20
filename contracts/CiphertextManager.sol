// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import { httpzAddress } from "../addresses/HttpzAddress.sol";
import { keyManagerAddress } from "../addresses/KeyManagerAddress.sol";
import { Ownable2StepUpgradeable } from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import { UUPSUpgradeable } from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import "./interfaces/ICiphertextManager.sol";
import "./interfaces/IHTTPZ.sol";
import "./interfaces/IKeyManager.sol";

/**
 * @title CiphertextManager smart contract
 * @dev See {ICiphertextManager}.
 */
contract CiphertextManager is ICiphertextManager, Ownable2StepUpgradeable, UUPSUpgradeable {
    /// @notice The address of the HTTPZ contract, used for fetching information about coprocessors.
    IHTTPZ private constant _HTTPZ = IHTTPZ(httpzAddress);

    /// @notice The address of the KeyManager contract, used for fetching information about the current key.
    IKeyManager private constant _KEY_MANAGER = IKeyManager(keyManagerAddress);

    string private constant CONTRACT_NAME = "CiphertextManager";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    /// @notice The contract's variable storage struct (@dev see ERC-7201)
    /// @custom:storage-location erc7201:httpz_gateway.storage.CiphertextManager
    struct CiphertextManagerStorage {
        /// @notice The regular ciphertext digests tied to the ciphertext handle.
        mapping(uint256 ctHandle => bytes32 ctDigest) _ciphertextDigests;
        /// @notice The SNS ciphertext digests tied to the ciphertext handle.
        mapping(uint256 ctHandle => bytes32 snsCtDigest) _snsCiphertextDigests;
        /// @notice The key IDs used for generating the ciphertext.
        /// @dev It's necessary in case new keys are generated: we need to know what key to use for using a ciphertext.
        mapping(uint256 ctHandle => uint256 keyId) _keyIds;
        /// @notice The chain IDs associated to the ciphertext handle.
        mapping(uint256 ctHandle => uint256 chainId) _chainIds;
        /// @notice The mapping of already added ciphertexts tied to the given handle.
        mapping(uint256 ctHandle => bool isAdded) _isCiphertextMaterialAdded;
        /// @notice The counter of confirmations received for a ciphertext to be added.
        mapping(bytes32 addCiphertextHash => uint8 counter) _addCiphertextHashCounters;
        /// @notice The mapping of the Coprocessors that have already added the ciphertext.
        mapping(bytes32 addCiphertextHash => mapping(address coprocessorAddress => bool hasSent)) _alreadyAddedCoprocessors;
        /// @notice The mapping of the Coprocessors that have added the ciphertext.
        mapping(uint256 ctHandle => address[] coprocessorAddresses) _coprocessorAddresses;
    }

    /// @dev keccak256(abi.encode(uint256(keccak256("httpz_gateway.storage.CiphertextManager")) - 1)) &
    /// @dev ~bytes32(uint256(0xff))
    bytes32 private constant CIPHERTEXT_MANAGER_STORAGE_LOCATION =
        0x481e71c2610c87311f2235c0e382133ada84bf381e3abe761ad2ab5c432d3c00;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice  Initializes the contract.
     */
    function initialize() public reinitializer(2) {
        __Ownable_init(owner());
    }

    /// @notice See {ICiphertextManager-checkCiphertextMaterial}.
    function checkCiphertextMaterial(uint256 ctHandle) public view virtual {
        CiphertextManagerStorage storage $ = _getCiphertextManagerStorage();
        if (!$._isCiphertextMaterialAdded[ctHandle]) {
            revert CiphertextMaterialNotFound(ctHandle);
        }
    }

    /// @notice See {ICiphertextManager-checkIsOnNetwork}.
    function checkIsOnNetwork(uint256 ctHandle, uint256 chainId) public view {
        CiphertextManagerStorage storage $ = _getCiphertextManagerStorage();
        if ($._chainIds[ctHandle] != chainId) {
            revert CiphertextMaterialNotOnNetwork(ctHandle, chainId);
        }
    }

    /// @notice See {ICiphertextManager-getCiphertextMaterials}.
    function getCiphertextMaterials(
        uint256[] calldata ctHandles
    ) public view returns (CiphertextMaterial[] memory ctMaterials) {
        CiphertextManagerStorage storage $ = _getCiphertextManagerStorage();
        ctMaterials = new CiphertextMaterial[](ctHandles.length);

        for (uint256 i = 0; i < ctHandles.length; i++) {
            checkCiphertextMaterial(ctHandles[i]);

            ctMaterials[i] = CiphertextMaterial(
                ctHandles[i],
                $._keyIds[ctHandles[i]],
                $._ciphertextDigests[ctHandles[i]],
                $._coprocessorAddresses[ctHandles[i]]
            );
        }

        return ctMaterials;
    }

    /// @notice See {ICiphertextManager-getSnsCiphertextMaterials}.
    function getSnsCiphertextMaterials(
        uint256[] calldata ctHandles
    ) public view returns (SnsCiphertextMaterial[] memory snsCtMaterials) {
        CiphertextManagerStorage storage $ = _getCiphertextManagerStorage();
        snsCtMaterials = new SnsCiphertextMaterial[](ctHandles.length);

        for (uint256 i = 0; i < ctHandles.length; i++) {
            checkCiphertextMaterial(ctHandles[i]);

            snsCtMaterials[i] = SnsCiphertextMaterial(
                ctHandles[i],
                $._keyIds[ctHandles[i]],
                $._snsCiphertextDigests[ctHandles[i]],
                $._coprocessorAddresses[ctHandles[i]]
            );
        }

        return snsCtMaterials;
    }

    /// @notice See {ICiphertextManager-addCiphertextMaterial}.
    /// @dev This function calls the HTTPZ contract to check that the sender address is a Coprocessor.
    function addCiphertextMaterial(
        uint256 ctHandle,
        uint256 keyId,
        uint256 chainId,
        bytes32 ciphertextDigest,
        bytes32 snsCiphertextDigest
    ) public {
        CiphertextManagerStorage storage $ = _getCiphertextManagerStorage();

        /// @dev Check if the sender is a Coprocessor
        _HTTPZ.checkIsCoprocessor(msg.sender);

        /// @dev The addCiphertextHash is the hash of all input arguments.
        /// @dev This hash is used to track the addition consensus over the received ciphertext digests.
        bytes32 addCiphertextHash = keccak256(
            abi.encode(ctHandle, keyId, chainId, ciphertextDigest, snsCiphertextDigest)
        );

        /// @dev Check if the Coprocessor has already added the ciphertext.
        if ($._alreadyAddedCoprocessors[addCiphertextHash][msg.sender]) {
            revert CoprocessorAlreadyAdded(msg.sender);
        }

        /// @dev Check if the received key ID is the latest activated.
        // TODO: Revisit the following line accordingly with key lifecycles issue
        // See: https://github.com/zama-ai/gateway-l2/issues/90
        // TODO: Re-enable this check once keys are generated through the Gateway
        // bool isCurrentKeyId = _KEY_MANAGER.isCurrentKeyId(keyId);
        // if (!isCurrentKeyId) {
        //     revert InvalidCurrentKeyId(keyId);
        // }

        $._addCiphertextHashCounters[addCiphertextHash]++;
        $._alreadyAddedCoprocessors[addCiphertextHash][msg.sender] = true;
        $._coprocessorAddresses[ctHandle].push(msg.sender);

        /// @dev Only send the event if consensus has not been reached in a previous call
        /// @dev and the consensus is reached in the current call.
        /// @dev This means a "late" allow will not be reverted, just ignored
        if (
            !$._isCiphertextMaterialAdded[ctHandle] &&
            _isConsensusReached($._addCiphertextHashCounters[addCiphertextHash])
        ) {
            $._ciphertextDigests[ctHandle] = ciphertextDigest;
            $._snsCiphertextDigests[ctHandle] = snsCiphertextDigest;
            $._keyIds[ctHandle] = keyId;
            $._chainIds[ctHandle] = chainId;
            $._isCiphertextMaterialAdded[ctHandle] = true;

            emit AddCiphertextMaterial(
                ctHandle,
                ciphertextDigest,
                snsCiphertextDigest,
                $._coprocessorAddresses[ctHandle]
            );
        }
    }

    /// @notice Returns the versions of the CiphertextManager contract in SemVer format.
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

    /**
     * @dev Returns the CiphertextManager storage location.
     */
    function _getCiphertextManagerStorage() internal pure returns (CiphertextManagerStorage storage $) {
        // solhint-disable-next-line no-inline-assembly
        assembly {
            $.slot := CIPHERTEXT_MANAGER_STORAGE_LOCATION
        }
    }

    /**
     * @dev Should revert when `msg.sender` is not authorized to upgrade the contract.
     */
    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyOwner {}
}
