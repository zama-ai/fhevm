// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/// @dev This contract is a mock of the CiphertextCommits contract from the Gateway.
/// source: github.com/zama-ai/fhevm-gateway/blob/main/contracts/CiphertextCommits.sol
contract CiphertextCommits {
    error CoprocessorAlreadyAdded(bytes32 ctHandle, address coprocessorTxSenderAddress);
    error NotCoprocessorTxSender(address txSenderAddress);

    event AddCiphertextMaterial(
        bytes32 indexed ctHandle,
        bytes32 ciphertextDigest,
        bytes32 snsCiphertextDigest,
        address[] coprocessorTxSenderAddresses
    );

    bool alreadyAddedRevert;
    ConfigErrorMode configErrorMode;

    enum ConfigErrorMode {
        None,
        NotCoprocessorTxSender
    }

    constructor(bool _alreadyAddedRevert) {
        alreadyAddedRevert = _alreadyAddedRevert;
    }

    function setConfigErrorMode(uint8 mode) external {
        require(mode <= uint8(ConfigErrorMode.NotCoprocessorTxSender), "invalid mode");
        configErrorMode = ConfigErrorMode(mode);
    }

    function maybeRevertConfigError() internal view {
        if (configErrorMode == ConfigErrorMode.NotCoprocessorTxSender) {
            revert NotCoprocessorTxSender(msg.sender);
        }
    }

    function addCiphertextMaterial(
        bytes32 ctHandle,
        uint256 /* keyId */,
        bytes32 ciphertextDigest,
        bytes32 snsCiphertextDigest
    ) public {
        maybeRevertConfigError();
        if (alreadyAddedRevert) {
            revert CoprocessorAlreadyAdded(ctHandle, msg.sender);
        }

        emit AddCiphertextMaterial(
            ctHandle,
            ciphertextDigest,
            snsCiphertextDigest,
            new address[](0)
        );
    }
}
