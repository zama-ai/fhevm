import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { BigNumberish, EventLog, Wallet } from "ethers";
import hre from "hardhat";

import { IDecryption, KmsManagement } from "../../typechain-types";
// The type needs to be imported separately because it is not properly detected by the linter
// as this type is defined as a shared structs instead of directly in the IDecryption interface
import {
  CtHandleContractPairStruct,
  SnsCiphertextMaterialStruct,
} from "../../typechain-types/contracts/interfaces/IDecryption";
import {
  createBytes32,
  createBytes32s,
  createCtHandle,
  createEIP712RequestUserDecrypt,
  createEIP712ResponsePublicDecrypt,
  createEIP712ResponseUserDecrypt,
  createRandomAddress,
  createRandomWallet,
  getSignaturesPublicDecrypt,
  getSignaturesUserDecryptRequest,
  getSignaturesUserDecryptResponse,
  loadHostChainIds,
  loadTestVariablesFixture,
} from "../utils";

// Get the current date in seconds. This is needed because Solidity works with seconds, not milliseconds
// See https://docs.soliditylang.org/en/develop/units-and-global-variables.html#time-units
function getDateInSeconds(): number {
  return Math.floor(Date.now() / 1000);
}

describe("Decryption gas report", function () {
  // Get one of the registered host chain IDs
  const hostChainId = loadHostChainIds()[0];

  // Get the gateway's chain ID
  const gatewayChainId = hre.network.config.chainId!;

  // Expected decryption request ID (after a first request) for each kind of decryption request
  // The IDs won't increase between requests made in different "describe" sections as the blockchain
  // state is cleaned each time `loadFixture` is called
  const decryptionId = 1;

  // Define input values
  const ciphertextDigest = createBytes32();
  const snsCiphertextDigest = createBytes32();

  // Define the ebytes128 ctHandle (which has a bit size of 1024 bits)
  const ebytes128CtHandle = createCtHandle(hostChainId, 10);
  // Create ciphertext handles for the host chain ID with different TFHE-rs types
  // Note that the list is made so that the total bit size represented by these handles (1024 bits)
  // does not exceed 2048 bits (the maximum allowed for a single list of handles)
  const ctHandles = [createCtHandle(hostChainId, 0), createCtHandle(hostChainId, 2), ebytes128CtHandle];

  // Create input values
  const decryptedResult = createBytes32();

  // Create valid input values
  const user = createRandomWallet();
  const contractAddress = createRandomAddress();
  const contractAddresses = [contractAddress];
  const publicKey = createBytes32();
  const startTimestamp = getDateInSeconds();
  const durationDays = 120;
  const requestValidity: IDecryption.RequestValidityStruct = {
    startTimestamp,
    durationDays,
  };

  // Trigger a key generation in KmsManagement contract and activate the key
  async function prepareAddCiphertextFixture() {
    const fixtureData = await loadFixture(loadTestVariablesFixture);
    const { kmsManagement, ciphertextCommits, owner, kmsTxSenders, coprocessorTxSenders, fheParamsName } = fixtureData;

    // Trigger a preprocessing keygen request
    const txRequest = await kmsManagement.connect(owner).preprocessKeygenRequest(fheParamsName);

    // Get the preKeyRequestId from the event in the transaction receipt
    const receipt = await txRequest.wait();
    const event = receipt?.logs[0] as EventLog;
    const preKeyRequestId = Number(event?.args[0]);

    // Define a preKeyId for the preprocessing keygen response
    const preKeyId = 1;

    // Trigger preprocessing keygen responses for all KMS nodes
    for (let i = 0; i < kmsTxSenders.length; i++) {
      await kmsManagement.connect(kmsTxSenders[i]).preprocessKeygenResponse(preKeyRequestId, preKeyId);
    }

    // Trigger a keygen request
    await kmsManagement.connect(owner).keygenRequest(preKeyId);

    // Define a keyId for keygen response
    const keyId1 = 1;

    // Trigger keygen responses for all KMS nodes
    for (let i = 0; i < kmsTxSenders.length; i++) {
      await kmsManagement.connect(kmsTxSenders[i]).keygenResponse(preKeyId, keyId1);
    }

    // Request activation of the key
    await kmsManagement.connect(owner).activateKeyRequest(keyId1);

    // Trigger activation responses for all coprocessors
    for (let i = 0; i < coprocessorTxSenders.length; i++) {
      await kmsManagement.connect(coprocessorTxSenders[i]).activateKeyResponse(keyId1);
    }

    let snsCiphertextMaterials: SnsCiphertextMaterialStruct[] = [];

    // Allow public decryption
    for (const ctHandle of ctHandles) {
      for (let i = 0; i < coprocessorTxSenders.length; i++) {
        await ciphertextCommits
          .connect(coprocessorTxSenders[i])
          .addCiphertextMaterial(ctHandle, keyId1, ciphertextDigest, snsCiphertextDigest);
      }

      // Store the SNS ciphertext materials for event checks
      snsCiphertextMaterials.push({
        ctHandle,
        keyId: keyId1,
        snsCiphertextDigest,
        coprocessorTxSenderAddresses: coprocessorTxSenders.map((s) => s.address),
      });
    }

    return { ...fixtureData, snsCiphertextMaterials, keyId1 };
  }

  // Allow handles for public decryption
  async function preparePublicDecryptEIP712Fixture() {
    const fixtureData = await loadFixture(prepareAddCiphertextFixture);
    const { multichainAcl, decryption, kmsSigners, coprocessorTxSenders } = fixtureData;

    // Allow public decryption
    for (const ctHandle of ctHandles) {
      for (let i = 0; i < coprocessorTxSenders.length; i++) {
        await multichainAcl.connect(coprocessorTxSenders[i]).allowPublicDecrypt(ctHandle);
      }
    }

    // Create EIP712 messages
    const decryptionAddress = await decryption.getAddress();
    const eip712Message = createEIP712ResponsePublicDecrypt(
      gatewayChainId,
      decryptionAddress,
      ctHandles,
      decryptedResult,
    );

    // Sign the message with all KMS signers
    const kmsSignatures = await getSignaturesPublicDecrypt(eip712Message, kmsSigners);

    return { ...fixtureData, eip712Message, kmsSignatures };
  }

  // Allow access the the handles for the user and the contract
  async function prepareUserDecryptEIP712Fixture() {
    const fixtureData = await loadFixture(prepareAddCiphertextFixture);
    const { decryption, multichainAcl, kmsSigners, coprocessorTxSenders } = fixtureData;

    // Allow user decryption for the user and contract address over all handles
    for (const ctHandle of ctHandles) {
      for (let i = 0; i < coprocessorTxSenders.length; i++) {
        await multichainAcl.connect(coprocessorTxSenders[i]).allowAccount(ctHandle, user.address);
        await multichainAcl.connect(coprocessorTxSenders[i]).allowAccount(ctHandle, contractAddress);
      }
    }

    // Create EIP712 messages
    const decryptionAddress = await decryption.getAddress();
    const eip712RequestMessage = createEIP712RequestUserDecrypt(
      decryptionAddress,
      publicKey,
      contractAddresses,
      hostChainId,
      requestValidity.startTimestamp.toString(),
      requestValidity.durationDays.toString(),
    );

    // Sign the message with the user
    const [userSignature] = await getSignaturesUserDecryptRequest(eip712RequestMessage, [user]);

    const userDecryptedShares = createBytes32s(kmsSigners.length);

    const eip712ResponseMessages = userDecryptedShares.map((userDecryptedShare) =>
      createEIP712ResponseUserDecrypt(gatewayChainId, decryptionAddress, publicKey, ctHandles, userDecryptedShare),
    );

    // Sign the message with all KMS signers
    const kmsSignatures = await getSignaturesUserDecryptResponse(eip712ResponseMessages, kmsSigners);

    return {
      ...fixtureData,
      userDecryptedShares,
      eip712RequestMessage,
      eip712ResponseMessages,
      userSignature,
      kmsSignatures,
      requestValidity,
    };
  }

  it("Public Decryption Response gas usage", async function () {
    const { decryption, nKmsNodes, kmsSignatures, kmsTxSenders, publicDecryptionThreshold } = await loadFixture(
      preparePublicDecryptEIP712Fixture,
    );

    // Request public decryption
    await decryption.publicDecryptionRequest(ctHandles);

    const noConsensusGasUsed: bigint[] = [];
    let consensusGasUsed: bigint = 0n;

    for (let i = 0; i < nKmsNodes; i++) {
      const txResponse = await decryption
        .connect(kmsTxSenders[i])
        .publicDecryptionResponse(decryptionId, decryptedResult, kmsSignatures[i]);

      const txReceipt = await txResponse.wait(1);
      if (!txReceipt) {
        throw new Error("Unable to estimate gas usage because transaction receipt is null.");
      }

      const consensusReached = i + 1 === publicDecryptionThreshold;
      if (consensusReached) {
        await expect(txResponse).to.emit(decryption, "PublicDecryptionResponse");
        consensusGasUsed += txReceipt.gasUsed;
        continue;
      }

      noConsensusGasUsed.push(txReceipt.gasUsed);
    }

    console.log(
      "Gas consumed in PublicDecryptionResponse transactions with no consensus:",
      "\x1b[1m" + noConsensusGasUsed.join(", ") + "\x1b[0m",
    );
    console.log(
      "Gas consumed in PublicDecryptionResponse transaction with consensus:",
      "\x1b[1m" + consensusGasUsed.toString() + "\x1b[0m",
    );

    // Check that the public decryption is done
    await expect(decryption.checkDecryptionDone(decryptionId)).to.not.be.reverted;
  });

  it("User Decryption Response gas usage", async function () {
    const {
      decryption,
      userSignature,
      kmsTxSenders,
      kmsSignatures,
      nKmsNodes,
      userDecryptedShares,
      userDecryptionThreshold,
    } = await loadFixture(prepareUserDecryptEIP712Fixture);

    // Define the ctHandleContractPairs (the handles have been added and allowed by default)
    const ctHandleContractPairs: CtHandleContractPairStruct[] = ctHandles.map((ctHandle) => ({
      contractAddress,
      ctHandle,
    }));

    // Request user decryption
    await decryption.userDecryptionRequest(
      ctHandleContractPairs,
      requestValidity,
      hostChainId,
      contractAddresses,
      user.address,
      publicKey,
      userSignature,
    );

    const noConsensusGasUsed: bigint[] = [];
    let consensusGasUsed: bigint = 0n;

    for (let i = 0; i < nKmsNodes; i++) {
      const txResponse = await decryption
        .connect(kmsTxSenders[i])
        .userDecryptionResponse(decryptionId, userDecryptedShares[i], kmsSignatures[i]);

      const txReceipt = await txResponse.wait(1);
      if (!txReceipt) {
        throw new Error("Unable to estimate gas usage because transaction receipt is null.");
      }

      const consensusReached = i + 1 === userDecryptionThreshold;
      if (consensusReached) {
        await expect(txResponse).to.emit(decryption, "UserDecryptionResponse");
        consensusGasUsed += txReceipt.gasUsed;
        continue;
      }

      noConsensusGasUsed.push(txReceipt.gasUsed);
    }

    console.log(
      "Gas consumed in UserDecryptionResponse transactions with no consensus:",
      "\x1b[1m" + noConsensusGasUsed.join(", ") + "\x1b[0m",
    );
    console.log(
      "Gas consumed in UserDecryptionResponse transaction with consensus:",
      "\x1b[1m" + consensusGasUsed.toString() + "\x1b[0m",
    );

    // Check that the user decryption is done
    await expect(decryption.checkDecryptionDone(decryptionId)).to.not.be.reverted;
  });
});
