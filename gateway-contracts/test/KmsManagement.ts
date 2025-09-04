import { HardhatEthersSigner, SignerWithAddress } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { EventLog, Wallet } from "ethers";
import hre from "hardhat";

import { IKmsManagement, KmsManagement, KmsManagement__factory } from "../typechain-types";
import {
  KeyTypeEnum,
  ParamsTypeEnum,
  createByteInput,
  createEIP712ResponseCrsgen,
  createEIP712ResponseKeygen,
  createEIP712ResponsePrepKeygen,
  createRandomWallet,
  getCrsId,
  getKeyId,
  getPrepKeygenId,
  getSignaturesCrsgen,
  getSignaturesKeygen,
  getSignaturesPrepKeygen,
  loadTestVariablesFixture,
  toValues,
} from "./utils";

// Trigger a key generation in KmsManagement contract
async function generateKey(
  kmsManagement: KmsManagement,
  owner: Wallet,
  gatewayChainId: number,
  kmsTxSenders: HardhatEthersSigner[],
  kmsSigners: HardhatEthersSigner[],
) {
  // Start a keygen with test parameters
  // This first triggers a preprocessing keygen request
  const txRequestPrepKeygen = await kmsManagement.connect(owner).keygen(ParamsTypeEnum.Test);

  // Get the prepKeygenId from the event in the transaction receipt
  const receiptPrepKeygen = await txRequestPrepKeygen.wait();
  const eventPrepKeygen = receiptPrepKeygen?.logs[0] as EventLog;
  const prepKeygenId = BigInt(eventPrepKeygen?.args[0]);

  const kmsManagementAddress = await kmsManagement.getAddress();

  // Create an EIP712 message for the preprocessing keygen response
  const eip712MessagePrepKeygen = createEIP712ResponsePrepKeygen(gatewayChainId, kmsManagementAddress, prepKeygenId);

  // Sign the preprocessing keygen EIP712 message with all KMS signers
  const kmsSignaturesPrepKeygen = await getSignaturesPrepKeygen(eip712MessagePrepKeygen, kmsSigners);

  // Trigger preprocessing keygen responses for all KMS nodes
  for (let i = 0; i < kmsTxSenders.length; i++) {
    await kmsManagement.connect(kmsTxSenders[i]).prepKeygenResponse(prepKeygenId, kmsSignaturesPrepKeygen[i]);
  }

  // Get the keyId from the keygen request event
  let keyId: bigint;
  const filter = kmsManagement.filters.KeygenRequest;
  const events = await kmsManagement.queryFilter(filter);
  if (events.length > 0) {
    keyId = BigInt(events[events.length - 1].args[1]);
  } else {
    throw new Error("No KeygenRequest event found");
  }

  // Create the key digests
  const serverKeyDigest: IKmsManagement.KeyDigestStruct = { keyType: KeyTypeEnum.Server, digest: createByteInput() };
  const publicKeyDigest: IKmsManagement.KeyDigestStruct = { keyType: KeyTypeEnum.Public, digest: createByteInput() };

  const keyDigests = [serverKeyDigest, publicKeyDigest];

  // Create an EIP712 message for the preprocessing keygen response
  const eip712MessageKeygen = createEIP712ResponseKeygen(
    gatewayChainId,
    kmsManagementAddress,
    prepKeygenId,
    keyId,
    keyDigests,
  );

  // Sign the preprocessing keygen EIP712 message with all KMS signers
  const kmsSignaturesKeygen = await getSignaturesKeygen(eip712MessageKeygen, kmsSigners);

  // Trigger keygen responses for all KMS nodes
  for (let i = 0; i < kmsTxSenders.length; i++) {
    await kmsManagement.connect(kmsTxSenders[i]).keygenResponse(keyId, keyDigests, kmsSignaturesKeygen[i]);
  }

  return {
    keyId,
    keyDigests,
  };
}

// Trigger a CRS generation in KmsManagement contract.
async function generateCrs(
  kmsManagement: KmsManagement,
  owner: Wallet,
  gatewayChainId: number,
  kmsTxSenders: HardhatEthersSigner[],
  kmsSigners: HardhatEthersSigner[],
  maxBitLength: number,
) {
  // Start a CRS generation with test parameters.
  const txRequestCrsgen = await kmsManagement.connect(owner).crsgenRequest(maxBitLength, ParamsTypeEnum.Test);

  // Get the crsId from the event in the transaction receipt.
  const receiptCrsgen = await txRequestCrsgen.wait();
  const eventCrsgen = receiptCrsgen?.logs[0] as EventLog;
  const crsId = BigInt(eventCrsgen?.args[0]);

  const kmsManagementAddress = await kmsManagement.getAddress();

  // Create an EIP712 message for the crsgen response.
  const crsDigest = createByteInput();
  const eip712MessageCrsgen = createEIP712ResponseCrsgen(
    gatewayChainId,
    kmsManagementAddress,
    crsId,
    maxBitLength,
    crsDigest,
  );

  // Sign the crsgen EIP712 message with all KMS signers.
  const kmsSignaturesCrsgen = await getSignaturesCrsgen(eip712MessageCrsgen, kmsSigners);

  // Trigger crsgen responses for all KMS nodes.
  for (let i = 0; i < kmsTxSenders.length; i++) {
    await kmsManagement.connect(kmsTxSenders[i]).crsgenResponse(crsId, crsDigest, kmsSignaturesCrsgen[i]);
  }

  return {
    crsId,
    crsDigest,
  };
}

describe("KmsManagement", function () {
  const fakeOwner = createRandomWallet();
  const maxBitLength = 256;

  // Fixture running a key generation.
  async function prepareKmsManagementKeygenFixture() {
    const fixtureData = await loadFixture(loadTestVariablesFixture);

    const { kmsManagement, owner, kmsTxSenders, kmsSigners } = fixtureData;

    // Get the gateway's chain ID.
    const gatewayChainId = hre.network.config.chainId!;

    // Generate key.
    const { keyId, keyDigests } = await generateKey(kmsManagement, owner, gatewayChainId, kmsTxSenders, kmsSigners);

    return { ...fixtureData, keyId, keyDigests };
  }

  // Fixture running a CRS generation.
  async function prepareKmsManagementCrsgenFixture() {
    const fixtureData = await loadFixture(loadTestVariablesFixture);

    const { kmsManagement, owner, kmsTxSenders, kmsSigners } = fixtureData;

    // Get the gateway's chain ID.
    const gatewayChainId = hre.network.config.chainId!;

    // Generate CRS.
    const { crsId, crsDigest } = await generateCrs(
      kmsManagement,
      owner,
      gatewayChainId,
      kmsTxSenders,
      kmsSigners,
      maxBitLength,
    );

    return {
      ...fixtureData,
      crsId,
      crsDigest,
    };
  }

  describe("Deployment", function () {
    let kmsManagementFactory: KmsManagement__factory;
    let kmsManagement: KmsManagement;
    let owner: Wallet;

    beforeEach(async function () {
      const fixtureData = await loadFixture(loadTestVariablesFixture);
      kmsManagement = fixtureData.kmsManagement;
      owner = fixtureData.owner;

      // Get the KmsManagement contract factory
      kmsManagementFactory = await hre.ethers.getContractFactory("KmsManagement", owner);
    });

    it("Should revert because initialization is not from an empty proxy", async function () {
      await expect(
        hre.upgrades.upgradeProxy(kmsManagement, kmsManagementFactory, {
          call: { fn: "initializeFromEmptyProxy", args: [] },
        }),
      ).to.be.revertedWithCustomError(kmsManagement, "NotInitializingFromEmptyProxy");
    });
  });

  describe("Key generation", function () {
    it("Should revert because of access controls", async function () {
      const { gatewayConfig, kmsManagement } = await loadFixture(loadTestVariablesFixture);

      // Check that only the owner can trigger a keygen request.
      await expect(kmsManagement.connect(fakeOwner).keygen(ParamsTypeEnum.Default))
        .to.be.revertedWithCustomError(kmsManagement, "NotGatewayOwner")
        .withArgs(fakeOwner.address);

      // Check that only the KMS transaction sender can send a preprocessing keygen response.
      await expect(kmsManagement.connect(fakeOwner).prepKeygenResponse(0n, "0x"))
        .to.be.revertedWithCustomError(gatewayConfig, "NotKmsTxSender")
        .withArgs(fakeOwner.address);

      // Check that only the KMS transaction sender can trigger a keygen response.
      await expect(kmsManagement.connect(fakeOwner).keygenResponse(0n, [], "0x"))
        .to.be.revertedWithCustomError(gatewayConfig, "NotKmsTxSender")
        .withArgs(fakeOwner.address);
    });

    it("Should handle a key generation", async function () {
      const { kmsManagement, owner, kmsTxSenders, kmsSigners, kmsNodeS3BucketUrls } =
        await loadFixture(loadTestVariablesFixture);
      const paramsType = ParamsTypeEnum.Test;
      const gatewayChainId = hre.network.config.chainId!;
      const kmsManagementAddress = await kmsManagement.getAddress();

      // Trigger a keygen request.
      const txRequest = await kmsManagement.connect(owner).keygen(paramsType);

      // Check for the PrepKeygenRequest event.
      const prepKeygenId = getPrepKeygenId(1);
      const epochId = 0;
      await expect(txRequest).to.emit(kmsManagement, "PrepKeygenRequest").withArgs(prepKeygenId, epochId, paramsType);

      // Define a keyId for keygen responses.
      const keyId = getKeyId(1);

      // Create the EIP712 message
      const eip712MessagePrepKeygen = createEIP712ResponsePrepKeygen(
        gatewayChainId,
        kmsManagementAddress,
        prepKeygenId,
      );

      // Sign the preprocessing keygen EIP712 message with all KMS signers.
      const kmsSignaturesPrepKeygen = await getSignaturesPrepKeygen(eip712MessagePrepKeygen, kmsSigners);

      // Trigger a preprocessing keygen responses.
      const txPrepKeygenResponse1 = await kmsManagement
        .connect(kmsTxSenders[0])
        .prepKeygenResponse(prepKeygenId, kmsSignaturesPrepKeygen[0]);

      // Check that the first response does not emit an event (consensus is not reached yet).
      await expect(txPrepKeygenResponse1).to.not.emit(kmsManagement, "KeygenRequest");

      // Check that a KMS node cannot respond twice to the same preprocessing keygen request.
      await expect(kmsManagement.connect(kmsTxSenders[0]).prepKeygenResponse(prepKeygenId, kmsSignaturesPrepKeygen[0]))
        .to.be.revertedWithCustomError(kmsManagement, "KmsAlreadySignedForPrepKeygen")
        .withArgs(prepKeygenId, kmsSigners[0]);

      // Trigger a second keygen response.
      await kmsManagement.connect(kmsTxSenders[1]).prepKeygenResponse(prepKeygenId, kmsSignaturesPrepKeygen[1]);

      // Trigger a third keygen response which should reach consensus (4 / 2 + 1 = 3) and thus emit an event.
      const txPrepKeygenResponse3 = await kmsManagement
        .connect(kmsTxSenders[2])
        .prepKeygenResponse(prepKeygenId, kmsSignaturesPrepKeygen[2]);

      // Check for the KeygenRequest event.
      await expect(txPrepKeygenResponse3).to.emit(kmsManagement, "KeygenRequest").withArgs(prepKeygenId, keyId);

      // The 4th response should be ignored (not reverted) and not emit the KeygenRequest event.
      const txPrepKeygenResponse4 = await kmsManagement
        .connect(kmsTxSenders[3])
        .prepKeygenResponse(prepKeygenId, kmsSignaturesPrepKeygen[3]);

      // Check that the 4th response does not emit the KeygenRequest event.
      await expect(txPrepKeygenResponse4).to.not.emit(kmsManagement, "KeygenRequest");

      // Prepare the keygen responses materials.
      const serverKeyDigest: IKmsManagement.KeyDigestStruct = {
        keyType: KeyTypeEnum.Server,
        digest: createByteInput(),
      };
      const publicKeyDigest: IKmsManagement.KeyDigestStruct = {
        keyType: KeyTypeEnum.Public,
        digest: createByteInput(),
      };
      const keyDigests = [serverKeyDigest, publicKeyDigest];

      // Create the EIP712 message.
      const eip712MessageKeygen = createEIP712ResponseKeygen(
        gatewayChainId,
        kmsManagementAddress,
        prepKeygenId,
        keyId,
        keyDigests,
      );

      // Sign the keygen EIP712 message with all KMS signers.
      const kmsSignaturesKeygen = await getSignaturesKeygen(eip712MessageKeygen, kmsSigners);

      // Trigger the keygen responses.
      const txKeygenResponse1 = await kmsManagement
        .connect(kmsTxSenders[0])
        .keygenResponse(keyId, keyDigests, kmsSignaturesKeygen[0]);

      // Check that the first response does not emit an event (consensus is not reached yet).
      await expect(txKeygenResponse1).to.not.emit(kmsManagement, "ActivateKey");

      // Check that a KMS node cannot respond twice to the same keygen request.
      await expect(kmsManagement.connect(kmsTxSenders[0]).keygenResponse(keyId, keyDigests, kmsSignaturesKeygen[0]))
        .to.be.revertedWithCustomError(kmsManagement, "KmsAlreadySignedForKeygen")
        .withArgs(keyId, kmsSigners[0]);

      // Trigger a second keygen response.
      await kmsManagement.connect(kmsTxSenders[1]).keygenResponse(keyId, keyDigests, kmsSignaturesKeygen[1]);

      // Trigger a third keygen response which should reach consensus (4 / 2 + 1 = 3) and thus emit the ActivateKey event.
      const txKeygenResponse3 = await kmsManagement
        .connect(kmsTxSenders[2])
        .keygenResponse(keyId, keyDigests, kmsSignaturesKeygen[2]);

      // Check for the ActivateKey event.
      await expect(txKeygenResponse3)
        .to.emit(kmsManagement, "ActivateKey")
        .withArgs(keyId, kmsNodeS3BucketUrls.slice(0, 3), toValues(keyDigests));

      // The 4th response should be ignored (not reverted).
      const txKeygenResponse4 = await kmsManagement
        .connect(kmsTxSenders[3])
        .keygenResponse(keyId, keyDigests, kmsSignaturesKeygen[3]);

      // Check that the 4th response does not emit the ActivateKey event.
      await expect(txKeygenResponse4).to.not.emit(kmsManagement, "ActivateKey");
    });

    it("Should revert on get params type because the key is not generated", async function () {
      const { kmsManagement } = await loadFixture(loadTestVariablesFixture);

      const fakeKeyId = getKeyId(1);

      // Check that getting the params type of a non-existing key reverts.
      await expect(kmsManagement.getKeyParamsType(fakeKeyId))
        .to.be.revertedWithCustomError(kmsManagement, "KeyNotGenerated")
        .withArgs(fakeKeyId);
    });

    it("Should get params type associated to the key", async function () {
      const { kmsManagement, keyId } = await loadFixture(prepareKmsManagementKeygenFixture);

      // Check that the params type associated to the key is correct.
      expect(await kmsManagement.getKeyParamsType(keyId)).to.equal(ParamsTypeEnum.Test);
    });

    it("Should revert on get materials because the key is not generated", async function () {
      const { kmsManagement } = await loadFixture(loadTestVariablesFixture);

      const fakeKeyId = getKeyId(5);

      // Check that getting the materials of a non-existing key reverts.
      await expect(kmsManagement.getKeyMaterials(fakeKeyId))
        .to.be.revertedWithCustomError(kmsManagement, "KeyNotGenerated")
        .withArgs(fakeKeyId);
    });

    it("Should get materials associated to the key", async function () {
      const { kmsManagement, keyId, keyDigests, kmsNodeS3BucketUrls } = await loadFixture(
        prepareKmsManagementKeygenFixture,
      );

      // Check that the materials associated to the key are correct.
      expect(await kmsManagement.getKeyMaterials(keyId)).to.deep.equal([kmsNodeS3BucketUrls, toValues(keyDigests)]);
    });

    it("Should get the current active key", async function () {
      const { kmsManagement, keyId } = await loadFixture(prepareKmsManagementKeygenFixture);

      // Check that the current active key is correct.
      expect(await kmsManagement.getActiveKeyId()).to.equal(keyId);
    });

    it("Should get the list of KMS transaction senders associated to the key", async function () {
      const { kmsManagement, keyId, kmsTxSenders } = await loadFixture(prepareKmsManagementKeygenFixture);

      // Check that the KMS transaction senders associated to the key are correct.
      const kmsTxSenderAddresses = kmsTxSenders.map((s) => s.address);
      expect(await kmsManagement.getConsensusTxSenders(keyId)).to.deep.equal(kmsTxSenderAddresses);
    });
  });

  describe("CRS generation", async function () {
    it("Should revert because of access controls", async function () {
      const { gatewayConfig, kmsManagement } = await loadFixture(loadTestVariablesFixture);

      // Check that only the owner can trigger a CRS generation request.
      await expect(kmsManagement.connect(fakeOwner).crsgenRequest(maxBitLength, ParamsTypeEnum.Test))
        .to.be.revertedWithCustomError(kmsManagement, "NotGatewayOwner")
        .withArgs(fakeOwner.address);

      // Check that only the KMS transaction sender can send a CRS generation response.
      await expect(kmsManagement.connect(fakeOwner).crsgenResponse(0n, "0x", "0x"))
        .to.be.revertedWithCustomError(gatewayConfig, "NotKmsTxSender")
        .withArgs(fakeOwner.address);
    });

    it("Should handle a CRS generation", async function () {
      const { kmsManagement, owner, kmsTxSenders, kmsSigners, kmsNodeS3BucketUrls } =
        await loadFixture(loadTestVariablesFixture);

      // Define an expected crsId.
      const crsId = getCrsId(1);

      // Trigger a CRS generation request.
      const txRequest = await kmsManagement.connect(owner).crsgenRequest(maxBitLength, ParamsTypeEnum.Test);

      // Check for the CrsgenRequest event.
      await expect(txRequest)
        .to.emit(kmsManagement, "CrsgenRequest")
        .withArgs(crsId, maxBitLength, ParamsTypeEnum.Test);

      // Trigger a CRS generation response with the first KMS node.
      const crsDigest = createByteInput();
      const kmsManagementAddress = await kmsManagement.getAddress();
      const eip712MessageCrsgen = createEIP712ResponseCrsgen(
        hre.network.config.chainId!,
        kmsManagementAddress,
        crsId,
        maxBitLength,
        crsDigest,
      );

      // Sign the crsgen EIP712 message with all KMS signers.
      const kmsSignaturesCrsgen = await getSignaturesCrsgen(eip712MessageCrsgen, kmsSigners);

      const txResponse1 = await kmsManagement
        .connect(kmsTxSenders[0])
        .crsgenResponse(crsId, crsDigest, kmsSignaturesCrsgen[0]);

      // Check that the first response does not emit an event (consensus is not reached yet).
      await expect(txResponse1).to.not.emit(kmsManagement, "ActivateCrs");

      // Check that a KMS node cannot respond twice to the same CRS generation request.
      await expect(kmsManagement.connect(kmsTxSenders[0]).crsgenResponse(crsId, crsDigest, kmsSignaturesCrsgen[0]))
        .to.be.revertedWithCustomError(kmsManagement, "KmsAlreadySignedForCrsgen")
        .withArgs(crsId, kmsSigners[0]);

      // Trigger a second CRS generation response with the first KMS node.
      await kmsManagement.connect(kmsTxSenders[1]).crsgenResponse(crsId, crsDigest, kmsSignaturesCrsgen[1]);

      // Trigger a third CRS generation response which should reach consensus (4 / 2 + 1 = 3) and thus emit an event.
      const txResponse3 = await kmsManagement
        .connect(kmsTxSenders[2])
        .crsgenResponse(crsId, crsDigest, kmsSignaturesCrsgen[2]);

      // Check for the ActivateCrs event.
      await expect(txResponse3)
        .to.emit(kmsManagement, "ActivateCrs")
        .withArgs(crsId, kmsNodeS3BucketUrls.slice(0, 3), crsDigest);

      // The 4th response should be ignored (not reverted) and not emit the ActivateCrs event.
      const txResponse4 = await kmsManagement
        .connect(kmsTxSenders[3])
        .crsgenResponse(crsId, crsDigest, kmsSignaturesCrsgen[3]);

      // Check that the 4th response does not emit the ActivateCrs event.
      await expect(txResponse4).to.not.emit(kmsManagement, "ActivateCrs");
    });

    it("Should revert on get params type because the CRS is not generated", async function () {
      const { kmsManagement } = await loadFixture(loadTestVariablesFixture);

      const fakeCrsId = getCrsId(1);

      // Check that getting the params type of a non-existing CRS reverts
      await expect(kmsManagement.getCrsParamsType(fakeCrsId))
        .to.be.revertedWithCustomError(kmsManagement, "CrsNotGenerated")
        .withArgs(fakeCrsId);
    });

    it("Should get params type associated to the CRS", async function () {
      const { kmsManagement, crsId } = await loadFixture(prepareKmsManagementCrsgenFixture);

      // Check that the params type associated to the CRS is correct.
      expect(await kmsManagement.getCrsParamsType(crsId)).to.equal(ParamsTypeEnum.Test);
    });

    it("Should revert on get materials because the CRS is not generated", async function () {
      const { kmsManagement } = await loadFixture(loadTestVariablesFixture);

      const fakeCrsId = getCrsId(5);

      // Check that getting the materials of a non-existing CRS reverts.
      await expect(kmsManagement.getCrsMaterials(fakeCrsId))
        .to.be.revertedWithCustomError(kmsManagement, "CrsNotGenerated")
        .withArgs(fakeCrsId);
    });

    it("Should get materials associated to the CRS", async function () {
      const { kmsManagement, crsId, crsDigest, kmsNodeS3BucketUrls } = await loadFixture(
        prepareKmsManagementCrsgenFixture,
      );

      // Check that the materials associated to the CRS are correct.
      expect(await kmsManagement.getCrsMaterials(crsId)).to.deep.equal([kmsNodeS3BucketUrls, crsDigest]);
    });

    it("Should get the current active CRS", async function () {
      const { kmsManagement, crsId } = await loadFixture(prepareKmsManagementCrsgenFixture);

      // Check that the current active CRS is correct.
      expect(await kmsManagement.getActiveCrsId()).to.equal(crsId);
    });

    it("Should get the list of KMS transaction senders associated to the CRS", async function () {
      const { kmsManagement, crsId, kmsTxSenders } = await loadFixture(prepareKmsManagementCrsgenFixture);

      // Check that the KMS transaction senders associated to the CRS are correct.
      const kmsTxSenderAddresses = kmsTxSenders.map((s) => s.address);
      expect(await kmsManagement.getConsensusTxSenders(crsId)).to.deep.equal(kmsTxSenderAddresses);
    });
  });

  describe("FHE parameters", async function () {});

  describe("Pause", async function () {
    let kmsManagement: KmsManagement;
    let owner: Wallet;
    let pauser: SignerWithAddress;

    beforeEach(async function () {
      const fixtureData = await loadFixture(loadTestVariablesFixture);
      kmsManagement = fixtureData.kmsManagement;
      owner = fixtureData.owner;
      pauser = fixtureData.pauser;
    });

    it("Should pause the contract with the pauser and unpause with the owner", async function () {
      // Check that the contract is not paused
      expect(await kmsManagement.paused()).to.be.false;

      // Pause the contract with the pauser address
      await expect(kmsManagement.connect(pauser).pause()).to.emit(kmsManagement, "Paused").withArgs(pauser);
      expect(await kmsManagement.paused()).to.be.true;

      // Unpause the contract with the owner address
      await expect(kmsManagement.connect(owner).unpause()).to.emit(kmsManagement, "Unpaused").withArgs(owner);
      expect(await kmsManagement.paused()).to.be.false;
    });

    it("Should revert on pause because sender is not the pauser", async function () {
      const fakePauser = createRandomWallet();

      await expect(kmsManagement.connect(fakePauser).pause())
        .to.be.revertedWithCustomError(kmsManagement, "NotPauserOrGatewayConfig")
        .withArgs(fakePauser.address);
    });

    it("Should revert on unpause because sender is not the owner", async function () {
      // Pause the contract with the pauser address
      await kmsManagement.connect(pauser).pause();

      const fakeOwner = createRandomWallet();

      await expect(kmsManagement.connect(fakeOwner).unpause())
        .to.be.revertedWithCustomError(kmsManagement, "NotOwnerOrGatewayConfig")
        .withArgs(fakeOwner.address);
    });
  });
});
