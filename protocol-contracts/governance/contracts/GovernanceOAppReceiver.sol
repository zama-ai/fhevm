// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { OAppReceiver, OAppCore, Origin, MessagingFee } from "@layerzerolabs/oapp-evm/contracts/oapp/OApp.sol";
import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol";
import { Operation } from "./shared/Structs.sol";

/// @notice Interface of the AdminModule Safe Module which has privileged ownership of the Safe multi-sig owning GatewayConfig contract.
interface IAdminModule {
    function executeSafeTransactions(
        address[] calldata targets,
        uint256[] calldata values,
        bytes[] calldata datas,
        Operation[] calldata operations
    ) external;
}

contract GovernanceOAppReceiver is OAppReceiver {
    /// @notice The address of the privileged AdminModule of the Safe owning GatewayConfig contract.
    IAdminModule public adminSafeModule;

    /// @notice Thrown when trying to receive cross-chain tx while adminSafeModule is not set yet.
    error AdminSafeModuleNotSet();
    /// @notice Thrown when trying to set the adminSafeModule to the null address.
    error AdminSafeModuleIsNull();

    /// @notice Emitted when a proposal has been successfully received and executed by the Safe through the AdminModule.
    event ProposalExecuted(
        Origin origin,
        bytes32 guid,
        address[] targets,
        uint256[] values,
        string[] functionSignatures,
        bytes[] datas,
        Operation[] operations
    );

    /// @notice Initialize with Endpoint V2 and owner address.
    /// @param endpoint The local chain's LayerZero Endpoint V2 address.
    /// @param owner    The address permitted to configure this OApp.
    constructor(address endpoint, address owner) OAppCore(endpoint, owner) Ownable(owner) {}

    /// @notice Sets the adminSafeModule address, can only be called by the owner.
    /// @param adminModule  The address of the privileged AdminModule of the Safe owning GatewayConfig contract.
    function setAdminSafeModule(address adminModule) external onlyOwner {
        if (adminModule == address(0)) revert AdminSafeModuleIsNull();
        adminSafeModule = IAdminModule(adminModule);
    }

    /// @notice Invoked by GovernanceOAppReceiver when EndpointV2.lzReceive is called.
    /// @notice Reverts if adminSafeModule was not set.
    /// @dev   origin    Metadata (source chain, sender address, nonce).
    /// @dev   guid      Global unique ID for tracking this message.
    /// @param message   ABI-encoded bytes (the string we sent earlier).
    /// @dev   executor  Executor address that delivered the message.
    /// @dev   extraData Additional data from the Executor (unused here).
    function _lzReceive(
        Origin calldata origin,
        bytes32 guid,
        bytes calldata message,
        address /*executor*/,
        bytes calldata /*extraData*/
    ) internal override {
        if (address(adminSafeModule) == address(0)) revert AdminSafeModuleNotSet();

        (
            address[] memory targets,
            uint256[] memory values,
            string[] memory functionSignatures,
            bytes[] memory datas,
            Operation[] memory operations
        ) = abi.decode(message, (address[], uint256[], string[], bytes[], Operation[]));

        uint256 targetLen = targets.length;
        for (uint256 idx = 0; idx < targetLen; idx++) {
            /// @dev if function signature is an empty string, datas is the full calldata already starting with the selector
            datas[idx] = bytes(functionSignatures[idx]).length == 0
                ? datas[idx]
                : abi.encodePacked(bytes4(keccak256(bytes(functionSignatures[idx]))), datas[idx]);
        }

        adminSafeModule.executeSafeTransactions(targets, values, datas, operations);

        emit ProposalExecuted(origin, guid, targets, values, functionSignatures, datas, operations);
    }
}
