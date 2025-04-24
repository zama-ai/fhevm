// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {UnsafeUpgrades} from "@openzeppelin/foundry-upgrades/src/Upgrades.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";

import {InputVerifier} from "../../contracts/InputVerifier.sol";
import {EmptyUUPSProxy} from "../../contracts/emptyProxy/EmptyUUPSProxy.sol";
import {FheType} from "../../contracts/FheType.sol";
import {FHEVMExecutorNoEvents} from "../../contracts/FHEVMExecutorNoEvents.sol";

contract InputVerifierTest is Test {
    InputVerifier internal inputVerifier;

    address internal constant verifyingContractSource = address(10000);
    address internal constant owner = address(456);
    uint8 internal constant HANDLE_VERSION = 0;

    /// @dev Signer variables.
    uint256 internal constant privateKeySigner0 = 0x022;
    uint256 internal constant privateKeySigner1 = 0x03;
    uint256 internal constant privateKeySigner2 = 0x04;
    uint256 internal constant privateKeySigner3 = 0x05;
    uint256 internal constant privateKeySigner4 = 0x06;
    address[] internal activeSigners;
    mapping(address => uint256) internal signerPrivateKeys;
    address internal signer0;
    address internal signer1;
    address internal signer2;
    address internal signer3;
    address internal signer4;

    /// @dev Proxy and implementation variables
    address internal proxy;
    address internal implementation;

    /**
     * @dev Computes the signature for a given digest using the provided private key.
     * @param privateKey The private key used to sign the digest.
     * @param digest The hash of the data to be signed.
     * @return signature The computed signature as a byte array, encoded as {r}{s}{v}.
     */
    function _computeSignature(uint256 privateKey, bytes32 digest) internal pure returns (bytes memory signature) {
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(privateKey, digest);
        return abi.encodePacked(r, s, v);
    }

    /**
     * @dev Computes the EIP-712 digest for a given set of inputs.
     *
     * This function generates a digest by hashing the provided data using the EIP-712 standard.
     * It combines the domain separator and the struct hash to produce a unique digest.
     *
     * @param handlesList An array of bytes32 values representing the handles list.
     * @param userAddress The address of the user for whom the digest is being computed.
     * @param contractAddress The address of the contract involved in the computation.
     * @param chainId The chain ID of the blockchain network.
     *
     * @return bytes32 The computed EIP-712 digest.
     */
    function _computeDigest(
        bytes32[] memory handlesList,
        address userAddress,
        address contractAddress,
        uint256 chainId
    ) internal view returns (bytes32) {
        bytes32 structHash = keccak256(
            abi.encode(
                inputVerifier.EIP712_INPUT_VERIFICATION_TYPEHASH(),
                keccak256(abi.encodePacked(handlesList)),
                userAddress,
                contractAddress,
                chainId
            )
        );

        bytes32 hashTypeData = MessageHashUtils.toTypedDataHash(_computeDomainSeparator(), structHash);
        return hashTypeData;
    }

    /**
     * @dev Computes the EIP-712 domain separator.
     * This function retrieves the domain parameters from the `inputVerifier` contract,
     * including the name, version, chain ID, and verifying contract address.
     * It then encodes these parameters and hashes them using the keccak256 algorithm
     * to produce the domain separator.
     *
     * @return bytes32 The computed domain separator.
     */
    function _computeDomainSeparator() internal view returns (bytes32) {
        (, string memory name, string memory version, uint256 chainId, address verifyingContract, , ) = inputVerifier
            .eip712Domain();

        return
            keccak256(
                abi.encode(
                    keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"),
                    keccak256(bytes(name)),
                    keccak256(bytes(version)),
                    chainId,
                    verifyingContract
                )
            );
    }

    /**
     * @dev Internal function to deploy a UUPS proxy contract.
     * The proxy is deployed using the UnsafeUpgrades library and initialized with the owner address.
     */
    function _deployProxy() internal {
        proxy = UnsafeUpgrades.deployUUPSProxy(
            address(new EmptyUUPSProxy()),
            abi.encodeCall(EmptyUUPSProxy.initialize, owner)
        );
    }

    /**
     * @dev Internal function to upgrade a proxy contract to a new implementation of `InputVerifier`.
     *
     * This function deploys a new instance of the `InputVerifier` contract and upgrades the proxy
     * to use the new implementation. It also reinitializes the proxy with the provided parameters.
     *
     * @param signers An array of addresses representing the signers to be used during reinitialization.
     *
     * The function performs the following steps:
     * 1. Deploys a new `InputVerifier` contract and sets its address as the new implementation.
     * 2. Uses `UnsafeUpgrades.upgradeProxy` to upgrade the proxy to the new implementation.
     *    - Passes the encoded call to `InputVerifier.reinitialize` with the required parameters:
     *      - `verifyingContractSource`: The source of the verifying contract.
     *      - `uint64(block.chainid)`: The chain ID of the current blockchain.
     *      - `signers`: The array of signers.
     *    - `owner`: The owner of the proxy.
     * 3. Updates the `inputVerifier` reference to point to the proxy.
     */
    function _upgradeProxy(address[] memory signers) internal {
        implementation = address(new InputVerifier());
        UnsafeUpgrades.upgradeProxy(
            proxy,
            implementation,
            abi.encodeCall(InputVerifier.reinitialize, (verifyingContractSource, uint64(block.chainid), signers)),
            owner
        );
        inputVerifier = InputVerifier(proxy);
    }

    /**
     * @dev Initializes signer addresses and maps them to their private keys.
     */
    function _initializeSigners() internal {
        signer0 = vm.addr(privateKeySigner0);
        signer1 = vm.addr(privateKeySigner1);
        signer2 = vm.addr(privateKeySigner2);
        signer3 = vm.addr(privateKeySigner3);
        signer4 = vm.addr(privateKeySigner4);

        signerPrivateKeys[signer0] = privateKeySigner0;
        signerPrivateKeys[signer1] = privateKeySigner1;
        signerPrivateKeys[signer2] = privateKeySigner2;
        signerPrivateKeys[signer3] = privateKeySigner3;
        signerPrivateKeys[signer4] = privateKeySigner4;
    }

    /**
     * @dev Appends metadata to `prehandle` by modifying specific bytes.
     * - Clears bytes 21-31.
     * - Sets byte 21 to `0x00`.
     * - Inserts `chainId`, `fheType`, and `handleVersion` into respective bytes.
     * @return result Modified `prehandle` with metadata.
     */
    function _appendMetadataToPrehandle(
        FheType fheType,
        bytes32 prehandle,
        uint256 chainId,
        uint8 handleVersion
    ) internal view virtual returns (bytes32 result) {
        /// @dev Clear bytes 21-31.
        result = prehandle & 0xffffffffffffffffffffffffffffffffffffffffff0000000000000000000000;
        /// @dev Set byte 21 to 0x00 to make sure it passes the check in InputVerifier (since the number of handles is always not null)
        result = result | (bytes32(uint256(0x00)) << 80);
        /// @dev chainId is cast to uint64 first to make sure it does not take more than 8 bytes before shifting.
        /// If EIP2294 gets approved, it will force the chainID's size to be lower than MAX_UINT64.
        result = result | (bytes32(uint256(uint64(chainId))) << 16);
        /// @dev Insert handleType into byte 30.
        result = result | (bytes32(uint256(uint8(fheType))) << 8);
        /// @dev Insert HANDLE_VERSION into byte 31.
        result = result | bytes32(uint256(handleVersion));
    }

    /**
     * @dev Computes a mock handle by hashing the input value and appending metadata.
     * @param value The input value to be hashed.
     * @param fheType The FHE (Fully Homomorphic Encryption) type associated with the handle.
     * @param chainId The ID of the blockchain network.
     * @param handleVersion The version of the handle format.
     * @return handle The computed handle as a bytes32 value.
     */
    function computeMockHandle(
        bytes32 value,
        FheType fheType,
        uint256 chainId,
        uint8 handleVersion
    ) internal view returns (bytes32 handle) {
        bytes32 prehandle = keccak256(abi.encodePacked(value));
        handle = _appendMetadataToPrehandle(fheType, prehandle, chainId, handleVersion);
    }

    /**
     * @dev Computes an input proof by encoding the provided handles and signatures.
     *      The resulting proof is a concatenation of the number of handles, the number
     *      of signatures, the handles themselves, and the signatures.
     *
     * @param handles An array of bytes32 values representing the handles.
     * @param signatures An array of bytes representing the signatures.
     * @return inputProof A bytes array containing the encoded input proof.
     */
    function _computeInputProof(
        bytes32[] memory handles,
        bytes[] memory signatures
    ) internal pure returns (bytes memory inputProof) {
        inputProof = abi.encodePacked(uint8(handles.length), uint8(signatures.length), handles);
        for (uint256 i = 0; i < signatures.length; i++) {
            inputProof = abi.encodePacked(inputProof, signatures[i]);
        }
    }

    /**
     * @dev Generates signatures for the given inputs.
     * @param handles Handles included in the digest.
     * @param userAddress User's address.
     * @param contractAddress Contract's address.
     * @param signers Signers' addresses.
     * @param chainId Blockchain network ID.
     * @return signatures Array of generated signatures.
     */
    function _generateSignatures(
        bytes32[] memory handles,
        address userAddress,
        address contractAddress,
        address[] memory signers,
        uint256 chainId
    ) internal view returns (bytes[] memory signatures) {
        signatures = new bytes[](signers.length);
        for (uint256 i = 0; i < signers.length; i++) {
            /// @dev The signer address must have its private key in the mapping.
            assert(signerPrivateKeys[signers[i]] != 0);
            bytes32 digest = _computeDigest(handles, userAddress, contractAddress, chainId);
            signatures[i] = _computeSignature(signerPrivateKeys[signers[i]], digest);
        }
    }

    /**
     * @dev Generates input parameters for FHEVMExecutorNoEvents context.
     *
     * @param cleartextValues Cleartext values for handles.
     * @param fheTypes FHE types for each cleartext value.
     * @param userAddress User's address.
     * @param contractAddress Contract's address.
     * @param chainId Blockchain ID.
     * @param handleVersion Handle version.
     * @param signers Signers' addresses.
     *
     * @return context Context with user and contract addresses.
     * @return mockInputHandle Computed input handle.
     * @return inputProof Computed input proof.
     *
     * @notice Asserts `cleartextValues` and `fheTypes` lengths match.
     */
    function _generateMockInputParameters(
        bytes32[] memory cleartextValues,
        FheType[] memory fheTypes,
        address userAddress,
        address contractAddress,
        uint256 chainId,
        uint8 handleVersion,
        address[] memory signers
    )
        internal
        view
        returns (
            FHEVMExecutorNoEvents.ContextUserInputs memory context,
            bytes32 mockInputHandle,
            bytes memory inputProof
        )
    {
        assert(cleartextValues.length == fheTypes.length && cleartextValues.length > 0);
        bytes32[] memory handles = new bytes32[](cleartextValues.length);

        for (uint256 i = 0; i < cleartextValues.length; i++) {
            handles[i] = computeMockHandle(cleartextValues[i], fheTypes[i], chainId, handleVersion);
        }

        /// @dev The first handle is used as the input handle for mock purposes.
        mockInputHandle = handles[0];

        bytes[] memory signatures = _generateSignatures(handles, userAddress, contractAddress, signers, chainId);
        inputProof = _computeInputProof(handles, signatures);

        context.userAddress = userAddress;
        context.contractAddress = contractAddress;
    }

    /**
     * @dev Generates input parameters with a single mock handle for testing.
     * @param chainId Blockchain ID.
     * @param handleVersion Handle version.
     * @param signers Signers' addresses.
     * @return context Context with user and contract addresses.
     * @return mockInputHandle Mock input handle.
     * @return inputProof Input proof.
     */
    function _generateInputParametersWithOneMockHandle(
        uint256 chainId,
        uint8 handleVersion,
        address[] memory signers
    )
        internal
        view
        returns (
            FHEVMExecutorNoEvents.ContextUserInputs memory context,
            bytes32 mockInputHandle,
            bytes memory inputProof
        )
    {
        address userAddress = address(1234);
        address contractAddress = address(2222);
        bytes32[] memory cleartextValues = new bytes32[](1);
        FheType[] memory fheTypes = new FheType[](1);
        fheTypes[0] = FheType.Uint64;
        cleartextValues[0] = bytes32(uint256(250));

        return
            _generateMockInputParameters(
                cleartextValues,
                fheTypes,
                userAddress,
                contractAddress,
                chainId,
                handleVersion,
                signers
            );
    }

    /**
     * @dev Upgrades the proxy with a specified number of signers (1-5).
     * Adds signers (signer0 to signer4) to `activeSigners` based on `numberSigners`.
     * Calls `_upgradeProxy` with the updated `activeSigners`.
     *
     * @param numberSigners Number of signers (1-5).
     */
    function _upgradeProxyWithSigners(uint256 numberSigners) internal {
        assert(numberSigners > 0 && numberSigners < 6);

        if (numberSigners >= 1) {
            activeSigners.push(signer0);
        }
        if (numberSigners >= 2) {
            activeSigners.push(signer1);
        }
        if (numberSigners >= 3) {
            activeSigners.push(signer2);
        }
        if (numberSigners >= 4) {
            activeSigners.push(signer3);
        }
        if (numberSigners == 5) {
            activeSigners.push(signer4);
        }

        _upgradeProxy(activeSigners);
    }

    /**
     * @dev Sets up the testing environment by deploying a proxy contract and initializing signers.
     * This function is executed before each test to ensure a consistent and isolated state.
     */
    function setUp() public {
        _deployProxy();
        _initializeSigners();
    }

    /**
     * @dev Tests that the contract is initialized correctly.
     */
    function test_PostProxyUpgradeCheck() public {
        _upgradeProxyWithSigners(3);
        assertEq(inputVerifier.owner(), owner);
        assertEq(inputVerifier.getVersion(), string(abi.encodePacked("InputVerifier v0.1.0")));
    }

    /**
     * @dev Tests that the getCoprocessorSigners view function works as expected.
     */
    function test_GetCoprocessorSigners() public {
        _upgradeProxyWithSigners(3);
        address[] memory signers = inputVerifier.getCoprocessorSigners();
        assertEq(signers.length, 3);
        assertEq(signers[0], signer0);
        assertEq(signers[1], signer1);
        assertEq(signers[2], signer2);
        assertTrue(inputVerifier.isSigner(signers[0]));
        assertTrue(inputVerifier.isSigner(signer1));
        assertTrue(inputVerifier.isSigner(signer2));
    }

    /**
     * @dev Tests that the initial threshold is set correctly for two signers.
     */
    function test_GetInitialThresholdForOneSigner() public {
        _upgradeProxyWithSigners(1);

        /// @dev The threshold is 1 since we have 1 signers.
        uint256 threshold = inputVerifier.getThreshold();
        assertEq(threshold, 1);
    }

    /**
     * @dev Tests that the initial threshold is set correctly for two signers.
     */
    function test_GetInitialThresholdForTwoSigners() public {
        _upgradeProxyWithSigners(2);

        /// @dev The threshold is 2 since we have 2 signers.
        uint256 threshold = inputVerifier.getThreshold();
        assertEq(threshold, 2);
    }

    /**
     * @dev Tests that the initial threshold is set correctly for three signers.
     */
    function test_GetInitialThresholdForThreeSigners() public {
        _upgradeProxyWithSigners(3);

        /// @dev The threshold is 2 since we have 3 signers.
        uint256 threshold = inputVerifier.getThreshold();
        assertEq(threshold, 2);
    }

    /**
     * @dev Tests that the initial threshold is set correctly for four signers.
     */
    function test_GetInitialThresholdForFourSigners() public {
        _upgradeProxyWithSigners(4);

        /// @dev The threshold is 3 since we have 4 signers.
        uint256 threshold = inputVerifier.getThreshold();
        assertEq(threshold, 3);
    }

    /**
     * @dev Tests that the contract owner cannot add a null address as a signer.
     */
    function test_OwnerCannotAddNullAddressAsSigner() public {
        _upgradeProxyWithSigners(3);
        vm.expectPartialRevert(InputVerifier.SignerNull.selector);
        vm.prank(owner);
        inputVerifier.addSigner(address(0));
    }

    /**
     * @dev Tests that the verify ciphertext function works as expected for one input.
     */
    function test_VerifyCiphertextWorksAsExpectedForOneInput(uint64 cleartextValue) public {
        _upgradeProxyWithSigners(3);
        address userAddress = address(1111);
        address contractAddress = address(2222);
        bytes32[] memory cleartextValues = new bytes32[](1);
        FheType[] memory fheTypes = new FheType[](1);
        fheTypes[0] = FheType.Uint64;
        cleartextValues[0] = bytes32(uint256(cleartextValue));

        (
            FHEVMExecutorNoEvents.ContextUserInputs memory context,
            bytes32 mockInputHandle,
            bytes memory inputProof
        ) = _generateMockInputParameters(
                cleartextValues,
                fheTypes,
                userAddress,
                contractAddress,
                block.chainid,
                HANDLE_VERSION,
                activeSigners
            );

        vm.assertEq(mockInputHandle, inputVerifier.verifyCiphertext(context, mockInputHandle, inputProof));
    }

    /**
     * @dev Tests that the verify ciphertext function works as expected for one input.
     */
    function test_VerifyCiphertextWorksAsExpectedForTwoInputs(uint64 cleartextValue) public {
        _upgradeProxyWithSigners(3);

        address userAddress = address(1111);
        address contractAddress = address(2222);

        bytes32[] memory cleartextValues = new bytes32[](2);
        FheType[] memory fheTypes = new FheType[](2);
        fheTypes[0] = FheType.Uint64;
        fheTypes[1] = FheType.Bool;

        cleartextValues[0] = bytes32(uint256(cleartextValue));
        cleartextValues[1] = bytes32(0);

        (
            FHEVMExecutorNoEvents.ContextUserInputs memory context,
            bytes32 mockInputHandle,
            bytes memory inputProof
        ) = _generateMockInputParameters(
                cleartextValues,
                fheTypes,
                userAddress,
                contractAddress,
                block.chainid,
                HANDLE_VERSION,
                activeSigners
            );

        vm.assertEq(mockInputHandle, inputVerifier.verifyCiphertext(context, mockInputHandle, inputProof));
    }

    /**
     * @dev Tests that the verifyCiphertext function fails if the chainId is invalid.
     */
    function test_VerifyCiphertextFailsIfInvalidChainId(uint64 invalidChainId) public {
        _upgradeProxyWithSigners(3);
        vm.assume(invalidChainId != block.chainid);

        (
            FHEVMExecutorNoEvents.ContextUserInputs memory context,
            bytes32 mockInputHandle,
            bytes memory inputProof
        ) = _generateInputParametersWithOneMockHandle(invalidChainId, HANDLE_VERSION, activeSigners);

        vm.expectRevert(InputVerifier.InvalidChainId.selector);
        inputVerifier.verifyCiphertext(context, mockInputHandle, inputProof);
    }

    /**
     * @dev Tests that the verifyCiphertext function fails if the inputProof is empty.
     */
    function test_VerifyCiphertextFailsIfEmptyInputProof() public {
        _upgradeProxyWithSigners(3);

        (
            FHEVMExecutorNoEvents.ContextUserInputs memory context,
            bytes32 mockInputHandle,
            bytes memory inputProof
        ) = _generateInputParametersWithOneMockHandle(block.chainid, HANDLE_VERSION, activeSigners);

        inputProof = new bytes(0);

        vm.expectRevert(InputVerifier.EmptyInputProof.selector);
        inputVerifier.verifyCiphertext(context, mockInputHandle, inputProof);
    }

    /**
     * @dev Tests that the verifyCiphertext function fails if the index is invalid since it is greater than 254.
     */
    function test_VerifyCiphertextFailsIfInvalidIndexIfEqual255() public {
        _upgradeProxyWithSigners(3);

        (
            FHEVMExecutorNoEvents.ContextUserInputs memory context,
            bytes32 mockInputHandle,
            bytes memory inputProof
        ) = _generateInputParametersWithOneMockHandle(block.chainid, HANDLE_VERSION, activeSigners);

        /// @dev It is invalid since it is 255.
        mockInputHandle = mockInputHandle | (bytes32(uint256(255)) << 80);

        vm.expectRevert(InputVerifier.InvalidIndex.selector);
        inputVerifier.verifyCiphertext(context, mockInputHandle, inputProof);
    }
    /**
     * @dev Tests that the verifyCiphertext function fails if the index is invalid since it is greater than 254.
     */
    function test_VerifyCiphertextFailsIfInvalidIndexIfEqual255WithProofCached() public {
        _upgradeProxyWithSigners(3);

        (
            FHEVMExecutorNoEvents.ContextUserInputs memory context,
            bytes32 mockInputHandle,
            bytes memory inputProof
        ) = _generateInputParametersWithOneMockHandle(block.chainid, HANDLE_VERSION, activeSigners);

        inputVerifier.verifyCiphertext(context, mockInputHandle, inputProof);

        /// @dev It is invalid since it is 255.
        mockInputHandle = mockInputHandle | (bytes32(uint256(255)) << 80);

        vm.expectRevert(InputVerifier.InvalidIndex.selector);
        inputVerifier.verifyCiphertext(context, mockInputHandle, inputProof);
    }

    /**
     * @dev Tests that the verifyCiphertext function fails if the index is invalid if it is greater than (or equal to) the number of handles.
     */
    function test_VerifyCiphertextFailsIfInvalidIndexGreaterThanNumberHandles(uint8 indexHandle) public {
        _upgradeProxyWithSigners(3);
        vm.assume(indexHandle > 0 && indexHandle < 255);

        (
            FHEVMExecutorNoEvents.ContextUserInputs memory context,
            bytes32 mockInputHandle,
            bytes memory inputProof
        ) = _generateInputParametersWithOneMockHandle(block.chainid, HANDLE_VERSION, activeSigners);

        /// @dev It is invalid since it is greater than (equal to) the number of handles.
        mockInputHandle = mockInputHandle | (bytes32(uint256(indexHandle)) << 80);

        vm.expectRevert(InputVerifier.InvalidIndex.selector);
        inputVerifier.verifyCiphertext(context, mockInputHandle, inputProof);
    }

    /**
     * @dev Tests that the verifyCiphertext function fails if the index is invalid since it is greater than 254.
     */
    function test_VerifyCiphertextFailsIfInvalidIndexGreaterThanNumberHandlesWithProofCached(uint8 indexHandle) public {
        _upgradeProxyWithSigners(3);
        vm.assume(indexHandle > 0 && indexHandle < 255);

        (
            FHEVMExecutorNoEvents.ContextUserInputs memory context,
            bytes32 mockInputHandle,
            bytes memory inputProof
        ) = _generateInputParametersWithOneMockHandle(block.chainid, HANDLE_VERSION, activeSigners);

        inputVerifier.verifyCiphertext(context, mockInputHandle, inputProof);

        /// @dev It is invalid since it is greater than (equal to) the number of handles.
        mockInputHandle = mockInputHandle | (bytes32(uint256(indexHandle)) << 80);

        vm.expectRevert(InputVerifier.InvalidIndex.selector);
        inputVerifier.verifyCiphertext(context, mockInputHandle, inputProof);
    }

    /**
     * @dev Tests that the verifyCiphertext function fails if the length of the input proof is invalid.
     */
    function test_VerifyCiphertextFailsIfDeserializingInputProofFail(uint256 randomValue) public {
        _upgradeProxyWithSigners(3);

        (
            FHEVMExecutorNoEvents.ContextUserInputs memory context,
            bytes32 mockInputHandle,
            bytes memory inputProof
        ) = _generateInputParametersWithOneMockHandle(block.chainid, HANDLE_VERSION, activeSigners);

        /// @dev We increase the length of the input proof by adding a random value at the end.
        inputProof = abi.encodePacked(inputProof, randomValue);

        vm.expectRevert(InputVerifier.DeserializingInputProofFail.selector);
        inputVerifier.verifyCiphertext(context, mockInputHandle, inputProof);
    }

    /**
     * @dev Tests that the verifyCiphertext function fails if the handle version is not the one matching the InputVerifier's contract storage.
     */
    function test_VerifyCiphertextFailsIfInvalidHandleVersion(uint8 handleVersion) public {
        _upgradeProxyWithSigners(3);
        vm.assume(handleVersion != HANDLE_VERSION);

        (
            FHEVMExecutorNoEvents.ContextUserInputs memory context,
            bytes32 mockInputHandle,
            bytes memory inputProof
        ) = _generateInputParametersWithOneMockHandle(block.chainid, handleVersion, activeSigners);

        mockInputHandle = mockInputHandle | bytes32(uint256(handleVersion));

        vm.expectRevert(InputVerifier.InvalidHandleVersion.selector);
        inputVerifier.verifyCiphertext(context, mockInputHandle, inputProof);
    }

    /**
     * @dev Tests that the verifyCiphertext function fails if the inputHandle is invalid when the proof is not cached.
     */
    function test_VerifyCiphertextFailsIfInvalidInputHandle() public {
        _upgradeProxyWithSigners(3);
        address userAddress = address(1111);
        address contractAddress = address(2222);
        uint64 initialCleartextValue = 123456789;

        FheType[] memory fheTypes = new FheType[](1);
        bytes32[] memory cleartextValues = new bytes32[](1);
        fheTypes[0] = FheType.Uint64;
        cleartextValues[0] = bytes32(uint256(initialCleartextValue));

        (
            FHEVMExecutorNoEvents.ContextUserInputs memory context,
            ,
            bytes memory inputProof
        ) = _generateMockInputParameters(
                cleartextValues,
                fheTypes,
                userAddress,
                contractAddress,
                block.chainid,
                HANDLE_VERSION,
                activeSigners
            );

        uint64 updatedCleartextValue = 987654321;
        cleartextValues[0] = bytes32(uint256(updatedCleartextValue));

        (, bytes32 invalidInputHandle, ) = _generateMockInputParameters(
            cleartextValues,
            fheTypes,
            userAddress,
            contractAddress,
            block.chainid,
            HANDLE_VERSION,
            activeSigners
        );

        vm.expectRevert(InputVerifier.InvalidInputHandle.selector);
        inputVerifier.verifyCiphertext(context, invalidInputHandle, inputProof);
    }

    /**
     * @dev Tests that the verifyCiphertext function fails if the inputHandle is invalid when the proof is cached.
     */
    function test_VerifyCiphertextFailsIfInvalidInputHandleWithProofCached() public {
        _upgradeProxyWithSigners(3);
        address userAddress = address(1111);
        address contractAddress = address(2222);
        uint64 initialCleartextValue = 123456789;

        FheType[] memory fheTypes = new FheType[](1);
        bytes32[] memory cleartextValues = new bytes32[](1);
        fheTypes[0] = FheType.Uint64;
        cleartextValues[0] = bytes32(uint256(initialCleartextValue));

        (
            FHEVMExecutorNoEvents.ContextUserInputs memory context,
            bytes32 mockInputHandle,
            bytes memory inputProof
        ) = _generateMockInputParameters(
                cleartextValues,
                fheTypes,
                userAddress,
                contractAddress,
                block.chainid,
                HANDLE_VERSION,
                activeSigners
            );

        inputVerifier.verifyCiphertext(context, mockInputHandle, inputProof);

        uint64 updatedCleartextValue = 987654321;
        cleartextValues[0] = bytes32(uint256(updatedCleartextValue));

        (, bytes32 invalidInputHandle, ) = _generateMockInputParameters(
            cleartextValues,
            fheTypes,
            userAddress,
            contractAddress,
            block.chainid,
            HANDLE_VERSION,
            activeSigners
        );

        vm.expectRevert(InputVerifier.InvalidInputHandle.selector);
        inputVerifier.verifyCiphertext(context, invalidInputHandle, inputProof);
    }

    /**
     * @dev Tests that the verifyCiphertext function fails if the signature threshold is not reached.
     */
    function test_VerifyCiphertextFailsIfSignatureThresholdNotReached() public {
        _upgradeProxyWithSigners(3);

        /// @dev We only use one signer to generate the input parameters but the threshold is 2.
        address[] memory signers = new address[](1);
        signers[0] = signer0;

        (
            FHEVMExecutorNoEvents.ContextUserInputs memory context,
            bytes32 mockInputHandle,
            bytes memory inputProof
        ) = _generateInputParametersWithOneMockHandle(block.chainid, HANDLE_VERSION, signers);

        vm.expectPartialRevert(InputVerifier.SignatureThresholdNotReached.selector);
        inputVerifier.verifyCiphertext(context, mockInputHandle, inputProof);
    }

    /**
     * @dev Tests that the verifyCiphertext function fails if an invalid signer address is recovered.
     */
    function test_VerifyCiphertextFailsIfInvalidSignerIsRecovered() public {
        _upgradeProxyWithSigners(3);

        /// @dev We use 2 signers (threshold is 2) but one of the signers is not a signer in the InputVerifier contract!
        address[] memory signers = new address[](2);
        signers[0] = signer0;
        signers[1] = signer4;

        assertFalse(inputVerifier.isSigner(signer4));

        (
            FHEVMExecutorNoEvents.ContextUserInputs memory context,
            bytes32 mockInputHandle,
            bytes memory inputProof
        ) = _generateInputParametersWithOneMockHandle(block.chainid, HANDLE_VERSION, signers);

        vm.expectPartialRevert(InputVerifier.InvalidSigner.selector);
        inputVerifier.verifyCiphertext(context, mockInputHandle, inputProof);
    }

    /**
     * @dev Tests that the verifyCiphertext function fails if the signatures verification fails for other reasons.
     */
    function test_VerifyCiphertextFailsIfSignaturesVerificationFailed() public {
        _upgradeProxyWithSigners(3);

        /// @dev We use 2 signers (threshold is 2) but it is the same signer!
        address[] memory signers = new address[](2);
        signers[0] = signer0;
        signers[1] = signer0;

        (
            FHEVMExecutorNoEvents.ContextUserInputs memory context,
            bytes32 mockInputHandle,
            bytes memory inputProof
        ) = _generateInputParametersWithOneMockHandle(block.chainid, HANDLE_VERSION, signers);

        vm.expectRevert(InputVerifier.SignaturesVerificationFailed.selector);
        inputVerifier.verifyCiphertext(context, mockInputHandle, inputProof);
    }

    /**
     * @dev Tests that the verifyCiphertext function fails if no signature is provided.
     */
    function test_VerifyCiphertextFailsIfNoSignatureIsProvided() public {
        _upgradeProxyWithSigners(3);

        /// @dev We use 0 signer.
        address[] memory signers = new address[](0);

        (
            FHEVMExecutorNoEvents.ContextUserInputs memory context,
            bytes32 mockInputHandle,
            bytes memory inputProof
        ) = _generateInputParametersWithOneMockHandle(block.chainid, HANDLE_VERSION, signers);

        vm.expectRevert(InputVerifier.ZeroSignature.selector);
        inputVerifier.verifyCiphertext(context, mockInputHandle, inputProof);
    }

    /// @dev This function exists for the test below to call it externally.
    function upgrade(address randomAccount) external {
        UnsafeUpgrades.upgradeProxy(proxy, address(new EmptyUUPSProxy()), "", randomAccount);
    }

    /**
     * @dev Tests that only the owner can authorize an upgrade.
     */
    function test_OnlyOwnerCanAuthorizeUpgrade(address randomAccount) public {
        _upgradeProxyWithSigners(3);
        vm.assume(randomAccount != owner);
        /// @dev Have to use external call to this to avoid this issue:
        ///      https://github.com/foundry-rs/foundry/issues/5806
        vm.expectPartialRevert(OwnableUpgradeable.OwnableUnauthorizedAccount.selector);
        this.upgrade(randomAccount);
    }

    function test_OnlyOwnerCanAuthorizeUpgrade() public {
        _upgradeProxyWithSigners(3);
        /// @dev It does not revert since it called by the owner.
        UnsafeUpgrades.upgradeProxy(proxy, address(new EmptyUUPSProxy()), "", owner);
    }

    /**
     * @dev Tests that only the owner can add a signer.
     */
    function test_OnlyOwnerCanAddSigner(address randomAccount) public {
        _upgradeProxyWithSigners(3);
        vm.assume(randomAccount != owner);
        vm.expectPartialRevert(OwnableUpgradeable.OwnableUnauthorizedAccount.selector);
        vm.prank(randomAccount);
        inputVerifier.addSigner(randomAccount);
    }

    /**
     * @dev Tests that only the owner can remove a signer.
     */
    function test_OnlyOwnerCanRemoveSigner(address randomAccount) public {
        _upgradeProxyWithSigners(3);
        vm.assume(randomAccount != owner);
        vm.expectPartialRevert(OwnableUpgradeable.OwnableUnauthorizedAccount.selector);
        vm.prank(randomAccount);
        inputVerifier.removeSigner(randomAccount);
    }

    /**
     * @dev Tests that the owner cannot add the same signer twice.
     */
    function test_OwnerCannotAddTwiceSameSigner() public {
        _upgradeProxyWithSigners(3);
        vm.expectPartialRevert(InputVerifier.AlreadySigner.selector);
        vm.prank(owner);
        inputVerifier.addSigner(signer0);
    }

    /**
     * @dev Tests that the owner cannot remove a signer if the signer is not in the list of active signers.
     */
    function test_OwnerCannotRemoveSignerIfNotASigner() public {
        _upgradeProxyWithSigners(3);
        address notASigner = address(12345);
        vm.assertFalse(inputVerifier.isSigner(notASigner));
        vm.expectPartialRevert(InputVerifier.NotASigner.selector);
        vm.prank(owner);
        inputVerifier.removeSigner(address(12345));
    }

    /**
     * @dev Tests that the owner can add a signer and it emits the SignerAdded event.
     */
    function test_OwnerCanAddSigner() public {
        _upgradeProxyWithSigners(3);
        vm.expectEmit(true, true, true, true);
        emit InputVerifier.SignerAdded(signer3);
        vm.prank(owner);
        inputVerifier.addSigner(signer3);

        address[] memory signers = inputVerifier.getCoprocessorSigners();
        assertEq(signers.length, 4);
        assertEq(signers[0], signer0);
        assertEq(signers[1], signer1);
        assertEq(signers[2], signer2);
        assertEq(signers[3], signer3);
    }

    /**
     * @dev Tests that the owner can remove a signer and it emits the SignerRemoved event.
     */
    function test_OwnerCanRemoveSigner() public {
        _upgradeProxyWithSigners(3);
        vm.expectEmit(true, true, true, true);
        emit InputVerifier.SignerRemoved(signer0);
        vm.prank(owner);
        inputVerifier.removeSigner(signer0);

        address[] memory signers = inputVerifier.getCoprocessorSigners();
        assertEq(signers.length, 2);
        assertEq(signers[0], signer2);
        assertEq(signers[1], signer1);
    }

    /// @dev This function exists for the test below to call it externally.
    function emptyUpgrade() public {
        address[] memory emptySigners = new address[](0);
        implementation = address(new InputVerifier());

        UnsafeUpgrades.upgradeProxy(
            proxy,
            implementation,
            abi.encodeCall(InputVerifier.reinitialize, (verifyingContractSource, uint64(block.chainid), emptySigners)),
            owner
        );
    }

    /**
     * @dev Tests that the contract cannot be reinitialized if the initial signers set is empty.
     */
    function test_CannotReinitializeIfInitialSignersSetIsEmpty() public {
        vm.expectPartialRevert(InputVerifier.InitialSignersSetIsEmpty.selector);
        this.emptyUpgrade();
    }

    /**
     * @dev Tests that anyone can call cleanTransientStorage.
     */
    function test_AnyoneCanCallCleanTransientStorage(address randomAccount) public {
        _upgradeProxyWithSigners(3);
        vm.prank(randomAccount);
        inputVerifier.cleanTransientStorage();
    }
}
