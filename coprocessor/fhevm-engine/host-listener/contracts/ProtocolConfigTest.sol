// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.28;

/// @dev Test mock of the host-chain ProtocolConfig contract (coprocessor proposal surface only).
/// Event signatures match host-contracts/contracts/interfaces/IProtocolConfig.sol so that
/// the production event decoders in host-listener can decode events emitted by this mock.
contract ProtocolConfigTest {
    struct ChainUpgradeWindow {
        uint64 chainId;
        uint64 startBlock;
        uint64 endBlock;
    }

    event CoprocessorUpgradeProposed(
        uint256 indexed proposalId,
        string softwareVersion,
        ChainUpgradeWindow[] chainUpgradeWindows,
        uint64 gwStartBlock,
        uint16 ciphertextVersion
    );

    /// @notice Emit a `CoprocessorUpgradeProposed` event with caller-provided fields.
    function emitCoprocessorUpgradeProposed(
        uint256 proposalId,
        string calldata softwareVersion,
        ChainUpgradeWindow[] calldata chainUpgradeWindows,
        uint64 gwStartBlock,
        uint16 ciphertextVersion
    ) external {
        emit CoprocessorUpgradeProposed(
            proposalId,
            softwareVersion,
            chainUpgradeWindows,
            gwStartBlock,
            ciphertextVersion
        );
    }
}
