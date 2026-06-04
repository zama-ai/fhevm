// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {OAppCoreUpgradeable} from "@layerzerolabs/oapp-evm-upgradeable/contracts/oapp/OAppCoreUpgradeable.sol";
import {Ownable2StepUpgradeable} from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";

import {ACLOwnable} from "../shared/ACLOwnable.sol";
import {aclAdd} from "../../addresses/FHEVMHostAddresses.sol";
import {UUPSUpgradeableEmptyProxy} from "../shared/UUPSUpgradeableEmptyProxy.sol";
import {HandlesSender} from "./HandlesSender.sol";
import {HandlesReceiver} from "./HandlesReceiver.sol";

/**
 * @title ConfidentialBridge
 * @notice Sole deployed contract for confidential handle bridging on a given chain.
 *         Combines the source-side {HandlesSender} mixin (`send` + ACL check + outbound
 *         `BridgeHandle` event) and the destination-side {HandlesReceiver} mixin
 *         (`_lzReceive` + handle derivation + `HandleBridged` event + lzCompose
 *         dispatch). One bridge instance per chain serves both directions: outbound
 *         sends as source, inbound receives as destination.
 *
 * @dev    Upgradeable UUPS proxy, following the same two-phase pattern as the other
 *         host contracts: an `EmptyUUPSProxy*` proxy is deployed first, then upgraded
 *         to this implementation via `initializeFromEmptyProxy(...)`. The LayerZero
 *         endpoint is set as an immutable in the implementation's constructor; the
 *         proxy's storage holds the `_dstChainIdForEid` map (ERC-7201 namespaced in
 *         `HandlesSender`) and the OApp peers map (ERC-7201 namespaced in
 *         `OAppCoreUpgradeable`).
 *
 *         ACL tracks this contract via {ACL.getConfidentialBridgeAddress}.
 *
 * @dev    Single ownership model: the bridge has exactly one owner, the ACL owner.
 *         Both the LayerZero operational surface (`setPeer`, `setDelegate`,
 *         {setDstChainId}, {grantFallbackPlaintext}) and UUPS upgrades resolve to the
 *         same governance account. This is implemented by overriding {owner} to read
 *         `Ownable2StepUpgradeable(aclAdd).owner()` on every call, so OZ's
 *         `_checkOwner` (and therefore every `onlyOwner` site in the inheritance
 *         chain, including the LayerZero ones) is gated by the ACL owner without
 *         maintaining a separate ownership slot. The local `OwnableUpgradeable`
 *         storage is left uninitialized and is unreadable through the public surface;
 *         {transferOwnership} and {renounceOwnership} are blocked to prevent writing
 *         to a now-dead slot.
 */
/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
contract ConfidentialBridge is UUPSUpgradeableEmptyProxy, ACLOwnable, HandlesSender, HandlesReceiver {
    /// @notice Returned when `dstEids` and `dstChainIds` initializer arrays differ in length.
    error DstChainIdArrayLengthMismatch(uint256 dstEidsLength, uint256 dstChainIdsLength);

    /// @notice Returned when {transferOwnership} or {renounceOwnership} is called.
    /// @dev    The bridge owner is bound to the ACL owner; it cannot be moved locally.
    error OwnershipNotTransferable();

    /// @notice Name of the contract.
    string private constant CONTRACT_NAME = "ConfidentialBridge";

    /// @notice Major version of the contract.
    uint256 private constant MAJOR_VERSION = 0;

    /// @notice Minor version of the contract.
    uint256 private constant MINOR_VERSION = 1;

    /// @notice Patch version of the contract.
    uint256 private constant PATCH_VERSION = 0;

    /// Constant used for making sure the version number used in the `reinitializer` modifier is
    /// identical between `initializeFromEmptyProxy` and the `reinitializeVX` method
    uint64 private constant REINITIALIZER_VERSION = 2;

    /**
     * @param _lzEndpoint LayerZero V2 endpoint address on this chain. Stored as an
     *                    immutable on the implementation by {OAppCoreUpgradeable}.
     */
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor(address _lzEndpoint) OAppCoreUpgradeable(_lzEndpoint) {
        _disableInitializers();
    }

    /**
     * @notice Initializes the bridge proxy from the empty-proxy shell.
     *
     * @param dstEids      LayerZero endpoint ids to seed the dstEid → dstChainId map with.
     *                     May be empty; pairs can also be added later via {setDstChainId}.
     * @param dstChainIds  Destination chain ids paired index-by-index with `dstEids`.
     *
     * @dev    The LayerZero "delegate" (the address with endpoint-side config
     *         rights on the LZ endpoint contract) is seeded with the current ACL owner;
     *         it can be reassigned afterwards via the OApp's `setDelegate`, which is
     *         itself gated by `onlyOwner` → ACL owner.
     */
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy(
        uint32[] calldata dstEids,
        uint64[] calldata dstChainIds
    ) public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        // OAppCore.__OAppCore_init only registers the delegate on the LayerZero endpoint;
        // it does not write the OApp owner. We deliberately skip __Ownable_init: the
        // owner() override below makes the local Ownable storage slot unread (and the
        // overrides of transferOwnership/renounceOwnership prevent it from being written).
        __OAppCore_init(Ownable2StepUpgradeable(aclAdd).owner());
        __OAppReceiver_init_unchained();
        __OAppSender_init_unchained();

        if (dstEids.length != dstChainIds.length) {
            revert DstChainIdArrayLengthMismatch(dstEids.length, dstChainIds.length);
        }
        for (uint256 i = 0; i < dstEids.length; i++) {
            _setDstChainId(dstEids[i], dstChainIds[i]);
        }
    }

    /**
     * @notice Returns the bridge owner — always equal to the current ACL owner.
     * @dev    Overrides {OwnableUpgradeable-owner} so every `onlyOwner` check in the
     *         inheritance chain (LayerZero's `setPeer`/`setDelegate`, this bridge's
     *         {setDstChainId}/{grantFallbackPlaintext}) resolves to the ACL owner
     *         without maintaining a second source of truth. Transfers of the ACL
     *         owner therefore propagate to the bridge automatically.
     */
    function owner() public view virtual override returns (address) {
        return Ownable2StepUpgradeable(aclAdd).owner();
    }

    /**
     * @notice Disabled. The bridge owner is bound to the ACL owner; move it on the
     *         ACL contract instead.
     */
    function transferOwnership(address) public pure virtual override {
        revert OwnershipNotTransferable();
    }

    /// @notice Disabled. See {transferOwnership}.
    function renounceOwnership() public pure virtual override {
        revert OwnershipNotTransferable();
    }

    /// @notice OApp version tuple — both send (1) and receive (2) paths are active.
    function oAppVersion()
        public
        pure
        override(HandlesSender, HandlesReceiver)
        returns (uint64 senderVersion, uint64 receiverVersion)
    {
        return (1, 2);
    }

    /**
     * @notice        Getter for the name and version of the contract.
     * @return string Name and the version of the contract.
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
     * @dev Should revert when msg.sender is not authorized to upgrade the contract.
     */
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyACLOwner {}
}
