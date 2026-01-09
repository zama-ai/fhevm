// SPDX-License-Identifier: BSD-3-Clause-Clear
// Ported from https://github.com/OpenZeppelin/openzeppelin-confidential-contracts/blob/f0914b66f9f3766915403587b1ef1432d53054d3/contracts/mocks/token/ERC7984ReceiverMock.sol
// (0.3.0 version)
pragma solidity 0.8.27;

import {ZamaEthereumConfig} from "@fhevm/solidity/config/ZamaConfig.sol";
import {FHE, ebool, euint64} from "@fhevm/solidity/lib/FHE.sol";
import {IERC7984Receiver} from "@openzeppelin/confidential-contracts/interfaces/IERC7984Receiver.sol";

contract ERC7984ReceiverMock is IERC7984Receiver, ZamaEthereumConfig {
    event ConfidentialTransferCallback(bool success);

    error InvalidInput(uint8 input);

    /// Data should contain a success boolean (plaintext). Revert if not.
    function onConfidentialTransferReceived(address, address, euint64, bytes calldata data) external returns (ebool) {
        uint8 input = abi.decode(data, (uint8));

        if (input > 1) revert InvalidInput(input);

        bool success = input == 1;
        emit ConfidentialTransferCallback(success);

        ebool returnVal = FHE.asEbool(success);
        FHE.allowTransient(returnVal, msg.sender);

        return returnVal;
    }
}
