// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

contract ProtocolPaymentMock {
    event InitializeProtocolPayment(
        uint256 inputVerificationPrice,
        uint256 publicDecryptionPrice,
        uint256 userDecryptionPrice
    );

    event NewInputVerificationPrice(uint256 price);

    event NewPublicDecryptionPrice(uint256 price);

    event NewUserDecryptionPrice(uint256 price);

    function initializeFromEmptyProxy(
        uint256 initialInputVerificationPrice,
        uint256 initialPublicDecryptionPrice,
        uint256 initialUserDecryptionPrice
    ) public {
        uint256 inputVerificationPrice;
        uint256 publicDecryptionPrice;
        uint256 userDecryptionPrice;

        emit InitializeProtocolPayment(inputVerificationPrice, publicDecryptionPrice, userDecryptionPrice);
    }

    function setInputVerificationPrice(uint256 price) external {
        emit NewInputVerificationPrice(price);
    }

    function setPublicDecryptionPrice(uint256 price) external {
        emit NewPublicDecryptionPrice(price);
    }

    function setUserDecryptionPrice(uint256 price) external {
        emit NewUserDecryptionPrice(price);
    }
}
