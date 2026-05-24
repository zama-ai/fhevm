// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.28;

/// @dev Test mock of the host-chain ProtocolConfig contract (coprocessor context surface only).
/// Event signatures match host-contracts/contracts/interfaces/IProtocolConfig.sol so that
/// the production event decoders in host-listener can decode events emitted by this mock.
contract ProtocolConfigTest {
    struct ChainUpgradeWindow {
        uint64 chainId;
        uint64 startBlock;
        uint64 endBlock;
    }

    event NewCoprocessorContext(
        uint256 indexed coprocessorContextId,
        string softwareVersion,
        ChainUpgradeWindow[] chainUpgradeWindows,
        uint64 gwStartBlock
    );

    event CoprocessorContextDestroyed(uint256 indexed coprocessorContextId);

    /// @notice Emit a `NewCoprocessorContext` event with caller-provided fields.
    function emitNewCoprocessorContext(
        uint256 coprocessorContextId,
        string calldata softwareVersion,
        ChainUpgradeWindow[] calldata chainUpgradeWindows,
        uint64 gwStartBlock
    ) external {
        emit NewCoprocessorContext(
            coprocessorContextId,
            softwareVersion,
            chainUpgradeWindows,
            gwStartBlock
        );
    }

    /// @notice Emit a `CoprocessorContextDestroyed` event.
    function emitCoprocessorContextDestroyed(uint256 coprocessorContextId) external {
        emit CoprocessorContextDestroyed(coprocessorContextId);
    }
}
