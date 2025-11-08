// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.22;

import { IOFT, SendParam, MessagingFee } from "@layerzerolabs/oft-evm/contracts/interfaces/IOFT.sol";
import { ERC20 } from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import { OptionsBuilder } from "@layerzerolabs/oapp-evm/contracts/oapp/libs/OptionsBuilder.sol";

/**
 * @title FeesSenderToBurner Contract
 * @dev FeesSenderToBurner is a contract that should be deployed on Gateway chain
 * @dev Anyone can call sendFeesToBurner() function
 */
contract FeesSenderToBurner {
    using OptionsBuilder for bytes;

    address public immutable ZAMA_OFT;
    address public immutable PROTOCOL_FEES_BURNER;
    uint32 public immutable DESTINATION_EID; /// @dev 40161 for Sepolia and 30101 for Ethereum mainnet

    uint256 immutable private _DECIMAL_CONVERSION_RATE;

    event FeesForwarded(uint256 amount, uint32 dstEid, address to, bytes options, uint256 nativeFeePaid);

    error NotEnoughZAMAToSend();
    error UnsupportedChainID();

    constructor(address _oft, address _protocolFeesBurner) {
        ZAMA_OFT = _oft;
        PROTOCOL_FEES_BURNER = _protocolFeesBurner;
        uint256 chainID = block.chainid;
        if (chainID == 261131) { // chainID of gateway-mainnet i.e linked to ethereum-mainnet
            DESTINATION_EID = 30101;
        } else if (chainID == 10901) { // chainID of gateway-testnet i.e linked to ethereum-testnet
            DESTINATION_EID = 40161;
        } else {
            revert UnsupportedChainID();
        }
        uint8 sharedDecimals = IOFT(_oft).sharedDecimals();
        uint8 decimals = ERC20(_oft).decimals();
        _DECIMAL_CONVERSION_RATE = 10**(decimals-sharedDecimals);
    }

    /// @notice Send all ZAMA held by this contract to the burner on the destination chain.
    /// @dev Caller must send enough native gas (ETH) to pay the LayerZero fee.
    function sendFeesToBurner() external payable {
        uint256 amount = ERC20(ZAMA_OFT).balanceOf(address(this));
        uint256 amountNormalized = (amount/(_DECIMAL_CONVERSION_RATE))*_DECIMAL_CONVERSION_RATE;

        if (amountNormalized == 0) revert NotEnoughZAMAToSend();

        bytes memory options = OptionsBuilder.newOptions();

        SendParam memory sendParam = SendParam({
            dstEid: DESTINATION_EID,
            to: bytes32(uint256(uint160(PROTOCOL_FEES_BURNER))),
            amountLD: amountNormalized,
            minAmountLD: amountNormalized,
            extraOptions: options,
            composeMsg: bytes(""),
            oftCmd: bytes("")
        });

        MessagingFee memory msgFee = MessagingFee({ nativeFee: msg.value, lzTokenFee: 0 });

        IOFT(ZAMA_OFT).send{ value: msg.value }(sendParam, msgFee, msg.sender);

        emit FeesForwarded(amount, DESTINATION_EID, PROTOCOL_FEES_BURNER, options, msg.value);
    }

    function quote() external view returns (uint256) {
        uint256 amount = ERC20(ZAMA_OFT).balanceOf(address(this));
        uint256 amountNormalized = (amount/(_DECIMAL_CONVERSION_RATE))*_DECIMAL_CONVERSION_RATE;
        if (amountNormalized == 0) revert NotEnoughZAMAToSend();

        bytes memory options = OptionsBuilder.newOptions();

        SendParam memory sendParam = SendParam({
            dstEid: DESTINATION_EID,
            to: bytes32(uint256(uint160(PROTOCOL_FEES_BURNER))),
            amountLD: amountNormalized,
            minAmountLD: amountNormalized,
            extraOptions: options,
            composeMsg: bytes(""),
            oftCmd: bytes("")
        });

        MessagingFee memory quotedFee = IOFT(ZAMA_OFT).quoteSend(sendParam, false);
        return quotedFee.nativeFee;
    }
}
