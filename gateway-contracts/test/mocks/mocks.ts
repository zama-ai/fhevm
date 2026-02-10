import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { ethers } from "hardhat";

import {
  CiphertextCommitsMock,
  DecryptionMock,
  GatewayConfigMock,
  InputVerificationMock,
  KMSGenerationMock,
} from "../../typechain-types";
import { KeyTypeEnum, ParamsTypeEnum, getCrsId, getKeyId, getKeyReshareId, getPrepKeygenId, toValues } from "../utils";

describe("Mock contracts", function () {
  // Mock contracts
  let ciphertextCommitsMock: CiphertextCommitsMock;
  let decryptionMock: DecryptionMock;
  let gatewayConfigMock: GatewayConfigMock;
  let kmsGenerationMock: KMSGenerationMock;
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

  const DefaultParamsType = ParamsTypeEnum.Default;

  const DefaultKmsDigest = {
    keyType: KeyTypeEnum.Server,
    digest: DefaultBytes,
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

    const kmsGenerationFactory = await ethers.getContractFactory("KMSGenerationMock");
    const kmsGenerationMock = await kmsGenerationFactory.deploy();

    return {
      ciphertextCommitsMock,
      decryptionMock,
      gatewayConfigMock,
      kmsGenerationMock,
      inputVerificationMock,
    };
  }

  before(async function () {
    // Initialize globally used variables before each test
    const fixture = await loadFixture(loadMockContractsFixture);
    ciphertextCommitsMock = fixture.ciphertextCommitsMock;
    decryptionMock = fixture.decryptionMock;
    gatewayConfigMock = fixture.gatewayConfigMock;
    kmsGenerationMock = fixture.kmsGenerationMock;
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
          DefaultProtocolMetadata,
          DefaultThresholds,
          [DefaultKmsNode],
          [DefaultCoprocessor],
          [DefaultCustodian],
        ),
      )
        .to.emit(gatewayConfigMock, "InitializeGatewayConfig")
        .withArgs(
          toValues(DefaultProtocolMetadata),
          toValues(DefaultThresholds),
          toValues([DefaultKmsNode]),
          toValues([DefaultCoprocessor]),
          toValues([DefaultCustodian]),
        );
    });

    it("Should emit UpdateKmsNodes event on update KMS nodes call", async function () {
      await expect(
        gatewayConfigMock.updateKmsNodes(
          [DefaultKmsNode],
          DefaultUint256,
          DefaultUint256,
          DefaultUint256,
          DefaultUint256,
        ),
      )
        .to.emit(gatewayConfigMock, "UpdateKmsNodes")
        .withArgs(toValues([DefaultKmsNode]), DefaultUint256, DefaultUint256, DefaultUint256, DefaultUint256);
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

    it("Should emit UpdateUserDecryptionThreshold event on update UserDecryption threshold call", async function () {
      await expect(gatewayConfigMock.updateUserDecryptionThreshold(DefaultUint256))
        .to.emit(gatewayConfigMock, "UpdateUserDecryptionThreshold")
        .withArgs(DefaultUint256);
    });

    it("Should emit UpdateKmsGenThreshold event on update KmsGen threshold call", async function () {
      await expect(gatewayConfigMock.updateKmsGenThreshold(DefaultUint256))
        .to.emit(gatewayConfigMock, "UpdateKmsGenThreshold")
        .withArgs(DefaultUint256);
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

  describe("KMSGenerationMock", async function () {
    const prepKeygenId = getPrepKeygenId(1);
    const keyId = getKeyId(1);
    const crsgenId = getCrsId(1);
    const epochId = 0;

    it("Should emit PrepKeygenRequest event on keygen request", async function () {
      await expect(kmsGenerationMock.keygen(DefaultParamsType))
        .to.emit(kmsGenerationMock, "PrepKeygenRequest")
        .withArgs(prepKeygenId, epochId, DefaultParamsType);
    });

    it("Should emit KeygenRequest and KeygenResponse events on preprocessing keygen response", async function () {
      await expect(kmsGenerationMock.prepKeygenResponse(prepKeygenId, DefaultBytes))
        .to.emit(kmsGenerationMock, "KeygenRequest")
        .withArgs(prepKeygenId, keyId)
        .to.emit(kmsGenerationMock, "PrepKeygenResponse")
        .withArgs(prepKeygenId, DefaultBytes, DefaultAddress);
    });

    it("Should emit ActivateKey and KeygenResponse events on keygen response", async function () {
      await expect(kmsGenerationMock.keygenResponse(keyId, [DefaultKmsDigest], DefaultBytes))
        .to.emit(kmsGenerationMock, "ActivateKey")
        .withArgs(keyId, [DefaultString], toValues([DefaultKmsDigest]))
        .to.emit(kmsGenerationMock, "KeygenResponse")
        .withArgs(keyId, toValues([DefaultKmsDigest]), DefaultBytes, DefaultAddress);
    });

    it("Should emit CrsgenRequest event on crsgen request", async function () {
      await expect(kmsGenerationMock.crsgenRequest(DefaultUint256, DefaultParamsType))
        .to.emit(kmsGenerationMock, "CrsgenRequest")
        .withArgs(crsgenId, DefaultUint256, DefaultParamsType);
    });

    it("Should emit ActivateCrs and CrsgenResponse events on crsgen request", async function () {
      await expect(kmsGenerationMock.crsgenResponse(crsgenId, DefaultBytes, DefaultBytes))
        .to.emit(kmsGenerationMock, "ActivateCrs")
        .withArgs(crsgenId, [DefaultString], DefaultBytes)
        .to.emit(kmsGenerationMock, "CrsgenResponse")
        .withArgs(crsgenId, DefaultBytes, DefaultBytes, DefaultAddress);
    });

    it("Should emit PRSSInit event on prssInit call", async function () {
      await expect(kmsGenerationMock.prssInit()).to.emit(kmsGenerationMock, "PRSSInit");
    });

    it("Should emit KeyReshareSameSet event on keyReshareSameSet call", async function () {
      // Define incremented prepKeygenId since the mock contract increments
      // this value internally from previous test cases.
      const prepKeygenId = getPrepKeygenId(2);
      const keyReshareId = getKeyReshareId(1);

      await expect(kmsGenerationMock.keyReshareSameSet(keyId))
        .to.emit(kmsGenerationMock, "KeyReshareSameSet")
        .withArgs(prepKeygenId, keyId, keyReshareId, DefaultParamsType);
    });
  });
});
