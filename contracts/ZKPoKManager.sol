// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.28;

import "./interfaces/IZKPoKManager.sol";

/// @title ZKPoKManager
/// @dev Implementation of the IZKPoKManager for the ZKPoK verifications on the Gateway L2 roll-up.
contract ZKPoKManager is IZKPoKManager {
    /// @notice The address of the Payment Manager contract for service fees, burn and distribution
    address internal immutable _PAYMENT_MANAGER;

    constructor(address paymentManager) {
        _PAYMENT_MANAGER = paymentManager;
    }

    /// @dev See {IZKPoKManager-verifyProofRequest}.
    function verifyProofRequest(
        uint256 chainId,
        address contractAddress,
        address userAddress,
        bytes calldata ctProofHandle
    ) public {
        if (!_isNetworkRegistered(chainId)) {
            revert NetworkNotRegistered();
        }
        _sendServiceFees();
        uint256 zkProofId = _generateZKProofId();
        emit VerifyProofRequest(zkProofId, chainId, contractAddress, userAddress, ctProofHandle);
    }

    /// @dev See {IZKPoKManager-verifyProofResponse}.
    function verifyProofResponse(uint256 zkProofId, bytes32[] calldata handles, bytes calldata signature) public {
        if (_isConsensusAgreed()) {
            _burnAndDistributeFees();
            bytes[] memory signatures = _getSignatures(zkProofId);
            emit VerifyProofResponse(zkProofId, handles, signatures);
        }
    }

    /// @notice Generates an identifier for a ZKProof verification
    function _generateZKProofId() internal pure returns (uint256) {
        // TODO: Implement zkProofId generation
    }

    /// @notice Checks if the given chain ID is already registered
    function _isNetworkRegistered(uint256 chainId) internal pure returns (bool) {
        // TODO: Implement DomainManager contract call to check network is registered
        return true;
    }

    /// @notice Checks if the consensus is reached
    function _isConsensusAgreed() internal pure returns (bool) {
        // TODO: Implement consensus logic
        return true;
    }

    /// @dev Service fees should be refunded if there is no consensus or response after N blocks
    function _sendServiceFees() internal pure {
        // TODO: Implement PaymentManager contract call to send service fees
    }

    /// @notice Burns and distributes the ZKProof verification fees
    function _burnAndDistributeFees() internal pure {
        // TODO: Implement PaymentManager contract call to burn and distribute fees
    }

    /// @notice Aggregates the received ZKProof verification signatures
    function _getSignatures(uint256 zkProofId) internal pure returns (bytes[] memory) {
        // TODO: Implement signature aggregation logic
        return new bytes[](0);
    }
}
