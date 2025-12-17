// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity 0.8.27;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

/// @notice Mock ERC20 token that charges a fee on every transfer
/// @dev Used for testing wrapper behavior with fee-on-transfer tokens
contract FeeOnTransferERC20 is ERC20 {
    uint8 public _decimals;
    uint256 public transferFeeBasisPoints; // 10_000 == 100%
    address public feeCollector;

    constructor(
        string memory name_,
        string memory symbol_,
        uint8 decimals_,
        uint256 transferFeeBasisPoints_
    ) ERC20(name_, symbol_) {
        _decimals = decimals_;
        transferFeeBasisPoints = transferFeeBasisPoints_;
        feeCollector = msg.sender;
    }

    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }

    function decimals() public view override returns (uint8) {
        return _decimals;
    }

    function setTransferFeeBasisPoints(uint256 feeBasisPoints) external {
        require(feeBasisPoints <= 10_000, "Fee too high");
        transferFeeBasisPoints = feeBasisPoints;
    }

    function setFeeCollector(address collector) external {
        feeCollector = collector;
    }

    function _update(address from, address to, uint256 value) internal virtual override {
        if (from == address(0) || to == address(0)) {
            // No fee on mint/burn
            super._update(from, to, value);
            return;
        }

        // Calculate fee
        uint256 fee = (value * transferFeeBasisPoints) / 10_000;
        uint256 amountAfterFee = value - fee;

        // Transfer the amount minus fee
        super._update(from, to, amountAfterFee);

        // Transfer fee to collector
        if (fee > 0) {
            super._update(from, feeCollector, fee);
        }
    }
}
