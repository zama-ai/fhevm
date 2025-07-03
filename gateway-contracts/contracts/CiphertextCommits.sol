// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import { gatewayConfigAddress } from "../addresses/GatewayConfigAddress.sol";
import { kmsManagementAddress } from "../addresses/KmsManagementAddress.sol";
import { Ownable2StepUpgradeable } from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import "./interfaces/ICiphertextCommits.sol";
import "./interfaces/IGatewayConfig.sol";
import "./interfaces/IKmsManagement.sol";
import "./shared/UUPSUpgradeableEmptyProxy.sol";
import "./shared/GatewayConfigChecks.sol";
import "./shared/Pausable.sol";
import "./libraries/HandleOps.sol";

/**
 * @title CiphertextCommits smart contract
 * @dev See {ICiphertextCommits}.
 */
contract CiphertextCommits is
    ICiphertextCommits,
    Ownable2StepUpgradeable,
    UUPSUpgradeableEmptyProxy,
    GatewayConfigChecks,
    Pausable
{
    /// @notice The address of the GatewayConfig contract, used for fetching information about coprocessors.
    IGatewayConfig private constant GATEWAY_CONFIG = IGatewayConfig(gatewayConfigAddress);

    /// @notice The address of the KmsManagement contract, used for fetching information about the current key.
    IKmsManagement private constant KMS_MANAGEMENT = IKmsManagement(kmsManagementAddress);

    /// @dev The following constants are used for versioning the contract. They are made private
    /// @dev in order to force derived contracts to consider a different version. Note that
    /// @dev they can still define their own private constants with the same name.
    string private constant CONTRACT_NAME = "CiphertextCommits";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    /// Constant used for making sure the version number using in the `reinitializer` modifier is
    /// identical between `initializeFromEmptyProxy` and the reinitializeVX` method
    uint64 private constant REINITIALIZER_VERSION = 2;

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
        mapping(bytes32 ctHandle => bytes32 addCiphertextHash) _ctHandleConsensusHash;
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
     * @dev This function needs to be public in order to be called by the UUPS proxy.
     */
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy() public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        __Ownable_init(owner());
        __Pausable_init();
    }

    /// @notice See {ICiphertextCommits-addCiphertextMaterial}.
    /// @dev This function calls the GatewayConfig contract to check that the sender address is a Coprocessor.
    function addCiphertextMaterial(
        bytes32 ctHandle,
        uint256 keyId,
        bytes32 ciphertextDigest,
        bytes32 snsCiphertextDigest
    ) external virtual onlyCoprocessorTxSender whenNotPaused {
        // Extract the chainId from the ciphertext handle
        uint256 chainId = HandleOps.extractChainId(ctHandle);

        // Check that the associated host chain is registered
        GATEWAY_CONFIG.checkHostChainIsRegistered(chainId);

        CiphertextCommitsStorage storage $ = _getCiphertextCommitsStorage();

        // Check if the coprocessor transaction sender has already added the ciphertext handle.
        if ($._alreadyAddedCoprocessorTxSenders[ctHandle][msg.sender]) {
            revert CoprocessorAlreadyAdded(ctHandle, msg.sender);
        }

        // Check if the received key ID is the latest activated.
        // TODO: Revisit the following line accordingly with key life-cycles issue
        // See: https://github.com/zama-ai/fhevm-gateway/issues/90
        // TODO: Re-enable this check once keys are generated through the Gateway
        // KMS_MANAGEMENT.checkCurrentKeyId(keyId);

        // The addCiphertextHash is the hash of all received input arguments which means that multiple
        // Coprocessors can only have a consensus on a ciphertext material with the same information.
        // This hash is used to differentiate different calls to the function, in particular when
        // tracking the consensus on the received ciphertext material.
        // Note that chainId is not included in the hash because it is already contained in the ctHandle.
        bytes32 addCiphertextHash = keccak256(abi.encode(ctHandle, keyId, ciphertextDigest, snsCiphertextDigest));
        $._addCiphertextHashCounters[addCiphertextHash]++;

        // It is ok to only the handle can be considered here as a handle should only be added once
        // in the contract anyway
        $._alreadyAddedCoprocessorTxSenders[ctHandle][msg.sender] = true;

        // It's important to consider the hash and not the handle to make sure only the transaction
        // senders associated to the same inputs are considered in tne event emitted once consensus
        // is reached. Else, malicious copro that provide fake digests along valid handles could see
        // their transaction sender being added to this event
        // In particular, this means that a "late" (see right below) valid coprocessor's transaction
        // sender address will still be added in the list, which than can be retrieved using the
        // `getCiphertextMaterials` and `getSnsCiphertextMaterials` methods
        $._coprocessorTxSenderAddresses[addCiphertextHash].push(msg.sender);

        // Only send the event if consensus has not been reached in a previous call and the consensus
        // is reached in the current call.
        // This means a "late" valid addition will not be reverted, just ignored
        if (
            !$._isCiphertextMaterialAdded[ctHandle] &&
            _isConsensusReached($._addCiphertextHashCounters[addCiphertextHash])
        ) {
            $._ciphertextDigests[ctHandle] = ciphertextDigest;
            $._snsCiphertextDigests[ctHandle] = snsCiphertextDigest;
            $._keyIds[ctHandle] = keyId;
            $._chainIds[ctHandle] = chainId;

            // A ciphertext handle should only be added once, ever
            $._isCiphertextMaterialAdded[ctHandle] = true;

            // As explained above, a "late" valid coprocessor could still see its transaction sender
            // address be added to the list after consensus. This var is here to make the
            // `getCiphertextMaterials` and `getSneCiphertextMaterials` methods retrieve this list
            // mor easily
            $._ctHandleConsensusHash[ctHandle] = addCiphertextHash;

            emit AddCiphertextMaterial(
                ctHandle,
                ciphertextDigest,
                snsCiphertextDigest,
                $._coprocessorTxSenderAddresses[addCiphertextHash]
            );
        }
    }

    /// @notice See {ICiphertextCommits-getCiphertextMaterials}.
    function getCiphertextMaterials(
        bytes32[] calldata ctHandles
    ) external view virtual returns (CiphertextMaterial[] memory ctMaterials) {
        CiphertextCommitsStorage storage $ = _getCiphertextCommitsStorage();
        ctMaterials = new CiphertextMaterial[](ctHandles.length);

        for (uint256 i = 0; i < ctHandles.length; i++) {
            checkCiphertextMaterial(ctHandles[i]);

            // Get the unique hash associated to the handle in order to retrieve the list of transaction
            // sender address that participated in the consensus
            bytes32 addCiphertextHash = $._ctHandleConsensusHash[ctHandles[i]];

            ctMaterials[i] = CiphertextMaterial(
                ctHandles[i],
                $._keyIds[ctHandles[i]],
                $._ciphertextDigests[ctHandles[i]],
                $._coprocessorTxSenderAddresses[addCiphertextHash]
            );
        }

        return ctMaterials;
    }

    /// @notice See {ICiphertextCommits-getSnsCiphertextMaterials}.
    function getSnsCiphertextMaterials(
        bytes32[] calldata ctHandles
    ) external view virtual returns (SnsCiphertextMaterial[] memory snsCtMaterials) {
        CiphertextCommitsStorage storage $ = _getCiphertextCommitsStorage();
        snsCtMaterials = new SnsCiphertextMaterial[](ctHandles.length);

        for (uint256 i = 0; i < ctHandles.length; i++) {
            checkCiphertextMaterial(ctHandles[i]);

            // Get the unique hash associated to the handle in order to retrieve the list of transaction
            // sender address that participated in the consensus
            bytes32 addCiphertextHash = $._ctHandleConsensusHash[ctHandles[i]];

            snsCtMaterials[i] = SnsCiphertextMaterial(
                ctHandles[i],
                $._keyIds[ctHandles[i]],
                $._snsCiphertextDigests[ctHandles[i]],
                $._coprocessorTxSenderAddresses[addCiphertextHash]
            );
        }

        return snsCtMaterials;
    }

    /// @notice See {ICiphertextCommits-getVersion}.
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

    /// @notice See {ICiphertextCommits-checkCiphertextMaterial}.
    function checkCiphertextMaterial(bytes32 ctHandle) public view virtual {
        CiphertextCommitsStorage storage $ = _getCiphertextCommitsStorage();
        if (!$._isCiphertextMaterialAdded[ctHandle]) {
            revert CiphertextMaterialNotFound(ctHandle);
        }
    }

    /**
     * @dev Should revert when `msg.sender` is not authorized to upgrade the contract.
     */
    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyOwner {}

    /// @notice Checks if the consensus is reached among the Coprocessors.
    /// @param coprocessorCounter The number of coprocessors that agreed
    /// @return Whether the consensus is reached
    function _isConsensusReached(uint256 coprocessorCounter) internal view virtual returns (bool) {
        uint256 consensusThreshold = GATEWAY_CONFIG.getCoprocessorMajorityThreshold();
        return coprocessorCounter >= consensusThreshold;
    }

    /**
     * @dev Returns the CiphertextCommits storage location.
     * Note that this function is internal but not virtual: derived contracts should be able to
     * access it, but if the underlying storage struct version changes, we force them to define a new
     * getter function and use that one instead in order to avoid overriding the storage location.
     */
    function _getCiphertextCommitsStorage() internal pure returns (CiphertextCommitsStorage storage $) {
        // solhint-disable-next-line no-inline-assembly
        assembly {
            $.slot := CIPHERTEXT_COMMITS_STORAGE_LOCATION
        }
    }
}
