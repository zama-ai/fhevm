// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.27;

import {Test} from "forge-std/Test.sol";

import {CleartextACL} from "../cleartext/CleartextACL.sol";
import {CleartextFHEVMExecutor} from "../cleartext/CleartextFHEVMExecutor.sol";
import {CleartextInputVerifier} from "../cleartext/CleartextInputVerifier.sol";
import {CleartextKMSVerifier} from "../cleartext/CleartextKMSVerifier.sol";
import {CleartextHCULimit} from "../cleartext/CleartextHCULimit.sol";
import {CleartextProtocolConfig} from "../cleartext/CleartextProtocolConfig.sol";
import {CleartextKMSGeneration} from "../cleartext/CleartextKMSGeneration.sol";
import {CleartextArithmetic} from "../cleartext/CleartextArithmetic.sol";

import {ACL} from "../host-contracts/contracts/ACL.sol";
import {FHEVMExecutor} from "../host-contracts/contracts/FHEVMExecutor.sol";
import {InputVerifier} from "../host-contracts/contracts/InputVerifier.sol";
import {KMSVerifier} from "../host-contracts/contracts/KMSVerifier.sol";
import {HCULimit} from "../host-contracts/contracts/HCULimit.sol";
import {ProtocolConfig} from "../host-contracts/contracts/ProtocolConfig.sol";
import {KMSGeneration} from "../host-contracts/contracts/KMSGeneration.sol";
import {IProtocolConfig} from "../host-contracts/contracts/interfaces/IProtocolConfig.sol";
import {KmsNodeParams, PcrValues} from "../host-contracts/contracts/shared/Structs.sol";
import {PauserSet} from "../host-contracts/contracts/immutable/PauserSet.sol";
import {EmptyUUPSProxy} from "../host-contracts/contracts/emptyProxy/EmptyUUPSProxy.sol";
import {EmptyUUPSProxyACL} from "../host-contracts/contracts/emptyProxyACL/EmptyUUPSProxyACL.sol";
import {FheType} from "../host-contracts/contracts/shared/FheType.sol";
import {
    aclAdd,
    fhevmExecutorAdd,
    hcuLimitAdd,
    inputVerifierAdd,
    kmsVerifierAdd,
    protocolConfigAdd,
    kmsGenerationAdd,
    pauserSetAdd
} from "../host-contracts/addresses/FHEVMHostAddresses.sol";

import {InputProofHelper} from "./InputProofHelper.sol";
import {KMSDecryptionProofHelper} from "./KMSDecryptionProofHelper.sol";
import {UserDecryptHelper} from "./UserDecryptHelper.sol";
// Imported so consumers compile the artifact that `deployCodeTo(ERC1967_PROXY_ARTIFACT, ...)` resolves
// by name below. Referenced by string, not by type, hence the explicit import.
import {CleartextTestProxy} from "./CleartextTestProxy.sol";

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

/// @title FhevmTest — in-process Foundry test harness for the canonical cleartext stack.
/// @notice A Foundry `Test` base contract that gives you an encrypt/decrypt DSL over the
///         canonical on-chain cleartext contracts shipped in this package. Inherit it and call
///         `encryptUintXX` / `userDecrypt` / `publicDecrypt` from your test.
/// @dev Single-engine design: the harness deploys THIS package's `CleartextFHEVMExecutor` (and the
///      rest of the Cleartext* split set) at the canonical FHEVM host addresses. Cleartext is computed
///      ON-CHAIN by the executor and stored in `CleartextACL`; the harness reads it back with
///      `acl.plaintext(handle)`. There is no off-chain event replay and no duplicated op-semantics —
///      the op logic lives only in `CleartextArithmetic` (this package).
///
///      The cleartext executor is >24 KB (EIP-170). This is a non-issue in-process: `forge test` does
///      not enforce the runtime code-size limit, so the executor deploys under a plain `forge test`
///      (no flags). Node-based consumers configure the limit off (anvil `--disable-code-size-limit`,
///      Hardhat `allowUnlimitedContractSize`).
///
///      Two setup modes:
///        - `setUp()` (default) deploys the stack in-process for pure `forge test` unit tests.
///        - override `setUp` to call `attach()` to run against an already-deployed stack (a live/forked
///          node), reusing the exact same DSL. Both modes read cleartext the same way.
abstract contract FhevmTest is Test {
    error HandleNotAllowedForPublicDecryption(bytes32 handle);
    error UserAddressEqualsContractAddress();
    error UserNotAuthorizedForDecrypt(bytes32 handle, address userAddress);
    error ContractNotAuthorizedForDecrypt(bytes32 handle, address contractAddress);
    error HandleNotDelegatedForDecrypt(bytes32 handle, address delegator, address delegate, address contractAddress);
    error InvalidUserDecryptSignature();
    error EncryptInputLengthMismatch(uint256 valuesLength, uint256 fheTypesLength);
    error EncryptInputTooLong();

    uint256 internal constant MOCK_INPUT_SIGNER_PK = 0x7ec8ada6642fc4ccfb7729bc29c17cf8d21b61abd5642d1db992c0b8672ab901;
    uint256 internal constant MOCK_KMS_SIGNER_PK = 0x388b7680e4e1afa06efbfd45cdd1fe39f3c6af381df6555a19661f283b97de91;
    /// @dev Mock KMS node tx-sender registered in ProtocolConfig (v0.14); distinct from the signer.
    address internal constant MOCK_KMS_TX_SENDER = address(0xC0FFEE);

    bytes internal constant EMPTY_EXTRA_DATA = hex"00";
    uint256 internal constant DEFAULT_USER_DECRYPT_DURATION_DAYS = 1;
    address internal constant PROXY_OWNER = address(0xBEEF);
    string internal constant ERC1967_PROXY_ARTIFACT = "CleartextTestProxy.sol:CleartextTestProxy";

    CleartextFHEVMExecutor internal _executor;
    CleartextACL internal _acl;
    CleartextInputVerifier internal _inputVerifier;
    CleartextKMSVerifier internal _kmsVerifier;

    address internal mockInputSigner;
    address internal mockKmsSigner;

    uint256 private _encryptNonce;

    function setUp() public virtual {
        vm.chainId(31337);
        deploy();
    }

    /// @notice Deploys the canonical cleartext stack in-process at the canonical host addresses.
    /// @dev Proxies are placed at the fixed addresses with `deployCodeTo`; implementations (including
    ///      the >24 KB cleartext executor) are created normally — `forge test` does not enforce the
    ///      EIP-170 runtime code-size limit, so no node flag is needed.
    function deploy() internal {
        mockInputSigner = vm.addr(MOCK_INPUT_SIGNER_PK);
        mockKmsSigner = vm.addr(MOCK_KMS_SIGNER_PK);
        _deployAllContracts();
    }

    /// @notice Binds the harness to an already-deployed cleartext stack (live/forked node).
    /// @dev Same DSL, no in-process deploy. Signer keys must match the deployed InputVerifier /
    ///      ProtocolConfig signer set for `_encrypt` and decryption-proof building to verify.
    /// @param inputSignerPk Private key registered as the input-verifier signer on the deployed stack.
    /// @param kmsSignerPk Private key registered as the KMS signer on the deployed stack.
    function attach(uint256 inputSignerPk, uint256 kmsSignerPk) internal {
        mockInputSigner = vm.addr(inputSignerPk);
        mockKmsSigner = vm.addr(kmsSignerPk);
        _executor = CleartextFHEVMExecutor(fhevmExecutorAdd);
        _acl = CleartextACL(aclAdd);
        _inputVerifier = CleartextInputVerifier(inputVerifierAdd);
        _kmsVerifier = CleartextKMSVerifier(kmsVerifierAdd);
    }

    // ------------------------------------------------------------------------
    // Encrypt DSL
    // ------------------------------------------------------------------------

    function encryptBool(bool value, address target) internal returns (externalEbool, bytes memory) {
        return encryptBool(value, address(this), target);
    }

    function encryptBool(bool value, address user, address target) internal returns (externalEbool, bytes memory) {
        (bytes32 handle, bytes memory inputProof) = _encrypt(value ? 1 : 0, FheType.Bool, user, target);
        return (externalEbool.wrap(handle), inputProof);
    }

    function encryptUint8(uint8 value, address target) internal returns (externalEuint8, bytes memory) {
        return encryptUint8(value, address(this), target);
    }

    function encryptUint8(uint8 value, address user, address target) internal returns (externalEuint8, bytes memory) {
        (bytes32 handle, bytes memory inputProof) = _encrypt(value, FheType.Uint8, user, target);
        return (externalEuint8.wrap(handle), inputProof);
    }

    function encryptUint16(uint16 value, address target) internal returns (externalEuint16, bytes memory) {
        return encryptUint16(value, address(this), target);
    }

    function encryptUint16(uint16 value, address user, address target)
        internal
        returns (externalEuint16, bytes memory)
    {
        (bytes32 handle, bytes memory inputProof) = _encrypt(value, FheType.Uint16, user, target);
        return (externalEuint16.wrap(handle), inputProof);
    }

    function encryptUint32(uint32 value, address target) internal returns (externalEuint32, bytes memory) {
        return encryptUint32(value, address(this), target);
    }

    function encryptUint32(uint32 value, address user, address target)
        internal
        returns (externalEuint32, bytes memory)
    {
        (bytes32 handle, bytes memory inputProof) = _encrypt(value, FheType.Uint32, user, target);
        return (externalEuint32.wrap(handle), inputProof);
    }

    function encryptUint64(uint64 value, address target) internal returns (externalEuint64, bytes memory) {
        return encryptUint64(value, address(this), target);
    }

    function encryptUint64(uint64 value, address user, address target)
        internal
        returns (externalEuint64, bytes memory)
    {
        (bytes32 handle, bytes memory inputProof) = _encrypt(value, FheType.Uint64, user, target);
        return (externalEuint64.wrap(handle), inputProof);
    }

    function encryptUint128(uint128 value, address target) internal returns (externalEuint128, bytes memory) {
        return encryptUint128(value, address(this), target);
    }

    function encryptUint128(uint128 value, address user, address target)
        internal
        returns (externalEuint128, bytes memory)
    {
        (bytes32 handle, bytes memory inputProof) = _encrypt(value, FheType.Uint128, user, target);
        return (externalEuint128.wrap(handle), inputProof);
    }

    function encryptUint256(uint256 value, address target) internal returns (externalEuint256, bytes memory) {
        return encryptUint256(value, address(this), target);
    }

    function encryptUint256(uint256 value, address user, address target)
        internal
        returns (externalEuint256, bytes memory)
    {
        (bytes32 handle, bytes memory inputProof) = _encrypt(value, FheType.Uint256, user, target);
        return (externalEuint256.wrap(handle), inputProof);
    }

    function encryptAddress(address value, address target) internal returns (externalEaddress, bytes memory) {
        return encryptAddress(value, address(this), target);
    }

    function encryptAddress(address value, address user, address target)
        internal
        returns (externalEaddress, bytes memory)
    {
        (bytes32 handle, bytes memory inputProof) = _encrypt(uint256(uint160(value)), FheType.Uint160, user, target);
        return (externalEaddress.wrap(handle), inputProof);
    }

    function encrypt(uint256[] memory values, FheType[] memory fheTypes, address target)
        internal
        returns (bytes32[] memory handles, bytes memory inputProof)
    {
        return _encrypt(values, fheTypes, address(this), target);
    }

    function encrypt(uint256[] memory values, FheType[] memory fheTypes, address user, address target)
        internal
        returns (bytes32[] memory handles, bytes memory inputProof)
    {
        return _encrypt(values, fheTypes, user, target);
    }

    // ------------------------------------------------------------------------
    // Decrypt DSL — reads cleartext ON-CHAIN from CleartextACL (no event replay)
    // ------------------------------------------------------------------------

    /// @notice Reads a cleartext value by handle from the on-chain CleartextACL.
    function decrypt(bytes32 handle) internal view returns (uint256) {
        return _acl.plaintext(handle);
    }

    function decrypt(ebool value) internal view returns (bool) {
        return decrypt(ebool.unwrap(value)) != 0;
    }

    function decrypt(euint8 value) internal view returns (uint8) {
        return uint8(decrypt(euint8.unwrap(value)));
    }

    function decrypt(euint16 value) internal view returns (uint16) {
        return uint16(decrypt(euint16.unwrap(value)));
    }

    function decrypt(euint32 value) internal view returns (uint32) {
        return uint32(decrypt(euint32.unwrap(value)));
    }

    function decrypt(euint64 value) internal view returns (uint64) {
        return uint64(decrypt(euint64.unwrap(value)));
    }

    function decrypt(euint128 value) internal view returns (uint128) {
        return uint128(decrypt(euint128.unwrap(value)));
    }

    function decrypt(euint256 value) internal view returns (uint256) {
        return decrypt(euint256.unwrap(value));
    }

    function decrypt(eaddress value) internal view returns (address) {
        return address(uint160(decrypt(eaddress.unwrap(value))));
    }

    /// @notice Decrypts handles that were marked as publicly decryptable and returns a KMS-style proof.
    function publicDecrypt(bytes32[] memory handles)
        internal
        view
        returns (uint256[] memory cleartexts, bytes memory decryptionProof)
    {
        cleartexts = new uint256[](handles.length);
        for (uint256 i = 0; i < handles.length; i++) {
            if (!_acl.isAllowedForDecryption(handles[i])) {
                revert HandleNotAllowedForPublicDecryption(handles[i]);
            }
            cleartexts[i] = _acl.plaintext(handles[i]);
        }

        bytes memory abiEncodedCleartexts = abi.encodePacked(cleartexts);
        decryptionProof = buildDecryptionProof(handles, abiEncodedCleartexts);
    }

    /// @notice Decrypts a single handle for a user after persistent ACL checks and signature verification.
    function userDecrypt(bytes32 handle, address userAddress, address contractAddress, bytes memory userSignature)
        internal
        view
        returns (uint256)
    {
        if (userAddress == contractAddress) {
            revert UserAddressEqualsContractAddress();
        }
        if (!_acl.persistAllowed(handle, userAddress)) {
            revert UserNotAuthorizedForDecrypt(handle, userAddress);
        }
        if (!_acl.persistAllowed(handle, contractAddress)) {
            revert ContractNotAuthorizedForDecrypt(handle, contractAddress);
        }

        address[] memory contractAddresses = new address[](1);
        contractAddresses[0] = contractAddress;
        bytes32 domainSeparator = UserDecryptHelper.computeUserDecryptDomainSeparator(block.chainid, kmsVerifierAdd);
        bytes32 digest = UserDecryptHelper.computeUserDecryptDigest(
            abi.encodePacked(userAddress),
            contractAddresses,
            block.timestamp,
            DEFAULT_USER_DECRYPT_DURATION_DAYS,
            EMPTY_EXTRA_DATA,
            domainSeparator
        );

        (uint8 v, bytes32 r, bytes32 s) = _decodeSignature(userSignature);
        address recoveredSigner = ecrecover(digest, v, r, s);
        if (recoveredSigner == address(0) || recoveredSigner != userAddress) {
            revert InvalidUserDecryptSignature();
        }

        return _acl.plaintext(handle);
    }

    /// @notice Decrypts a handle delegated for user decryption, verifying the delegate's signature.
    function userDecryptDelegated(
        bytes32 handle,
        address delegator,
        address delegate,
        address contractAddress,
        bytes memory delegateSignature
    ) internal view returns (uint256) {
        if (!_acl.isHandleDelegatedForUserDecryption(delegator, delegate, contractAddress, handle)) {
            revert HandleNotDelegatedForDecrypt(handle, delegator, delegate, contractAddress);
        }

        address[] memory contractAddresses = new address[](1);
        contractAddresses[0] = contractAddress;
        bytes32 domainSeparator = UserDecryptHelper.computeUserDecryptDomainSeparator(block.chainid, kmsVerifierAdd);
        bytes32 digest = UserDecryptHelper.computeDelegatedUserDecryptDigest(
            abi.encodePacked(delegate),
            contractAddresses,
            delegator,
            block.timestamp,
            DEFAULT_USER_DECRYPT_DURATION_DAYS,
            EMPTY_EXTRA_DATA,
            domainSeparator
        );

        (uint8 v, bytes32 r, bytes32 s) = _decodeSignature(delegateSignature);
        address recoveredSigner = ecrecover(digest, v, r, s);
        if (recoveredSigner == address(0) || recoveredSigner != delegate) {
            revert InvalidUserDecryptSignature();
        }

        return _acl.plaintext(handle);
    }

    // ------------------------------------------------------------------------
    // Decryption-proof + signing helpers (EIP-712)
    // ------------------------------------------------------------------------

    function buildDecryptionProof(bytes32[] memory handles, bytes memory abiEncodedCleartexts)
        internal
        view
        returns (bytes memory proof)
    {
        (, string memory name, string memory version, uint256 chainId, address verifyingContract,,) =
            _kmsVerifier.eip712Domain();
        bytes32 domainSeparator =
            KMSDecryptionProofHelper.computeKMSDecryptionDomainSeparator(name, version, chainId, verifyingContract);
        bytes32 digest = KMSDecryptionProofHelper.computeDecryptionDigest(
            handles, abiEncodedCleartexts, EMPTY_EXTRA_DATA, domainSeparator
        );

        bytes[] memory signatures = new bytes[](1);
        signatures[0] = _signDigest(MOCK_KMS_SIGNER_PK, digest);
        proof = KMSDecryptionProofHelper.assembleDecryptionProof(signatures, EMPTY_EXTRA_DATA);
    }

    function buildDecryptionProof(bytes32 handle, bytes memory abiEncodedCleartext)
        internal
        view
        returns (bytes memory proof)
    {
        bytes32[] memory handles = new bytes32[](1);
        handles[0] = handle;
        proof = buildDecryptionProof(handles, abiEncodedCleartext);
    }

    function signUserDecrypt(uint256 userPk, address contractAddress) internal view returns (bytes memory signature) {
        address[] memory contractAddresses = new address[](1);
        contractAddresses[0] = contractAddress;
        return signUserDecrypt(userPk, contractAddresses, block.timestamp, DEFAULT_USER_DECRYPT_DURATION_DAYS);
    }

    function signUserDecrypt(
        uint256 userPk,
        address[] memory contractAddresses,
        uint256 startTimestamp,
        uint256 durationDays
    ) internal view returns (bytes memory signature) {
        address userAddress = vm.addr(userPk);
        bytes32 domainSeparator = UserDecryptHelper.computeUserDecryptDomainSeparator(block.chainid, kmsVerifierAdd);
        bytes32 digest = UserDecryptHelper.computeUserDecryptDigest(
            abi.encodePacked(userAddress),
            contractAddresses,
            startTimestamp,
            durationDays,
            EMPTY_EXTRA_DATA,
            domainSeparator
        );
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(userPk, digest);
        signature = abi.encodePacked(r, s, v);
    }

    function signDelegatedUserDecrypt(uint256 delegatePk, address delegator, address contractAddress)
        internal
        view
        returns (bytes memory signature)
    {
        address[] memory contractAddresses = new address[](1);
        contractAddresses[0] = contractAddress;
        return signDelegatedUserDecrypt(
            delegatePk, delegator, contractAddresses, block.timestamp, DEFAULT_USER_DECRYPT_DURATION_DAYS
        );
    }

    function signDelegatedUserDecrypt(
        uint256 delegatePk,
        address delegator,
        address[] memory contractAddresses,
        uint256 startTimestamp,
        uint256 durationDays
    ) internal view returns (bytes memory signature) {
        address delegate = vm.addr(delegatePk);
        bytes32 domainSeparator = UserDecryptHelper.computeUserDecryptDomainSeparator(block.chainid, kmsVerifierAdd);
        bytes32 digest = UserDecryptHelper.computeDelegatedUserDecryptDigest(
            abi.encodePacked(delegate),
            contractAddresses,
            delegator,
            startTimestamp,
            durationDays,
            EMPTY_EXTRA_DATA,
            domainSeparator
        );
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(delegatePk, digest);
        signature = abi.encodePacked(r, s, v);
    }

    // ------------------------------------------------------------------------
    // Internal: deploy the cleartext stack at canonical addresses
    // ------------------------------------------------------------------------

    function _deployAllContracts() internal {
        _deployPauserSet();
        _deployACL();
        _deployHCULimit();
        _deployExecutor();
        _deployInputVerifier();
        // ProtocolConfig holds the KMS signer set that KMSVerifier reads at verification time,
        // so it must be deployed and initialized before KMSVerifier is used.
        _deployProtocolConfig();
        _deployKMSGeneration();
        _deployKMSVerifier();

        _executor = CleartextFHEVMExecutor(fhevmExecutorAdd);
        _acl = CleartextACL(aclAdd);
        _inputVerifier = CleartextInputVerifier(inputVerifierAdd);
        _kmsVerifier = CleartextKMSVerifier(kmsVerifierAdd);
    }

    function _deployPauserSet() internal {
        vm.etch(pauserSetAdd, address(new PauserSet()).code);
    }

    function _deployACL() internal {
        address emptyProxyAclImpl = address(new EmptyUUPSProxyACL());
        deployCodeTo(
            ERC1967_PROXY_ARTIFACT,
            abi.encode(emptyProxyAclImpl, abi.encodeCall(EmptyUUPSProxyACL.initialize, (PROXY_OWNER))),
            aclAdd
        );

        address aclImpl = address(new CleartextACL());
        vm.prank(PROXY_OWNER);
        EmptyUUPSProxyACL(aclAdd).upgradeToAndCall(aclImpl, abi.encodeCall(ACL.initializeFromEmptyProxy, ()));
    }

    function _deployHCULimit() internal {
        address emptyProxyImpl = address(new EmptyUUPSProxy());
        deployCodeTo(
            ERC1967_PROXY_ARTIFACT,
            abi.encode(emptyProxyImpl, abi.encodeCall(EmptyUUPSProxy.initialize, ())),
            hcuLimitAdd
        );

        address hcuLimitImpl = address(new CleartextHCULimit());
        vm.prank(PROXY_OWNER);
        EmptyUUPSProxy(hcuLimitAdd).upgradeToAndCall(
            hcuLimitImpl, abi.encodeCall(HCULimit.initializeFromEmptyProxy, (20_000_000, 5_000_000, 20_000_000))
        );
    }

    function _deployExecutor() internal {
        address emptyProxyImpl = address(new EmptyUUPSProxy());
        deployCodeTo(
            ERC1967_PROXY_ARTIFACT,
            abi.encode(emptyProxyImpl, abi.encodeCall(EmptyUUPSProxy.initialize, ())),
            fhevmExecutorAdd
        );

        address executorImpl = address(new CleartextFHEVMExecutor());
        vm.prank(PROXY_OWNER);
        EmptyUUPSProxy(fhevmExecutorAdd).upgradeToAndCall(
            executorImpl, abi.encodeCall(FHEVMExecutor.initializeFromEmptyProxy, ())
        );
    }

    function _deployInputVerifier() internal {
        address emptyProxyImpl = address(new EmptyUUPSProxy());
        deployCodeTo(
            ERC1967_PROXY_ARTIFACT,
            abi.encode(emptyProxyImpl, abi.encodeCall(EmptyUUPSProxy.initialize, ())),
            inputVerifierAdd
        );

        address inputVerifierImpl = address(new CleartextInputVerifier());
        address[] memory signers = new address[](1);
        signers[0] = mockInputSigner;

        vm.prank(PROXY_OWNER);
        EmptyUUPSProxy(inputVerifierAdd).upgradeToAndCall(
            inputVerifierImpl,
            abi.encodeCall(
                InputVerifier.initializeFromEmptyProxy, (inputVerifierAdd, uint64(block.chainid), signers, 1)
            )
        );
    }

    /// @dev v0.14: the KMS signer set lives in ProtocolConfig (KMSVerifier reads it via
    ///      `PROTOCOL_CONFIG.getKmsSigners()`). Register a single mock node whose signer is
    ///      `mockKmsSigner`, with all thresholds set to 1.
    function _deployProtocolConfig() internal {
        address emptyProxyImpl = address(new EmptyUUPSProxy());
        deployCodeTo(
            ERC1967_PROXY_ARTIFACT,
            abi.encode(emptyProxyImpl, abi.encodeCall(EmptyUUPSProxy.initialize, ())),
            protocolConfigAdd
        );

        KmsNodeParams[] memory nodes = new KmsNodeParams[](1);
        nodes[0] = KmsNodeParams({
            txSenderAddress: MOCK_KMS_TX_SENDER,
            signerAddress: mockKmsSigner,
            ipAddress: "",
            storageUrl: "",
            partyId: int32(1),
            mpcIdentity: "",
            caCert: "",
            storagePrefix: ""
        });
        IProtocolConfig.KmsThresholds memory thresholds =
            IProtocolConfig.KmsThresholds({publicDecryption: 1, userDecryption: 1, kmsGen: 1, mpc: 1});
        PcrValues[] memory pcrValues = new PcrValues[](0);

        address protocolConfigImpl = address(new CleartextProtocolConfig());
        vm.prank(PROXY_OWNER);
        EmptyUUPSProxy(protocolConfigAdd).upgradeToAndCall(
            protocolConfigImpl,
            abi.encodeCall(ProtocolConfig.initializeFromEmptyProxy, (nodes, thresholds, "1", pcrValues))
        );
    }

    function _deployKMSGeneration() internal {
        address emptyProxyImpl = address(new EmptyUUPSProxy());
        deployCodeTo(
            ERC1967_PROXY_ARTIFACT,
            abi.encode(emptyProxyImpl, abi.encodeCall(EmptyUUPSProxy.initialize, ())),
            kmsGenerationAdd
        );

        address kmsGenerationImpl = address(new CleartextKMSGeneration());
        vm.prank(PROXY_OWNER);
        EmptyUUPSProxy(kmsGenerationAdd).upgradeToAndCall(
            kmsGenerationImpl, abi.encodeCall(KMSGeneration.initializeFromEmptyProxy, ())
        );
    }

    function _deployKMSVerifier() internal {
        address emptyProxyImpl = address(new EmptyUUPSProxy());
        deployCodeTo(
            ERC1967_PROXY_ARTIFACT,
            abi.encode(emptyProxyImpl, abi.encodeCall(EmptyUUPSProxy.initialize, ())),
            kmsVerifierAdd
        );

        address kmsVerifierImpl = address(new CleartextKMSVerifier());
        vm.prank(PROXY_OWNER);
        EmptyUUPSProxy(kmsVerifierAdd).upgradeToAndCall(
            kmsVerifierImpl,
            abi.encodeCall(KMSVerifier.initializeFromEmptyProxy, (kmsVerifierAdd, uint64(block.chainid)))
        );
    }

    // ------------------------------------------------------------------------
    // Internal: encrypt
    // ------------------------------------------------------------------------

    function _encrypt(uint256 value, FheType fheType, address user, address target)
        internal
        returns (bytes32 handle, bytes memory inputProof)
    {
        uint256[] memory values = new uint256[](1);
        values[0] = value;
        FheType[] memory fheTypes = new FheType[](1);
        fheTypes[0] = fheType;

        (bytes32[] memory handles, bytes memory proof) = _encrypt(values, fheTypes, user, target);
        handle = handles[0];
        inputProof = proof;
    }

    /// @dev Builds a mock input proof whose `extraData` region carries the normalized cleartexts.
    ///      The canonical `CleartextFHEVMExecutor.verifyInput` reads those cleartexts back
    ///      (`_tryReadCleartextFromProof`) and records them in `CleartextACL`; the real
    ///      `InputVerifier` treats the same bytes as EIP-712 `extraData`, so the signature is
    ///      computed over them.
    function _encrypt(uint256[] memory values, FheType[] memory fheTypes, address user, address target)
        internal
        returns (bytes32[] memory handles, bytes memory inputProof)
    {
        uint256 valuesLength = values.length;
        if (valuesLength != fheTypes.length) {
            revert EncryptInputLengthMismatch(valuesLength, fheTypes.length);
        }
        if (valuesLength > type(uint8).max) {
            revert EncryptInputTooLong();
        }

        handles = new bytes32[](valuesLength);
        bytes memory cleartexts;
        for (uint256 i; i < valuesLength; ++i) {
            _encryptNonce += 1;

            bytes memory ciphertext =
                abi.encodePacked(keccak256(abi.encodePacked(values[i], uint8(fheTypes[i]), _encryptNonce)));
            handles[i] =
                InputProofHelper.computeInputHandle(ciphertext, uint8(i), fheTypes[i], aclAdd, uint64(block.chainid));

            cleartexts = abi.encodePacked(
                cleartexts, CleartextArithmetic.normalizePlaintextToType(values[i], fheTypes[i])
            );
        }

        bytes32 domainSeparator = InputProofHelper.computeInputVerifierDomainSeparator(inputVerifierAdd, block.chainid);
        bytes32 digest = InputProofHelper.computeInputVerificationDigest(
            handles, user, target, block.chainid, cleartexts, domainSeparator
        );

        bytes[] memory signatures = new bytes[](1);
        signatures[0] = _signDigest(MOCK_INPUT_SIGNER_PK, digest);
        inputProof = InputProofHelper.assembleInputProof(handles, signatures, cleartexts);
    }

    function _decodeSignature(bytes memory signature) internal pure returns (uint8 v, bytes32 r, bytes32 s) {
        if (signature.length != 65) {
            revert InvalidUserDecryptSignature();
        }
        assembly {
            r := mload(add(signature, 0x20))
            s := mload(add(signature, 0x40))
            v := byte(0, mload(add(signature, 0x60)))
        }
        if (v < 27) {
            v += 27;
        }
        if (v != 27 && v != 28) {
            revert InvalidUserDecryptSignature();
        }
    }

    function _signDigest(uint256 signerPk, bytes32 digest) internal pure returns (bytes memory signature) {
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPk, digest);
        signature = abi.encodePacked(r, s, v);
    }
}
