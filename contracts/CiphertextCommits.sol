// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import { gatewayConfigAddress } from "../addresses/GatewayConfigAddress.sol";
import { kmsManagementAddress } from "../addresses/KmsManagementAddress.sol";
import { Ownable2StepUpgradeable } from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import { UUPSUpgradeable } from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import "./interfaces/ICiphertextCommits.sol";
import "./interfaces/IGatewayConfig.sol";
import "./interfaces/IKmsManagement.sol";
import "./shared/GatewayConfigChecks.sol";
import "./libraries/HandleOps.sol";

/**
 * @title CiphertextCommits smart contract
 * @dev See {ICiphertextCommits}.
 */
contract CiphertextCommits is ICiphertextCommits, Ownable2StepUpgradeable, UUPSUpgradeable, GatewayConfigChecks {
    /// @notice The address of the GatewayConfig contract, used for fetching information about coprocessors.
    IGatewayConfig private constant _GATEWAY_CONFIG = IGatewayConfig(gatewayConfigAddress);

    /// @notice The address of the KmsManagement contract, used for fetching information about the current key.
    IKmsManagement private constant _KMS_MANAGEMENT = IKmsManagement(kmsManagementAddress);

    string private constant CONTRACT_NAME = "CiphertextCommits";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    /// @notice The contract's variable storage struct (@dev see ERC-7201)
    /// @custom:storage-location erc7201:fhevm_gateway.storage.CiphertextCommits
    struct CiphertextCommitsStorage {
        /// @notice The regular ciphertext digests tied to the ciphertext handle.
        mapping(bytes32 ctHandle => bytes32 ctDigest) _ciphertextDigests;
        /// @notice The SNS ciphertext digests tied to the ciphertext handle.
        mapping(bytes32 ctHandle => bytes32 snsCtDigest) _snsCiphertextDigests;
        /// @notice The key IDs used for generating the ciphertext.
        /// @dev It's necessary in case new keys are generated: we need to know what key to use for using a ciphertext.
        mapping(bytes32 ctHandle => uint256 keyId) _keyIds;
        /// @notice The chain IDs associated to the ciphertext handle.
        mapping(bytes32 ctHandle => uint256 chainId) _chainIds;
        /// @notice The mapping of already added ciphertexts tied to the given handle.
        mapping(bytes32 ctHandle => bool isAdded) _isCiphertextMaterialAdded;
        /// @notice The counter of confirmations received for a ciphertext to be added.
        mapping(bytes32 addCiphertextHash => uint8 counter) _addCiphertextHashCounters;
        // prettier-ignore
        /// @notice The mapping of the coprocessor transaction senders that have already added the ciphertext handle.
        mapping(bytes32 ctHandle => mapping(address coprocessorTxSenderAddress => bool hasAdded)) 
            _alreadyAddedCoprocessorTxSenders;
        /// @notice The mapping of the coprocessor transaction senders that have added the ciphertext.
        mapping(bytes32 ctHandle => address[] coprocessorTxSenderAddresses) _coprocessorTxSenderAddresses;
    }

    /// @dev Storage location has been computed using the following command:
    /// @dev keccak256(abi.encode(uint256(keccak256("fhevm_gateway.storage.CiphertextCommits")) - 1)) &
    /// @dev ~bytes32(uint256(0xff))
    bytes32 private constant CIPHERTEXT_COMMITS_STORAGE_LOCATION =
        0xf41c60ea5b83c8f19b663613ffdd3fa441a59933b8a4fdf4da891b38433d1a00;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice  Initializes the contract.
     */
    function initialize() public virtual reinitializer(2) {
        __Ownable_init(owner());
    }

    /// @notice See {ICiphertextCommits-addCiphertextMaterial}.
    /// @dev This function calls the GatewayConfig contract to check that the sender address is a Coprocessor.
    function addCiphertextMaterial(
        bytes32 ctHandle,
        uint256 keyId,
        bytes32 ciphertextDigest,
        bytes32 snsCiphertextDigest
    ) public virtual onlyCoprocessorTxSender {
        CiphertextCommitsStorage storage $ = _getCiphertextCommitsStorage();

        /// @dev Extract the chainId from the ciphertext handle
        uint256 chainId = HandleOps.extractChainId(ctHandle);

        /// @dev Check that the associated network is registered
        _GATEWAY_CONFIG.checkNetworkIsRegistered(chainId);

        /**
         * @dev Check if the coprocessor transaction sender has already added the ciphertext handle.
         * Note that a coprocessor transaction sender cannot add the same ciphertext material on
         * two different networks.
         */
        if ($._alreadyAddedCoprocessorTxSenders[ctHandle][msg.sender]) {
            revert CoprocessorTxSenderAlreadyAdded(msg.sender);
        }

        /// @dev Check if the received key ID is the latest activated.
        // TODO: Revisit the following line accordingly with key lifecycles issue
        // See: https://github.com/zama-ai/httpz-gateway/issues/90
        // TODO: Re-enable this check once keys are generated through the Gateway
        // bool isCurrentKeyId = _KMS_MANAGEMENT.isCurrentKeyId(keyId);
        // if (!isCurrentKeyId) {
        //     revert InvalidCurrentKeyId(keyId);
        // }

        /**
         * @dev The addCiphertextHash is the hash of all received input arguments which means that multiple
         * Coprocessors can only have a consensus on a ciphertext material with the same information.
         * This hash is used to track the addition consensus on the received ciphertext material.
         */
        bytes32 addCiphertextHash = keccak256(
            abi.encode(ctHandle, chainId, keyId, ciphertextDigest, snsCiphertextDigest)
        );
        $._addCiphertextHashCounters[addCiphertextHash]++;

        $._alreadyAddedCoprocessorTxSenders[ctHandle][msg.sender] = true;
        $._coprocessorTxSenderAddresses[ctHandle].push(msg.sender);

        /// @dev Only send the event if consensus has not been reached in a previous call
        /// @dev and the consensus is reached in the current call.
        /// @dev This means a "late" addition will not be reverted, just ignored
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
                $._coprocessorTxSenderAddresses[ctHandle]
            );
        }
    }

    /// @notice See {ICiphertextCommits-checkCiphertextMaterial}.
    function checkCiphertextMaterial(bytes32 ctHandle) public view virtual {
        CiphertextCommitsStorage storage $ = _getCiphertextCommitsStorage();
        if (!$._isCiphertextMaterialAdded[ctHandle]) {
            revert CiphertextMaterialNotFound(ctHandle);
        }
    }

    /// @notice See {ICiphertextCommits-getCiphertextMaterials}.
    function getCiphertextMaterials(
        bytes32[] calldata ctHandles
    ) public view virtual returns (CiphertextMaterial[] memory ctMaterials) {
        CiphertextCommitsStorage storage $ = _getCiphertextCommitsStorage();
        ctMaterials = new CiphertextMaterial[](ctHandles.length);

        for (uint256 i = 0; i < ctHandles.length; i++) {
            checkCiphertextMaterial(ctHandles[i]);

            ctMaterials[i] = CiphertextMaterial(
                ctHandles[i],
                $._keyIds[ctHandles[i]],
                $._ciphertextDigests[ctHandles[i]],
                $._coprocessorTxSenderAddresses[ctHandles[i]]
            );
        }

        return ctMaterials;
    }

    /// @notice See {ICiphertextCommits-getSnsCiphertextMaterials}.
    function getSnsCiphertextMaterials(
        bytes32[] calldata ctHandles
    ) public view virtual returns (SnsCiphertextMaterial[] memory snsCtMaterials) {
        CiphertextCommitsStorage storage $ = _getCiphertextCommitsStorage();
        snsCtMaterials = new SnsCiphertextMaterial[](ctHandles.length);

        for (uint256 i = 0; i < ctHandles.length; i++) {
            checkCiphertextMaterial(ctHandles[i]);

            snsCtMaterials[i] = SnsCiphertextMaterial(
                ctHandles[i],
                $._keyIds[ctHandles[i]],
                $._snsCiphertextDigests[ctHandles[i]],
                $._coprocessorTxSenderAddresses[ctHandles[i]]
            );
        }

        return snsCtMaterials;
    }

    /// @notice Returns the versions of the CiphertextCommits contract in SemVer format.
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
    function _isConsensusReached(uint256 coprocessorCounter) internal view virtual returns (bool) {
        uint256 consensusThreshold = _GATEWAY_CONFIG.getCoprocessorMajorityThreshold();
        return coprocessorCounter >= consensusThreshold;
    }

    /**
     * @dev Returns the CiphertextCommits storage location.
     */
    function _getCiphertextCommitsStorage() internal pure returns (CiphertextCommitsStorage storage $) {
        // solhint-disable-next-line no-inline-assembly
        assembly {
            $.slot := CIPHERTEXT_COMMITS_STORAGE_LOCATION
        }
    }

    /**
     * @dev Should revert when `msg.sender` is not authorized to upgrade the contract.
     */
    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyOwner {}
}
