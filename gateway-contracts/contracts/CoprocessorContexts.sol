// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "./interfaces/ICoprocessorContexts.sol";
import { Ownable2StepUpgradeable } from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import { ECDSA } from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import "./shared/Pausable.sol";
import { ContextLifecycle } from "./libraries/ContextLifecycle.sol";
import { CoprocessorContext, CoprocessorContextBlockPeriods } from "./shared/Structs.sol";
import { ContextStatus } from "./shared/Enums.sol";
import { UUPSUpgradeableEmptyProxy } from "./shared/UUPSUpgradeableEmptyProxy.sol";

/**
 * @title CoprocessorContexts contract
 * @dev See {ICoprocessorContexts}.
 */
contract CoprocessorContexts is ICoprocessorContexts, Ownable2StepUpgradeable, UUPSUpgradeableEmptyProxy, Pausable {
    // The following constants are used for versioning the contract. They are made private
    // in order to force derived contracts to consider a different version. Note that
    // they can still define their own private constants with the same name.
    string private constant CONTRACT_NAME = "CoprocessorContexts";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    /**
     * @notice The contract's variable storage struct (@dev see ERC-7201)
     */
    /// @custom:storage-location erc7201:fhevm_gateway.storage.CoprocessorContexts
    struct CoprocessorContextsStorage {
        /// @notice The coprocessor context lifecycle library storage
        ContextLifecycle.ContextLifecycleStorage coprocessorContextLifecycle;
        /// @notice The coprocessor contexts
        mapping(uint256 contextId => CoprocessorContext coprocessorContext) coprocessorContexts;
        /// @notice The number of coprocessor contexts
        uint256 coprocessorContextCount;
        /// @notice The coprocessors' metadata per context
        mapping(uint256 contextId => mapping(address coprocessorTxSenderAddress => Coprocessor coprocessor)) coprocessors;
        /// @notice Whether a coprocessor is a transaction sender in a context
        mapping(uint256 contextId => mapping(address coprocessorTxSenderAddress => bool isCoprocessorTxSender)) isCoprocessorTxSender;
        /// @notice The coprocessors' transaction sender address list per context
        mapping(uint256 contextId => address[] coprocessorTxSenderAddresses) coprocessorTxSenderAddresses;
        /// @notice Whether a coprocessor is a signer in a context
        mapping(uint256 contextId => mapping(address coprocessorSignerAddress => bool isCoprocessorSigner)) isCoprocessorSigner;
        /// @notice The coprocessors' signer address list per context
        mapping(uint256 contextId => address[] coprocessorSignerAddresses) coprocessorSignerAddresses;
        /// @notice The block number at which the coprocessor context is activated
        mapping(uint256 contextId => uint256 activationBlockNumber) coprocessorContextActivationBlockNumber;
        /// @notice The block number at which the coprocessor context is deactivated
        mapping(uint256 contextId => uint256 deactivatedBlockNumber) coprocessorContextDeactivatedBlockNumber;
        /// @notice The suspended block period for the coprocessor context
        mapping(uint256 contextId => uint256 suspendedBlockPeriod) coprocessorContextSuspendedBlockPeriod;
    }

    // Storage location has been computed using the following command:
    // keccak256(abi.encode(uint256(keccak256("fhevm_gateway.storage.CoprocessorContexts")) - 1))
    // & ~bytes32(uint256(0xff))
    bytes32 private constant COPROCESSOR_CONTEXTS_STORAGE_LOCATION =
        0x1da8a9a065a2f0a895c457065eddd3cf4a4d0d5340aaa0ca54d3cd5b4a6aaf00;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @dev Modifier to ensure that a coprocessor context is initialized.
     * @param contextId The coprocessor context ID.
     */
    modifier ensureContextInitialized(uint256 contextId) {
        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();
        if ($.coprocessorContexts[contextId].contextId == 0) {
            revert CoprocessorContextNotInitialized(contextId);
        }
        _;
    }

    /**
     * @notice Initializes the contract
     * @dev This function needs to be public in order to be called by the UUPS proxy.
     * @param initialFeatureSet The feature set of the initial coprocessor context
     * @param initialCoprocessors The coprocessors of the initial coprocessor context
     */
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy(
        uint256 initialFeatureSet,
        Coprocessor[] calldata initialCoprocessors
    ) public virtual onlyFromEmptyProxy reinitializer(2) {
        __Ownable_init(owner());
        __Pausable_init();

        // The first coprocessor context is the initial coprocessor context and thus does not have a
        // previous context (indicated by a null context ID)
        CoprocessorContext memory newCoprocessorContext = _addCoprocessorContext(
            0,
            initialFeatureSet,
            initialCoprocessors
        );

        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();

        // It is exceptionally allowed to set the active context directly at initialization. In other
        // cases, the context must go through the pre-activation state first.
        ContextLifecycle.setActive($.coprocessorContextLifecycle, newCoprocessorContext.contextId);

        emit InitializeCoprocessorContexts(initialFeatureSet, initialCoprocessors);
    }

    /**
     * @dev See {ICoprocessorContexts-getPreActivationCoprocessorContextId}.
     */
    function getPreActivationCoprocessorContextId() public view virtual returns (uint256) {
        uint256 preActivationContextId = _getPreActivationCoprocessorContextId();

        // A null context ID indicates that there is no pre-activation coprocessor context
        if (preActivationContextId == 0) {
            revert NoPreActivationCoprocessorContext();
        }
        return preActivationContextId;
    }

    /**
     * @dev See {ICoprocessorContexts-getSuspendedCoprocessorContextId}.
     */
    function getSuspendedCoprocessorContextId() public view virtual returns (uint256) {
        uint256 suspendedContextId = _getSuspendedCoprocessorContextId();

        // A null context ID indicates that there is no suspended coprocessor context
        if (suspendedContextId == 0) {
            revert NoSuspendedCoprocessorContext();
        }
        return suspendedContextId;
    }

    /**
     * @dev See {ICoprocessorContexts-getActiveCoprocessorContextId}.
     */
    function getActiveCoprocessorContextId() public view virtual returns (uint256) {
        uint256 activeContextId = _getActiveCoprocessorContextId();

        // A null context ID indicates that there is no active coprocessor context
        if (activeContextId == 0) {
            revert NoActiveCoprocessorContext();
        }
        return activeContextId;
    }

    /**
     * @dev See {ICoprocessorContexts-getActiveCoprocessorContext}.
     */
    function getActiveCoprocessorContext() public view virtual returns (CoprocessorContext memory) {
        uint256 activeContextId = getActiveCoprocessorContextId();

        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();
        return $.coprocessorContexts[activeContextId];
    }

    /**
     * @dev See {ICoprocessorContexts-getCoprocessorFromContext}.
     */
    function getCoprocessorFromContext(
        uint256 contextId,
        address coprocessorTxSenderAddress
    ) public view virtual ensureContextInitialized(contextId) returns (Coprocessor memory) {
        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();
        Coprocessor memory coprocessor = $.coprocessors[contextId][coprocessorTxSenderAddress];

        // A null address for the transaction sender address indicates that the coprocessor is not part
        // of the coprocessor context
        if (coprocessor.txSenderAddress == address(0)) {
            revert NotCoprocessorFromContext(contextId, coprocessorTxSenderAddress);
        }

        return coprocessor;
    }

    /**
     * @dev See {ICoprocessorContexts-addCoprocessorContext}.
     */
    function addCoprocessorContext(
        uint256 featureSet,
        CoprocessorContextBlockPeriods calldata blockPeriods,
        Coprocessor[] calldata coprocessors
    ) external virtual onlyOwner {
        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();
        // This will revert if there is no active coprocessor context. Although this should never
        // happen, it acts as a safeguard to prevent any unexpected behaviors. If such scenario
        // ever happens, this means no new contexts could be added. Instead, the gateway will need to be
        // paused and upgraded to a new version which puts back a working active coprocessor context
        // manually.
        CoprocessorContext memory activeCoprocessorContext = getActiveCoprocessorContext();

        // Add the new coprocessor context, if valid
        // The previous context ID is the active coprocessor context ID
        CoprocessorContext memory newCoprocessorContext = _addCoprocessorContext(
            activeCoprocessorContext.contextId,
            featureSet,
            coprocessors
        );

        // Emit the event that indicates that a valid coprocessor context has been suggested to be added.
        emit NewCoprocessorContext(activeCoprocessorContext, newCoprocessorContext, blockPeriods);

        // Set the coprocessor context to the generating state
        // This currently has no implications on the coprocessor contexts, except that it will check
        // that there is no pre-activation or suspended coprocessor context ongoing, as it is
        // forbidden to generate a new context if there is already one in one of these states.
        // Still, we need to follow the general notion of context lifecycle states, which requires
        // that a context goes through the generating state before being pre-activated
        ContextLifecycle.setGenerating($.coprocessorContextLifecycle, newCoprocessorContext.contextId);

        // Directly pre-activate the coprocessor context
        ContextLifecycle.setPreActivation($.coprocessorContextLifecycle, newCoprocessorContext.contextId);

        // Define the activation block number for the new coprocessor context
        uint256 activationBlockNumber = block.number + blockPeriods.preActivationBlockPeriod;
        $.coprocessorContextActivationBlockNumber[newCoprocessorContext.contextId] = activationBlockNumber;

        // Store the suspended block period for the previous coprocessor context
        // This value will be considered once the new coprocessor context is activated and the old one
        // is suspended
        $.coprocessorContextSuspendedBlockPeriod[activeCoprocessorContext.contextId] = blockPeriods
            .suspendedBlockPeriod;

        // Emit the event that indicates that the new coprocessor context has been pre-activated
        emit PreActivateCoprocessorContext(newCoprocessorContext, activationBlockNumber);
    }

    /**
     * @dev See {ICoprocessorContexts-refreshCoprocessorContextStatuses}.
     */
    function refreshCoprocessorContextStatuses() external virtual whenNotPaused {
        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();

        // If there is a pre-activation coprocessor context, then check if it is time to activate it
        // and thus suspend the current active coprocessor context
        uint256 preActivationContextId = $.coprocessorContextLifecycle.preActivationContextId;
        if (preActivationContextId != 0) {
            if (block.number >= $.coprocessorContextActivationBlockNumber[preActivationContextId]) {
                uint256 activeContextId = getActiveCoprocessorContextId();

                // Define the deactivation block number for the current active coprocessor context
                uint256 deactivatedBlockNumber = block.number +
                    $.coprocessorContextSuspendedBlockPeriod[activeContextId];
                $.coprocessorContextDeactivatedBlockNumber[activeContextId] = deactivatedBlockNumber;

                // Set the current active coprocessor context to the suspended state
                ContextLifecycle.setSuspended($.coprocessorContextLifecycle, activeContextId);
                emit SuspendCoprocessorContext(activeContextId, deactivatedBlockNumber);

                // Set the new active coprocessor context
                ContextLifecycle.setActive($.coprocessorContextLifecycle, preActivationContextId);
                emit ActivateCoprocessorContext(preActivationContextId);
            }
        }

        // If there is a suspended coprocessor context, then check if it is time to deactivate it
        uint256 suspendedContextId = _getSuspendedCoprocessorContextId();
        if (suspendedContextId != 0) {
            if (block.number >= $.coprocessorContextDeactivatedBlockNumber[suspendedContextId]) {
                ContextLifecycle.setDeactivated($.coprocessorContextLifecycle, suspendedContextId);
                emit DeactivateCoprocessorContext(suspendedContextId);
            }
        }
    }

    /**
     * @dev See {ICoprocessorContexts-compromiseCoprocessorContext}.
     */
    function compromiseCoprocessorContext(
        uint256 contextId
    ) external virtual onlyOwner ensureContextInitialized(contextId) {
        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();

        // This will revert if the context is active in order to ensure that the gateway can
        // always provide an active coprocessor context
        // If too many parties are compromised for this coprocessor context, then the relevant functions
        // should be paused instead
        ContextLifecycle.setCompromised($.coprocessorContextLifecycle, contextId);
        emit CompromiseCoprocessorContext(contextId);
    }

    /**
     * @dev See {ICoprocessorContexts-destroyCoprocessorContext}.
     */
    function destroyCoprocessorContext(
        uint256 contextId
    ) external virtual onlyOwner ensureContextInitialized(contextId) {
        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();

        // This will revert if the context is active in order to ensure that the gateway can
        // always provide an active coprocessor context
        ContextLifecycle.setDestroyed($.coprocessorContextLifecycle, contextId);
        emit DestroyCoprocessorContext(contextId);
    }

    /**
     * @dev See {ICoprocessorContexts-moveSuspendedCoprocessorContextToActive}.
     */
    function moveSuspendedCoprocessorContextToActive() external virtual onlyOwner {
        // This will revert if there is no suspended coprocessor context
        uint256 suspendedContextId = getSuspendedCoprocessorContextId();

        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();

        // Deactivate the (problematic) active coprocessor context
        // This will not revert if there is no active coprocessor context (something that should never
        // happen), as this function is only meant to be used in case of emergency.
        uint256 activeContextId = _getActiveCoprocessorContextId();
        ContextLifecycle.setDeactivated($.coprocessorContextLifecycle, activeContextId);
        emit DeactivateCoprocessorContext(activeContextId);

        // Set the new active coprocessor context
        ContextLifecycle.setActive($.coprocessorContextLifecycle, suspendedContextId);
        emit ActivateCoprocessorContext(suspendedContextId);
    }

    /**
     * @dev See {ICoprocessorContexts-checkIsCoprocessorTxSenderFromContext}.
     */
    function checkIsCoprocessorTxSenderFromContext(
        uint256 contextId,
        address txSenderAddress
    ) external view virtual ensureContextInitialized(contextId) {
        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();
        if (!$.isCoprocessorTxSender[contextId][txSenderAddress]) {
            revert NotCoprocessorTxSenderFromContext(contextId, txSenderAddress);
        }
    }

    /**
     * @dev See {ICoprocessorContexts-checkIsCoprocessorSignerFromContext}.
     */
    function checkIsCoprocessorSignerFromContext(
        uint256 contextId,
        address signerAddress
    ) external view virtual ensureContextInitialized(contextId) {
        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();
        if (!$.isCoprocessorSigner[contextId][signerAddress]) {
            revert NotCoprocessorSignerFromContext(contextId, signerAddress);
        }
    }

    /**
     * @dev See {ICoprocessorContexts-isCoprocessorContextActiveOrSuspended}.
     */
    function isCoprocessorContextActiveOrSuspended(
        uint256 contextId
    ) external view virtual ensureContextInitialized(contextId) returns (bool) {
        return (contextId == getActiveCoprocessorContextId() || contextId == _getSuspendedCoprocessorContextId());
    }

    /**
     * @dev See {ICoprocessorContexts-getCoprocessorContextActivationBlockNumber}.
     */
    function getCoprocessorContextActivationBlockNumber(
        uint256 contextId
    ) external view virtual ensureContextInitialized(contextId) returns (uint256) {
        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();
        return $.coprocessorContextActivationBlockNumber[contextId];
    }

    /**
     * @dev See {ICoprocessorContexts-getCoprocessorContextDeactivatedBlockNumber}.
     */
    function getCoprocessorContextDeactivatedBlockNumber(
        uint256 contextId
    ) external view virtual ensureContextInitialized(contextId) returns (uint256) {
        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();
        return $.coprocessorContextDeactivatedBlockNumber[contextId];
    }

    /**
     * @dev See {IGatewayConfig-getCoprocessorMajorityThresholdFromContext}.
     */
    function getCoprocessorMajorityThresholdFromContext(
        uint256 contextId
    ) external view virtual ensureContextInitialized(contextId) returns (uint256) {
        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();

        // The majority threshold is the number of coprocessors that is required to validate consensus
        // in the fhevm Gateway
        // It is currently defined as a strict majority within the coprocessor context (50% + 1)
        return ($.coprocessorTxSenderAddresses[contextId].length >> 1) + 1;
    }

    /**
     * @dev See {ICoprocessorContexts-getCoprocessor}.
     */
    function getCoprocessor(address coprocessorTxSenderAddress) external view virtual returns (Coprocessor memory) {
        uint256 activeContextId = getActiveCoprocessorContextId();
        return getCoprocessorFromContext(activeContextId, coprocessorTxSenderAddress);
    }

    /**
     * @dev See {ICoprocessorContexts-getCoprocessorTxSenders}.
     */
    function getCoprocessorTxSenders() external view virtual returns (address[] memory) {
        uint256 activeContextId = getActiveCoprocessorContextId();
        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();
        return $.coprocessorTxSenderAddresses[activeContextId];
    }

    /**
     * @dev See {ICoprocessorContexts-getCoprocessorSigners}.
     */
    function getCoprocessorSigners() external view virtual returns (address[] memory) {
        uint256 activeContextId = getActiveCoprocessorContextId();
        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();
        return $.coprocessorSignerAddresses[activeContextId];
    }

    /**
     * @dev See {ICoprocessorContexts-getCoprocessorContextStatus}.
     */
    function getCoprocessorContextStatus(uint256 contextId) external view virtual returns (ContextStatus) {
        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();
        return ContextLifecycle.getContextStatus($.coprocessorContextLifecycle, contextId);
    }

    /**
     * @dev See {ICoprocessorContexts-getVersion}.
     */
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

    /**
     * @dev Add a new coprocessor context to the state.
     * @param previousContextId The ID of the previous coprocessor context.
     * @param featureSet The feature set.
     * @param coprocessors The coprocessors.
     * @return The new coprocessor context.
     */
    function _addCoprocessorContext(
        uint256 previousContextId,
        uint256 featureSet,
        Coprocessor[] calldata coprocessors
    ) internal virtual returns (CoprocessorContext memory) {
        // A coprocessor context must contain at least one coprocessor
        if (coprocessors.length == 0) {
            revert EmptyCoprocessors();
        }

        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();

        // A coprocessor context ID starts at 1 and should never be null
        $.coprocessorContextCount++;
        uint256 contextId = $.coprocessorContextCount;

        // Solidity doesn't support directly copying complex data structures like `coprocessors`
        // (array of structs), so we need to instead create the struct field by field
        $.coprocessorContexts[contextId].contextId = contextId;
        $.coprocessorContexts[contextId].previousContextId = previousContextId;
        $.coprocessorContexts[contextId].featureSet = featureSet;

        // Then, we need to copy each coprocessor struct one by one
        for (uint256 i = 0; i < coprocessors.length; i++) {
            $.coprocessorContexts[contextId].coprocessors.push(coprocessors[i]);
        }

        // Register several additional mappings for faster lookups (in getters and checks)
        for (uint256 i = 0; i < coprocessors.length; i++) {
            if (coprocessors[i].txSenderAddress == address(0)) {
                revert NullCoprocessorTxSenderAddress(contextId, i);
            }

            if (coprocessors[i].signerAddress == address(0)) {
                revert NullCoprocessorSignerAddress(contextId, i);
            }

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
     * @dev Get the ID of the pre-activation coprocessor context, without reverting if it does not exist.
     * @return The ID of the pre-activation coprocessor context.
     */
    function _getPreActivationCoprocessorContextId() internal view virtual returns (uint256) {
        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();
        return $.coprocessorContextLifecycle.preActivationContextId;
    }

    /**
     * @dev Get the ID of the suspended coprocessor context, without reverting if it does not exist.
     * @return The ID of the suspended coprocessor context.
     */
    function _getSuspendedCoprocessorContextId() internal view virtual returns (uint256) {
        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();
        return $.coprocessorContextLifecycle.suspendedContextId;
    }

    /**
     * @dev Get the ID of the active coprocessor context, without reverting if it does not exist.
     * @return The ID of the active coprocessor context.
     */
    function _getActiveCoprocessorContextId() internal view virtual returns (uint256) {
        CoprocessorContextsStorage storage $ = _getCoprocessorContextsStorage();
        return $.coprocessorContextLifecycle.activeContextId;
    }

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
