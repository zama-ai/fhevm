// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity 0.8.27;

import {ZamaEthereumConfig} from "@fhevm/solidity/config/ZamaConfig.sol";
import {FHE, ebool, euint64} from "@fhevm/solidity/lib/FHE.sol";
import {IERC7984Receiver} from "openzeppelin-confidential-contracts/contracts/interfaces/IERC7984Receiver.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {Ownable2Step, Ownable} from "@openzeppelin/contracts/access/Ownable2Step.sol";
import {IUniswapV2Router02} from "../interfaces/IUniswap.sol";

import {IWrapperReceiver} from "../interfaces/IWrapperReceiver.sol";
import {DeploymentCoordinator} from "../factory/DeploymentCoordinator.sol";
import {RegulatedERC7984Upgradeable} from "../token/RegulatedERC7984Upgradeable.sol";
import {WrapperUpgradeable} from "../wrapper/WrapperUpgradeable.sol";


struct SwapData {
    address routerAddress;
    uint256 amountOutMin;
    address[] path;
    uint256 deadline;
    address to;
}


/// @title SwapV0
/// @notice Enables atomic swaps between confidential tokens via Uniswap V2
/// @dev Implements IWrapperReceiver to receive unwrapped tokens and execute swaps
/// @dev Flow: User initiates confidentialTransferAndCall → unwrap → swap → wrap → refund on failure
/// @dev Key features:
///      - Validates swap paths before execution
///      - Automatic refund to confidential tokens on any failure
///      - Only accepts unwraps from DeploymentCoordinator-registered wrappers
///      - Uses forceApprove for USDT-like token compatibility
///      - Router whitelisting managed by SwapV0 owner (Ownable2Step)
/// @custom:security-contact contact@zaiffer.org
contract SwapV0 is IWrapperReceiver, Ownable2Step {
    using SafeERC20 for IERC20;

    /// @dev DeploymentCoordinator for validating wrapper registrations
    DeploymentCoordinator public coordinator;

    /// @dev Mapping of whitelisted router addresses (address => bool)
    mapping(address => bool) public whitelistedRouters;

    /// @dev Mapping of whitelisted token addresses for swap paths (address => bool)
    mapping(address => bool) public whitelistedTokens;

    /// @dev Error code: Path[0] must match confidential token's underlying token
    bytes public constant INPUT_PATH_IS_NOT_UNDERLYING = abi.encodePacked(keccak256("INPUT_PATH_IS_NOT_UNDERLYING"));

    /// @dev Error code: Input token must have a registered wrapper
    bytes public constant INPUT_PATH_HAS_NO_WRAPPER = abi.encodePacked(keccak256("INPUT_PATH_HAS_NO_WRAPPER"));

    /// @dev Error code: Output token must have a registered wrapper
    bytes public constant OUTPUT_PATH_HAS_NO_WRAPPER = abi.encodePacked(keccak256("OUTPUT_PATH_HAS_NO_WRAPPER"));

    /// @dev Error code: Recipient address cannot be zero address
    bytes public constant RECIPIENT_CANNOT_BE_ZERO_ADDRESS = abi.encodePacked(keccak256("RECIPIENT_CANNOT_BE_ZERO_ADDRESS"));

    /// @dev Error code: Router address is not whitelisted
    bytes public constant ROUTER_NOT_WHITELISTED = abi.encodePacked(keccak256("ROUTER_NOT_WHITELISTED"));

    /// @dev Error code: Token in swap path is not whitelisted
    bytes public constant TOKEN_NOT_WHITELISTED = abi.encodePacked(keccak256("TOKEN_NOT_WHITELISTED"));

    /// @dev Error code: Recipient is sanctioned
    bytes public constant RECIPIENT_IS_SANCTIONED = abi.encodePacked(keccak256("RECIPIENT_IS_SANCTIONED"));

    /// @dev Error code: Refund address is sanctioned
    bytes public constant REFUND_ADDRESS_IS_SANCTIONED = abi.encodePacked(keccak256("REFUND_ADDRESS_IS_SANCTIONED"));

    error UnknownWrapper();
    error ZeroAddressRouter();
    error RouterAlreadyWhitelisted();
    error RouterNotWhitelisted();
    error ZeroAddressToken();
    error TokenAlreadyWhitelisted();
    error TokenNotWhitelisted();
    error ZeroAddressRecipient();
    error TokenTransferFailed();

    /// @notice Emitted after swap attempt (successful or failed)
    /// @param success True if swap succeeded, false if refunded
    /// @param path Uniswap swap path (token addresses)
    /// @param unwrapRequestId Original unwrap request ID from wrapper
    /// @param wrapTxId Transaction ID of output token wrap (0 if failed)
    /// @param errorReasonString Error string if swap failed
    /// @param errorLowLevelData Low-level error data if swap failed
    event Swap(
        bool indexed success,
        address[] path,
        uint256 indexed unwrapRequestId,
        uint256 wrapTxId,
        string errorReasonString,
        bytes errorLowLevelData
    );

    /// @notice Emitted when router is added to whitelist
    /// @param router Router address added to whitelist
    event RouterAddedToWhitelist(address indexed router);

    /// @notice Emitted when router is removed from whitelist
    /// @param router Router address removed from whitelist
    event RouterRemovedFromWhitelist(address indexed router);

    /// @notice Emitted when token is added to whitelist
    /// @param token Token address added to whitelist
    event TokenAddedToWhitelist(address indexed token);

    /// @notice Emitted when token is removed from whitelist
    /// @param token Token address removed from whitelist
    event TokenRemovedFromWhitelist(address indexed token);

    /// @notice Emitted when tokens are rescued by owner
    /// @param token Token address rescued (address(0) for ETH)
    /// @param to Recipient address
    /// @param amount Amount rescued
    event TokensRescued(address indexed token, address indexed to, uint256 amount);

    /// @notice Constructs SwapV0 with deployment coordinator reference
    /// @param coordinator_ DeploymentCoordinator for wrapper validation
    constructor(DeploymentCoordinator coordinator_) Ownable(msg.sender) {
        coordinator = coordinator_;
    }

    /// @notice Receives ETH from wrapper unwraps and Uniswap swaps
    /// @dev Required to support ETH swaps (cETH → token or token → cETH)
    receive() external payable {}

    /// @notice Adds a router address to the whitelist
    /// @param router Router address to whitelist
    /// @dev Only callable by owner
    /// @dev Reverts if router is zero address
    /// @dev Reverts if router is already whitelisted
    /// @dev Emits RouterAddedToWhitelist event
    function addRouterToWhitelist(address router) external onlyOwner {
        require(router != address(0), ZeroAddressRouter());
        require(!whitelistedRouters[router], RouterAlreadyWhitelisted());
        whitelistedRouters[router] = true;
        emit RouterAddedToWhitelist(router);
    }

    /// @notice Removes a router address from the whitelist
    /// @param router Router address to remove
    /// @dev Only callable by owner
    /// @dev Reverts if router is zero address
    /// @dev Reverts if router is not currently whitelisted
    /// @dev Emits RouterRemovedFromWhitelist event
    function removeRouterFromWhitelist(address router) external onlyOwner {
        require(router != address(0), ZeroAddressRouter());
        require(whitelistedRouters[router], RouterNotWhitelisted());
        whitelistedRouters[router] = false;
        emit RouterRemovedFromWhitelist(router);
    }

    /// @notice Adds a token address to the whitelist
    /// @param token Token address to whitelist
    /// @dev Only callable by owner
    /// @dev Reverts if token is zero address
    /// @dev Reverts if token is already whitelisted
    /// @dev Emits TokenAddedToWhitelist event
    function addTokenToWhitelist(address token) external onlyOwner {
        require(token != address(0), ZeroAddressToken());
        require(!whitelistedTokens[token], TokenAlreadyWhitelisted());
        whitelistedTokens[token] = true;
        emit TokenAddedToWhitelist(token);
    }

    /// @notice Removes a token address from the whitelist
    /// @param token Token address to remove
    /// @dev Only callable by owner
    /// @dev Reverts if token is zero address
    /// @dev Reverts if token is not currently whitelisted
    /// @dev Emits TokenRemovedFromWhitelist event
    function removeTokenFromWhitelist(address token) external onlyOwner {
        require(token != address(0), ZeroAddressToken());
        require(whitelistedTokens[token], TokenNotWhitelisted());
        whitelistedTokens[token] = false;
        emit TokenRemovedFromWhitelist(token);
    }

    /// @notice Rescues stuck tokens from the contract
    /// @param token Token address to rescue (address(0) for ETH)
    /// @param to Recipient address
    /// @param amount Amount to rescue
    /// @dev Only callable by owner (governance)
    /// @dev Reverts if recipient is zero address
    /// @dev Reverts if token transfer fails
    /// @dev Emits TokensRescued event
    /// @dev This function is a safety mechanism to recover tokens that may get stuck in the contract
    ///      due to failed swaps or other unforeseen circumstances
    function rescueTokens(address token, address to, uint256 amount) external onlyOwner {
        require(to != address(0), ZeroAddressRecipient());

        if (token == address(0)) {
            // Rescue ETH
            (bool success, ) = to.call{value: amount}("");
            require(success, TokenTransferFailed());
        } else {
            // Rescue ERC20 tokens
            IERC20(token).safeTransfer(to, amount);
        }

        emit TokensRescued(token, to, amount);
    }

    /// @notice Validates a swap path for execution
    /// @param wrapper The wrapper that initiated the unwrap
    /// @param router Uniswap router address for path verification
    /// @param path Array of token addresses defining the swap route
    /// @return success True if path is valid, false otherwise
    /// @return errorReasonString Error string if validation failed
    /// @return errorLowLevelData Low-level error data if validation failed
    /// @dev Performs six critical validations:
    ///      1. Router whitelist: whitelistedRouters[router] == true
    ///      2. Token whitelist: all tokens in path must be whitelisted
    ///      3. Underlying token match: For ETH, wrapper.originalToken() == address(0) and path[0] == WETH
    ///      4. Input wrapper exists: coordinator.deployedWrappers(underlyingOrWeth) != address(0)
    ///      5. Output wrapper exists: coordinator.deployedWrappers(outputUnderlyingOrWeth) != address(0)
    ///      6. Wrapper consistency: wrapper == coordinator.deployedWrappers(underlyingAddress)
    function checkPath(WrapperUpgradeable wrapper, address router, address[] memory path) public view returns (bool, string memory, bytes memory) {
        address underlyingAddress = wrapper.originalToken();
        WrapperUpgradeable wrapperIn = coordinator.deployedWrappers(underlyingAddress);

        // check if router is whitelisted
        if (!whitelistedRouters[router]) {
            return (false, "", ROUTER_NOT_WHITELISTED);
        }

        // check if all tokens in the path are whitelisted
        for (uint256 i = 0; i < path.length; i++) {
            if (!whitelistedTokens[path[i]]) {
                return (false, "", TOKEN_NOT_WHITELISTED);
            }
        }

        address weth = IUniswapV2Router02(router).WETH();

        // For ETH swaps, the underlying is address(0) but Uniswap path uses WETH
        // Input validation: ETH → Token swap
        if (underlyingAddress == address(0)) {
            // ETH input: path[0] must be WETH
            if (path[0] != weth) {
                return (false, "", INPUT_PATH_IS_NOT_UNDERLYING);
            }
        } else {
            // ERC20 input: path[0] must match underlying exactly
            if (underlyingAddress != path[0]) {
                return (false, "", INPUT_PATH_IS_NOT_UNDERLYING);
            }
        }

        // does a wrapper exist for input token with the configured coordinator
        // this ensures that only unwrapping from coordinator wrappers can interact with this contract
        if (address(wrapperIn) == address(0)) {
            return (false, "", INPUT_PATH_HAS_NO_WRAPPER);
        }

        // Output validation: check if wrapper exists for output token
        // For Token → ETH swaps, path[last] is WETH but we need wrapper for address(0)
        address outputToken = path[path.length - 1];
        address outputUnderlying = (outputToken == weth) ? address(0) : outputToken;

        WrapperUpgradeable wrapperOut = coordinator.deployedWrappers(outputUnderlying);
        if (address(wrapperOut) == address(0)) {
            return (false, "", OUTPUT_PATH_HAS_NO_WRAPPER);
        }

        return (true, "", new bytes(0));
    }

    /// @notice Validates unwrap inputs and refunds user if path is invalid
    /// @param swapData Uniswap swap data
    /// @param amountIn Amount of input tokens unwrapped
    /// @param refundTo Address to refund to should the inputs be invalid
    /// @param unwrapRequestId Unwrap request ID for event tracking
    /// @return True if validation passed, false if refund was issued
    /// @dev Internal helper to validate path and automatically refund on failure
    function _validateUnwrapFinalizedReceivedInputsOrRefund(
        SwapData memory swapData,
        uint256 amountIn,
        address refundTo,
        uint256 unwrapRequestId
    ) internal returns (bool) {
        if(coordinator.adminProvider().sanctionsList().isSanctioned(swapData.to)) {
            if(coordinator.adminProvider().sanctionsList().isSanctioned(refundTo)) {
                address feeRecipient = address(coordinator.adminProvider().feeManager().feeRecipient());
                _refundUser(swapData.routerAddress, swapData.path, amountIn, feeRecipient, unwrapRequestId, "", REFUND_ADDRESS_IS_SANCTIONED);
            } else {
                _refundUser(swapData.routerAddress, swapData.path, amountIn, refundTo, unwrapRequestId, "", RECIPIENT_IS_SANCTIONED);
            }
            return false;
        }

        // Validate recipient address is not zero address
        // This prevents funds from being stuck when wrap() would revert on minting to address(0)
        if (swapData.to == address(0)) {
            _refundUser(swapData.routerAddress, swapData.path, amountIn, refundTo, unwrapRequestId, "", RECIPIENT_CANNOT_BE_ZERO_ADDRESS);
            return false;
        }

        (
            bool isValidPath,
            string memory errorString,
            bytes memory errorLowLevelData
        ) = checkPath(WrapperUpgradeable(payable(msg.sender)), swapData.routerAddress, swapData.path);

        if (!isValidPath) {
            _refundUser(swapData.routerAddress, swapData.path, amountIn, refundTo, unwrapRequestId, errorString, errorLowLevelData);
            return false;
        }

        return true;
    }

    /// @notice Callback invoked by wrapper when unwrap is finalized
    /// @param amountIn Amount of original tokens received from unwrap
    /// @param unwrapRequestId Unwrap request ID for tracking
    /// @param data ABI-encoded swap parameters: (router, amountOutMin, path, deadline, to)
    /// @return ebool Success indicator (encrypted bool) for wrapper to track swap status
    /// @dev Implements IWrapperReceiver interface
    /// @dev Flow on success: Validate path → Approve router → Swap on Uniswap → Wrap output → Emit Swap event
    /// @dev Flow on failure: Validate path → Refund by wrapping input tokens back → Emit Swap event with error
    /// @dev Supports three swap types:
    ///      1. Token → Token: swapExactTokensForTokens
    ///      2. ETH → Token: swapExactETHForTokens (path[0] = WETH, receives ETH)
    ///      3. Token → ETH: swapExactTokensForETH (path[last] = WETH, outputs ETH)
    /// @dev Uses forceApprove for USDT-like token compatibility
    /// @dev Returns encrypted bool to allow wrapper to track success/failure on-chain
    function onUnwrapFinalizedReceived(
        address /* operator */,
        uint256 amountIn,
        uint256 unwrapRequestId,
        address refundTo,
        bytes calldata data
    ) external returns (bool) {
        WrapperUpgradeable wrapper = WrapperUpgradeable(payable(msg.sender));
        address underlyingAddress = wrapper.originalToken();
        WrapperUpgradeable wrapperIn = coordinator.deployedWrappers(underlyingAddress);

        // is the wrapper that called onUnwrapFinalizedReceived the same as the one that's
        // associated with the underlying at the coordinator level.
        require(address(wrapper) == address(wrapperIn), UnknownWrapper());

        SwapData memory swapData = abi.decode(data, (SwapData));
        bool returnVal = true;

        if (!_validateUnwrapFinalizedReceivedInputsOrRefund(swapData, amountIn, refundTo, unwrapRequestId)) {
            return false;
        }

        returnVal = _executeSwap(swapData, amountIn, refundTo, unwrapRequestId);

        return returnVal;
    }

    /// @notice Executes the appropriate Uniswap swap based on input/output types
    /// @param swapData Swap parameters
    /// @param amountIn Amount of input tokens
    /// @param refundTo Address to refund on failure
    /// @param unwrapRequestId Unwrap request ID
    /// @return bool True if swap succeeded, false if failed
    /// @dev Internal helper to execute ETH or ERC20 swaps
    function _executeSwap(
        SwapData memory swapData,
        uint256 amountIn,
        address refundTo,
        uint256 unwrapRequestId
    ) internal returns (bool) {
        address weth = IUniswapV2Router02(swapData.routerAddress).WETH();

        // ETH → Token swap
        if (swapData.path[0] == weth && address(this).balance >= amountIn) {
            try IUniswapV2Router02(swapData.routerAddress).swapExactETHForTokens{value: amountIn}(
                swapData.amountOutMin,
                swapData.path,
                address(this),
                swapData.deadline
            ) returns (uint256[] memory amounts) {
                _handleUniswapSuccess(swapData.routerAddress, swapData.path, amounts, swapData.to, refundTo, unwrapRequestId);
                return true;
            } catch Error(string memory reason) {
                return _refundUser(swapData.routerAddress, swapData.path, amountIn, refundTo, unwrapRequestId, reason, new bytes(0));
            } catch (bytes memory lowLevelData) {
                return _refundUser(swapData.routerAddress, swapData.path, amountIn, refundTo, unwrapRequestId, "", lowLevelData);
            }
        }

        // Token → ETH swap
        if (swapData.path[swapData.path.length - 1] == weth) {
            IERC20(swapData.path[0]).forceApprove(swapData.routerAddress, amountIn);
            try IUniswapV2Router02(swapData.routerAddress).swapExactTokensForETH(
                amountIn,
                swapData.amountOutMin,
                swapData.path,
                address(this),
                swapData.deadline
            ) returns (uint256[] memory amounts) {
                _handleUniswapSuccess(swapData.routerAddress, swapData.path, amounts, swapData.to, refundTo, unwrapRequestId);
                return true;
            } catch Error(string memory reason) {
                return _refundUser(swapData.routerAddress, swapData.path, amountIn, refundTo, unwrapRequestId, reason, new bytes(0));
            } catch (bytes memory lowLevelData) {
                return _refundUser(swapData.routerAddress, swapData.path, amountIn, refundTo, unwrapRequestId, "", lowLevelData);
            }
        }

        // Token → Token swap
        IERC20(swapData.path[0]).forceApprove(swapData.routerAddress, amountIn);
        try IUniswapV2Router02(swapData.routerAddress).swapExactTokensForTokens(
            amountIn,
            swapData.amountOutMin,
            swapData.path,
            address(this),
            swapData.deadline
        ) returns (uint256[] memory amounts) {
            _handleUniswapSuccess(swapData.routerAddress, swapData.path, amounts, swapData.to, refundTo, unwrapRequestId);
            return true;
        } catch Error(string memory reason) {
            return _refundUser(swapData.routerAddress, swapData.path, amountIn, refundTo, unwrapRequestId, reason, new bytes(0));
        } catch (bytes memory lowLevelData) {
            return _refundUser(swapData.routerAddress, swapData.path, amountIn, refundTo, unwrapRequestId, "", lowLevelData);
        }
    }

    /// @notice Handles successful Uniswap swap by wrapping output tokens
    /// @param router Uniswap router address (for WETH lookup)
    /// @param path Swap path array
    /// @param amounts Output amounts from Uniswap swap
    /// @param to Recipient address for wrapped output tokens
    /// @param unwrapRequestId Original unwrap request ID
    /// @dev Internal helper called when Uniswap swap succeeds
    /// @dev For ERC20 output: Approves output wrapper → Wraps output tokens
    /// @dev For ETH output: Sends ETH value to wrapper.wrap()
    /// @dev Emits success Swap event
    function _handleUniswapSuccess(address router, address[] memory path, uint256[] memory amounts, address to, address refundTo, uint256 unwrapRequestId) private {
        address tokenOut = path[path.length - 1];
        uint256 amountOut = amounts[amounts.length - 1];
        address weth = IUniswapV2Router02(router).WETH();

        // Determine the actual underlying token (ETH is address(0), not WETH)
        address outputUnderlying = (tokenOut == weth) ? address(0) : tokenOut;
        WrapperUpgradeable wrapperOut = coordinator.deployedWrappers(outputUnderlying);

        RegulatedERC7984Upgradeable cTokenOut = wrapperOut.confidentialToken();
        uint256 wrapTxId = cTokenOut.nextTxId();

        if (tokenOut == weth) {
            // ETH output: send ETH value to wrapper
            wrapperOut.wrap{value: amountOut}(to, amountOut);
        } else {
            // ERC20 output: approve and wrap
            IERC20(tokenOut).forceApprove(address(wrapperOut), amountOut);
            wrapperOut.wrap(to, amountOut);
        }

        emit Swap(true, path, unwrapRequestId, wrapTxId, "", new bytes(0));
    }

    /// @notice Refunds user by re-wrapping input tokens when swap fails
    /// @param router Uniswap router address (to reset approval and get WETH address)
    /// @param path Swap path array
    /// @param amountIn Amount of input tokens expected (may differ from actual balance for FOT tokens)
    /// @param to Recipient address for refunded confidential tokens
    /// @param unwrapRequestId Original unwrap request ID
    /// @param errorReasonString Error string from failed operation
    /// @param errorLowLevelData Low-level error data from failed operation
    /// @return ebool Encrypted false to indicate failure to wrapper
    /// @dev Internal helper called on any swap failure (path validation or Uniswap execution)
    /// @dev For ERC20: Resets router approval → Approves wrapper → Wraps tokens back
    /// @dev For ETH: Sends ETH value to wrapper.wrap()
    /// @dev For fee-on-transfer tokens, uses actual balance instead of amountIn to prevent revert
    /// @dev Emits failure Swap event
    function _refundUser(
        address router,
        address[] memory path,
        uint256 amountIn,
        address to,
        uint256 unwrapRequestId,
        string memory errorReasonString,
        bytes memory errorLowLevelData
    ) internal returns (bool) {
        WrapperUpgradeable wrapperIn = WrapperUpgradeable(payable(msg.sender));

        uint256 wrapTxId = wrapperIn.confidentialToken().nextTxId();

        // ETH refund
        if (wrapperIn.originalToken() == address(0)) {
            uint256 refundAmount = address(this).balance < amountIn ? address(this).balance : amountIn;
            wrapperIn.wrap{value: refundAmount}(to, refundAmount);
        } else {
            // ERC20 refund: handle fee-on-transfer tokens
            uint256 balance = IERC20(wrapperIn.originalToken()).balanceOf(address(this));
            uint256 refundAmount = balance < amountIn ? balance : amountIn;
            IERC20(wrapperIn.originalToken()).forceApprove(router, 0);
            IERC20(wrapperIn.originalToken()).forceApprove(address(wrapperIn), refundAmount);
            wrapperIn.wrap(to, refundAmount);
        }

        emit Swap(false, path, unwrapRequestId, wrapTxId, errorReasonString, errorLowLevelData);

        return false;
    }
}
