import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { ethers } from "hardhat";

import { GatewayConfigMock, KMSGenerationMock } from "../../typechain-types";
import { KeyTypeEnum, ParamsTypeEnum, getCrsId, getKeyId, getKeyReshareId, getPrepKeygenId, toValues } from "../utils";

describe("Mock contracts", function () {
  let gatewayConfigMock: GatewayConfigMock;
  let kmsGenerationMock: KMSGenerationMock;

  const DefaultBytes = ethers.hexlify(new Uint8Array(0));
  const DefaultBytes32 = ethers.ZeroHash;
  const DefaultAddress = ethers.ZeroAddress;
  const DefaultUint256 = 0;
  const DefaultString = "";

  const DefaultProtocolMetadata = { name: DefaultString, website: DefaultString };

  const DefaultKmsNode = {
    txSenderAddress: DefaultAddress,
    signerAddress: DefaultAddress,
    ipAddress: DefaultString,
    storageUrl: DefaultString,
    apiUrl: DefaultString,
  };

  const DefaultCoprocessor = {
    txSenderAddress: DefaultAddress,
    signerAddress: DefaultAddress,
    s3BucketUrl: DefaultString,
    apiUrl: DefaultString,
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

  const DefaultParamsType = ParamsTypeEnum.Default;

  const DefaultKmsDigest = {
    keyType: KeyTypeEnum.Server,
    digest: DefaultBytes,
  };

  async function loadMockContractsFixture() {
    const gatewayConfigFactory = await ethers.getContractFactory("GatewayConfigMock");
    const gatewayConfigMock = await gatewayConfigFactory.deploy();

    const kmsGenerationFactory = await ethers.getContractFactory("KMSGenerationMock");
    const kmsGenerationMock = await kmsGenerationFactory.deploy();

    return {
      gatewayConfigMock,
      kmsGenerationMock,
    };
  }

  before(async function () {
    const fixture = await loadFixture(loadMockContractsFixture);
    gatewayConfigMock = fixture.gatewayConfigMock;
    kmsGenerationMock = fixture.kmsGenerationMock;
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
      const prepKeygenId = getPrepKeygenId(2);
      const keyReshareId = getKeyReshareId(1);

      await expect(kmsGenerationMock.keyReshareSameSet(keyId))
        .to.emit(kmsGenerationMock, "KeyReshareSameSet")
        .withArgs(prepKeygenId, keyId, keyReshareId, DefaultParamsType);
    });
  });
});
