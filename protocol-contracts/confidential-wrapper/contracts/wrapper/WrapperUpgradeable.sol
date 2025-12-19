// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity 0.8.27;

import {FHESafeMath} from "openzeppelin-confidential-contracts/contracts/utils/FHESafeMath.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {RegulatedERC7984Upgradeable} from "../token/RegulatedERC7984Upgradeable.sol";
import {EthereumConfigUpgradeable} from "../fhevm/EthereumConfigUpgradeable.sol";
import {FHE, ebool, euint64, externalEuint64 } from "@fhevm/solidity/lib/FHE.sol";
import {AdminProvider} from "../admin/AdminProvider.sol";
import {FeeManager} from "../admin/FeeManager.sol";
import {IWrapperReceiver} from "../interfaces/IWrapperReceiver.sol";
import {IDeploymentCoordinator} from "../interfaces/IDeploymentCoordinator.sol";
import {SafeCast} from "@openzeppelin/contracts/utils/math/SafeCast.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";
import {AccessControlDefaultAdminRulesUpgradeable} from "@openzeppelin/contracts-upgradeable/access/extensions/AccessControlDefaultAdminRulesUpgradeable.sol";
import {ReentrancyGuardUpgradeable} from "@openzeppelin/contracts-upgradeable/utils/ReentrancyGuardUpgradeable.sol";


/// @notice Wrapper contract for a single token type, providing wrapping/unwrapping functionality
/// @dev Each wrapper handles exactly one underlying token (ERC20 or ETH) and one confidential token
/// @custom:security-contact contact@zaiffer.org
contract WrapperUpgradeable is RegulatedERC7984Upgradeable, ReentrancyGuardUpgradeable {
    using SafeERC20 for IERC20;

    /// @custom:storage-location erc7201:zaiffer.storage.Wrapper
    struct WrapperStorage {
        address _originalToken;
        IDeploymentCoordinator _deploymentCoordinator;
        uint256 _requestId;
        mapping(uint256 decryptionRequest => ReceiverEntry receiverEntry) _receivers;
        uint64 _mintedSupply;
        mapping(address holder => mapping(address operator => uint48 validUntilTimestamp)) _finalizeUnwrapOperators;
    }

    struct ReceiverEntry {
        address to;
        address refund;
        bytes callbackData;
        euint64 expectedBurnAmount;
        euint64 actualBurnAmount;
        uint64 committedFeeBasisPoints;
        address from;
    }

    struct FinalizeSuccessParams {
        uint256 requestId;
        uint64 actualBurnAmount;
        ReceiverEntry receiver;
    }

    // keccak256(abi.encode(uint256(keccak256("zaiffer.storage.Wrapper")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant WrapperStorageLocation =
        0x13479f93871f24bad5cbd972b5250a5ad213f4c22829c24117e4b54f246a4500;

    function _getWrapperStorage() internal pure returns (WrapperStorage storage $) {
        assembly {
            $.slot := WrapperStorageLocation
        }
    }

    error IncorrectEthAmount();
    error CannotReceiveEthForTokenWrap();
    error CannotSendToZeroAddress();
    error EthFeeTransferFailed();
    error ZeroAddressConfidentialToken();
    error ZeroAddressDeploymentCoordinator();
    error WrapperBalanceExceedsMaxSupply();

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
        address refund,
        euint64 requestedAmount,
        euint64 burnAmount
    );
    event FinalizeUnwrapOperatorSet(address indexed holder, address indexed operator, uint48 until);
    /// @dev The caller is not authorized to finalize this unwrap request
    error UnauthorizedFinalizeUnwrapCaller(uint256 requestId, address caller, address unwrapInitiator);

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function initialize(
        string memory name_,
        string memory symbol_,
        uint8 decimals_,
        address admin_,
        uint256 rate_,
        IDeploymentCoordinator deploymentCoordinator_,
        address originalToken_
    ) public initializer {
        RegulatedERC7984Upgradeable.initialize(
            name_,
            symbol_,
            decimals_,
            admin_,
            rate_,
            deploymentCoordinator_
        );

        require(address(deploymentCoordinator_) != address(0), ZeroAddressDeploymentCoordinator());

        __ReentrancyGuard_init();

        WrapperStorage storage $ = _getWrapperStorage();
        $._originalToken = originalToken_;
        $._deploymentCoordinator = deploymentCoordinator_;
        $._requestId = 0;
    }

    function originalToken() public view returns (address) {
        WrapperStorage storage $ = _getWrapperStorage();
        return $._originalToken;
    }

    function requestId() public view returns (uint256) {
        WrapperStorage storage $ = _getWrapperStorage();
        return $._requestId;
    }

    function getReceiverEntry(uint256 requestId_) public view returns (ReceiverEntry memory) {
        WrapperStorage storage $ = _getWrapperStorage();
        return $._receivers[requestId_];
    }

    /// @notice Wraps original tokens (ETH or ERC20) into confidential tokens
    /// @dev Handles fee-on-transfer tokens by tracking actual balances received
    /// @dev Protected against reentrancy attacks from ERC777 and other callback-enabled tokens
    /// @param to_ The recipient address for the wrapped confidential tokens
    /// @param amount_ The total amount of original tokens to wrap (including fees)
    function wrap(address to_, uint256 amount_) external payable nonReentrant {
        WrapperStorage storage $ = _getWrapperStorage();

        uint256 mintTxId = nextTxId();
        uint64 mintAmount;
        uint256 actualFeeReceived;

        uint256 baseFee = _getWrapFee(amount_, to_);
        uint256 rate = rate();
        uint256 baseAmount = amount_ - baseFee;
        uint256 wrapDust = baseAmount % rate;
        uint256 transferAmount = baseAmount - wrapDust;  // == baseAmount / rate * rate
        uint256 totalFee = amount_ - transferAmount;

        if ($._originalToken == address(0)) {
            require(msg.value == amount_, IncorrectEthAmount());
            (mintAmount, actualFeeReceived) = _processETHDeposit(transferAmount, totalFee);
        } else {
            (mintAmount, actualFeeReceived) = _processERC20Deposit(transferAmount, totalFee);
        }

        _mintWrappedTokens(to_, mintAmount);
        emit Wrapped(mintAmount, amount_, actualFeeReceived, to_, mintTxId);
    }

    function _processETHDeposit(uint256 transferAmount_, uint256 totalFee_) private returns (uint64 mintAmount, uint256 actualFeeReceived) {
        WrapperStorage storage $ = _getWrapperStorage();
        uint256 rate = rate();

        mintAmount = SafeCast.toUint64(transferAmount_ / rate);

        address feeRecipient = _getFeeRecipient();
        (bool ethTransferSuccess, ) = feeRecipient.call{value: totalFee_}("");
        if (!ethTransferSuccess) {
            revert EthFeeTransferFailed();
        }

        actualFeeReceived = totalFee_;
    }

    function _processERC20Deposit(uint256 transferAmount_, uint256 totalFee_) private returns (uint64 mintAmount, uint256 actualFeeReceived) {
        require(msg.value == 0, CannotReceiveEthForTokenWrap());

        WrapperStorage storage $ = _getWrapperStorage();
        uint256 rate = rate();
        address feeRecipient = _getFeeRecipient();

        // Transfer and track wrapper balance
        uint256 balanceBefore = IERC20($._originalToken).balanceOf(address(this));
        IERC20($._originalToken).safeTransferFrom(msg.sender, address(this), transferAmount_);
        uint256 balanceDifference = IERC20($._originalToken).balanceOf(address(this)) - balanceBefore;

        mintAmount = SafeCast.toUint64(balanceDifference / rate);

        // Track fee recipient balance to emit accurate fee amount
        uint256 feeBalBefore = IERC20($._originalToken).balanceOf(feeRecipient);

        // Transfer fee and track actual received
        IERC20($._originalToken).safeTransferFrom(msg.sender, feeRecipient, totalFee_);

        // Transfer dust
        uint256 transferDust = balanceDifference % rate;
        if (transferDust > 0) {
            IERC20($._originalToken).safeTransfer(feeRecipient, transferDust);
        }

        actualFeeReceived = IERC20($._originalToken).balanceOf(feeRecipient) - feeBalBefore;
    }

    /// @dev Unwraps tokens from `from` and sends the underlying tokens to `to`. The caller must be `from`
    /// or be an approved operator for `from`. `amount * rate()` underlying tokens are sent to `to`.
    ///
    /// NOTE: The unwrap request created by this function must be finalized by calling {finalizeUnwrap}.
    /// NOTE: The caller *must* already be approved by ACL for the given `amount`.
    ///
    function unwrap(address from, address to, euint64 amount) public virtual {
        require(FHE.isAllowed(amount, msg.sender), ERC7984UnauthorizedUseOfEncryptedAmount(amount, msg.sender));
        _unwrap(from, to, amount, to, new bytes(0));
    }

    /// @dev Variant of {unwrap} that passes an `inputProof` which approves the caller for the `encryptedAmount`
    /// in the ACL.
    ///
    function unwrap(
        address from,
        address to,
        externalEuint64 encryptedAmount,
        bytes calldata inputProof
    ) public virtual {
        _unwrap(from, to, FHE.fromExternal(encryptedAmount, inputProof), to, new bytes(0));
    }

    /// @dev Variant of {unwrap}. `refund` is used to send back unwrapped tokens to in case of unwrap failure.
    ///
    /// NOTE: The unwrap request created by this function must be finalized by calling {finalizeUnwrap}.
    /// NOTE: The caller *must* already be approved by ACL for the given `amount`.
    ///
    function unwrapWithRefund(address from, address to, euint64 amount, address refund) public virtual {
        require(FHE.isAllowed(amount, msg.sender), ERC7984UnauthorizedUseOfEncryptedAmount(amount, msg.sender));
        _unwrap(from, to, amount, refund, new bytes(0));
    }

    /// @dev Variant of {unwrapWithRefund} that passes an `inputProof` which approves the caller for the `encryptedAmount`
    /// in the ACL.
    ///
    function unwrapWithRefund(
        address from,
        address to,
        externalEuint64 encryptedAmount,
        bytes calldata inputProof,
        address refund
    ) public virtual {
        _unwrap(from, to, FHE.fromExternal(encryptedAmount, inputProof), refund, new bytes(0));
    }

    /// @dev Variant of {unwrap}. if `to` has code, `IWrapperReceiver.onUnwrapFinalizedReceived` will be called.
    /// `refund` is used to send back unwrapped tokens to in case the callback fails.
    ///
    /// NOTE: The unwrap request created by this function must be finalized by calling {finalizeUnwrap}.
    /// NOTE: The caller *must* already be approved by ACL for the given `amount`.
    ///
    function unwrapAndCall(address from, address to, euint64 amount, address refund, bytes memory unwrapCallbackData) public virtual {
        require(FHE.isAllowed(amount, msg.sender), ERC7984UnauthorizedUseOfEncryptedAmount(amount, msg.sender));
        _unwrap(from, to, amount, refund, unwrapCallbackData);
    }

    /// @dev Variant of {unwrapAndCall} that passes an `inputProof` which approves the caller for the `encryptedAmount`
    /// in the ACL.
    ///
    function unwrapAndCall(
        address from,
        address to,
        externalEuint64 encryptedAmount,
        bytes calldata inputProof,
        address refund,
        bytes memory unwrapCallbackData
    ) public virtual {
        _unwrap(from, to, FHE.fromExternal(encryptedAmount, inputProof), refund, unwrapCallbackData);
    }

    function _unwrap(
        address from,
        address to,
        euint64 amount,
        address refund,
        bytes memory unwrapCallbackData
    ) internal returns (ebool) {
        WrapperStorage storage $ = _getWrapperStorage();

        require(FHE.isSenderAllowed(amount), SenderNotAllowed());

        ebool eReturnVal = FHE.asEbool(true);
        FHE.allowTransient(eReturnVal, msg.sender);

        require(to != address(0), CannotSendToZeroAddress());
        require(refund != address(0), CannotSendToZeroAddress());

        _processUnwrap(from, to, refund, amount, unwrapCallbackData);

        return eReturnVal;
    }

    function _processUnwrap(
        address from,
        address to,
        address refund,
        euint64 amount,
        bytes memory unwrapCallbackData
    ) private {
        WrapperStorage storage $ = _getWrapperStorage();

        uint256 txId = nextTxId();
        euint64 actualBurnAmount = _burn(from, amount);

        FHE.makePubliclyDecryptable(amount);
        FHE.makePubliclyDecryptable(actualBurnAmount);

        uint256 requestId = $._requestId;
        $._receivers[requestId] = ReceiverEntry({
            to: to,
            refund: refund,
            callbackData: unwrapCallbackData,
            expectedBurnAmount: amount,
            actualBurnAmount: actualBurnAmount,
            committedFeeBasisPoints: _getUnwrapFeeBasisPoints(to),
            from: from
        });

        emit UnwrappedStarted(true, requestId, txId, to, refund, amount, actualBurnAmount);

        $._requestId++;
    }

    /// @notice Completes the unwrap process using publicly decrypted values
    /// @dev This function uses the public decrypt flow where any user (typically the unwrapper)
    ///      retrieves encrypted handles from the UnwrappedStarted event, decrypts them publicly,
    ///      and calls this function with the decrypted values and proof.
    /// @param requestId The unique identifier for this unwrap request (from UnwrappedStarted event)
    /// @param abiEncodedClearBurnAmounts The ABI-encoded clear values (uint64, uint64) associated to the `decryptionProof`.
    /// @param decryptionProof The proof that validates the decryption.
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
        bytes memory abiEncodedClearBurnAmounts,
        bytes memory decryptionProof
    ) external virtual returns (bool) {
        WrapperStorage storage $ = _getWrapperStorage();

        ReceiverEntry memory receiver = $._receivers[requestId];
        require(receiver.to != address(0), ERC7984InvalidGatewayRequest(requestId));

        // Permission check - only unwrap initiator or authorized operator can finalize
        require(
            isFinalizeUnwrapOperator(receiver.from, msg.sender),
            UnauthorizedFinalizeUnwrapCaller(requestId, msg.sender, receiver.from)
        );

        bytes32[] memory cts = new bytes32[](2);
        cts[0] = FHE.toBytes32(receiver.expectedBurnAmount);
        cts[1] = FHE.toBytes32(receiver.actualBurnAmount);

        FHE.checkSignatures(cts, abiEncodedClearBurnAmounts, decryptionProof);

        (uint64 expectedBurnAmount, uint64 actualBurnAmount) = abi.decode(
            abiEncodedClearBurnAmounts,
            (uint64, uint64)
        );

        delete $._receivers[requestId];

        // Decrement burned supply in onConfidentialTransferReceived
        $._mintedSupply -= actualBurnAmount;

        if (actualBurnAmount > 0 && expectedBurnAmount == actualBurnAmount) {
            return _finalizeSuccessfulUnwrap(FinalizeSuccessParams(requestId, actualBurnAmount, receiver));
        } else {
            _finalizeFailedUnwrap(requestId, actualBurnAmount, receiver);
            return false;
        }
    }

    function _finalizeSuccessfulUnwrap(
        FinalizeSuccessParams memory params
    ) private returns (bool) {
        WrapperStorage storage $ = _getWrapperStorage();

        uint256 rate = rate();
        uint64 feeAmount64 = _getUnwrapFee(params.actualBurnAmount, params.receiver.committedFeeBasisPoints);
        uint256 feeAmount256 = feeAmount64 * rate;
        uint256 unwrapAmount = params.actualBurnAmount * rate - feeAmount256;
        address feeRecipient = _getFeeRecipient();

        // Transfer fee to fee recipient
        bool feeSuccess = _transferUnderlying($._originalToken, feeRecipient, feeAmount256);

        if (feeSuccess == false) {
            // if fees failed, protocol takes the hit and fees are transferred to user
            // on top of unwrap amount to maintain backing token parity
            unwrapAmount += feeAmount256;
            feeAmount256 = 0;
        }

        // Transfer principal to receiver
        bool unwrapSuccess = _transferUnderlying($._originalToken, params.receiver.to, unwrapAmount);

        // Reimbursement txId if unwrapSuccess is false
        uint256 mintTxId = nextTxId();

        if (unwrapSuccess == false) {
            unwrapAmount = 0;
            if (feeSuccess == false) {
                // Mint everything back to user if both transfers failed
                _mintWrappedTokens(params.receiver.refund, params.actualBurnAmount);
                feeAmount256 = 0;
            } else {
                // Mint principal back to user, protocol keeps fees, we'll handle
                // this offchain by paying back the user if need be.
                // Indeed, either this is a genuine problem it'll be settled offchain
                // or the receiver does not accept tokens and the protocol fee should still be paid.
                // This ensures token parity is always maintained.
                // Note that should the receiver accept tokens, this should never occur.
                uint64 reimbursementAmount = params.actualBurnAmount - feeAmount64;
                _mintWrappedTokens(params.receiver.refund, reimbursementAmount);
            }
        }

        emit UnwrappedFinalized(
            params.requestId,
            unwrapSuccess,
            feeSuccess,
            params.actualBurnAmount,
            unwrapAmount,
            feeAmount256,
            mintTxId
        );

        return _executeWrapperReceiverCallback(params.receiver, unwrapAmount, params.requestId);
    }

    function _executeWrapperReceiverCallback(ReceiverEntry memory receiver, uint256 unwrapAmount, uint256 requestId) internal returns (bool) {
        if (receiver.to.code.length > 0 && receiver.callbackData.length > 0) {
            return IWrapperReceiver(receiver.to).onUnwrapFinalizedReceived(msg.sender, unwrapAmount, requestId, receiver.refund, receiver.callbackData);
        }
        return true;
    }

    function _finalizeFailedUnwrap(
        uint256 requestId,
        uint64 actualBurnAmount,
        ReceiverEntry memory receiver
    ) private {
        WrapperStorage storage $ = _getWrapperStorage();

        // Reimbursement txId if actualBurnAmount > 0
        uint256 mintTxId = nextTxId();

        if (actualBurnAmount > 0) {
            _mintWrappedTokens(receiver.to, actualBurnAmount);
        }
        emit UnwrappedFinalized(requestId, false, false, actualBurnAmount, 0, 0, mintTxId);
    }

    function _mintWrappedTokens(address to_, uint64 amount_) private {
        WrapperStorage storage $ = _getWrapperStorage();

        // Safety check: Verify minted supply won't overflow euint64
        require(uint256($._mintedSupply) + uint256(amount_) <= type(uint64).max, WrapperBalanceExceedsMaxSupply());

        _mint(to_, FHE.asEuint64(amount_));
        $._mintedSupply += amount_;
    }

    function mintedSupply() public view returns (uint64) {
        WrapperStorage storage $ = _getWrapperStorage();
        return $._mintedSupply;
    }

    /// @notice Checks if an address is authorized to finalize unwraps on behalf of a holder
    /// @param holder The address that initiated unwraps
    /// @param operator The address to check operator status for
    /// @return True if operator is authorized (either is the holder or has valid operator permission)
    function isFinalizeUnwrapOperator(address holder, address operator) public view virtual returns (bool) {
        WrapperStorage storage $ = _getWrapperStorage();
        return holder == operator || block.timestamp <= $._finalizeUnwrapOperators[holder][operator];
    }

    /// @notice Sets an operator that can finalize unwraps on behalf of msg.sender
    /// @param operator The address to grant operator permissions to
    /// @param until Timestamp until which the operator permission is valid (uint48)
    function setFinalizeUnwrapOperator(address operator, uint48 until) public virtual {
        _setFinalizeUnwrapOperator(msg.sender, operator, until);
    }

    /// @dev Internal function to set operator with event emission
    function _setFinalizeUnwrapOperator(address holder, address operator, uint48 until) internal virtual {
        WrapperStorage storage $ = _getWrapperStorage();
        $._finalizeUnwrapOperators[holder][operator] = until;
        emit FinalizeUnwrapOperatorSet(holder, operator, until);
    }

    function _getWrapFee(uint256 amount_, address to) private view returns (uint256) {
        FeeManager feeManager = adminProvider().feeManager();
        return feeManager.getWrapFee(amount_, msg.sender, to);
    }

    function _getUnwrapFeeBasisPoints(address to) private view returns (uint64) {
        FeeManager feeManager = adminProvider().feeManager();
        return feeManager.getUnwrapFeeBasisPoints(msg.sender, to);
    }

    function _getUnwrapFee(uint64 amount_, uint64 basisPoints) private view returns (uint64) {
        FeeManager feeManager = adminProvider().feeManager();
        return feeManager.getFee(amount_, basisPoints);
    }

    function _getFeeRecipient() private view returns (address) {
        FeeManager feeManager = adminProvider().feeManager();
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
