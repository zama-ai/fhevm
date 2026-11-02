// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {HostContractsDeployerTestUtils} from "@fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {CiphertextRoots} from "@fhevm-host-contracts/contracts/CiphertextRoots.sol";
import {protocolConfigAdd} from "@fhevm-host-contracts/addresses/FHEVMHostAddresses.sol";
import {ProtocolConfigMock} from "./ProtocolConfigMock.sol";

/**
 * @dev Gas estimate of posting roots, to choose a host chain's epoch length (RFC 034): prints what
 *      one node pays per post and per year, for a coprocessor set and an epoch schedule. Run with
 *      `npm run gas:ciphertextRoots`, with these env variables, all optional:
 *      - `CIPHERTEXT_ROOTS_GAS_NODES`: number of coprocessor nodes, 5 by default;
 *      - `CIPHERTEXT_ROOTS_GAS_THRESHOLD`: majority threshold of the context, 3 by default;
 *      - `CIPHERTEXT_ROOTS_GAS_EPOCH_LENGTH`: host chain blocks per epoch, 300 by default, one hour of
 *        12-second blocks;
 *      - `CIPHERTEXT_ROOTS_GAS_BLOCK_TIME_MS`: host chain block time in milliseconds, 12000 by default.
 *      Every post is priced as its own Ethereum transaction: the 21k base cost and the calldata are
 *      included and nothing is warm from a previous post.
 */
contract CiphertextRootsGasTest is HostContractsDeployerTestUtils {
    address internal constant owner = address(456);
    uint64 internal constant CHAIN_ID = 1;
    uint256 internal constant EPOCH = 5;
    uint256 internal constant TX_BASE_GAS = 21_000;
    /// @dev postRoot(uint64,uint256,bytes32): 4 + 3 * 32 bytes, at 16 gas per non-zero byte.
    uint256 internal constant TX_CALLDATA_GAS = 100 * 16;
    uint256 internal constant SECONDS_PER_YEAR = 365 days;

    function test_printPostGas() public {
        uint256 nodeCount = vm.envOr("CIPHERTEXT_ROOTS_GAS_NODES", uint256(5));
        uint256 threshold = vm.envOr("CIPHERTEXT_ROOTS_GAS_THRESHOLD", uint256(3));
        uint256 epochLength = vm.envOr("CIPHERTEXT_ROOTS_GAS_EPOCH_LENGTH", uint256(300));
        uint256 blockTimeMs = vm.envOr("CIPHERTEXT_ROOTS_GAS_BLOCK_TIME_MS", uint256(12_000));
        uint256 epochsPerYear = (SECONDS_PER_YEAR * 1000) / (epochLength * blockTimeMs);

        emit log_named_uint("setting: nodes", nodeCount);
        emit log_named_uint("setting: threshold", threshold);
        emit log_named_uint("setting: epoch length in blocks", epochLength);
        emit log_named_uint("setting: block time in ms", blockTimeMs);
        emit log_named_uint("=> epoch duration in seconds", (epochLength * blockTimeMs) / 1000);
        emit log_named_uint("=> epochs per year", epochsPerYear);

        _deployACL(owner);
        deployCodeTo("test/ciphertextRoots/ProtocolConfigMock.sol:ProtocolConfigMock", protocolConfigAdd);
        (CiphertextRoots ciphertextRoots, address implementation) = _deployCiphertextRoots(owner);
        address[] memory nodes = new address[](nodeCount);
        for (uint256 i = 0; i < nodeCount; i++) {
            nodes[i] = address(uint160(0x1000 + i));
        }
        ProtocolConfigMock(protocolConfigAdd).defineContext((uint256(9) << 248) + 1, nodes, threshold);
        vm.prank(owner);
        ciphertextRoots.setEpochSchedule(CHAIN_ID, 0, epochLength);

        bytes32 root = keccak256("root");
        uint256 epochGas;
        for (uint256 i = 0; i < nodeCount; i++) {
            // Price each post as its own transaction: forget the storage and contracts the previous
            // posts warmed, then warm the proxy account only, as a transaction's target is.
            vm.cool(address(ciphertextRoots));
            vm.cool(implementation);
            vm.cool(protocolConfigAdd);
            address(ciphertextRoots).balance;
            vm.prank(nodes[i]);
            uint256 gasBefore = gasleft();
            ciphertextRoots.postRoot(CHAIN_ID, EPOCH, root);
            uint256 postGas = gasBefore - gasleft() + TX_BASE_GAS + TX_CALLDATA_GAS;
            epochGas += postGas;
            // One line per node. The node whose post reaches the threshold records the root.
            string memory label = string.concat("gas of node ", vm.toString(i + 1), "'s post");
            if (i + 1 == threshold) {
                label = string.concat(label, " (records the root)");
            }
            emit log_named_uint(label, postGas);
        }

        uint256 averagePostGas = epochGas / nodeCount;
        emit log_named_uint("gas per post, average for a node", averagePostGas);
        emit log_named_uint("gas per year for a node", averagePostGas * epochsPerYear);
    }
}
