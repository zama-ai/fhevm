// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/// @notice The slice of an `InputVerifier` this library reads. Every one of these is public on the
///         PRODUCTION contract — which is the point: the digest can be rebuilt from outside.
interface IInputVerifierView {
    function eip712Domain()
        external
        view
        returns (
            bytes1 fields,
            string memory name,
            string memory version,
            uint256 chainId,
            address verifyingContract,
            bytes32 salt,
            uint256[] memory extensions
        );

    function EIP712_INPUT_VERIFICATION_TYPEHASH() external view returns (bytes32);
    function getCoprocessorSigners() external view returns (address[] memory);
    function getThreshold() external view returns (uint256);
}

/**
 * @title LibInputVerifier
 * @notice Reproduces `CleartextInputVerifier.inputProof()` against an `InputVerifier` that does not
 *         have it — i.e. a real one, on a forked chain.
 *
 * @dev WHY IT EXISTS. Minting an input proof needs three things: the EIP-712 digest to sign, the
 *      signers whose signatures will be accepted, and how many are required. The cleartext stack hands
 *      all three over in one call, because it was built to be driven from a test. A production
 *      `InputVerifier` has no such courtesy — but it does expose every ingredient, so the answer can be
 *      assembled rather than asked for:
 *
 *          digest    <- eip712Domain() + EIP712_INPUT_VERIFICATION_TYPEHASH() + the caller's fields
 *          signers   <- getCoprocessorSigners()
 *          threshold <- getThreshold()
 *
 *      `isCleartextVerifier` picks between the two, so a fork needs no configuration: the local stack
 *      answers `IS_CLEARTEXT`, everything else does not.
 *
 * @dev TWO CHAIN IDS, AND THEY DIFFER. `InputVerifier` extends `EIP712UpgradeableCrossChain`, so its
 *      domain is pinned to the chain the signatures are MADE on — Sepolia's verifier reports chainId
 *      10901 and a verifyingContract on the gateway, not its own address. The struct's
 *      `contractChainId` is the HOST chain, `block.chainid`. Substituting one for the other yields a
 *      digest that recovers to the wrong addresses and a proof the verifier rejects, so the domain is
 *      always read at runtime and never reconstructed from `block.chainid`.
 */
import {LibCleartextProbe} from "./LibCleartextProbe.sol";

library LibInputVerifier {
    /// @dev `keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)")`
    bytes32 private constant EIP712_DOMAIN_TYPEHASH =
        0x8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f;

    /// @dev ERC-5267 field mask for name + version + chainId + verifyingContract, and nothing else.
    bytes1 private constant EXPECTED_DOMAIN_FIELDS = 0x0f;

    /// @notice The verifier's EIP-712 domain is not the plain four-field one this library can rebuild.
    /// @dev    A salt or an extension changes the separator; carrying on would sign the wrong digest and
    ///         fail later as an unrecoverable signature, so it stops here where the cause is visible.
    error UnsupportedEip712Domain(bytes1 fields);

    /// @notice Whether `verifier` is a cleartext mock, and so answers `inputProof` itself.
    function isCleartextVerifier(address verifier) internal view returns (bool) {
        return LibCleartextProbe.isCleartext(verifier);
    }

    /**
     * @notice What `CleartextInputVerifier.inputProof` would have returned, computed from outside.
     * @param  verifier         The `InputVerifier` the proof will be presented to.
     * @param  ctHandles        The handles this proof covers, in order.
     * @param  userAddress      The user the input is minted for.
     * @param  contractAddress  The contract allowed to consume it.
     * @param  extraData        The proof tail — for this stack, the cleartexts.
     */
    function inputProof(
        address verifier,
        bytes32[] memory ctHandles,
        address userAddress,
        address contractAddress,
        bytes memory extraData
    ) internal view returns (bytes32 digest, address[] memory signers, uint256 threshold) {
        IInputVerifierView v = IInputVerifierView(verifier);

        bytes32 structHash = keccak256(
            abi.encode(
                v.EIP712_INPUT_VERIFICATION_TYPEHASH(),
                keccak256(abi.encodePacked(ctHandles)),
                userAddress,
                contractAddress,
                block.chainid,
                keccak256(abi.encodePacked(extraData))
            )
        );

        digest = keccak256(abi.encodePacked(hex"1901", domainSeparator(verifier), structHash));
        signers = v.getCoprocessorSigners();
        threshold = v.getThreshold();
    }

    /// @notice The verifier's EIP-712 domain separator, rebuilt from what it reports about itself.
    function domainSeparator(address verifier) internal view returns (bytes32) {
        (bytes1 fields, string memory name, string memory version, uint256 chainId, address verifyingContract,,) =
            IInputVerifierView(verifier).eip712Domain();

        if (fields != EXPECTED_DOMAIN_FIELDS) revert UnsupportedEip712Domain(fields);

        return keccak256(
            abi.encode(
                EIP712_DOMAIN_TYPEHASH, keccak256(bytes(name)), keccak256(bytes(version)), chainId, verifyingContract
            )
        );
    }
}
