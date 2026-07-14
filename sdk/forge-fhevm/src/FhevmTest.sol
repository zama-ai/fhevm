// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.27;

import {InputProofLib} from "./internal/InputProofLib.sol";
import {KmsProofLib} from "./internal/KmsProofLib.sol";

import {FhevmStack} from "@fhevm/host-contracts-cleartext/deploy/FhevmStack.sol";

import {IERC7984ERC20Wrapper} from "@openzeppelin/confidential-contracts/interfaces/IERC7984ERC20Wrapper.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import {FheType} from "@fhevm/host-contracts-cleartext/contracts/shared/FheType.sol";
import {
    CleartextKMSVerifier,
    HandleContractPair
} from "@fhevm/host-contracts-cleartext/cleartext/CleartextKMSVerifier.sol";
import {CleartextFHEVMExecutor} from "@fhevm/host-contracts-cleartext/cleartext/CleartextFHEVMExecutor.sol";
import {
    aclAdd,
    fhevmExecutorAdd,
    kmsVerifierAdd,
    inputVerifierAdd
} from "@fhevm/host-contracts-cleartext/addresses/FHEVMHostAddresses.sol";

import {
    ebool,
    euint8,
    euint16,
    euint32,
    euint64,
    euint128,
    euint256,
    eaddress,
    externalEbool,
    externalEuint8,
    externalEuint16,
    externalEuint32,
    externalEuint64,
    externalEuint128,
    externalEuint256,
    externalEaddress
} from "encrypted-types/EncryptedTypes.sol";

/**
 * Base contract for Foundry tests of FHEVM contracts.
 *
 * Inherit it, call `super.setUp()`, and the cleartext host stack is live at the addresses your contract's
 * `ZamaEthereumConfig` already points at. Then:
 *
 *   (externalEuint32 v, bytes memory proof) = encryptUint32(42, alice.addr, address(counter));
 *   vm.prank(alice.addr);
 *   counter.increment(v, proof);
 *   uint256 clear = userDecrypt(handle, alice.addr, address(counter), signUserDecrypt(alice.key, address(counter)));
 *
 * The stack is the real thing — real ACL enforcement, real signature verification, real HCU accounting.
 * What is faked is only the CRYPTOGRAPHY: values travel in the clear inside the input proof, and the
 * "encrypted" result is XOR-masked rather than re-encrypted. So this proves your ACL and control flow are
 * right; it proves nothing about FHE itself.
 */
abstract contract FhevmTest is FhevmStack {
    /// @dev The mock coprocessor and KMS keys. Arbitrary, but the addresses they derive to are what the
    ///      InputVerifier and ProtocolConfig are initialized with, so signing and verification agree.
    uint256 internal constant MOCK_INPUT_SIGNER_PK = 0x7ec8ada6642fc4ccfb7729bc29c17cf8d21b61abd5642d1db992c0b8672ab901;
    uint256 internal constant MOCK_KMS_SIGNER_PK = 0x388b7680e4e1afa06efbfd45cdd1fe39f3c6af381df6555a19661f283b97de91;

    /// @dev `CleartextKMSVerifier` XOR-masks results with the first 32 bytes of the caller's public key,
    ///      standing in for the KMS re-encrypting under a transport key. Any 32+ bytes work; we unmask with
    ///      the same word.
    bytes internal constant USER_DECRYPT_PUBLIC_KEY =
        hex"dededededededededededededededededededededededededededededededede";

    uint256 internal constant DEFAULT_START_TIMESTAMP = 0;
    uint256 internal constant DEFAULT_DURATION_DAYS = 365;

    address internal mockInputSigner;
    address internal mockKmsSigner;

    /// @dev Makes each handle in a run unique; the chain never inspects the ciphertext blob.
    uint256 private _encryptNonce;

    function setUp() public virtual {
        // ZamaConfig._getLocalConfig() is the `block.chainid == 31337` branch. On any other chain id a
        // contract under test reverts with ZamaProtocolUnsupported before it ever reaches the stack.
        vm.chainId(31337);

        mockInputSigner = vm.addr(MOCK_INPUT_SIGNER_PK);
        mockKmsSigner = vm.addr(MOCK_KMS_SIGNER_PK);

        _deployFhevmStack(mockKmsSigner, mockInputSigner);
    }

    /*//////////////////////////////////////////////////////////////
                                ENCRYPT
    //////////////////////////////////////////////////////////////*/

    function encryptBool(bool value, address user, address target) internal returns (externalEbool, bytes memory) {
        (bytes32 h, bytes memory p) = _encrypt(value ? 1 : 0, FheType.Bool, user, target);
        return (externalEbool.wrap(h), p);
    }

    function encryptUint8(uint8 value, address user, address target) internal returns (externalEuint8, bytes memory) {
        (bytes32 h, bytes memory p) = _encrypt(value, FheType.Uint8, user, target);
        return (externalEuint8.wrap(h), p);
    }

    function encryptUint16(uint16 value, address user, address target)
        internal
        returns (externalEuint16, bytes memory)
    {
        (bytes32 h, bytes memory p) = _encrypt(value, FheType.Uint16, user, target);
        return (externalEuint16.wrap(h), p);
    }

    function encryptUint32(uint32 value, address user, address target)
        internal
        returns (externalEuint32, bytes memory)
    {
        (bytes32 h, bytes memory p) = _encrypt(value, FheType.Uint32, user, target);
        return (externalEuint32.wrap(h), p);
    }

    function encryptUint64(uint64 value, address user, address target)
        internal
        returns (externalEuint64, bytes memory)
    {
        (bytes32 h, bytes memory p) = _encrypt(value, FheType.Uint64, user, target);
        return (externalEuint64.wrap(h), p);
    }

    function encryptUint128(uint128 value, address user, address target)
        internal
        returns (externalEuint128, bytes memory)
    {
        (bytes32 h, bytes memory p) = _encrypt(value, FheType.Uint128, user, target);
        return (externalEuint128.wrap(h), p);
    }

    function encryptUint256(uint256 value, address user, address target)
        internal
        returns (externalEuint256, bytes memory)
    {
        (bytes32 h, bytes memory p) = _encrypt(value, FheType.Uint256, user, target);
        return (externalEuint256.wrap(h), p);
    }

    function encryptAddress(address value, address user, address target)
        internal
        returns (externalEaddress, bytes memory)
    {
        (bytes32 h, bytes memory p) = _encrypt(uint256(uint160(value)), FheType.Uint160, user, target);
        return (externalEaddress.wrap(h), p);
    }

    /*//////////////////////////////////////////////////////////////
                                DECRYPT
    //////////////////////////////////////////////////////////////*/

    /**
     * Signs a user-decrypt request as `userPk`.
     *
     * The signature is verified ON-CHAIN by `CleartextKMSVerifier`, over a digest that includes `extraData`
     * — which the contract rebuilds itself from the live KMS context id. So we must read that id and
     * reproduce `extraData` byte-for-byte; a mismatch surfaces only as `InvalidUserDecryptSignature`.
     *
     * The domain uses the HOST chain id, not the gateway one the verifier was initialized with: user
     * decryption deliberately overrides it (`_domainHashWithHostChainId`), unlike public decryption.
     */
    function signUserDecrypt(uint256 userPk, address contractAddress) internal view returns (bytes memory) {
        address[] memory contracts = new address[](1);
        contracts[0] = contractAddress;
        return signUserDecrypt(userPk, contracts);
    }

    function signUserDecrypt(uint256 userPk, address[] memory contractAddresses)
        internal
        view
        returns (bytes memory)
    {
        bytes32 structHash = keccak256(
            abi.encode(
                // Adjacent string literals concatenate — the typehash is unchanged, it just fits in 120 cols.
                keccak256(
                    "UserDecryptRequestVerification(bytes publicKey,address[] contractAddresses,"
                    "uint256 startTimestamp,uint256 durationDays,bytes extraData)"
                ),
                keccak256(USER_DECRYPT_PUBLIC_KEY),
                keccak256(abi.encodePacked(contractAddresses)),
                DEFAULT_START_TIMESTAMP,
                DEFAULT_DURATION_DAYS,
                keccak256(_currentExtraData())
            )
        );

        // Name/version/verifyingContract come from the live verifier; only the chain id is overridden. User
        // decryption is domain-separated by the HOST chain id (`_domainHashWithHostChainId`), whereas the
        // verifier's own `eip712Domain()` reports the GATEWAY chain id it was initialized with. Using the
        // latter here recovers a different signer and fails on-chain as InvalidUserDecryptSignature.
        (, string memory name, string memory version,, address verifyingContract,,) =
            CleartextKMSVerifier(kmsVerifierAdd).eip712Domain();

        bytes32 domain = KmsProofLib.domainSeparator(name, version, block.chainid, verifyingContract);

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(userPk, keccak256(abi.encodePacked(hex"1901", domain, structHash)));
        return abi.encodePacked(r, s, v);
    }

    /// @notice User-decrypts one handle. ACL is enforced on-chain and reverts if `user` was never granted it.
    function userDecrypt(bytes32 handle, address user, address contractAddress, bytes memory signature)
        internal
        view
        returns (uint256)
    {
        HandleContractPair[] memory pairs = new HandleContractPair[](1);
        pairs[0] = HandleContractPair({handle: handle, contractAddress: contractAddress});

        address[] memory contracts = new address[](1);
        contracts[0] = contractAddress;

        (bytes memory payload,,,) = CleartextKMSVerifier(kmsVerifierAdd).userDecrypt(
            pairs,
            user,
            USER_DECRYPT_PUBLIC_KEY,
            contracts,
            DEFAULT_START_TIMESTAMP,
            DEFAULT_DURATION_DAYS,
            signature
        );

        // payload = abi.encode(uint256[] maskedCleartexts, bytes extraData)
        (uint256[] memory masked,) = abi.decode(payload, (uint256[], bytes));
        return masked[0] ^ uint256(bytes32(USER_DECRYPT_PUBLIC_KEY));
    }

    /**
     * External wrapper around {userDecrypt}, for asserting that a decrypt is REJECTED.
     *
     * `vm.expectRevert()` latches onto the next EXTERNAL call. `userDecrypt` is internal, so a revert inside
     * it is swallowed by the cheatcode and execution continues with empty returndata — which then blows up
     * in `abi.decode` and fails the test for the wrong reason. Call this through `this.` instead:
     *
     *   vm.expectRevert();
     *   this.userDecryptExternal(handle, bob, address(counter), sig);
     */
    function userDecryptExternal(bytes32 handle, address user, address contractAddress, bytes memory signature)
        external
        view
        returns (uint256)
    {
        return userDecrypt(handle, user, contractAddress, signature);
    }

    /// @notice Public-decrypts one handle. Requires the handle to have been made publicly decryptable.
    function publicDecrypt(bytes32 handle) internal view returns (uint256) {
        bytes32[] memory handles = new bytes32[](1);
        handles[0] = handle;

        (bytes memory encoded,,,,) = CleartextKMSVerifier(kmsVerifierAdd).publicDecrypt(handles);
        // Not standard ABI encoding: one right-aligned 32-byte word per handle.
        return abi.decode(encoded, (uint256));
    }

    /// @notice The raw clear value behind a handle, bypassing ACL entirely. A debugging aid — prefer
    ///         `userDecrypt`, which exercises the permissions your contract is supposed to grant.
    function peek(bytes32 handle) internal view returns (uint256) {
        return CleartextFHEVMExecutor(fhevmExecutorAdd).plaintexts(handle);
    }

    /*//////////////////////////////////////////////////////////////
                        ASYNC (CALLBACK) DECRYPTION
    //////////////////////////////////////////////////////////////*/

    /**
     * Forges the KMS-signed proof a gateway would post back to a contract awaiting a decryption result.
     *
     * This is a different thing from {publicDecrypt} and {userDecrypt}, which read a value out for the TEST
     * to assert on. This one lets you drive the CONTRACT UNDER TEST through its own result-verification
     * branch — the one guarded by `KMSVerifier.verifyDecryptionEIP712KMSSignatures`. Without it that branch,
     * which usually holds the interesting state transitions, is unreachable in a test:
     *
     *   bytes memory result = abi.encode(uint256(42));
     *   myContract.onDecryptionResult(requestId, result, kmsDecryptionProof(handles, result));
     */
    function kmsDecryptionProof(bytes32[] memory handles, bytes memory decryptedResult)
        internal
        view
        returns (bytes memory)
    {
        bytes memory extraData = _currentExtraData();

        // Public decryption is domain-separated by the GATEWAY chain id the verifier was initialized with —
        // unlike user decryption, which overrides it to the host chain id. Read it back rather than assume.
        (, string memory name, string memory version, uint256 gatewayChainId, address verifyingContract,,) =
            CleartextKMSVerifier(kmsVerifierAdd).eip712Domain();

        bytes32 digest = KmsProofLib.digest(
            handles,
            decryptedResult,
            extraData,
            KmsProofLib.domainSeparator(name, version, gatewayChainId, verifyingContract)
        );

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(MOCK_KMS_SIGNER_PK, digest);
        bytes[] memory signatures = new bytes[](1);
        signatures[0] = abi.encodePacked(r, s, v);

        return KmsProofLib.assemble(signatures, extraData);
    }

    /*//////////////////////////////////////////////////////////////
                                 UTIL
    //////////////////////////////////////////////////////////////*/

    /**
     * The confidential-token equivalent of Foundry's `deal`: funds `user` with the wrapper's underlying
     * ERC-20 and wraps `amount` of it into confidential balance.
     *
     * Confidential balances are encrypted handles, so they cannot be written directly the way `deal` pokes an
     * ERC-20 balance slot. Going through `wrap` is the only way to mint one that the ACL will actually let
     * `user` spend.
     */
    function dealConfidential(IERC7984ERC20Wrapper wrapper, address user, uint256 amount) internal {
        IERC20 underlying = IERC20(wrapper.underlying());
        deal(address(underlying), user, underlying.balanceOf(user) + amount);

        vm.startPrank(user);
        underlying.approve(address(wrapper), type(uint256).max);
        wrapper.wrap(user, amount);
        vm.stopPrank();
    }

    /*//////////////////////////////////////////////////////////////
                               INTERNAL
    //////////////////////////////////////////////////////////////*/

    function _encrypt(uint256 value, FheType fheType, address user, address target)
        private
        returns (bytes32 handle, bytes memory inputProof)
    {
        InputProofLib.assertFits(value, fheType);

        _encryptNonce += 1;
        bytes memory ciphertext = abi.encodePacked(keccak256(abi.encodePacked(value, uint8(fheType), _encryptNonce)));

        bytes32[] memory handles = new bytes32[](1);
        handles[0] = InputProofLib.computeHandle(ciphertext, 0, fheType, aclAdd, uint64(block.chainid));

        uint256[] memory values = new uint256[](1);
        values[0] = value;
        bytes memory extraData = InputProofLib.packExtraData(values);

        bytes32 digest = InputProofLib.digest(
            handles,
            user,
            target,
            block.chainid,
            extraData,
            InputProofLib.domainSeparator(inputVerifierAdd, block.chainid)
        );

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(MOCK_INPUT_SIGNER_PK, digest);
        bytes[] memory signatures = new bytes[](1);
        signatures[0] = abi.encodePacked(r, s, v);

        handle = handles[0];
        inputProof = InputProofLib.assemble(handles, signatures, extraData);
    }

    /// @dev Mirrors `CleartextKMSVerifier._buildCurrentExtradata()`: 0x01 followed by the KMS context id.
    function _currentExtraData() private view returns (bytes memory) {
        uint256 contextId = CleartextKMSVerifier(kmsVerifierAdd).getCurrentKmsContextId();
        return abi.encodePacked(bytes1(0x01), contextId);
    }
}
