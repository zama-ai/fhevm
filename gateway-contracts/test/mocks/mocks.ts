import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { ethers } from "hardhat";

import {
  CiphertextCommitsMock,
  CoprocessorContextsMock,
  DecryptionMock,
  GatewayConfigMock,
  InputVerificationMock,
  KmsManagementMock,
  MultichainAclMock,
} from "../../typechain-types";
import { toValues } from "../utils";

describe("Mock contracts", function () {
  // Mock contracts
  let ciphertextCommitsMock: CiphertextCommitsMock;
  let coprocessorContextsMock: CoprocessorContextsMock;
  let decryptionMock: DecryptionMock;
  let gatewayConfigMock: GatewayConfigMock;
  let kmsManagementMock: KmsManagementMock;
  let inputVerificationMock: InputVerificationMock;
  let multichainAclMock: MultichainAclMock;

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
  };

  const DefaultCoprocessor = {
    name: DefaultString,
    txSenderAddress: DefaultAddress,
    signerAddress: DefaultAddress,
    s3BucketUrl: DefaultString,
  };

  const DefaultCoprocessorContext = {
    contextId: DefaultUint256,
    previousContextId: DefaultUint256,
    featureSet: DefaultUint256,
    coprocessors: [],
  };

  const DefaultCoprocessorContextBlockPeriods = {
    preActivationBlockPeriod: DefaultUint256,
    suspendedBlockPeriod: DefaultUint256,
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

  const DefaultDelegationAccounts = {
    delegatorAddress: DefaultAddress,
    delegatedAddress: DefaultAddress,
  };

  async function loadMockContractsFixture() {
    const ciphertextCommitsFactory = await ethers.getContractFactory("CiphertextCommitsMock");
    const ciphertextCommitsMock = await ciphertextCommitsFactory.deploy();

    const coprocessorContextsFactory = await ethers.getContractFactory("CoprocessorContextsMock");
    const coprocessorContextsMock = await coprocessorContextsFactory.deploy();

    const decryptionFactory = await ethers.getContractFactory("DecryptionMock");
    const decryptionMock = await decryptionFactory.deploy();

    const gatewayConfigFactory = await ethers.getContractFactory("GatewayConfigMock");
    const gatewayConfigMock = await gatewayConfigFactory.deploy();

    const inputVerificationFactory = await ethers.getContractFactory("InputVerificationMock");
    const inputVerificationMock = await inputVerificationFactory.deploy();

    const kmsManagementFactory = await ethers.getContractFactory("KmsManagementMock");
    const kmsManagementMock = await kmsManagementFactory.deploy();

    const multichainAclFactory = await ethers.getContractFactory("MultichainAclMock");
    const multichainAclMock = await multichainAclFactory.deploy();

    return {
      multichainAclMock,
      ciphertextCommitsMock,
      coprocessorContextsMock,
      decryptionMock,
      gatewayConfigMock,
      kmsManagementMock,
      inputVerificationMock,
    };
  }

  before(async function () {
    // Initialize globally used variables before each test
    const fixture = await loadFixture(loadMockContractsFixture);
    ciphertextCommitsMock = fixture.ciphertextCommitsMock;
    coprocessorContextsMock = fixture.coprocessorContextsMock;
    decryptionMock = fixture.decryptionMock;
    gatewayConfigMock = fixture.gatewayConfigMock;
    kmsManagementMock = fixture.kmsManagementMock;
    inputVerificationMock = fixture.inputVerificationMock;
    multichainAclMock = fixture.multichainAclMock;
  });

  describe("CiphertextCommitsMock", async function () {
    it("Should emit AddCiphertextMaterial event on add ciphertext material call", async function () {
      await expect(
        ciphertextCommitsMock.addCiphertextMaterial(DefaultBytes32, DefaultUint256, DefaultBytes32, DefaultBytes32),
      )
        .to.emit(ciphertextCommitsMock, "AddCiphertextMaterial")
        .withArgs(DefaultBytes32, DefaultUint256, DefaultBytes32, DefaultBytes32, [DefaultAddress]);
    });
  });

  describe("CoprocessorContextsMock", async function () {
    it("Should emit PreActivateCoprocessorContext event on add coprocessor context call", async function () {
      await expect(
        coprocessorContextsMock.addCoprocessorContext(DefaultUint256, DefaultCoprocessorContextBlockPeriods, [
          DefaultCoprocessor,
        ]),
      )
        .to.emit(coprocessorContextsMock, "PreActivateCoprocessorContext")
        .withArgs(toValues(DefaultCoprocessorContext), DefaultUint256);
    });

    it("Should emit several status events on refresh coprocessor context statuses call", async function () {
      await expect(coprocessorContextsMock.refreshCoprocessorContextStatuses())
        .to.emit(coprocessorContextsMock, "SuspendCoprocessorContext")
        .withArgs(DefaultUint256, DefaultUint256)
        .and.emit(coprocessorContextsMock, "ActivateCoprocessorContext")
        .withArgs(DefaultUint256)
        .and.emit(coprocessorContextsMock, "DeactivateCoprocessorContext")
        .withArgs(DefaultUint256);
    });

    it("Should emit compromiseCoprocessorContext event on compromise coprocessor context call", async function () {
      await expect(coprocessorContextsMock.compromiseCoprocessorContext(DefaultUint256))
        .to.emit(coprocessorContextsMock, "CompromiseCoprocessorContext")
        .withArgs(DefaultUint256);
    });

    it("Should emit destroyCoprocessorContext event on destroy coprocessor context call", async function () {
      await expect(coprocessorContextsMock.destroyCoprocessorContext(DefaultUint256))
        .to.emit(coprocessorContextsMock, "DestroyCoprocessorContext")
        .withArgs(DefaultUint256);
    });

    it("Should emit several events on move suspended coprocessor context to active call", async function () {
      await expect(coprocessorContextsMock.moveSuspendedCoprocessorContextToActive())
        .to.emit(coprocessorContextsMock, "DeactivateCoprocessorContext")
        .withArgs(DefaultUint256)
        .and.emit(coprocessorContextsMock, "ActivateCoprocessorContext")
        .withArgs(DefaultUint256);
    });
  });

  describe("DecryptionMock", async function () {
    let decryptionCounterId = DefaultUint256;
    it("Should emit PublicDecryptionRequest event on public decryption request", async function () {
      decryptionCounterId++;
      await expect(decryptionMock.publicDecryptionRequest([DefaultBytes32]))
        .to.emit(decryptionMock, "PublicDecryptionRequest")
        .withArgs(decryptionCounterId, toValues([DefaultSnsCiphertextMaterial]));
    });

    it("Should emit PublicDecryptionResponse event on public decryption response", async function () {
      await expect(decryptionMock.publicDecryptionResponse(decryptionCounterId, DefaultBytes, DefaultBytes))
        .to.emit(decryptionMock, "PublicDecryptionResponse")
        .withArgs(decryptionCounterId, DefaultBytes, [DefaultBytes]);
    });

    it("Should emit UserDecryptionRequest event on user decryption request", async function () {
      decryptionCounterId++;
      await expect(
        decryptionMock.userDecryptionRequest(
          EmptyArray,
          DefaultRequestValidity,
          DefaultUint256,
          EmptyArray,
          DefaultAddress,
          DefaultBytes,
          DefaultBytes,
        ),
      )
        .to.emit(decryptionMock, "UserDecryptionRequest")
        .withArgs(decryptionCounterId, toValues([DefaultSnsCiphertextMaterial]), DefaultAddress, DefaultBytes);
    });

    it("Should emit UserDecryptionRequest event on delegated user decryption request", async function () {
      decryptionCounterId++;
      await expect(
        decryptionMock.delegatedUserDecryptionRequest(
          EmptyArray,
          DefaultRequestValidity,
          DefaultDelegationAccounts,
          DefaultUint256,
          EmptyArray,
          DefaultBytes,
          DefaultBytes,
        ),
      )
        .to.emit(decryptionMock, "UserDecryptionRequest")
        .withArgs(decryptionCounterId, toValues([DefaultSnsCiphertextMaterial]), DefaultAddress, DefaultBytes);
    });

    it("Should emit UserDecryptionResponse event on user decryption response", async function () {
      await expect(decryptionMock.userDecryptionResponse(decryptionCounterId, DefaultBytes, DefaultBytes))
        .to.emit(decryptionMock, "UserDecryptionResponse")
        .withArgs(decryptionCounterId, [DefaultBytes], [DefaultBytes]);
    });
  });

  describe("GatewayConfigMock", async function () {
    it("Should emit InitializeGatewayConfig event on initialization", async function () {
      await expect(
        gatewayConfigMock.initializeFromEmptyProxy(
          DefaultAddress,
          DefaultProtocolMetadata,
          DefaultUint256,
          DefaultUint256,
          DefaultUint256,
          [DefaultKmsNode],
          [DefaultCustodian],
        ),
      )
        .to.emit(gatewayConfigMock, "InitializeGatewayConfig")
        .withArgs(
          DefaultAddress,
          toValues(DefaultProtocolMetadata),
          DefaultUint256,
          toValues([DefaultKmsNode]),
          toValues([DefaultCustodian]),
        );
    });

    it("Should emit Reinitialization event on reinitialization", async function () {
      await expect(gatewayConfigMock.reinitializeV2([DefaultCustodian]))
        .to.emit(gatewayConfigMock, "ReinitializeGatewayConfigV2")
        .withArgs(toValues([DefaultCustodian]));
    });

    it("Should emit UpdatePauser event on update pauser call", async function () {
      await expect(gatewayConfigMock.updatePauser(DefaultAddress))
        .to.emit(gatewayConfigMock, "UpdatePauser")
        .withArgs(DefaultAddress);
    });

    it("Should emit UpdateMpcThreshold event on update MPC threshold call", async function () {
      await expect(gatewayConfigMock.updateMpcThreshold(DefaultUint256))
        .to.emit(gatewayConfigMock, "UpdateMpcThreshold")
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
        inputVerificationMock.verifyProofRequest(DefaultUint256, DefaultAddress, DefaultAddress, DefaultBytes),
      )
        .to.emit(inputVerificationMock, "VerifyProofRequest")
        .withArgs(zkProofCounterId, DefaultUint256, DefaultUint256, DefaultAddress, DefaultAddress, DefaultBytes);
    });

    it("Should emit VerifyProofResponse event on verify proof response", async function () {
      await expect(inputVerificationMock.verifyProofResponse(zkProofCounterId, [DefaultBytes32], DefaultBytes))
        .to.emit(inputVerificationMock, "VerifyProofResponse")
        .withArgs(zkProofCounterId, [DefaultBytes32], [DefaultBytes]);
    });

    it("Should emit RejectProofResponse event on reject proof response", async function () {
      await expect(inputVerificationMock.rejectProofResponse(zkProofCounterId))
        .to.emit(inputVerificationMock, "RejectProofResponse")
        .withArgs(zkProofCounterId);
    });
  });

  describe("KmsManagementMock", async function () {
    let preKeygenCounterId = DefaultUint256;
    let preKskgenCounterId = DefaultUint256;
    let crsgenCounterId = DefaultUint256;
    it("Should emit PreprocessKeygenRequest event on pre-keygen request", async function () {
      preKeygenCounterId++;
      await expect(kmsManagementMock.preprocessKeygenRequest(DefaultString))
        .to.emit(kmsManagementMock, "PreprocessKeygenRequest")
        .withArgs(preKeygenCounterId, DefaultBytes32);
    });

    it("Should emit PreprocessKeygenResponse event on pre-keygen response", async function () {
      await expect(kmsManagementMock.preprocessKeygenResponse(preKeygenCounterId, DefaultUint256))
        .to.emit(kmsManagementMock, "PreprocessKeygenResponse")
        .withArgs(preKeygenCounterId, DefaultUint256);
    });

    it("Should emit PreprocessKskgenRequest event on pre-kskgen request", async function () {
      preKskgenCounterId++;
      await expect(kmsManagementMock.preprocessKskgenRequest(DefaultString))
        .to.emit(kmsManagementMock, "PreprocessKskgenRequest")
        .withArgs(preKskgenCounterId, DefaultBytes32);
    });

    it("Should emit PreprocessKskgenResponse event on pre-kskgen response", async function () {
      await expect(kmsManagementMock.preprocessKskgenResponse(preKskgenCounterId, DefaultUint256))
        .to.emit(kmsManagementMock, "PreprocessKskgenResponse")
        .withArgs(preKskgenCounterId, DefaultUint256);
    });

    it("Should emit KeygenRequest event on keygen request", async function () {
      await expect(kmsManagementMock.keygenRequest(DefaultUint256))
        .to.emit(kmsManagementMock, "KeygenRequest")
        .withArgs(DefaultUint256, DefaultBytes32);
    });

    it("Should emit KeygenResponse event on keygen response", async function () {
      await expect(kmsManagementMock.keygenResponse(DefaultUint256, DefaultUint256))
        .to.emit(kmsManagementMock, "KeygenResponse")
        .withArgs(DefaultUint256, DefaultUint256, DefaultBytes32);
    });

    it("Should emit CrsgenRequest event on crsgen request", async function () {
      crsgenCounterId++;
      await expect(kmsManagementMock.crsgenRequest(DefaultString))
        .to.emit(kmsManagementMock, "CrsgenRequest")
        .withArgs(crsgenCounterId, DefaultBytes32);
    });

    it("Should emit CrsgenResponse event on crsgen request", async function () {
      await expect(kmsManagementMock.crsgenResponse(crsgenCounterId, DefaultUint256))
        .to.emit(kmsManagementMock, "CrsgenResponse")
        .withArgs(crsgenCounterId, DefaultUint256, DefaultBytes32);
    });

    it("Should emit KskgenRequest event on kskgen request", async function () {
      await expect(kmsManagementMock.kskgenRequest(DefaultUint256, DefaultUint256, DefaultUint256))
        .to.emit(kmsManagementMock, "KskgenRequest")
        .withArgs(DefaultUint256, DefaultUint256, DefaultUint256, DefaultBytes32);
    });

    it("Should emit KskgenResponse event on kskgen response", async function () {
      await expect(kmsManagementMock.kskgenResponse(DefaultUint256, DefaultUint256))
        .to.emit(kmsManagementMock, "KskgenResponse")
        .withArgs(DefaultUint256, DefaultUint256, DefaultBytes32);
    });

    it("Should emit ActivateKeyRequest event on activate key request", async function () {
      await expect(kmsManagementMock.activateKeyRequest(DefaultUint256))
        .to.emit(kmsManagementMock, "ActivateKeyRequest")
        .withArgs(DefaultUint256);
    });

    it("Should emit ActivateKeyResponse event on activate key response", async function () {
      await expect(kmsManagementMock.activateKeyResponse(DefaultUint256))
        .to.emit(kmsManagementMock, "ActivateKeyResponse")
        .withArgs(DefaultUint256);
    });

    it("Should emit AddFheParams event on add FHE params call", async function () {
      await expect(kmsManagementMock.addFheParams(DefaultString, DefaultBytes32))
        .to.emit(kmsManagementMock, "AddFheParams")
        .withArgs(DefaultString, DefaultBytes32);
    });

    it("Should emit UpdateFheParams event on update FHE params call", async function () {
      await expect(kmsManagementMock.updateFheParams(DefaultString, DefaultBytes32))
        .to.emit(kmsManagementMock, "UpdateFheParams")
        .withArgs(DefaultString, DefaultBytes32);
    });
  });

  describe("MultichainAclMock", async function () {
    it("Should emit AllowPublicDecrypt event on allow public decrypt call", async function () {
      await expect(multichainAclMock.allowPublicDecrypt(DefaultBytes32))
        .to.emit(multichainAclMock, "AllowPublicDecrypt")
        .withArgs(DefaultBytes32);
    });

    it("Should emit AllowAccount event on allow account call", async function () {
      await expect(multichainAclMock.allowAccount(DefaultBytes32, DefaultAddress))
        .to.emit(multichainAclMock, "AllowAccount")
        .withArgs(DefaultBytes32, DefaultAddress);
    });

    it("Should emit DelegateAccount event on delegate account call", async function () {
      await expect(multichainAclMock.delegateAccount(DefaultUint256, DefaultDelegationAccounts, [DefaultAddress]))
        .to.emit(multichainAclMock, "DelegateAccount")
        .withArgs(DefaultUint256, toValues(DefaultDelegationAccounts), [DefaultAddress]);
    });
  });
});
