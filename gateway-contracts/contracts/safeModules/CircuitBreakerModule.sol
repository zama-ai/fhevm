// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import "@safe-global/safe-contracts/contracts/common/Enum.sol";
import "@safe-global/safe-contracts/contracts/Safe.sol";

contract CircuitBreakerModule {
    bytes32 public immutable PAUSE_CONTRACT_TYPE_HASH =
        keccak256("PauseContract(address contractAddress,address signer,uint256 nonce,uint256 deadline)");

    address public immutable safeAddress;
    mapping(address => uint256) public nonces;

    /**
     * @dev Constructor function for the contract
     * @param _safeAddress address of the Safe contract
     */
    constructor(address _safeAddress) {
        safeAddress = _safeAddress;
    }

    /**
     * @dev Generates the EIP-712 domain separator for the contract.
     *
     * @return The EIP-712 domain separator.
     */
    function getDomainSeparator() private view returns (bytes32) {
        return
            keccak256(
                abi.encode(
                    keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"),
                    keccak256(bytes("CircuitBreaker")),
                    keccak256(bytes("1")),
                    block.chainid,
                    address(this)
                )
            );
    }

    /**
     * @dev Pauses a contract by executing the `pause()` function on the specified contract address.
     * The msg.sender must hold a valid signature. The msg.sender address must be used as the `signer`
     * parameter in the EIP-712 structured data for signature generation.
     * @param contractAddress amount of tokens to be transferred
     * @param deadline deadline for the validity of the signature
     * @param signature signature of the Safe owner(s)
     */
    function pauseContract(address contractAddress, uint256 deadline, bytes memory signature) public {
        require(deadline >= block.timestamp, "expired deadline");

        bytes32 signatureData = keccak256(
            abi.encode(PAUSE_CONTRACT_TYPE_HASH, contractAddress, msg.sender, nonces[msg.sender]++, deadline)
        );

        bytes32 hash = keccak256(abi.encodePacked("\x19\x01", getDomainSeparator(), signatureData));

        Safe(payable(safeAddress)).checkSignatures(hash, abi.encodePacked(signatureData), signature);

        bytes memory data = abi.encodeWithSignature("pause()");

        require(
            Safe(payable(safeAddress)).execTransactionFromModule(contractAddress, 0, data, Enum.Operation.Call),
            "Could not execute contract pause"
        );
    }
}
