import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { ethers } from "hardhat";

import {
  CiphertextCommitsMock,
  DecryptionMock,
  GatewayConfigMock,
  InputVerificationMock,
  KMSGenerationMock,
  MultichainACLMock,
} from "../../typechain-types";
import { KeyTypeEnum, ParamsTypeEnum, getCrsId, getKeyId, getPrepKeygenId, toValues } from "../utils";

describe("Mock contracts", function () {
  // Mock contracts
  let ciphertextCommitsMock: CiphertextCommitsMock;
  let decryptionMock: DecryptionMock;
  let gatewayConfigMock: GatewayConfigMock;
  let kmsGenerationMock: KMSGenerationMock;
  let inputVerificationMock: InputVerificationMock;
  let multichainACLMock: MultichainACLMock;

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

    const multichainACLFactory = await ethers.getContractFactory("MultichainACLMock");
    const multichainACLMock = await multichainACLFactory.deploy();

    return {
      multichainACLMock,
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
    multichainACLMock = fixture.multichainACLMock;
  });

  describe("CiphertextCommitsMock", async function () {
    it("Should emit AddCiphertextMaterial event on add ciphertext material call", async function () {
      await expect(
        ciphertextCommitsMock.addCiphertextMaterial(DefaultBytes32, DefaultUint256, DefaultBytes32, DefaultBytes32),
      )
        .to.emit(ciphertextCommitsMock, "AddCiphertextMaterial")
        .withArgs(DefaultBytes32, DefaultBytes32, DefaultBytes32, [DefaultAddress]);
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

    it("Should emit PublicDecryptionResponse event on public decryption response", async function () {
      await expect(
        decryptionMock.publicDecryptionResponse(publicDecryptionCounterId, DefaultBytes, DefaultBytes, DefaultBytes),
      )
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
      await expect(
        gatewayConfigMock.initializeFromEmptyProxy(
          DefaultProtocolMetadata,
          DefaultUint256,
          DefaultUint256,
          DefaultUint256,
          DefaultUint256,
          [DefaultKmsNode],
          [DefaultCoprocessor],
          [DefaultCustodian],
        ),
      )
        .to.emit(gatewayConfigMock, "InitializeGatewayConfig")
        .withArgs(
          toValues(DefaultProtocolMetadata),
          DefaultUint256,
          toValues([DefaultKmsNode]),
          toValues([DefaultCoprocessor]),
          toValues([DefaultCustodian]),
        );
    });

    it("Should emit UpdateMpcThreshold event on update MPC threshold call", async function () {
      await expect(gatewayConfigMock.updateMpcThreshold(DefaultUint256))
        .to.emit(gatewayConfigMock, "UpdateMpcThreshold")
        .withArgs(DefaultUint256);
    });

    it("Should emit UpdatePublicDecryptionThreshold event on update PublicDecryption threshold call", async function () {
      await expect(gatewayConfigMock.updatePublicDecryptionThreshold(DefaultUint256))
        .to.emit(gatewayConfigMock, "UpdatePublicDecryptionThreshold")
        .withArgs(DefaultUint256);
    });

    it("Should emit UpdateUserDecryptionThreshold event on update UserDecryption threshold call", async function () {
      await expect(gatewayConfigMock.updateUserDecryptionThreshold(DefaultUint256))
        .to.emit(gatewayConfigMock, "UpdateUserDecryptionThreshold")
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

    it("Should emit VerifyProofResponse event on verify proof response", async function () {
      await expect(
        inputVerificationMock.verifyProofResponse(zkProofCounterId, [DefaultBytes32], DefaultBytes, DefaultBytes),
      )
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

    it("Should emit KeygenRequest event on preprocessing keygen response", async function () {
      await expect(kmsGenerationMock.prepKeygenResponse(prepKeygenId, DefaultBytes))
        .to.emit(kmsGenerationMock, "KeygenRequest")
        .withArgs(prepKeygenId, keyId);
    });

    it("Should emit ActivateKey event on keygen response", async function () {
      await expect(kmsGenerationMock.keygenResponse(keyId, [DefaultKmsDigest], DefaultBytes))
        .to.emit(kmsGenerationMock, "ActivateKey")
        .withArgs(keyId, [DefaultString], toValues([DefaultKmsDigest]));
    });

    it("Should emit CrsgenRequest event on crsgen request", async function () {
      await expect(kmsGenerationMock.crsgenRequest(DefaultUint256, DefaultParamsType))
        .to.emit(kmsGenerationMock, "CrsgenRequest")
        .withArgs(crsgenId, DefaultUint256, DefaultParamsType);
    });

    it("Should emit ActivateCrs event on crsgen request", async function () {
      await expect(kmsGenerationMock.crsgenResponse(crsgenId, DefaultBytes, DefaultBytes))
        .to.emit(kmsGenerationMock, "ActivateCrs")
        .withArgs(crsgenId, [DefaultString], DefaultBytes);
    });
  });

  describe("MultichainACLMock", async function () {
    it("Should emit AllowPublicDecrypt event on allow public decrypt call", async function () {
      await expect(multichainACLMock.allowPublicDecrypt(DefaultBytes32, DefaultBytes))
        .to.emit(multichainACLMock, "AllowPublicDecrypt")
        .withArgs(DefaultBytes32);
    });

    it("Should emit AllowAccount event on allow account call", async function () {
      await expect(multichainACLMock.allowAccount(DefaultBytes32, DefaultAddress, DefaultBytes))
        .to.emit(multichainACLMock, "AllowAccount")
        .withArgs(DefaultBytes32, DefaultAddress);
    });

    it("Should emit DelegateUserDecryption event on delegate user decryption call", async function () {
      await expect(
        multichainACLMock.delegateUserDecryption(
          DefaultUint256,
          DefaultAddress,
          DefaultAddress,
          DefaultAddress,
          DefaultUint256,
          DefaultUint256,
        ),
      )
        .to.emit(multichainACLMock, "DelegateUserDecryption")
        .withArgs(DefaultUint256, DefaultAddress, DefaultAddress, DefaultAddress);
    });

    it("Should emit RevokeUserDecryption event on revoke user decryption delegation call", async function () {
      await expect(
        multichainACLMock.revokeUserDecryption(
          DefaultUint256,
          DefaultAddress,
          DefaultAddress,
          DefaultAddress,
          DefaultUint256,
          DefaultUint256,
        ),
      )
        .to.emit(multichainACLMock, "RevokeUserDecryption")
        .withArgs(DefaultUint256, DefaultAddress, DefaultAddress, DefaultAddress);
    });
  });
});
