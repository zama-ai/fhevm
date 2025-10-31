import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { EventLog, Wallet } from "ethers";
import hre from "hardhat";

import { IKMSGeneration, KMSGeneration, KMSGeneration__factory } from "../typechain-types";
import {
  EIP712,
  KeyTypeEnum,
  ParamsTypeEnum,
  createByteInput,
  createEIP712ResponseCrsgen,
  createEIP712ResponseKeygen,
  createEIP712ResponsePrepKeygen,
  createRandomWallet,
  getCrsId,
  getKeyId,
  getKeyReshareId,
  getPrepKeygenId,
  getSignaturesCrsgen,
  getSignaturesKeygen,
  getSignaturesPrepKeygen,
  loadTestVariablesFixture,
  toValues,
} from "./utils";

// Trigger a key generation in KMSGeneration contract
export async function generateKey(
  kmsGeneration: KMSGeneration,
  owner: Wallet,
  gatewayChainId: number,
  kmsTxSenders: HardhatEthersSigner[],
  kmsSigners: HardhatEthersSigner[],
  keyDigests: IKMSGeneration.KeyDigestStruct[],
): Promise<bigint> {
  // Start a keygen with test parameters
  // This first triggers a preprocessing keygen request
  const paramsType = ParamsTypeEnum.Test;
  const txRequestPrepKeygen = await kmsGeneration.connect(owner).keygen(paramsType);

  // Get the prepKeygenId from the event in the transaction receipt
  const receiptPrepKeygen = await txRequestPrepKeygen.wait();
  const eventPrepKeygen = receiptPrepKeygen?.logs[0] as EventLog;
  const prepKeygenId = BigInt(eventPrepKeygen?.args[0]);

  const kmsGenerationAddress = await kmsGeneration.getAddress();

  // Create an EIP712 message for the preprocessing keygen response
  const eip712MessagePrepKeygen = createEIP712ResponsePrepKeygen(gatewayChainId, kmsGenerationAddress, prepKeygenId);

  // Sign the preprocessing keygen EIP712 message with all KMS signers
  const kmsSignaturesPrepKeygen = await getSignaturesPrepKeygen(eip712MessagePrepKeygen, kmsSigners);

  // Trigger preprocessing keygen responses for all KMS nodes
  for (let i = 0; i < kmsTxSenders.length; i++) {
    await kmsGeneration.connect(kmsTxSenders[i]).prepKeygenResponse(prepKeygenId, kmsSignaturesPrepKeygen[i]);
  }

  // Get the keyId from the keygen request event
  let keyId: bigint;
  const filter = kmsGeneration.filters.KeygenRequest;
  const events = await kmsGeneration.queryFilter(filter);
  if (events.length > 0) {
    keyId = BigInt(events[events.length - 1].args[1]);
  } else {
    throw new Error("No KeygenRequest event found");
  }

  // Create an EIP712 message for the preprocessing keygen response
  const eip712MessageKeygen = createEIP712ResponseKeygen(
    gatewayChainId,
    kmsGenerationAddress,
    prepKeygenId,
    keyId,
    keyDigests,
  );

  // Sign the preprocessing keygen EIP712 message with all KMS signers
  const kmsSignaturesKeygen = await getSignaturesKeygen(eip712MessageKeygen, kmsSigners);

  // Trigger keygen responses for all KMS nodes
  for (let i = 0; i < kmsTxSenders.length; i++) {
    await kmsGeneration.connect(kmsTxSenders[i]).keygenResponse(keyId, keyDigests, kmsSignaturesKeygen[i]);
  }

  return keyId;
}

// Trigger a CRS generation in KMSGeneration contract.
async function generateCrs(
  kmsGeneration: KMSGeneration,
  owner: Wallet,
  gatewayChainId: number,
  kmsTxSenders: HardhatEthersSigner[],
  kmsSigners: HardhatEthersSigner[],
  maxBitLength: number,
  crsDigest: string,
) {
  // Start a CRS generation with test parameters.
  const txRequestCrsgen = await kmsGeneration.connect(owner).crsgenRequest(maxBitLength, ParamsTypeEnum.Test);

  // Get the crsId from the event in the transaction receipt.
  const receiptCrsgen = await txRequestCrsgen.wait();
  const eventCrsgen = receiptCrsgen?.logs[0] as EventLog;
  const crsId = BigInt(eventCrsgen?.args[0]);

  const kmsGenerationAddress = await kmsGeneration.getAddress();

  // Create an EIP712 message for the crsgen response.
  const eip712MessageCrsgen = createEIP712ResponseCrsgen(
    gatewayChainId,
    kmsGenerationAddress,
    crsId,
    maxBitLength,
    crsDigest,
  );

  // Sign the crsgen EIP712 message with all KMS signers.
  const kmsSignaturesCrsgen = await getSignaturesCrsgen(eip712MessageCrsgen, kmsSigners);

  // Trigger crsgen responses for all KMS nodes.
  for (let i = 0; i < kmsTxSenders.length; i++) {
    await kmsGeneration.connect(kmsTxSenders[i]).crsgenResponse(crsId, crsDigest, kmsSignaturesCrsgen[i]);
  }

  return crsId;
}

describe("KMSGeneration", function () {
  // Get the gateway's chain ID.
  const gatewayChainId = hre.network.config.chainId!;

  // Define a fake values.
  const fakeOwner = createRandomWallet();

  let kmsGeneration: KMSGeneration;
  let owner: Wallet;
  let kmsTxSenders: HardhatEthersSigner[];
  let kmsSigners: HardhatEthersSigner[];
  let kmsNodeStorageUrls: string[];

  describe("Deployment", function () {
    let kmsGenerationFactory: KMSGeneration__factory;

    beforeEach(async function () {
      const fixtureData = await loadFixture(loadTestVariablesFixture);
      kmsGeneration = fixtureData.kmsGeneration;
      owner = fixtureData.owner;

      // Get the KMSGeneration contract factory
      kmsGenerationFactory = await hre.ethers.getContractFactory("KMSGeneration", owner);
    });

    it("Should revert because initialization is not from an empty proxy", async function () {
      await expect(
        hre.upgrades.upgradeProxy(kmsGeneration, kmsGenerationFactory, {
          call: { fn: "initializeFromEmptyProxy", args: [] },
        }),
      ).to.be.revertedWithCustomError(kmsGeneration, "NotInitializingFromEmptyProxy");
    });
  });

  describe("Key generation", function () {
    // Define the key digests.
    const serverKeyDigest: IKMSGeneration.KeyDigestStruct = {
      keyType: KeyTypeEnum.Server,
      digest: createByteInput(),
    };
    const publicKeyDigest: IKMSGeneration.KeyDigestStruct = {
      keyType: KeyTypeEnum.Public,
      digest: createByteInput(),
    };
    const keyDigests = [serverKeyDigest, publicKeyDigest];

    describe("Before key generation", function () {
      // Define a fake key ID.
      const fakeKeyId = getKeyId(1);

      beforeEach(async function () {
        const fixtureData = await loadFixture(loadTestVariablesFixture);
        kmsGeneration = fixtureData.kmsGeneration;
      });

      it("Should revert because of access controls", async function () {
        // Check that only the owner can trigger a keygen request.
        await expect(kmsGeneration.connect(fakeOwner).keygen(ParamsTypeEnum.Default))
          .to.be.revertedWithCustomError(kmsGeneration, "NotGatewayOwner")
          .withArgs(fakeOwner.address);

        // Check that only the KMS transaction sender can send a preprocessing keygen response.
        await expect(kmsGeneration.connect(fakeOwner).prepKeygenResponse(0n, "0x"))
          .to.be.revertedWithCustomError(kmsGeneration, "NotKmsTxSender")
          .withArgs(fakeOwner.address);

        // Check that only the KMS transaction sender can trigger a keygen response.
        await expect(kmsGeneration.connect(fakeOwner).keygenResponse(0n, [], "0x"))
          .to.be.revertedWithCustomError(kmsGeneration, "NotKmsTxSender")
          .withArgs(fakeOwner.address);
      });

      it("Should revert on get params type because the key is not generated", async function () {
        // Check that getting the params type of a non-existing key reverts.
        await expect(kmsGeneration.getKeyParamsType(fakeKeyId))
          .to.be.revertedWithCustomError(kmsGeneration, "KeyNotGenerated")
          .withArgs(fakeKeyId);
      });

      it("Should revert on get materials because the key is not generated", async function () {
        // Check that getting the materials of a non-existing key reverts.
        await expect(kmsGeneration.getKeyMaterials(fakeKeyId))
          .to.be.revertedWithCustomError(kmsGeneration, "KeyNotGenerated")
          .withArgs(fakeKeyId);
      });
    });

    describe("During key generation", function () {
      // Define input values
      const paramsType = ParamsTypeEnum.Test;
      const epochId = 0;

      // Define the expected keyId.
      const keyId = getKeyId(1);

      // Define the expected prepKeygenId.
      const prepKeygenId = getPrepKeygenId(1);

      let kmsGenerationAddress: string;
      let kmsSigners: HardhatEthersSigner[];
      let eip712MessagePrepKeygen: EIP712;
      let kmsSignaturesPrepKeygen: string[];
      let eip712MessageKeygen: EIP712;
      let kmsSignaturesKeygen: string[];

      beforeEach(async function () {
        const fixtureData = await loadFixture(loadTestVariablesFixture);
        kmsGeneration = fixtureData.kmsGeneration;
        owner = fixtureData.owner;
        kmsTxSenders = fixtureData.kmsTxSenders;
        kmsSigners = fixtureData.kmsSigners;
        kmsNodeStorageUrls = fixtureData.kmsNodeStorageUrls;

        // Get the KMSGeneration contract address.
        kmsGenerationAddress = await kmsGeneration.getAddress();

        // Create the EIP712 message
        eip712MessagePrepKeygen = createEIP712ResponsePrepKeygen(gatewayChainId, kmsGenerationAddress, prepKeygenId);

        // Sign the preprocessing keygen EIP712 message with all KMS signers.
        kmsSignaturesPrepKeygen = await getSignaturesPrepKeygen(eip712MessagePrepKeygen, kmsSigners);

        // Create the EIP712 message.
        eip712MessageKeygen = createEIP712ResponseKeygen(
          gatewayChainId,
          kmsGenerationAddress,
          prepKeygenId,
          keyId,
          keyDigests,
        );

        // Sign the keygen EIP712 message with all KMS signers.
        kmsSignaturesKeygen = await getSignaturesKeygen(eip712MessageKeygen, kmsSigners);
      });

      it("Should handle a key generation", async function () {
        // Trigger a keygen request.
        const txRequest = await kmsGeneration.connect(owner).keygen(paramsType);

        // Check for the PrepKeygenRequest event.
        await expect(txRequest).to.emit(kmsGeneration, "PrepKeygenRequest").withArgs(prepKeygenId, epochId, paramsType);

        // Trigger a preprocessing keygen responses.
        const txPrepKeygenResponse1 = await kmsGeneration
          .connect(kmsTxSenders[0])
          .prepKeygenResponse(prepKeygenId, kmsSignaturesPrepKeygen[0]);

        // Check that the first response does not emit an event (consensus is not reached yet).
        await expect(txPrepKeygenResponse1).to.not.emit(kmsGeneration, "KeygenRequest");

        // Check that a KMS node cannot respond twice to the same preprocessing keygen request.
        await expect(
          kmsGeneration.connect(kmsTxSenders[0]).prepKeygenResponse(prepKeygenId, kmsSignaturesPrepKeygen[0]),
        )
          .to.be.revertedWithCustomError(kmsGeneration, "KmsAlreadySignedForPrepKeygen")
          .withArgs(prepKeygenId, kmsSigners[0]);

        // Trigger a second keygen response.
        await kmsGeneration.connect(kmsTxSenders[1]).prepKeygenResponse(prepKeygenId, kmsSignaturesPrepKeygen[1]);

        // Trigger a third keygen response which should reach consensus (4 / 2 + 1 = 3) and thus emit an event.
        const txPrepKeygenResponse3 = await kmsGeneration
          .connect(kmsTxSenders[2])
          .prepKeygenResponse(prepKeygenId, kmsSignaturesPrepKeygen[2]);

        // Check for the KeygenRequest event.
        await expect(txPrepKeygenResponse3).to.emit(kmsGeneration, "KeygenRequest").withArgs(prepKeygenId, keyId);

        // The 4th response should be ignored (not reverted) and not emit the KeygenRequest event.
        const txPrepKeygenResponse4 = await kmsGeneration
          .connect(kmsTxSenders[3])
          .prepKeygenResponse(prepKeygenId, kmsSignaturesPrepKeygen[3]);

        // Check that the 4th response does not emit the KeygenRequest event.
        await expect(txPrepKeygenResponse4).to.not.emit(kmsGeneration, "KeygenRequest");

        // Trigger the keygen responses.
        const txKeygenResponse1 = await kmsGeneration
          .connect(kmsTxSenders[0])
          .keygenResponse(keyId, keyDigests, kmsSignaturesKeygen[0]);

        // Check that the first response does not emit an event (consensus is not reached yet).
        await expect(txKeygenResponse1).to.not.emit(kmsGeneration, "ActivateKey");

        // Check that a KMS node cannot respond twice to the same keygen request.
        await expect(kmsGeneration.connect(kmsTxSenders[0]).keygenResponse(keyId, keyDigests, kmsSignaturesKeygen[0]))
          .to.be.revertedWithCustomError(kmsGeneration, "KmsAlreadySignedForKeygen")
          .withArgs(keyId, kmsSigners[0]);

        // Trigger a second keygen response.
        await kmsGeneration.connect(kmsTxSenders[1]).keygenResponse(keyId, keyDigests, kmsSignaturesKeygen[1]);

        // Trigger a third keygen response which should reach consensus (4 / 2 + 1 = 3) and thus emit the ActivateKey event.
        const txKeygenResponse3 = await kmsGeneration
          .connect(kmsTxSenders[2])
          .keygenResponse(keyId, keyDigests, kmsSignaturesKeygen[2]);

        // Check for the ActivateKey event.
        await expect(txKeygenResponse3)
          .to.emit(kmsGeneration, "ActivateKey")
          .withArgs(keyId, kmsNodeStorageUrls.slice(0, 3), toValues(keyDigests));

        // The 4th response should be ignored (not reverted).
        const txKeygenResponse4 = await kmsGeneration
          .connect(kmsTxSenders[3])
          .keygenResponse(keyId, keyDigests, kmsSignaturesKeygen[3]);

        // Check that the 4th response does not emit the ActivateKey event.
        await expect(txKeygenResponse4).to.not.emit(kmsGeneration, "ActivateKey");
      });

      it("Should emit an event when calling a single prepKeygenResponse", async function () {
        // Trigger a keygen request.
        // This is needed to generate and store the prepKeygenId
        await kmsGeneration.connect(owner).keygen(paramsType);

        await expect(
          kmsGeneration.connect(kmsTxSenders[0]).prepKeygenResponse(prepKeygenId, kmsSignaturesPrepKeygen[0]),
        )
          .to.emit(kmsGeneration, "PrepKeygenResponse")
          .withArgs(prepKeygenId, kmsSignaturesPrepKeygen[0], kmsTxSenders[0].address);
      });

      it("Should emit an event when calling a single keygenResponse", async function () {
        // Trigger a keygen request.
        // This is needed to generate and store the necessary values in the KMSGeneration contract
        // fetched in the keygen response.
        await kmsGeneration.connect(owner).keygen(paramsType);

        await expect(kmsGeneration.connect(kmsTxSenders[0]).keygenResponse(keyId, keyDigests, kmsSignaturesKeygen[0]))
          .to.emit(kmsGeneration, "KeygenResponse")
          .withArgs(keyId, toValues(keyDigests), kmsSignaturesKeygen[0], kmsTxSenders[0].address);
      });

      it("Should revert because the signer and the tx sender do not correspond to the same coprocessor during preprocessing keygen", async function () {
        // Trigger a keygen request.
        // This is needed to generate and store the prepKeygenId
        await kmsGeneration.connect(owner).keygen(paramsType);

        // Check that triggering a preprocessing keygen response using a signature from the first KMS signer
        // with the second KMS transaction sender reverts
        await expect(
          kmsGeneration.connect(kmsTxSenders[1]).prepKeygenResponse(prepKeygenId, kmsSignaturesPrepKeygen[0]),
        )
          .to.be.revertedWithCustomError(kmsGeneration, "KmsSignerDoesNotMatchTxSender")
          .withArgs(kmsSigners[0].address, kmsTxSenders[1].address);
      });

      it("Should revert because the signer and the tx sender do not correspond to the same coprocessor during keygen", async function () {
        // Trigger a keygen request.
        // This is needed to generate and store the necessary values in the KMSGeneration contract
        // fetched in the keygen response.
        await kmsGeneration.connect(owner).keygen(paramsType);

        // Check that triggering a keygen response using a signature from the first KMS signer
        // with the second KMS transaction sender reverts
        await expect(kmsGeneration.connect(kmsTxSenders[1]).keygenResponse(keyId, keyDigests, kmsSignaturesKeygen[0]))
          .to.be.revertedWithCustomError(kmsGeneration, "KmsSignerDoesNotMatchTxSender")
          .withArgs(kmsSigners[0].address, kmsTxSenders[1].address);
      });

      it("Should revert because the preprocessing keygen request is not requested yet", async function () {
        // Trigger a keygen request.
        // Check that triggering a preprocessing keygen response using a non-existing prepKeygenId reverts
        await expect(
          kmsGeneration.connect(kmsTxSenders[0]).prepKeygenResponse(prepKeygenId, kmsSignaturesPrepKeygen[0]),
        )
          .to.be.revertedWithCustomError(kmsGeneration, "PrepKeygenNotRequested")
          .withArgs(prepKeygenId);
      });

      it("Should revert because the keygen request is not requested yet", async function () {
        // Trigger a keygen request.
        // Check that triggering a keygen response using a non-existing keyId reverts
        await expect(kmsGeneration.connect(kmsTxSenders[0]).keygenResponse(keyId, keyDigests, kmsSignaturesKeygen[0]))
          .to.be.revertedWithCustomError(kmsGeneration, "KeygenNotRequested")
          .withArgs(keyId);
      });

      it("Should revert because the preprocessing keygen request is ongoing", async function () {
        // Trigger a first keygen request: `keyId`
        await kmsGeneration.connect(owner).keygen(paramsType);

        await expect(kmsGeneration.connect(owner).keygen(paramsType))
          .to.be.revertedWithCustomError(kmsGeneration, "KeygenOngoing")
          .withArgs(keyId);
      });
    });

    describe("After key generation", function () {
      let keyId: bigint;

      beforeEach(async function () {
        const fixtureData = await loadFixture(loadTestVariablesFixture);
        kmsGeneration = fixtureData.kmsGeneration;
        owner = fixtureData.owner;
        kmsTxSenders = fixtureData.kmsTxSenders;
        kmsSigners = fixtureData.kmsSigners;
        kmsNodeStorageUrls = fixtureData.kmsNodeStorageUrls;

        // Generate a key.
        keyId = await generateKey(kmsGeneration, owner, gatewayChainId, kmsTxSenders, kmsSigners, keyDigests);
      });

      it("Should get params type associated to the key", async function () {
        // Check that the params type associated to the key is correct.
        expect(await kmsGeneration.getKeyParamsType(keyId)).to.equal(ParamsTypeEnum.Test);
      });

      it("Should get materials associated to the key", async function () {
        // Check that the materials associated to the key are correct.
        expect(await kmsGeneration.getKeyMaterials(keyId)).to.deep.equal([kmsNodeStorageUrls, toValues(keyDigests)]);
      });

      it("Should get the current active key", async function () {
        // Check that the current active key is correct.
        expect(await kmsGeneration.getActiveKeyId()).to.equal(keyId);
      });

      it("Should get the list of KMS transaction senders associated to the key", async function () {
        // Check that the KMS transaction senders associated to the key are correct.
        const kmsTxSenderAddresses = kmsTxSenders.map((s) => s.address);
        expect(await kmsGeneration.getConsensusTxSenders(keyId)).to.deep.equal(kmsTxSenderAddresses);
      });
    });
  });

  describe("CRS generation", async function () {
    // Define input values.
    const maxBitLength = 256;

    // Define the CRS digest.
    const crsDigest = createByteInput();

    // Define fake values
    const fakeCrsId = getCrsId(1);

    describe("Before CRS generation", function () {
      beforeEach(async function () {
        const fixtureData = await loadFixture(loadTestVariablesFixture);
        kmsGeneration = fixtureData.kmsGeneration;
      });

      it("Should revert because of access controls", async function () {
        // Check that only the owner can trigger a CRS generation request.
        await expect(kmsGeneration.connect(fakeOwner).crsgenRequest(maxBitLength, ParamsTypeEnum.Test))
          .to.be.revertedWithCustomError(kmsGeneration, "NotGatewayOwner")
          .withArgs(fakeOwner.address);

        // Check that only the KMS transaction sender can send a CRS generation response.
        await expect(kmsGeneration.connect(fakeOwner).crsgenResponse(0n, "0x", "0x"))
          .to.be.revertedWithCustomError(kmsGeneration, "NotKmsTxSender")
          .withArgs(fakeOwner.address);
      });

      it("Should revert on get params type because the CRS is not generated", async function () {
        // Check that getting the params type of a non-existing CRS reverts
        await expect(kmsGeneration.getCrsParamsType(fakeCrsId))
          .to.be.revertedWithCustomError(kmsGeneration, "CrsNotGenerated")
          .withArgs(fakeCrsId);
      });

      it("Should revert on get materials because the CRS is not generated", async function () {
        const { kmsGeneration } = await loadFixture(loadTestVariablesFixture);

        const fakeCrsId = getCrsId(5);

        // Check that getting the materials of a non-existing CRS reverts.
        await expect(kmsGeneration.getCrsMaterials(fakeCrsId))
          .to.be.revertedWithCustomError(kmsGeneration, "CrsNotGenerated")
          .withArgs(fakeCrsId);
      });
    });

    describe("During CRS generation", function () {
      // Define the expected crsId.
      const crsId = getCrsId(1);

      let kmsGenerationAddress: string;
      let eip712MessageCrsgen: EIP712;
      let kmsSignaturesCrsgen: string[];

      beforeEach(async function () {
        const fixtureData = await loadFixture(loadTestVariablesFixture);
        kmsGeneration = fixtureData.kmsGeneration;
        owner = fixtureData.owner;
        kmsTxSenders = fixtureData.kmsTxSenders;
        kmsSigners = fixtureData.kmsSigners;
        kmsNodeStorageUrls = fixtureData.kmsNodeStorageUrls;

        // Get the KMSGeneration contract address.
        kmsGenerationAddress = await kmsGeneration.getAddress();

        // Create the EIP712 message for the CRS generation response.
        eip712MessageCrsgen = createEIP712ResponseCrsgen(
          hre.network.config.chainId!,
          kmsGenerationAddress,
          crsId,
          maxBitLength,
          crsDigest,
        );

        // Sign the crsgen EIP712 message with all KMS signers.
        kmsSignaturesCrsgen = await getSignaturesCrsgen(eip712MessageCrsgen, kmsSigners);
      });

      it("Should handle a CRS generation", async function () {
        // Trigger a CRS generation request.
        const txRequest = await kmsGeneration.connect(owner).crsgenRequest(maxBitLength, ParamsTypeEnum.Test);

        // Check for the CrsgenRequest event.
        await expect(txRequest)
          .to.emit(kmsGeneration, "CrsgenRequest")
          .withArgs(crsId, maxBitLength, ParamsTypeEnum.Test);

        const txResponse1 = await kmsGeneration
          .connect(kmsTxSenders[0])
          .crsgenResponse(crsId, crsDigest, kmsSignaturesCrsgen[0]);

        // Check that the first response does not emit an event (consensus is not reached yet).
        await expect(txResponse1).to.not.emit(kmsGeneration, "ActivateCrs");

        // Check that a KMS node cannot respond twice to the same CRS generation request.
        await expect(kmsGeneration.connect(kmsTxSenders[0]).crsgenResponse(crsId, crsDigest, kmsSignaturesCrsgen[0]))
          .to.be.revertedWithCustomError(kmsGeneration, "KmsAlreadySignedForCrsgen")
          .withArgs(crsId, kmsSigners[0]);

        // Trigger a second CRS generation response with the first KMS node.
        await kmsGeneration.connect(kmsTxSenders[1]).crsgenResponse(crsId, crsDigest, kmsSignaturesCrsgen[1]);

        // Trigger a third CRS generation response which should reach consensus (4 / 2 + 1 = 3) and thus emit an event.
        const txResponse3 = await kmsGeneration
          .connect(kmsTxSenders[2])
          .crsgenResponse(crsId, crsDigest, kmsSignaturesCrsgen[2]);

        // Check for the ActivateCrs event.
        await expect(txResponse3)
          .to.emit(kmsGeneration, "ActivateCrs")
          .withArgs(crsId, kmsNodeStorageUrls.slice(0, 3), crsDigest);

        // The 4th response should be ignored (not reverted) and not emit the ActivateCrs event.
        const txResponse4 = await kmsGeneration
          .connect(kmsTxSenders[3])
          .crsgenResponse(crsId, crsDigest, kmsSignaturesCrsgen[3]);

        // Check that the 4th response does not emit the ActivateCrs event.
        await expect(txResponse4).to.not.emit(kmsGeneration, "ActivateCrs");
      });

      it("Should emit an event when calling a single crsgenResponse", async function () {
        // Trigger a CRS generation request.
        // This is needed to generate and store the necessary values in the KMSGeneration contract
        // fetched in the crsgen response.
        await kmsGeneration.connect(owner).crsgenRequest(maxBitLength, ParamsTypeEnum.Test);

        await expect(kmsGeneration.connect(kmsTxSenders[0]).crsgenResponse(crsId, crsDigest, kmsSignaturesCrsgen[0]))
          .to.emit(kmsGeneration, "CrsgenResponse")
          .withArgs(crsId, crsDigest, kmsSignaturesCrsgen[0], kmsTxSenders[0].address);
      });

      it("Should revert because the signer and the tx sender do not correspond to the same coprocessor during crsgen", async function () {
        // Trigger a CRS generation request.
        // This is needed to generate and store the necessary values in the KMSGeneration contract
        // fetched in the crsgen response.
        await kmsGeneration.connect(owner).crsgenRequest(maxBitLength, ParamsTypeEnum.Test);

        // Check that triggering a crsgen response using a signature from the first KMS signer
        // with the second KMS transaction sender reverts
        await expect(kmsGeneration.connect(kmsTxSenders[1]).crsgenResponse(crsId, crsDigest, kmsSignaturesCrsgen[0]))
          .to.be.revertedWithCustomError(kmsGeneration, "KmsSignerDoesNotMatchTxSender")
          .withArgs(kmsSigners[0].address, kmsTxSenders[1].address);
      });

      it("Should revert because the CRS generation request is not requested yet", async function () {
        // Trigger a keygen request.
        // Check that triggering a CRS generation response using a non-existing crsId reverts
        await expect(kmsGeneration.connect(kmsTxSenders[0]).crsgenResponse(fakeCrsId, crsDigest, "0x"))
          .to.be.revertedWithCustomError(kmsGeneration, "CrsgenNotRequested")
          .withArgs(fakeCrsId);
      });

      it("Should revert because the CRS generation request is ongoing", async function () {
        // Trigger a first CRS generation request: `crsId`
        await kmsGeneration.connect(owner).crsgenRequest(maxBitLength, ParamsTypeEnum.Test);

        await expect(kmsGeneration.connect(owner).crsgenRequest(maxBitLength, ParamsTypeEnum.Test))
          .to.be.revertedWithCustomError(kmsGeneration, "CrsgenOngoing")
          .withArgs(crsId);
      });
    });

    describe("After CRS generation", function () {
      let crsId: bigint;

      beforeEach(async function () {
        const fixtureData = await loadFixture(loadTestVariablesFixture);
        kmsGeneration = fixtureData.kmsGeneration;
        owner = fixtureData.owner;
        kmsTxSenders = fixtureData.kmsTxSenders;
        kmsSigners = fixtureData.kmsSigners;
        kmsNodeStorageUrls = fixtureData.kmsNodeStorageUrls;

        // Generate a CRS.
        crsId = await generateCrs(
          kmsGeneration,
          owner,
          gatewayChainId,
          kmsTxSenders,
          kmsSigners,
          maxBitLength,
          crsDigest,
        );
      });

      it("Should get params type associated to the CRS", async function () {
        // Check that the params type associated to the CRS is correct.
        expect(await kmsGeneration.getCrsParamsType(crsId)).to.equal(ParamsTypeEnum.Test);
      });

      it("Should get materials associated to the CRS", async function () {
        // Check that the materials associated to the CRS are correct.
        expect(await kmsGeneration.getCrsMaterials(crsId)).to.deep.equal([kmsNodeStorageUrls, crsDigest]);
      });

      it("Should get the current active CRS", async function () {
        // Check that the current active CRS is correct.
        expect(await kmsGeneration.getActiveCrsId()).to.equal(crsId);
      });

      it("Should get the list of KMS transaction senders associated to the CRS", async function () {
        // Check that the KMS transaction senders associated to the CRS are correct.
        const kmsTxSenderAddresses = kmsTxSenders.map((s) => s.address);
        expect(await kmsGeneration.getConsensusTxSenders(crsId)).to.deep.equal(kmsTxSenderAddresses);
      });
    });
  });

  describe("Key resharing", function () {
    it("Should revert because of access controls", async function () {
      const { kmsGeneration } = await loadFixture(loadTestVariablesFixture);

      const keyId = getKeyId(1);

      // Check that only the owner can trigger a PRSS initialization.
      await expect(kmsGeneration.connect(fakeOwner).prssInit())
        .to.be.revertedWithCustomError(kmsGeneration, "NotGatewayOwner")
        .withArgs(fakeOwner.address);

      // Check that only the owner can trigger a key resharing.
      await expect(kmsGeneration.connect(fakeOwner).keyReshareSameSet(keyId))
        .to.be.revertedWithCustomError(kmsGeneration, "NotGatewayOwner")
        .withArgs(fakeOwner.address);
    });

    it("Should trigger the PRSS initialization", async function () {
      const { owner, kmsGeneration } = await loadFixture(loadTestVariablesFixture);

      await expect(kmsGeneration.connect(owner).prssInit()).to.emit(kmsGeneration, "PRSSInit");
    });

    it("Should trigger key resharing for the given key ID", async function () {
      const { owner, kmsGeneration } = await loadFixture(loadTestVariablesFixture);

      // Define the key digests.
      const serverKeyDigest: IKMSGeneration.KeyDigestStruct = {
        keyType: KeyTypeEnum.Server,
        digest: createByteInput(),
      };
      const publicKeyDigest: IKMSGeneration.KeyDigestStruct = {
        keyType: KeyTypeEnum.Public,
        digest: createByteInput(),
      };
      const keyDigests = [serverKeyDigest, publicKeyDigest];

      // Generate a key to reshare.
      const keyId = await generateKey(kmsGeneration, owner, gatewayChainId, kmsTxSenders, kmsSigners, keyDigests);

      // Declare expected values.
      const prepKeygenId = getPrepKeygenId(1);
      const keyReshareId = getKeyReshareId(1);
      const paramsType = ParamsTypeEnum.Test;

      await expect(kmsGeneration.connect(owner).keyReshareSameSet(keyId))
        .to.emit(kmsGeneration, "KeyReshareSameSet")
        .withArgs(prepKeygenId, keyId, keyReshareId, paramsType);
    });

    it("Should revert on reshare key because the key is not generated", async function () {
      const { owner, kmsGeneration } = await loadFixture(loadTestVariablesFixture);

      const fakeKeyId = getKeyId(5);

      await expect(kmsGeneration.connect(owner).keyReshareSameSet(fakeKeyId))
        .to.be.revertedWithCustomError(kmsGeneration, "KeyNotGenerated")
        .withArgs(fakeKeyId);
    });
  });
});
