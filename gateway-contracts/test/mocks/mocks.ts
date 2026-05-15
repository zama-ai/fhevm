import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { ethers } from "hardhat";

import { CiphertextCommitsMock, DecryptionMock, GatewayConfigMock, InputVerificationMock } from "../../typechain-types";
import { toValues } from "../utils";

describe("Mock contracts", function () {
  // Mock contracts
  let ciphertextCommitsMock: CiphertextCommitsMock;
  let decryptionMock: DecryptionMock;
  let gatewayConfigMock: GatewayConfigMock;
  let inputVerificationMock: InputVerificationMock;

  // Default values
  const DefaultBytes = ethers.hexlify(new Uint8Array(0));
  const DefaultBytes32 = ethers.ZeroHash;
  const DefaultAddress = ethers.ZeroAddress;
  const DefaultUint256 = 0;
  const DefaultString = "";
  const EmptyArray: never[] = [];

  const DefaultSnsCiphertextMaterial = {
    ctHandle: DefaultBytes32,
    keyId: DefaultUint256,
    snsCiphertextDigest: DefaultBytes32,
    coprocessorTxSenderAddresses: EmptyArray,
  };

  const DefaultProtocolMetadata = { name: DefaultString, website: DefaultString };

  const DefaultKmsNode = {
    txSenderAddress: DefaultAddress,
    signerAddress: DefaultAddress,
    ipAddress: DefaultString,
    storageUrl: DefaultString,
  };

  const DefaultCoprocessor = {
    txSenderAddress: DefaultAddress,
    signerAddress: DefaultAddress,
    s3BucketUrl: DefaultString,
  };

  const DefaultCustodian = {
    txSenderAddress: DefaultAddress,
    signerAddress: DefaultAddress,
    encryptionKey: DefaultBytes,
  };

  const DefaultHostChain = {
    chainId: DefaultUint256,
    fhevmExecutorAddress: DefaultAddress,
    aclAddress: DefaultAddress,
    name: DefaultString,
    website: DefaultString,
  };

  const DefaultRequestValidity = {
    durationDays: DefaultUint256,
    startTimestamp: DefaultUint256,
  };

  const DefaultContractsInfo = {
    chainId: DefaultUint256,
    addresses: [DefaultAddress],
  };

  const DefaultDelegationAccounts = {
    delegatorAddress: DefaultAddress,
    delegateAddress: DefaultAddress,
  };

  async function loadMockContractsFixture() {
    const ciphertextCommitsFactory = await ethers.getContractFactory("CiphertextCommitsMock");
    const ciphertextCommitsMock = await ciphertextCommitsFactory.deploy();

    const decryptionFactory = await ethers.getContractFactory("DecryptionMock");
    const decryptionMock = await decryptionFactory.deploy();

    const gatewayConfigFactory = await ethers.getContractFactory("GatewayConfigMock");
    const gatewayConfigMock = await gatewayConfigFactory.deploy();

    const inputVerificationFactory = await ethers.getContractFactory("InputVerificationMock");
    const inputVerificationMock = await inputVerificationFactory.deploy();

    return {
      ciphertextCommitsMock,
      decryptionMock,
      gatewayConfigMock,
      inputVerificationMock,
    };
  }

  before(async function () {
    // Initialize globally used variables before each test
    const fixture = await loadFixture(loadMockContractsFixture);
    ciphertextCommitsMock = fixture.ciphertextCommitsMock;
    decryptionMock = fixture.decryptionMock;
    gatewayConfigMock = fixture.gatewayConfigMock;
    inputVerificationMock = fixture.inputVerificationMock;
  });

  describe("CiphertextCommitsMock", async function () {
    it("Should emit AddCiphertextMaterial and AddCiphertextMaterialConsensus events on add ciphertext material call", async function () {
      await expect(
        ciphertextCommitsMock.addCiphertextMaterial(DefaultBytes32, DefaultUint256, DefaultBytes32, DefaultBytes32),
      )
        .to.emit(ciphertextCommitsMock, "AddCiphertextMaterial")
        .withArgs(DefaultBytes32, DefaultUint256, DefaultBytes32, DefaultBytes32, DefaultAddress)
        .to.emit(ciphertextCommitsMock, "AddCiphertextMaterialConsensus")
        .withArgs(DefaultBytes32, DefaultUint256, DefaultBytes32, DefaultBytes32, [DefaultAddress]);
    });
  });

  describe("DecryptionMock", async function () {
    // Define the decryption ID values. See `KmsRequestCounter.sol` for more details.
    let publicDecryptionCounterId = BigInt(1) << BigInt(248);
    let userDecryptionCounterId = BigInt(2) << BigInt(248);

    it("Should emit PublicDecryptionRequest event on public decryption request", async function () {
      publicDecryptionCounterId++;
      await expect(decryptionMock.publicDecryptionRequest([DefaultBytes32], DefaultBytes))
        .to.emit(decryptionMock, "PublicDecryptionRequest")
        .withArgs(publicDecryptionCounterId, toValues([DefaultSnsCiphertextMaterial]), DefaultBytes);
    });

    it("Should emit PublicDecryptionResponseCall and PublicDecryptionResponse events on public decryption response", async function () {
      await expect(
        decryptionMock.publicDecryptionResponse(publicDecryptionCounterId, DefaultBytes, DefaultBytes, DefaultBytes),
      )
        .to.emit(decryptionMock, "PublicDecryptionResponseCall")
        .withArgs(publicDecryptionCounterId, DefaultBytes, DefaultBytes, DefaultAddress, DefaultBytes)
        .to.emit(decryptionMock, "PublicDecryptionResponse")
        .withArgs(publicDecryptionCounterId, DefaultBytes, [DefaultBytes], DefaultBytes);
    });

    it("Should emit UserDecryptionRequest event on user decryption request", async function () {
      userDecryptionCounterId++;
      await expect(
        decryptionMock.userDecryptionRequest(
          EmptyArray,
          DefaultRequestValidity,
          DefaultContractsInfo,
          DefaultAddress,
          DefaultBytes,
          DefaultBytes,
          DefaultBytes,
        ),
      )
        .to.emit(decryptionMock, "UserDecryptionRequest")
        .withArgs(
          userDecryptionCounterId,
          toValues([DefaultSnsCiphertextMaterial]),
          DefaultAddress,
          DefaultBytes,
          DefaultBytes,
        );
    });

    it("Should emit UserDecryptionRequest event on delegated user decryption request", async function () {
      userDecryptionCounterId++;
      await expect(
        decryptionMock.delegatedUserDecryptionRequest(
          EmptyArray,
          DefaultRequestValidity,
          DefaultDelegationAccounts,
          DefaultContractsInfo,
          DefaultBytes,
          DefaultBytes,
          DefaultBytes,
        ),
      )
        .to.emit(decryptionMock, "UserDecryptionRequest")
        .withArgs(
          userDecryptionCounterId,
          toValues([DefaultSnsCiphertextMaterial]),
          DefaultAddress,
          DefaultBytes,
          DefaultBytes,
        );
    });

    it("Should emit response and consensus events on user decryption response", async function () {
      await expect(
        decryptionMock.userDecryptionResponse(userDecryptionCounterId, DefaultBytes, DefaultBytes, DefaultBytes),
      )
        .to.emit(decryptionMock, "UserDecryptionResponse")
        .withArgs(userDecryptionCounterId, DefaultUint256, DefaultBytes, DefaultBytes, DefaultBytes)
        .to.emit(decryptionMock, "UserDecryptionResponseThresholdReached")
        .withArgs(userDecryptionCounterId);
    });
  });

  describe("GatewayConfigMock", async function () {
    it("Should emit InitializeGatewayConfig event on initialization", async function () {
      const DefaultThresholds = {
        mpcThreshold: DefaultUint256,
        publicDecryptionThreshold: DefaultUint256,
        userDecryptionThreshold: DefaultUint256,
        kmsGenThreshold: DefaultUint256,
        coprocessorThreshold: DefaultUint256,
      };
      await expect(
        gatewayConfigMock.initializeFromEmptyProxy(
          DefaultUint256,
          DefaultProtocolMetadata,
          DefaultThresholds,
          [DefaultKmsNode],
          [DefaultCoprocessor],
          [DefaultCustodian],
        ),
      )
        .to.emit(gatewayConfigMock, "InitializeGatewayConfig")
        .withArgs(
          DefaultUint256,
          toValues(DefaultProtocolMetadata),
          toValues(DefaultThresholds),
          toValues([DefaultKmsNode]),
          toValues([DefaultCoprocessor]),
          toValues([DefaultCustodian]),
        );
    });

    it("Should emit UpdateKmsContext event on update KMS nodes call", async function () {
      await expect(
        gatewayConfigMock.updateKmsContext(
          DefaultUint256,
          [DefaultKmsNode],
          DefaultUint256,
          DefaultUint256,
          DefaultUint256,
          DefaultUint256,
        ),
      )
        .to.emit(gatewayConfigMock, "UpdateKmsContext")
        .withArgs(
          DefaultUint256,
          toValues([DefaultKmsNode]),
          DefaultUint256,
          DefaultUint256,
          DefaultUint256,
          DefaultUint256,
        );
    });

    it("Should emit UpdateCoprocessors event on update coprocessors call", async function () {
      await expect(gatewayConfigMock.updateCoprocessors([DefaultCoprocessor], DefaultUint256))
        .to.emit(gatewayConfigMock, "UpdateCoprocessors")
        .withArgs(toValues([DefaultCoprocessor]), DefaultUint256);
    });

    it("Should emit UpdateCustodians event on update custodians call", async function () {
      await expect(gatewayConfigMock.updateCustodians([DefaultCustodian]))
        .to.emit(gatewayConfigMock, "UpdateCustodians")
        .withArgs(toValues([DefaultCustodian]));
    });

    it("Should emit UpdateMpcThresholdForContext event on update Mpc threshold for context call", async function () {
      await expect(gatewayConfigMock.updateMpcThresholdForContext(DefaultUint256, DefaultUint256))
        .to.emit(gatewayConfigMock, "UpdateMpcThresholdForContext")
        .withArgs(DefaultUint256, DefaultUint256);
    });

    it("Should emit UpdatePublicDecryptionThresholdForContext event on update PublicDecryption threshold for context call", async function () {
      await expect(gatewayConfigMock.updatePublicDecryptionThresholdForContext(DefaultUint256, DefaultUint256))
        .to.emit(gatewayConfigMock, "UpdatePublicDecryptionThresholdForContext")
        .withArgs(DefaultUint256, DefaultUint256);
    });

    it("Should emit UpdateUserDecryptionThresholdForContext event on update UserDecryption threshold for context call", async function () {
      await expect(gatewayConfigMock.updateUserDecryptionThresholdForContext(DefaultUint256, DefaultUint256))
        .to.emit(gatewayConfigMock, "UpdateUserDecryptionThresholdForContext")
        .withArgs(DefaultUint256, DefaultUint256);
    });

    it("Should emit UpdateKmsGenThresholdForContext event on update KmsGen threshold for context call", async function () {
      await expect(gatewayConfigMock.updateKmsGenThresholdForContext(DefaultUint256, DefaultUint256))
        .to.emit(gatewayConfigMock, "UpdateKmsGenThresholdForContext")
        .withArgs(DefaultUint256, DefaultUint256);
    });

    it("Should emit UpdateCoprocessorThreshold event on update coprocessor threshold call", async function () {
      await expect(gatewayConfigMock.updateCoprocessorThreshold(DefaultUint256))
        .to.emit(gatewayConfigMock, "UpdateCoprocessorThreshold")
        .withArgs(DefaultUint256);
    });

    it("Should emit AddHostChain event on add host chain call", async function () {
      await expect(gatewayConfigMock.addHostChain(DefaultHostChain))
        .to.emit(gatewayConfigMock, "AddHostChain")
        .withArgs(toValues(DefaultHostChain));
    });
  });

  describe("InputVerificationMock", async function () {
    let zkProofCounterId = DefaultUint256;
    it("Should emit VerifyProofRequest event on verify proof request", async function () {
      zkProofCounterId++;
      await expect(
        inputVerificationMock.verifyProofRequest(
          DefaultUint256,
          DefaultAddress,
          DefaultAddress,
          DefaultBytes,
          DefaultBytes,
        ),
      )
        .to.emit(inputVerificationMock, "VerifyProofRequest")
        .withArgs(zkProofCounterId, DefaultUint256, DefaultAddress, DefaultAddress, DefaultBytes, DefaultBytes);
    });

    it("Should emit VerifyProofResponseCall and VerifyProofResponse events on verify proof response", async function () {
      await expect(
        inputVerificationMock.verifyProofResponse(zkProofCounterId, [DefaultBytes32], DefaultBytes, DefaultBytes),
      )
        .to.emit(inputVerificationMock, "VerifyProofResponseCall")
        .withArgs(zkProofCounterId, [DefaultBytes32], DefaultBytes, DefaultAddress, DefaultBytes)
        .to.emit(inputVerificationMock, "VerifyProofResponse")
        .withArgs(zkProofCounterId, [DefaultBytes32], [DefaultBytes]);
    });

    it("Should emit RejectProofResponse event on reject proof response", async function () {
      await expect(inputVerificationMock.rejectProofResponse(zkProofCounterId, DefaultBytes))
        .to.emit(inputVerificationMock, "RejectProofResponse")
        .withArgs(zkProofCounterId);
    });
  });
});
