// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "./interfaces/ICoprocessorContexts.sol";
import { Ownable2StepUpgradeable } from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import { UUPSUpgradeable } from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import { ECDSA } from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import "./shared/Pausable.sol";
import { ContextLifecycle } from "./libraries/ContextLifecycle.sol";
import { CoprocessorContext, CoprocessorContextBlockPeriods } from "./shared/Structs.sol";
import { ContextStatus } from "./shared/Enums.sol";

/**
 * @title CoprocessorContexts contract
 * @dev See {ICoprocessorContexts}.
 * @dev Add/remove methods will be added in the future for coprocessors, coprocessors and host chains.
 * @dev See https://github.com/zama-ai/fhevm-gateway/issues/98 for more details.
 */
contract CoprocessorContexts is ICoprocessorContexts, Ownable2StepUpgradeable, UUPSUpgradeable, Pausable {
    /// @dev The following constants are used for versioning the contract. They are made private
    /// @dev in order to force derived contracts to consider a different version. Note that
    /// @dev they can still define their own private constants with the same name.
    string private constant CONTRACT_NAME = "CoprocessorContexts";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    /// @notice The contract's variable storage struct (@dev see ERC-7201)
    /// @custom:storage-location erc7201:fhevm_gateway.storage.CoprocessorContexts
    struct CoprocessorContextsStorage {
        /// @notice The coprocessor context lifecycle
        ContextLifecycle.ContextLifecycleStorage coprocessorContextLifecycle;
        uint256 coprocessorContextGenerationBlockPeriod;
        mapping(uint256 contextId => uint256 coprocessorContextPreActivationBlockPeriod) coprocessorContextPreActivationBlockPeriod;
        uint256 coprocessorContextSuspensionBlockPeriod;
        /// @notice The coprocessor contexts
        mapping(uint256 contextId => CoprocessorContext coprocessorContext) coprocessorContexts;
        /// @notice The number of coprocessor contexts
        uint256 coprocessorContextCount;
        /// @notice Wether a coprocessor is done with key resharing
        mapping(uint256 contextId => mapping(address coprocessorSignerAddress => bool doneKeyResharing)) coprocessorDoneKeyResharing;
        mapping(uint256 contextId => uint256 activationBlockNumber) coprocessorContextActivationBlockNumber;
        /// @notice Verified signatures for key resharing responses
        mapping(uint256 contextId => bytes[] verifiedSignatures) verifiedKeyResharingSignatures;
        /// @notice The coprocessors' metadata
        mapping(uint256 contextId => mapping(address coprocessorTxSenderAddress => Coprocessor coprocessor)) coprocessors;
        /// @notice The coprocessors' transaction sender addresses
        mapping(uint256 contextId => mapping(address coprocessorTxSenderAddress => bool isCoprocessorTxSender)) isCoprocessorTxSender;
        /// @notice The coprocessors' signer addresses
        mapping(uint256 contextId => mapping(address coprocessorSignerAddress => bool isCoprocessorSigner)) isCoprocessorSigner;
        /// @notice The coprocessors' transaction sender address list
        mapping(uint256 contextId => address[] coprocessorTxSenderAddresses) coprocessorTxSenderAddresses;
        /// @notice The coprocessors' signer address list
        mapping(uint256 contextId => address[] coprocessorSignerAddresses) coprocessorSignerAddresses;
        mapping(uint256 contextId => uint256 generationBlockNumber) coprocessorContextGenerationBlockNumber;
        mapping(uint256 contextId => uint256 preActivationBlockNumber) coprocessorContextPreActivationBlockNumber;
        mapping(uint256 contextId => uint256 suspensionBlockNumber) coprocessorContextSuspensionBlockNumber;
        /// @notice The public decryption threshold per coprocessor context
        mapping(uint256 contextId => uint256 threshold) publicDecryptionThreshold;
        /// @notice The user decryption threshold per coprocessor context
        mapping(uint256 contextId => uint256 threshold) userDecryptionThreshold;
    }

    /// @dev Storage location has been computed using the following command:
    /// @dev keccak256(abi.encode(uint256(keccak256("fhevm_gateway.storage.CoprocessorContexts")) - 1))
    /// @dev & ~bytes32(uint256(0xff))
    bytes32 private constant COPROCESSOR_CONTEXTS_STORAGE_LOCATION =
        0x7d8159810a7ebf944e8fa93cc4fbd1cade6c71f8b0b86b37187ac7991777b100;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    modifier ensureContextInitialized(uint256 contextId) {
        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();
        if ($.coprocessorContexts[contextId].contextId == 0) {
            revert CoprocessorContextNotInitialized(contextId);
        }
        _;
    }

    /// @notice Initializes the contract
    /// @dev This function needs to be public in order to be called by the UUPS proxy.
    /// @param initialContextBlockPeriods The block periods of the coprocessor context
    /// @param initialFeatureSet The feature set of the coprocessor context
    /// @param initialCoprocessors The coprocessors of the coprocessor context
    function initialize(
        CoprocessorContextBlockPeriods calldata initialContextBlockPeriods,
        string calldata initialFeatureSet,
        Coprocessor[] calldata initialCoprocessors
    ) public virtual reinitializer(2) {
        __Ownable_init(owner());
        __Pausable_init();

        // The first coprocessor context is the initial coprocessor context and thus does not have a previous context
        CoprocessorContext memory newCoprocessorContext = _addCoprocessorContext(
            0,
            initialFeatureSet,
            initialCoprocessors
        );

        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();

        // It is exceptionally allowed to set the active context directly at initialization. In other
        // cases, the context must be pre-activated first.
        ContextLifecycle.setActive($.coprocessorContextLifecycle, newCoprocessorContext.contextId);

        _setBlockPeriods(initialContextBlockPeriods, newCoprocessorContext.contextId);

        emit InitCoprocessorContext(initialFeatureSet, initialContextBlockPeriods, initialCoprocessors);
    }

    /// @dev See {ICoprocessorContexts-checkIsCoprocessorTxSenderFromContext}.
    function checkIsCoprocessorTxSenderFromContext(
        uint256 contextId,
        address txSenderAddress
    ) public view virtual ensureContextInitialized(contextId) {
        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();
        if (!$.isCoprocessorTxSender[contextId][txSenderAddress]) {
            revert NotCoprocessorTxSenderFromContext(contextId, txSenderAddress);
        }
    }

    /**
     * @dev See {ICoprocessorContexts-getSuspendedCoprocessorContextId}.
     */
    function getSuspendedCoprocessorContextId() public view virtual returns (uint256) {
        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();
        return $.coprocessorContextLifecycle.suspendedContextId;
    }

    /**
     * @dev See {ICoprocessorContexts-getActiveCoprocessorContextId}.
     * There should always be an active coprocessor context defined in the gateway, as we do not allow
     * to manually set active coprocessor contexts to `Compromised` or `Destroyed` states
     */
    function getActiveCoprocessorContextId() public view virtual returns (uint256) {
        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();
        return $.coprocessorContextLifecycle.activeContextId;
    }

    /**
     * @dev See {ICoprocessorContexts-getActiveCoprocessorContext}.
     * There should always be an active coprocessor context defined in the gateway, as we do not allow
     * to manually set active coprocessor contexts to `Compromised` or `Destroyed` states
     */
    function getActiveCoprocessorContext() public view virtual returns (CoprocessorContext memory) {
        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();
        uint256 activeContextId = getActiveCoprocessorContextId();
        return $.coprocessorContexts[activeContextId];
    }

    /// @dev See {ICoprocessorContexts-getCoprocessorFromContext}.
    function getCoprocessorFromContext(
        uint256 contextId,
        address coprocessorTxSenderAddress
    ) public view virtual returns (Coprocessor memory) {
        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();
        Coprocessor memory coprocessor = $.coprocessors[contextId][coprocessorTxSenderAddress];

        if (coprocessor.txSenderAddress == address(0)) {
            revert NotCoprocessorFromContext(contextId, coprocessorTxSenderAddress);
        }

        return coprocessor;
    }

    /// @dev See {ICoprocessorContexts-updateCoprocessorContextSuspensionBlockPeriod}.
    function updateCoprocessorContextSuspensionBlockPeriod(
        uint256 newCoprocessorContextSuspensionBlockPeriod
    ) external virtual onlyOwner whenNotPaused {
        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();
        $.coprocessorContextSuspensionBlockPeriod = newCoprocessorContextSuspensionBlockPeriod;
        emit UpdateCoprocessorContextSuspensionBlockPeriod(newCoprocessorContextSuspensionBlockPeriod);
    }

    /// @dev See {ICoprocessorContexts-addCoprocessorContext}.
    function addCoprocessorContext(
        uint256 preActivationBlockPeriod,
        string memory featureSet,
        Coprocessor[] calldata coprocessors
    ) external virtual onlyOwner {
        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();

        // Do not allow adding a new coprocessor context if there is a suspended coprocessor context ongoing
        uint256 suspendedContextId = getSuspendedCoprocessorContextId();
        if (suspendedContextId != 0) {
            revert SuspendedCoprocessorContextOngoing(suspendedContextId);
        }

        CoprocessorContext memory activeCoprocessorContext = getActiveCoprocessorContext();

        CoprocessorContext memory newCoprocessorContext = _addCoprocessorContext(
            activeCoprocessorContext.contextId,
            featureSet,
            coprocessors
        );

        // Set the coprocessor context to the generating state
        ContextLifecycle.setGenerating($.coprocessorContextLifecycle, newCoprocessorContext.contextId);

        // Directly pre-activate the coprocessor context: there is currently no generating phase for
        // coprocessor contexts
        ContextLifecycle.setPreActivation($.coprocessorContextLifecycle, newCoprocessorContext.contextId);

        uint256 preActivationBlockNumber = block.number + preActivationBlockPeriod;
        $.coprocessorContextPreActivationBlockNumber[newCoprocessorContext.contextId] = preActivationBlockNumber;

        emit PreActivateCoprocessorContext(newCoprocessorContext, preActivationBlockNumber);
    }

    function refreshCoprocessorContextStatuses() external virtual whenNotPaused {
        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();

        uint256 preActivationContextId = $.coprocessorContextLifecycle.preActivationContextId;

        if (preActivationContextId != 0) {
            if (block.number > $.coprocessorContextPreActivationBlockNumber[preActivationContextId]) {
                uint256 activeContextId = getActiveCoprocessorContextId();
                ContextLifecycle.setSuspended($.coprocessorContextLifecycle, activeContextId);
                emit SuspendCoprocessorContext(activeContextId);

                ContextLifecycle.setActive($.coprocessorContextLifecycle, preActivationContextId);
                emit ActivateCoprocessorContext(preActivationContextId);
            }
        }

        uint256 suspendedContextId = getSuspendedCoprocessorContextId();

        if (suspendedContextId != 0) {
            if (block.number > $.coprocessorContextSuspensionBlockNumber[suspendedContextId]) {
                ContextLifecycle.setDeactivated($.coprocessorContextLifecycle, suspendedContextId);
                emit DeactivateCoprocessorContext(suspendedContextId);
            }
        }
    }

    function compromiseCoprocessorContext(
        uint256 contextId
    ) external virtual onlyOwner ensureContextInitialized(contextId) {
        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();

        // Do not allow compromising an active coprocessor context in order to ensure that the gateway can
        // always provide an active coprocessor context
        // If too many parties are compromised for this coprocessor context, then the relevant functions
        // should be paused instead
        if (ContextLifecycle.isActive($.coprocessorContextLifecycle, contextId)) {
            revert CompromiseActiveCoprocessorContextNotAllowed(contextId);
        }

        ContextLifecycle.setCompromised($.coprocessorContextLifecycle, contextId);
        emit CompromiseCoprocessorContext(contextId);
    }

    function destroyCoprocessorContext(
        uint256 contextId
    ) external virtual onlyOwner ensureContextInitialized(contextId) {
        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();

        // Do not allow destroying an active coprocessor context in order to ensure that the gateway can
        // always provide an active coprocessor context
        if (ContextLifecycle.isActive($.coprocessorContextLifecycle, contextId)) {
            revert DestroyActiveCoprocessorContextNotAllowed(contextId);
        }

        ContextLifecycle.setDestroyed($.coprocessorContextLifecycle, contextId);
        emit DestroyCoprocessorContext(contextId);
    }

    function moveSuspendedCoprocessorContextToActive() external virtual onlyOwner {
        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();
        uint256 suspendedContextId = getSuspendedCoprocessorContextId();

        if (suspendedContextId == 0) {
            revert NoSuspendedCoprocessorContext();
        }

        uint256 activeContextId = getActiveCoprocessorContextId();
        ContextLifecycle.setDeactivated($.coprocessorContextLifecycle, activeContextId);
        emit DeactivateCoprocessorContext(activeContextId);

        ContextLifecycle.setActive($.coprocessorContextLifecycle, suspendedContextId);
        emit ActivateCoprocessorContext(suspendedContextId);
    }

    /// @dev See {ICoprocessorContexts-checkIsCoprocessorTxSender}.
    function checkIsCoprocessorTxSender(address txSenderAddress) external view virtual {
        uint256 activeContextId = getActiveCoprocessorContextId();
        checkIsCoprocessorTxSenderFromContext(activeContextId, txSenderAddress);
    }

    /// @dev See {ICoprocessorContexts-checkIsCoprocessorSignerFromContext}.
    function checkIsCoprocessorSignerFromContext(
        uint256 contextId,
        address signerAddress
    ) external view virtual ensureContextInitialized(contextId) {
        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();
        if (!$.isCoprocessorSigner[contextId][signerAddress]) {
            revert NotCoprocessorSignerFromContext(contextId, signerAddress);
        }
    }

    /// @dev See {ICoprocessorContexts-getCoprocessorContextSuspensionBlockPeriod}.
    function getCoprocessorContextSuspensionBlockPeriod() external view virtual returns (uint256) {
        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();
        return $.coprocessorContextSuspensionBlockPeriod;
    }

    /// @dev See {IGatewayConfig-getCoprocessorMajorityThresholdFromContext}.
    function getCoprocessorMajorityThresholdFromContext(
        uint256 contextId
    ) external view virtual ensureContextInitialized(contextId) returns (uint256) {
        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();
        return $.coprocessorTxSenderAddresses[contextId].length / 2 + 1;
    }

    /// @dev See {ICoprocessorContexts-getCoprocessor}.
    function getCoprocessor(address coprocessorTxSenderAddress) external view virtual returns (Coprocessor memory) {
        uint256 activeContextId = getActiveCoprocessorContextId();
        return getCoprocessorFromContext(activeContextId, coprocessorTxSenderAddress);
    }

    /// @dev See {ICoprocessorContexts-getCoprocessors}.
    function getCoprocessors() external view virtual returns (Coprocessor[] memory) {
        return getActiveCoprocessorContext().coprocessors;
    }

    /// @dev See {ICoprocessorContexts-getCoprocessorTxSenders}.
    function getCoprocessorTxSenders() external view virtual returns (address[] memory) {
        uint256 activeContextId = getActiveCoprocessorContextId();
        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();
        return $.coprocessorTxSenderAddresses[activeContextId];
    }

    /// @dev See {ICoprocessorContexts-getCoprocessorSigners}.
    function getCoprocessorSigners() external view virtual returns (address[] memory) {
        uint256 activeContextId = getActiveCoprocessorContextId();
        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();
        return $.coprocessorSignerAddresses[activeContextId];
    }

    function getCoprocessorContextStatus(uint256 contextId) external view virtual returns (ContextStatus) {
        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();
        return ContextLifecycle.getContextStatus($.coprocessorContextLifecycle, contextId);
    }

    /// @dev See {ICoprocessorContexts-getVersion}.
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

    function _setBlockPeriods(
        CoprocessorContextBlockPeriods calldata coprocessorBlockPeriods,
        uint256 contextId
    ) internal virtual {
        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();
        $.coprocessorContextPreActivationBlockPeriod[contextId] = coprocessorBlockPeriods.preActivationBlockPeriod;
        $.coprocessorContextSuspensionBlockPeriod = coprocessorBlockPeriods.suspensionBlockPeriod;
    }

    function _addCoprocessorContext(
        uint256 previousContextId,
        string memory featureSet,
        Coprocessor[] calldata coprocessors
    ) internal virtual returns (CoprocessorContext memory) {
        if (coprocessors.length == 0) {
            revert EmptyCoprocessors();
        }

        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();

        // A coprocessor context ID is never null
        $.coprocessorContextCount++;
        uint256 contextId = $.coprocessorContextCount;

        // Solidity doesn't support directly copying complex data structures like Coprocessors (array
        // of structs), so we need to instead create the struct field by field
        $.coprocessorContexts[contextId].contextId = contextId;
        $.coprocessorContexts[contextId].previousContextId = previousContextId;
        $.coprocessorContexts[contextId].featureSet = featureSet;

        // Then, we need copy each coprocessor struct one by one
        for (uint256 i = 0; i < coprocessors.length; i++) {
            $.coprocessorContexts[contextId].coprocessors.push(coprocessors[i]);
        }

        // Register several mappings for faster lookups
        for (uint256 i = 0; i < coprocessors.length; i++) {
            $.coprocessors[contextId][coprocessors[i].txSenderAddress] = coprocessors[i];
            $.isCoprocessorTxSender[contextId][coprocessors[i].txSenderAddress] = true;
            $.coprocessorTxSenderAddresses[contextId].push(coprocessors[i].txSenderAddress);
            $.isCoprocessorSigner[contextId][coprocessors[i].signerAddress] = true;
            $.coprocessorSignerAddresses[contextId].push(coprocessors[i].signerAddress);
        }

        return $.coprocessorContexts[contextId];
    }

    /**
     * @dev Should revert when `msg.sender` is not authorized to upgrade the contract.
     */
    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyOwner {}

    /**
     * @dev Returns the CoprocessorContexts storage location.
     * Note that this function is internal but not virtual: derived contracts should be able to
     * access it, but if the underlying storage struct version changes, we force them to define a new
     * getter function and use that one instead in order to avoid overriding the storage location.
     */
    function _getCoprocessorContextsStorage() internal pure returns (CoprocessorContextsStorage storage $) {
        // solhint-disable-next-line no-inline-assembly
        assembly {
            $.slot := COPROCESSOR_CONTEXTS_STORAGE_LOCATION
        }
    }
}
