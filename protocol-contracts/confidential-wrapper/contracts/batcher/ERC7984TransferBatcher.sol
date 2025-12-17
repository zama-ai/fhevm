// SPDX-License-Identifier: MIT

pragma solidity ^0.8.27;

import {ZamaEthereumConfig} from "@fhevm/solidity/config/ZamaConfig.sol";
import {FHE, euint64, externalEuint64} from "@fhevm/solidity/lib/FHE.sol";
import {RegulatedERC7984Upgradeable} from "../token/RegulatedERC7984Upgradeable.sol";
import {AdminProvider} from "../admin/AdminProvider.sol";
import {FeeManager} from "../admin/FeeManager.sol";

/// @title ERC7984TransferBatcher
/// @notice Batch processor for confidential transfers with retry mechanism
/// @dev Enables efficient multi-transfer operations with fee payment and sender tracking
/// @dev Key features:
///      - Single transaction batches multiple confidential transfers
///      - Tracks original sender per transaction for retry authorization
///      - Charges fixed fee per batch (not per transfer)
///      - Supports retry mechanism for failed transfers with original sender verification
/// @custom:security-contact contact@zaiffer.org
contract ERC7984TransferBatcher is ZamaEthereumConfig {
    error OnlyOriginalSenderCanRetry();
    error InsufficientFee();
    error EmptyTransferArray();
    error FeeTransferFailed();

    /// @dev The given holder `holder` is not authorized to spend on behalf of `spender`.
    error ERC7984UnauthorizedSpender(address holder, address spender);

    event BatchTransfer(address indexed cToken, address indexed sender, uint256 startTxId, uint256 endTxId, uint256 fee);
    event RetryTransfer(address indexed cToken, address indexed sender, uint256 originalTxId, uint256 retryTxId);

    /// @dev AdminProvider contract for accessing fee manager
    AdminProvider public immutable adminProvider;

    /// @dev Maps cToken address => txId => original sender address for retry authorization
    /// @dev Used to verify that only the original sender can retry a specific transaction
    mapping(address cToken => mapping(uint256 txId => address sender)) public txIdToSender;

    /// @notice Constructs ERC7984TransferBatcher with admin provider reference
    /// @param adminProvider_ AdminProvider contract for accessing fee manager
    constructor(AdminProvider adminProvider_) {
        adminProvider = adminProvider_;
    }

    /// @notice Input structure for a single confidential transfer within a batch
    /// @param to Recipient address for the confidential transfer
    /// @param encryptedAmount Encrypted transfer amount (euint64)
    /// @param inputProof proof for the encrypted amount
    /// @param retryFor Transaction ID being retried (0 for new transfers)
    struct ConfidentialTransferInput {
        address to;
        externalEuint64 encryptedAmount;
        bytes inputProof;
        uint256 retryFor;
    }

    /// @notice Executes a batch of confidential transfers in a single transaction
    /// @param cToken The confidential token contract to transfer
    /// @param from The address from which tokens will be transferred (must be msg.sender or operator-approved)
    /// @param transfers Array of transfer inputs containing recipient, amount, proof, and retry info
    /// @dev Charges a fixed batch transfer fee (paid in ETH)
    /// @dev Fee amount determined by FeeManager.getBatchTransferFee()
    /// @dev Tracks original sender for each transaction ID to enable retry authorization
    /// @dev For retries, verifies msg.sender matches original sender via txIdToSender mapping
    /// @dev Emits BatchTransfer event with start and end transaction IDs
    /// @dev Emits RetryTransfer event for each retry operation
    /// @dev Reverts if:
    ///      - transfers array is empty (EmptyTransferArray)
    ///      - msg.value doesn't match required fee (InsufficientFee)
    ///      - fee transfer to recipient fails (FeeTransferFailed)
    ///      - retry attempted by non-original sender (OnlyOriginalSenderCanRetry)
    ///      - msg.sender is not an approved operator for `from` address
    function confidentialBatchTransfer(
        RegulatedERC7984Upgradeable cToken,
        address from,
        ConfidentialTransferInput[] calldata transfers
    ) external payable {
        if (transfers.length == 0) {
            revert EmptyTransferArray();
        }

        FeeManager feeManager = adminProvider.feeManager();
        uint64 requiredFee = feeManager.getBatchTransferFee();

        if (msg.value != requiredFee) {
            revert InsufficientFee();
        }

        require(cToken.isOperator(from, msg.sender), ERC7984UnauthorizedSpender(from, msg.sender));

        uint256 startTxId = cToken.nextTxId();

        for (uint256 i = 0; i < transfers.length; i++) {
            if (transfers[i].retryFor != 0) {
                require(
                    txIdToSender[address(cToken)][transfers[i].retryFor] == from,
                    OnlyOriginalSenderCanRetry()
                );
            }

            euint64 amount = FHE.fromExternal(
                transfers[i].encryptedAmount,
                transfers[i].inputProof
            );
            FHE.allowTransient(amount, address(cToken));

            uint256 currentTxId = cToken.nextTxId();
            cToken.confidentialTransferFrom(
                from,
                transfers[i].to,
                amount
            );

            txIdToSender[address(cToken)][currentTxId] = from;

            if (transfers[i].retryFor != 0) {
                emit RetryTransfer(address(cToken), from, transfers[i].retryFor, currentTxId);
            }
        }

        uint256 endTxId = cToken.nextTxId() - 1;

        if (msg.value > 0) {
            address feeRecipient = feeManager.getFeeRecipient();
            (bool success, ) = feeRecipient.call{value: msg.value}("");
            require(success, FeeTransferFailed());
        }

        emit BatchTransfer(address(cToken), from, startTxId, endTxId, requiredFee);
    }
}
