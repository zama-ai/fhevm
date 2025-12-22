// SPDX-License-Identifier: BSD-3-Clause-Clear
// Source: https://raw.githubusercontent.com/OpenZeppelin/openzeppelin-confidential-contracts/3a8689021a89e5d9c3101280cb2cd9435cbf28f1/contracts/mocks/token/ERC7984ReceiverMock.sol
pragma solidity 0.8.27;

import {ZamaEthereumConfig} from "@fhevm/solidity/config/ZamaConfig.sol";
import {FHE, ebool, euint64} from "@fhevm/solidity/lib/FHE.sol";
import {IERC7984Receiver} from "openzeppelin-confidential-contracts/contracts/interfaces/IERC7984Receiver.sol";

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
