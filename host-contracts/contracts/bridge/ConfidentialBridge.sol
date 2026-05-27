// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {OAppCoreUpgradeable} from "@layerzerolabs/oapp-evm-upgradeable/contracts/oapp/OAppCoreUpgradeable.sol";

import {ACLOwnable} from "../shared/ACLOwnable.sol";
import {UUPSUpgradeableEmptyProxy} from "../shared/UUPSUpgradeableEmptyProxy.sol";
import {HandlesSender} from "./HandlesSender.sol";
import {HandlesReceiver} from "./HandlesReceiver.sol";

/**
 * @title ConfidentialBridge
 * @notice Sole deployed artifact for confidential handle bridging on a given chain.
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
 *         ACL and external apps track this contract via {ACL.getConfidentialBridgeAddress}
 *         using the proxy address.
 *
 * @dev    Ownership has two layers:
 *         - **Operational owner** (`onlyOwner` from {OAppCoreUpgradeable}/Ownable): manages
 *           LayerZero peers, the dstEid → dstChainId map, the LZ delegate, and authorizes
 *           {grantFallbackPlaintext}. Set in `initializeFromEmptyProxy`.
 *         - **Upgrade owner** (`onlyACLOwner` from {ACLOwnable}): authorizes UUPS upgrades.
 *           Matches the rest of the host contracts and lets a single governance account
 *           upgrade the entire host stack.
 *         The two can be the same address in production; the test fixtures keep them
 *         distinct to exercise the boundary.
 */
/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
contract ConfidentialBridge is UUPSUpgradeableEmptyProxy, ACLOwnable, HandlesSender, HandlesReceiver {
    /// @notice Returned when `dstEids` and `dstChainIds` initializer arrays differ in length.
    error DstChainIdArrayLengthMismatch(uint256 dstEidsLength, uint256 dstChainIdsLength);

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
     * @param _owner       Initial owner. Receives both governance ownership (setPeer,
     *                     setDstChainId, grantFallbackPlaintext, upgrades) and the
     *                     LayerZero "delegate" role on the endpoint.
     * @param dstEids      LayerZero endpoint ids to seed the dstEid → dstChainId map with.
     *                     May be empty; pairs can also be added later via {setDstChainId}.
     * @param dstChainIds  Destination chain ids paired index-by-index with `dstEids`.
     */
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy(
        address _owner,
        uint32[] calldata dstEids,
        uint64[] calldata dstChainIds
    ) public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        // Order: own state first, then OApp parents. The OApp delegate (config-on-endpoint)
        // is set to the same address as the owner — matches the previous non-upgradeable
        // constructor's `OAppCore(_lzEndpoint, _owner)` semantics.
        __Ownable_init(_owner);
        __OAppCore_init(_owner);
        __OAppReceiver_init_unchained();
        __OAppSender_init_unchained();

        if (dstEids.length != dstChainIds.length) {
            revert DstChainIdArrayLengthMismatch(dstEids.length, dstChainIds.length);
        }
        for (uint256 i = 0; i < dstEids.length; i++) {
            _setDstChainId(dstEids[i], dstChainIds[i]);
        }
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

    /// @notice Returns the human-readable version string.
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

    /// @dev Gates UUPS upgrades on the ACL owner — matches the other host contracts
    ///      (FHEVMExecutor, KMSGeneration, KMSVerifier, InputVerifier, HCULimit).
    ///      Operational governance of the bridge (setPeer, setDstChainId,
    ///      grantFallbackPlaintext, ...) remains gated by the bridge's own `onlyOwner`
    ///      inherited from {OAppCoreUpgradeable}; only the upgrade path delegates to ACL.
    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyACLOwner {}
}
