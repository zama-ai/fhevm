// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.22;

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

    /// @notice Thrown when trying to send cross-chain tx without sending enough ETH fees.
    error InsufficientFee();
    /// @notice Thrown when trying to deploy this contract on an unsupported blockchain.
    error UnsupportedChainID();

    /// @notice Emitted when a proposal has been successfully sent to Zama Gateway chain;
    event RemoteProposalSent(address target, uint256 value, bytes data, Operation operation, bytes options, MessagingReceipt receipt);  

    /// @notice Initialize with Endpoint V2 and owner address.
    /// @param endpoint The local chain's LayerZero Endpoint V2 address.
    /// @param owner    The address permitted to configure this OApp.
    constructor(address endpoint, address owner) OAppCore(endpoint, owner) Ownable(owner) {
        uint256 chainID = block.chainid;
        if (chainID == 1) { // chainID of ethereum-mainnet i.e linked to gateway-mainnet.
            revert('TODO: to fill with correct value, destEid unknown yet for Zama mainnet');
        } else if (chainID == 11155111) { // chainID of ethereum-testnet i.e linked to gateway-testnet.
            DESTINATION_EID = 40424;
        } else {
            revert UnsupportedChainID();
        }
    }

    /// @notice Quotes the gas needed to pay for the full cross-chain transaction in native gas.
    /// @param target The target contract to be called.
    /// @param value The value to be sent.
    /// @param data The calldata to be used.
    /// @param operation The Safe operation.
    /// @param options Message execution options (e.g., for sending gas to destination).
    /// @return fee The calculated gas fee in native ETH token.
    function quoteSendCrossChainTransaction(
        address target,
        uint256 value,
        bytes calldata data,
        Operation operation,
        bytes calldata options
    ) public view returns (uint256 fee) {
        bytes memory message = abi.encode(target, value, data, operation);
        MessagingFee memory mfee = _quote(DESTINATION_EID, message, combineOptions(DESTINATION_EID, SEND, options), false);
        fee = mfee.nativeFee;
    }

    /// @notice Send a cross-chain proposal to Zama Gateway chain. Only the owner, i.e the Aragaon DAO, should be able to send proposals.
    /// @param target The target contract to be called.
    /// @param value The value to be sent.
    /// @param data The calldata to be used.
    /// @param operation The Safe operation.
    /// @param options Message execution options (e.g., for sending gas to destination).
    function sendRemoteProposal(
        address target,
        uint256 value,
        bytes calldata data,
        Operation operation,
        bytes calldata options
        ) external payable onlyOwner {
        uint256 quotedFee = quoteSendCrossChainTransaction(target, value, data, operation, options);
        if(msg.value < quotedFee) revert InsufficientFee();

        MessagingReceipt memory receipt = _lzSend(
            DESTINATION_EID,
            abi.encode(target, value, data, operation),
            combineOptions(DESTINATION_EID, SEND, options),
            MessagingFee(msg.value, 0),
            payable(msg.sender)
        );

        emit RemoteProposalSent(target, value, data, operation, options, receipt);  
    }
}