// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { OAppSender, OAppCore, Origin, MessagingFee, MessagingReceipt } from "@layerzerolabs/oapp-evm/contracts/oapp/OApp.sol";
import { OAppOptionsType3 } from "@layerzerolabs/oapp-evm/contracts/oapp/libs/OAppOptionsType3.sol";
import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol";
import { MessagingParams } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ILayerZeroEndpointV2.sol";
import { Operation } from "./shared/Structs.sol";

contract GovernanceOAppSender is OAppSender, OAppOptionsType3 {
    /// @notice Msg type for sending data, for use in OAppOptionsType3 as an enforced option.
    uint16 public constant SEND = 1;
    uint32 public immutable DESTINATION_EID; /// @dev 40424 for Zama testnet, and ??? for mainnet.

    /// @notice This struct is used to avoid stack too deep errors
    struct MessagingReceiptOptions {
        MessagingReceipt receipt;
        bytes options;
    }

    /// @notice Thrown when failing to withdraw ETH from the contract.
    error FailedToWithdrawETH();
    /// @notice Thrown when trying to send cross-chain tx if contract holds unsufficient ETH to pay the LZ fees.
    error InsufficientBalanceForFee();
    /// @notice Thrown when recipient is the null address.
    error InvalidNullRecipient();
    /// @notice Thrown when trying to deploy this contract on an unsupported blockchain.
    error UnsupportedChainID();

    /// @notice Thrown when targets array is empty.
    error TargetsIsEmpty();
    /// @notice Thrown when length of targets array is different than length of datas array.
    error TargetsNotSameLengthAsDatas();
    /// @notice Thrown when length of targets array is different than length of operations array.
    error TargetsNotSameLengthAsOperations();
    /// @notice Thrown when length of targets array is different than length of values array.
    error TargetsNotSameLengthAsValues();
    /// @notice Thrown when length of targets array is different than length of functionSignatures array.
    error TargetsNotSameLengthAsFunctionSignatures();

    /// @notice Emitted when a proposal has been successfully sent to Zama Gateway chain.
    event RemoteProposalSent(
        address[] targets,
        uint256[] values,
        string[] functionSignatures,
        bytes[] datas,
        Operation[] operations,
        MessagingReceiptOptions receiptOptions
    );
    /// @notice Emitted when ETH has been successfully withdrawn from the contract.
    event WithdrawnETH(address indexed recipient, uint256 amount);

    /// @notice Initialize with Endpoint V2 and owner address.
    /// @param endpoint The local chain's LayerZero Endpoint V2 address.
    /// @param owner    The address permitted to configure this OApp.
    constructor(address endpoint, address owner) OAppCore(endpoint, owner) Ownable(owner) {
        uint256 chainID = block.chainid;
        if (chainID == 1) {
            // chainID of ethereum-mainnet i.e linked to gateway-mainnet.
            revert("TODO: to fill with correct value, destEid unknown yet for Zama mainnet");
        } else if (chainID == 11155111) {
            // chainID of ethereum-testnet i.e linked to gateway-testnet.
            DESTINATION_EID = 40424;
        } else {
            revert UnsupportedChainID();
        }
    }

    /// @notice Quotes the gas needed to pay for the full cross-chain transaction in native gas.
    /// @param targets The target contracts to be called.
    /// @param values The values to be sent.
    /// @param datas The calldatas to be used.
    /// @param operations The Safe operations.
    /// @param options Message execution options (e.g., for sending gas to destination).
    /// @return fee The calculated gas fee in native ETH token.
    function quoteSendCrossChainTransaction(
        address[] calldata targets,
        uint256[] calldata values,
        string[] calldata functionSignatures,
        bytes[] calldata datas,
        Operation[] calldata operations,
        bytes calldata options
    ) public view returns (uint256 fee) {
        bytes memory message = abi.encode(targets, values, functionSignatures, datas, operations);
        MessagingFee memory mfee = _quote(
            DESTINATION_EID,
            message,
            combineOptions(DESTINATION_EID, SEND, options),
            false
        );
        fee = mfee.nativeFee;
    }

    /// @notice Send a cross-chain proposal to Zama Gateway chain. Only the owner, i.e the Aragaon DAO, should be able to send proposals.
    /// @notice The default LayerZero executor caps payload size at 10000 bytes. Proposals with many batched calls or large calldata can
    /// exceed this limit and cause `_lzSend` to revert on the source chain.
    /// @param targets The target contracts to be called.
    /// @param values The values to be sent.
    /// @param datas The calldatas to be used (with function selector, if functionSignatures[i] is an empty string).
    /// @param functionSignatures Function signatures - optional: if empty string, datas[i] is already starting with the function selector.
    /// @param operations The Safe operations.
    /// @param options Message execution options (e.g., for sending gas to destination).
    function sendRemoteProposal(
        address[] calldata targets,
        uint256[] calldata values,
        string[] calldata functionSignatures,
        bytes[] calldata datas,
        Operation[] calldata operations,
        bytes calldata options
    ) external payable onlyOwner {
        uint256 quotedFee = quoteSendCrossChainTransaction(
            targets,
            values,
            functionSignatures,
            datas,
            operations,
            options
        );
        if (address(this).balance < quotedFee) revert InsufficientBalanceForFee();
        {
            // local scope to avoid stack too deep error
            uint256 targetLen = targets.length;
            if (targetLen == 0) revert TargetsIsEmpty();
            if (targetLen != values.length) revert TargetsNotSameLengthAsValues();
            if (targetLen != datas.length) revert TargetsNotSameLengthAsDatas();
            if (targetLen != operations.length) revert TargetsNotSameLengthAsOperations();
            if (targetLen != functionSignatures.length) revert TargetsNotSameLengthAsFunctionSignatures();
        }

        bytes memory message = abi.encode(targets, values, functionSignatures, datas, operations);
        MessagingReceiptOptions memory receiptOptions = _sendWithOptions(message, options, quotedFee);

        emit RemoteProposalSent(targets, values, functionSignatures, datas, operations, receiptOptions);
    }

    /// @dev Combines options, sends the message, and returns the receipt bundled with the used options.
    function _sendWithOptions(
        bytes memory message,
        bytes calldata options,
        uint256 quotedFee
    ) private returns (MessagingReceiptOptions memory) {
        bytes memory combinedOptions = combineOptions(DESTINATION_EID, SEND, options);
        MessagingReceipt memory receipt = _lzSend(
            DESTINATION_EID,
            message,
            combinedOptions,
            MessagingFee(quotedFee, 0),
            address(this)
        );
        return MessagingReceiptOptions({ receipt: receipt, options: combinedOptions });
    }

    /// @dev We override the default LZ internal _lzSend function, to make the contract able to pay LZ fees.
    /// @dev This is to avoid race condition during quote, since it could be called by a DAO with timelock.
    /// @dev Internal function to interact with the LayerZero EndpointV2.send() for sending a message.
    /// @param dstEid The destination endpoint ID.
    /// @param message The message payload.
    /// @param options Additional options for the message.
    /// @param fee The calculated LayerZero fee for the message.
    /// @param refundAddress The address to receive any excess fee values sent to the endpoint.
    /// @return receipt The receipt for the sent message.
    function _lzSend(
        uint32 dstEid,
        bytes memory message,
        bytes memory options,
        MessagingFee memory fee,
        address refundAddress
    ) internal override returns (MessagingReceipt memory receipt) {
        return
            // solhint-disable-next-line check-send-result
            endpoint.send{ value: fee.nativeFee }(
                MessagingParams(dstEid, _getPeerOrRevert(dstEid), message, options, fee.lzTokenFee > 0),
                refundAddress
            );
    }

    /// @dev Allows the owner to withdraw ETH held by the contract.
    /// @param amount Amount of withdrawn ETH.
    /// @param recipient Receiver of the withdrawn ETH, should be non-null.
    function withdrawETH(uint256 amount, address recipient) external onlyOwner {
        if (recipient == address(0)) revert InvalidNullRecipient();

        (bool success, ) = recipient.call{ value: amount }("");
        if (!success) {
            revert FailedToWithdrawETH();
        }
        emit WithdrawnETH(recipient, amount);
    }

    receive() external payable {}
}
