// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { OAppSender, OAppCore, Origin, MessagingFee, MessagingReceipt } from "@layerzerolabs/oapp-evm/contracts/oapp/OApp.sol";
import { OAppOptionsType3 } from "@layerzerolabs/oapp-evm/contracts/oapp/libs/OAppOptionsType3.sol";
import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol";

contract GovernanceOAppSender is OAppSender, OAppOptionsType3 {
    /// @notice Msg type for sending data, for use in OAppOptionsType3 as an enforced option.
    uint16 public constant SEND = 1;
    uint32 public immutable DESTINATION_EID; /// @dev 40424 for Zama testnet, and ??? for mainnet.

    /// @notice A Safe transaction operation.
    /// @custom:variant Call The Safe transaction is executed with the `CALL` opcode.
    /// @custom:variant Delegatecall The Safe transaction is executed with the `DELEGATECALL` opcode.
    enum Operation {
        Call,
        DelegateCall
    }

    /// @notice This struct is used to avoid stack too deep errors
    struct MessagingReceiptOptions {
        MessagingReceipt receipt;
        bytes options;
    }

    /// @notice Thrown when trying to send cross-chain tx without sending enough ETH fees.
    error InsufficientFee();
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

    /// @notice Emitted when a proposal has been successfully sent to Zama Gateway chain;
    event RemoteProposalSent(
        address[] targets,
        uint256[] values,
        string[] functionSignatures,
        bytes[] datas,
        Operation[] operations,
        MessagingReceiptOptions receiptOptions
    );

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
        if (msg.value < quotedFee) revert InsufficientFee();

        {
            // local scope to avoid stack too deep error
            uint256 targetLen = targets.length;
            uint256 valueLen = values.length;
            uint256 dataLen = datas.length;
            uint256 operationLen = operations.length;
            uint256 functionSignatureLen = functionSignatures.length;
            if (targetLen == 0) revert TargetsIsEmpty();
            if (targetLen != valueLen) revert TargetsNotSameLengthAsValues();
            if (targetLen != dataLen) revert TargetsNotSameLengthAsDatas();
            if (targetLen != operationLen) revert TargetsNotSameLengthAsOperations();
            if (targetLen != functionSignatureLen) revert TargetsNotSameLengthAsFunctionSignatures();
        }

        MessagingReceipt memory receipt = _lzSend(
            DESTINATION_EID,
            abi.encode(targets, values, functionSignatures, datas, operations),
            combineOptions(DESTINATION_EID, SEND, options),
            MessagingFee(msg.value, 0),
            payable(msg.sender)
        );

        MessagingReceiptOptions memory receiptOptions = MessagingReceiptOptions({ receipt: receipt, options: options });

        emit RemoteProposalSent(targets, values, functionSignatures, datas, operations, receiptOptions);
    }
}
