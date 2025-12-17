// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity 0.8.27;

import {FHESafeMath} from "openzeppelin-confidential-contracts/contracts/utils/FHESafeMath.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {RegulatedERC7984Upgradeable} from "../token/RegulatedERC7984Upgradeable.sol";
import {ZamaEthereumConfig} from "@fhevm/solidity/config/ZamaConfig.sol";
import {FHE, ebool, euint64 } from "@fhevm/solidity/lib/FHE.sol";
import {AdminProvider} from "../admin/AdminProvider.sol";
import {FeeManager} from "../admin/FeeManager.sol";
import {IWrapperReceiver} from "../interfaces/IWrapperReceiver.sol";
import {SafeCast} from "@openzeppelin/contracts/utils/math/SafeCast.sol";


/// @notice Wrapper contract for a single token type, providing wrapping/unwrapping functionality
/// @dev Each wrapper handles exactly one underlying token (ERC20 or ETH) and one confidential token
/// @custom:security-contact contact@zaiffer.org
contract Wrapper is ZamaEthereumConfig {
    using SafeERC20 for IERC20;

    /// @notice Address of the original token (address(0) for ETH)
    address public immutable originalToken;
    
    /// @notice The paired confidential token
    RegulatedERC7984Upgradeable public immutable confidentialToken;
    
    /// @notice AdminProvider for shared configuration
    AdminProvider public immutable adminProvider;
    
    struct ReceiverEntry {
        address to;
        bytes callbackData;
    }

    uint256 public requestId = 0;

    mapping(uint256 decryptionRequest => ReceiverEntry receiverEntry) private _receivers;

    error IncorrectEthAmount();
    error CannotReceiveEthForTokenWrap();
    error CannotSendToZeroAddress();
    error EthFeeTransferFailed();

    event Wrapped(uint64 mintAmount, uint256 amountIn, uint256 feeAmount, address indexed to_, uint256 indexed mintTxId);
    event UnwrappedFinalized(
        uint256 indexed requestId,
        bool finalizeSuccess,
        bool feeTransferSuccess,
        uint64 burnAmount,
        uint256 unwrapAmount,
        uint256 feeAmount,
        uint256 indexed nextTxId
    );
    event UnwrappedStarted(
        bool returnVal,
        uint256 indexed requestId,
        uint256 indexed txId,
        address indexed to,
        euint64 requestedAmount,
        euint64 burnAmount
    );
    /// @dev The given gateway request ID `requestId` is invalid.
    error ERC7984InvalidGatewayRequest(uint256 requestId);
    /// @dev FHE.isSenderAllowed(encryptedValue) returned false
    error SenderNotAllowed();

    constructor(
        address originalToken_,
        RegulatedERC7984Upgradeable confidentialToken_,
        AdminProvider adminProvider_
    ) {
        originalToken = originalToken_;
        confidentialToken = confidentialToken_;
        adminProvider = adminProvider_;
    }

    /// @notice Wraps original tokens (ETH or ERC20) into confidential tokens
    /// @dev Handles fee-on-transfer tokens by tracking actual balances received
    /// @param to_ The recipient address for the wrapped confidential tokens
    /// @param amount_ The total amount of original tokens to wrap (including fees)
    function wrap(address to_, uint256 amount_) external payable {
        // Calculate base protocol fee
        uint256 baseFee = _getWrapFee(amount_, to_);
        uint256 baseAmount = amount_ - baseFee;

        // Calculate dust from uint256 to uint64 conversion
        uint256 rate = confidentialToken.rate();
        uint256 wrapDust = baseAmount % rate;
        uint256 transferAmount = baseAmount - wrapDust;  // == baseAmount / rate * rate
        uint256 totalFee = baseFee + wrapDust;

        uint256 mintTxId = confidentialToken.nextTxId();

        address feeRecipient = _getFeeRecipient();
        uint256 actualFeeReceived; // Actual amount received by fee recipient (for event)

        uint64 mintAmount;
        if (originalToken == address(0)) {
            require(msg.value == amount_, IncorrectEthAmount());

            mintAmount = SafeCast.toUint64(transferAmount / rate);

            // For ETH, the fee recipient receives exactly totalFee or the transaction reverts
            (bool ethTransferSuccess, ) = feeRecipient.call{value: totalFee}("");
            if (!ethTransferSuccess) {
                revert EthFeeTransferFailed();
            }
            actualFeeReceived = totalFee;
        } else {
            require(msg.value == 0, CannotReceiveEthForTokenWrap());

            // Track wrapper balance to handle fee-on-transfer tokens
            uint256 balanceBefore = IERC20(originalToken).balanceOf(address(this));
            IERC20(originalToken).safeTransferFrom(msg.sender, address(this), transferAmount);
            uint256 balanceAfter = IERC20(originalToken).balanceOf(address(this));

            // Track fee recipient balance to emit accurate fee amount
            uint256 feeRecipientBalanceBefore = IERC20(originalToken).balanceOf(feeRecipient);

            // Transfer protocol fee to fee recipient
            IERC20(originalToken).safeTransferFrom(msg.sender, feeRecipient, totalFee);

            // Calculate actual amount received by wrapper (handles fee-on-transfer tokens)
            uint256 balanceDifference = balanceAfter - balanceBefore;
            mintAmount = SafeCast.toUint64(balanceDifference / rate);

            // Transfer dust (remainder from uint256→uint64 conversion) to fee recipient
            uint256 transferDust = balanceDifference % rate;
            IERC20(originalToken).safeTransfer(feeRecipient, transferDust);

            // Calculate actual fee received by fee recipient (for accurate event emission)
            uint256 feeRecipientBalanceAfter = IERC20(originalToken).balanceOf(feeRecipient);
            actualFeeReceived = feeRecipientBalanceAfter - feeRecipientBalanceBefore;
        }

        confidentialToken.mint(to_, mintAmount);

        // Emit event with actual fee received by protocol (not the amount sent)
        emit Wrapped(mintAmount, amount_, actualFeeReceived, to_, mintTxId);
    }

    /// @notice Initiates unwrapping of confidential tokens back to original tokens (ETH or ERC20)
    /// @dev This is the ERC7984 receiver callback, triggered when confidential tokens are transferred to this wrapper
    /// @dev Security: Only accepts calls from the paired confidential token contract
    /// @dev The unwrap flow is asynchronous: this function burns tokens and requests decryption, then finalizeUnwrap completes the transfer
    /// @param from The address that initiated the confidential transfer (token holder)
    /// @param amount The encrypted amount of confidential tokens to unwrap
    /// @param data ABI-encoded (address to, bytes callbackData) where:
    ///        - to: recipient address for the unwrapped original tokens
    ///        - callbackData: optional data passed to IWrapperReceiver.onUnwrapFinalizedReceived if recipient is a contract
    /// @return ebool(true) if unwrap was accepted and initiated, ebool(false) if rejected (wrong caller)
    function onConfidentialTransferReceived(
        address /* operator */,
        address from,
        euint64 amount,
        bytes calldata data
    ) external returns (ebool) {
        require(FHE.isSenderAllowed(amount), SenderNotAllowed());

        // burn tx if success is true
        // reimpbursement tx if success is false
        uint256 nextTxId = confidentialToken.nextTxId();

        (address to, bytes memory unwrapCallbackData) = abi.decode(data, (address, bytes));

        bool returnVal = true;
        ebool eReturnVal = FHE.asEbool(returnVal);
        FHE.allowTransient(eReturnVal, msg.sender);
        if (msg.sender != address(confidentialToken)) {
            returnVal = false;
            eReturnVal = FHE.asEbool(returnVal);
            FHE.allowTransient(eReturnVal, msg.sender);
            emit UnwrappedStarted(returnVal, 0, nextTxId, to, amount, FHE.asEuint64(0));
            return eReturnVal;
        }

        require(to != address(0), CannotSendToZeroAddress());

        euint64 actualBurnAmount = confidentialToken.burn(amount, from);

        FHE.makePubliclyDecryptable(amount);
        FHE.makePubliclyDecryptable(actualBurnAmount);

        _receivers[requestId] = ReceiverEntry({
            to: to,
            callbackData: unwrapCallbackData
        });

        emit UnwrappedStarted(returnVal, requestId, nextTxId, to, amount, actualBurnAmount);

        requestId++;
        return eReturnVal;
    }

    /// @notice Completes the unwrap process using publicly decrypted values
    /// @dev This function uses the public decrypt flow where any user (typically the unwrapper)
    ///      retrieves encrypted handles from the UnwrappedStarted event, decrypts them publicly,
    ///      and calls this function with the decrypted values and proof.
    /// @param requestId The unique identifier for this unwrap request (from UnwrappedStarted event)
    /// @param handles Array of encrypted handles [expectedBurnAmount, actualBurnAmount] to verify
    /// @param clearTextsAndProof Array containing [abiEncodedClearValues, decryptionProof] from public decryption
    ///
    /// @dev Success path (actualBurnAmount > 0 && expectedBurnAmount == actualBurnAmount):
    ///      - Calculates fees in original token units (feeAmount64 * rate)
    ///      - Transfers fee to fee recipient (if fails, user receives fee + unwrap amount)
    ///      - Transfers unwrap amount to receiver (if fails, mints back cTokens)
    ///      - Calls onUnwrapFinalizedReceived if receiver is a contract
    ///
    /// @dev Failure path (actualBurnAmount == 0 || expectedBurnAmount != actualBurnAmount):
    ///      - Occurs when user attempted to unwrap 0 or burn more than their balance
    ///      - No tokens are transferred, emits failure event
    ///
    /// @dev Parity maintenance:
    ///      - Both transfers fail: Mints back full actualBurnAmount to maintain parity
    ///      - Unwrap fails, fee succeeds: Mints back principal only (protocol keeps fees)
    ///      - Fee fails, unwrap succeeds: User receives unwrapAmount + feeAmount (protocol takes hit)
    function finalizeUnwrap(
        uint256 requestId,
        euint64[] calldata handles,
        //euint64 expectedBurnAmountHandle,
        //euint64 actualBurnAmountHandle,
        bytes[] memory clearTextsAndProof
        //bytes memory cleartexts,
        //bytes memory decryptionProof
    ) external virtual {
        {
            bytes32[] memory cts = new bytes32[](2);
            cts[0] = euint64.unwrap(handles[0]);
            cts[1] = euint64.unwrap(handles[1]);
            FHE.checkSignatures(cts, clearTextsAndProof[0], clearTextsAndProof[1]);
        }

        (
            uint64 expectedBurnAmount,
            uint64 actualBurnAmount
        ) = abi.decode(clearTextsAndProof[0], (uint64, uint64));

        ReceiverEntry memory receiver = _receivers[requestId];
        require(receiver.to != address(0), ERC7984InvalidGatewayRequest(requestId));

        delete _receivers[requestId];

        uint256 nextTxId = confidentialToken.nextTxId();

        if (actualBurnAmount > 0 && expectedBurnAmount == actualBurnAmount) {
            uint256 rate = confidentialToken.rate();
            uint64 feeAmount64 = _getUnwrapFee(actualBurnAmount, receiver.to);
            uint256 feeAmount256 = feeAmount64 * rate;
            uint256 unwrapAmount = actualBurnAmount * rate - feeAmount256;
            address feeRecipient = _getFeeRecipient();

            // Transfer fee to fee recipient
            bool feeSuccess = _transferUnderlying(originalToken, feeRecipient, feeAmount256);

            if (feeSuccess == false) {
                // if fees failed, protocol takes the hit and fees are transferred to user
                // on top of unwrap amount to maintain backing token parity
                unwrapAmount += feeAmount256;
                feeAmount256 = 0;
            }

            // Transfer principal to receiver
            bool unwrapSuccess = _transferUnderlying(originalToken, receiver.to, unwrapAmount);

            if (unwrapSuccess == false) {
                unwrapAmount = 0;
                if (feeSuccess == false) {
                    // Mint everything back to user if both transfers failed
                    confidentialToken.mint(receiver.to, actualBurnAmount);
                    feeAmount256 = 0;
                } else {
                    // Mint principal back to user, protocol keeps fees, we'll handle
                    // this offchain by paying back the user if need be.
                    // Indeed, either this is a genuine problem it'll be settled offchain
                    // or the receiver does not accept tokens and the protocol fee should still be paid.
                    // This ensures token parity is always maintained.
                    // Note that should the receiver accept tokens, this should never occur.
                    confidentialToken.mint(receiver.to, actualBurnAmount - feeAmount64);
                }
            }
            emit UnwrappedFinalized(requestId, unwrapSuccess, feeSuccess, actualBurnAmount, unwrapAmount, feeAmount256, nextTxId);

            // solves stack too deep error.
            uint256 reqId = requestId;

            if (receiver.to.code.length > 0) {
                IWrapperReceiver(receiver.to).onUnwrapFinalizedReceived(msg.sender, unwrapAmount, reqId, msg.sender, receiver.callbackData);
            }
        } else {
            if (actualBurnAmount > 0) {
                confidentialToken.mint(receiver.to, actualBurnAmount);
            }
            emit UnwrappedFinalized(requestId, false, false, actualBurnAmount, 0, 0, nextTxId);
        }
    }

    function _getWrapFee(uint256 amount_, address to) private view returns (uint256) {
        FeeManager feeManager = adminProvider.feeManager();
        return feeManager.getWrapFee(amount_, msg.sender, to);
    }

    function _getUnwrapFee(uint64 amount_, address to) private view returns (uint64) {
        FeeManager feeManager = adminProvider.feeManager();
        return feeManager.getUnwrapFee(amount_, msg.sender, to);
    }

    function _getFeeRecipient() private view returns (address) {
        FeeManager feeManager = adminProvider.feeManager();
        return feeManager.getFeeRecipient();
    }

    /// @notice Internal helper to transfer underlying tokens (ETH or ERC20)
    /// @dev Abstracts the difference between ETH and ERC20 transfers
    /// @dev Uses trySafeTransfer for ERC20 to handle tokens that return false instead of reverting
    /// @param token The token address (address(0) for ETH)
    /// @param to The recipient address
    /// @param amount The amount to transfer
    /// @return success True if transfer succeeded, false otherwise
    function _transferUnderlying(address token, address to, uint256 amount) internal returns (bool success) {
        if (token == address(0)) {
            // ETH transfer
            (success, ) = to.call{value: amount}("");
        } else {
            // ERC20 transfer using trySafeTransfer
            success = IERC20(token).trySafeTransfer(to, amount);
        }
    }
}
