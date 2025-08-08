import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { Wallet } from "ethers";
import hre from "hardhat";

import { GatewayConfig, InputVerification, InputVerification__factory } from "../typechain-types";
import {
  EIP712,
  createBytes32,
  createCtHandles,
  createEIP712ResponseZKPoK,
  createRandomAddress,
  createRandomWallet,
  getSignaturesZKPoK,
  loadTestVariablesFixture,
} from "./utils";

describe("InputVerification", function () {
  // Define 3 ciphertext handles
  const ctHandles = createCtHandles(3);

  // Define input values
  const contractAddress = createRandomAddress();
  const userAddress = createRandomAddress();
  const ciphertextWithZKProof = createBytes32();

  // Expected ZK proof id (after first request)
  const zkProofId = 1;

  // Define 3 new valid ctHandles
  const newCtHandles = createCtHandles(3);

  // Define fake values
  const fakeHostChainId = 123;
  const fakeTxSender = createRandomWallet();
  const fakeSigner = createRandomWallet();
  const fakeZkProofId = 2;
  const nullZkProofId = 0;
  const tooHighZkProofId = 100000;

  // Define extra data for version 0
  const extraDataV0 = hre.ethers.solidityPacked(["uint8"], [0]);

  describe("Deployment", function () {
    let inputVerificationFactory: InputVerification__factory;
    let inputVerification: InputVerification;
    let owner: Wallet;

    beforeEach(async function () {
      const fixtureData = await loadFixture(loadTestVariablesFixture);
      inputVerification = fixtureData.inputVerification;
      owner = fixtureData.owner;

      // Get the InputVerification contract factory
      inputVerificationFactory = await hre.ethers.getContractFactory("InputVerification", owner);
    });

    it("Should revert because initialization is not from an empty proxy", async function () {
      await expect(
        hre.upgrades.upgradeProxy(inputVerification, inputVerificationFactory, {
          call: { fn: "initializeFromEmptyProxy" },
        }),
      ).to.be.revertedWithCustomError(inputVerification, "NotInitializingFromEmptyProxy");
    });
  });

  describe("Verify proof request", async function () {
    let gatewayConfig: GatewayConfig;
    let inputVerification: InputVerification;
    let contractChainId: number;
    let owner: Wallet;
    let pauser: HardhatEthersSigner;

    before(async function () {
      const fixture = await loadFixture(loadTestVariablesFixture);
      gatewayConfig = fixture.gatewayConfig;
      inputVerification = fixture.inputVerification;
      contractChainId = fixture.chainIds[0];
      owner = fixture.owner;
      pauser = fixture.pauser;
    });

    it("Should request a proof verification", async function () {
      // Trigger a proof verification request
      const txResponse = inputVerification.verifyProofRequest(
        contractChainId,
        contractAddress,
        userAddress,
        ciphertextWithZKProof,
        extraDataV0,
      );

      await expect(txResponse)
        .to.emit(inputVerification, "VerifyProofRequest")
        .withArgs(zkProofId, contractChainId, contractAddress, userAddress, ciphertextWithZKProof, extraDataV0);
    });

    it("Should revert because the contract's chain ID does not correspond to a registered host chain", async function () {
      await expect(
        inputVerification.verifyProofRequest(
          fakeHostChainId,
          contractAddress,
          userAddress,
          ciphertextWithZKProof,
          extraDataV0,
        ),
      )
        .revertedWithCustomError(gatewayConfig, "HostChainNotRegistered")
        .withArgs(fakeHostChainId);
    });

    it("Should revert because the contract is paused", async function () {
      // Pause the contract
      await inputVerification.connect(pauser).pause();

      // Try calling paused verify proof request
      await expect(
        inputVerification
          .connect(owner)
          .verifyProofRequest(contractChainId, contractAddress, userAddress, ciphertextWithZKProof, extraDataV0),
      ).to.be.revertedWithCustomError(gatewayConfig, "EnforcedPause");
    });
  });

  describe("Proof verification response", async function () {
    let gatewayConfig: GatewayConfig;
    let inputVerification: InputVerification;
    let coprocessorTxSenders: HardhatEthersSigner[];
    let coprocessorSigners: HardhatEthersSigner[];
    let contractChainId: number;
    let inputVerificationAddress: string;
    let eip712Message: EIP712;
    let signatures: string[];
    let pauser: HardhatEthersSigner;

    beforeEach(async function () {
      const fixture = await loadFixture(loadTestVariablesFixture);
      gatewayConfig = fixture.gatewayConfig;
      inputVerification = fixture.inputVerification;
      coprocessorTxSenders = fixture.coprocessorTxSenders;
      coprocessorSigners = fixture.coprocessorSigners;
      contractChainId = fixture.chainIds[0];
      pauser = fixture.pauser;

      inputVerificationAddress = await inputVerification.getAddress();

      // Create the EIP712 message
      eip712Message = createEIP712ResponseZKPoK(
        hre.network.config.chainId!,
        inputVerificationAddress,
        ctHandles,
        userAddress,
        contractAddress,
        contractChainId,
        extraDataV0,
      );

      // Get the EIP712 signatures
      signatures = await getSignaturesZKPoK(eip712Message, coprocessorSigners);

      // The ZK proof ID will always be 1 since we reset the state of the network before each test (using fixtures)
      await inputVerification.verifyProofRequest(
        contractChainId,
        contractAddress,
        userAddress,
        ciphertextWithZKProof,
        extraDataV0,
      );
    });

    it("Should verify proof with 2 valid responses", async function () {
      // Trigger two valid proof verification responses
      await inputVerification
        .connect(coprocessorTxSenders[0])
        .verifyProofResponse(zkProofId, ctHandles, signatures[0], extraDataV0);
      const txResponse = await inputVerification
        .connect(coprocessorTxSenders[1])
        .verifyProofResponse(zkProofId, ctHandles, signatures[1], extraDataV0);

      // Consensus should be reached at the second response
      // Check 2nd response event: it should only contain 2 valid signatures
      await expect(txResponse)
        .to.emit(inputVerification, "VerifyProofResponse")
        .withArgs(zkProofId, ctHandles, signatures.slice(0, 2));
    });

    it("Should verify proof with 2 valid responses and ignore the other valid one", async function () {
      // Trigger three valid proof verification responses
      const txResponse1 = await inputVerification
        .connect(coprocessorTxSenders[0])
        .verifyProofResponse(zkProofId, ctHandles, signatures[0], extraDataV0);
      await inputVerification
        .connect(coprocessorTxSenders[1])
        .verifyProofResponse(zkProofId, ctHandles, signatures[1], extraDataV0);
      const txResponse3 = inputVerification
        .connect(coprocessorTxSenders[2])
        .verifyProofResponse(zkProofId, ctHandles, signatures[2], extraDataV0);

      // Check that the 1st and 3rd responses do not emit an event:
      // - 1st response is ignored because consensus is not reached yet
      // - 3rd response is ignored (not reverted) even though it is late
      await expect(txResponse1).to.not.emit(inputVerification, "VerifyProofResponse");
      await expect(txResponse3).to.not.emit(inputVerification, "VerifyProofResponse");
    });

    it("Should verify a proof with 2 valid responses and 1 valid proof rejection response", async function () {
      // Trigger a valid proof rejection with the first coprocessor transaction sender
      await inputVerification.connect(coprocessorTxSenders[0]).rejectProofResponse(zkProofId, extraDataV0);

      // Trigger a first valid proof verification response with:
      // - the second coprocessor transaction sender
      // - the second coprocessor signer's signature
      const txResponse2 = inputVerification
        .connect(coprocessorTxSenders[1])
        .verifyProofResponse(zkProofId, ctHandles, signatures[1], extraDataV0);

      // Consensus should not be reached at the second response since the first response is a proof rejection
      // Check 2nd response event: it should not emit an event (either for proof verification or rejection)
      await expect(txResponse2)
        .to.not.emit(inputVerification, "VerifyProofResponse")
        .to.not.emit(inputVerification, "RejectProofResponse");

      // Trigger a second valid proof verification response with:
      // - the third coprocessor transaction sender
      // - the third coprocessor signer's signature
      const txResponse3 = inputVerification
        .connect(coprocessorTxSenders[2])
        .verifyProofResponse(zkProofId, ctHandles, signatures[2], extraDataV0);

      // Consensus should be reached at the third response
      // Check 3rd response event: it should only contain 2 valid signatures
      await expect(txResponse3)
        .to.emit(inputVerification, "VerifyProofResponse")
        .withArgs(zkProofId, ctHandles, signatures.slice(1, 3));
    });

    it("Should verify a proof with 2 valid and 1 malicious signatures", async function () {
      // Create a malicious EIP712 message: the ctHandles are different from the expected ones
      // but the signature is valid (the new handles will be given to the response call )
      const fakeEip712Message = createEIP712ResponseZKPoK(
        hre.network.config.chainId!,
        inputVerificationAddress,
        newCtHandles,
        userAddress,
        contractAddress,
        contractChainId,
        extraDataV0,
      );

      // Get the EIP712 signatures
      const [fakeSignature] = await getSignaturesZKPoK(fakeEip712Message, coprocessorSigners.slice(0, 1));

      // Trigger a malicious proof verification response with:
      // - the first coprocessor transaction sender (expected)
      // - a fake signature (unexpected)
      await inputVerification
        .connect(coprocessorTxSenders[0])
        .verifyProofResponse(zkProofId, newCtHandles, fakeSignature, extraDataV0);

      // Trigger a first valid proof verification response with:
      // - the second coprocessor transaction sender
      // - the second coprocessor signer's signature
      const txResponse2 = inputVerification
        .connect(coprocessorTxSenders[1])
        .verifyProofResponse(zkProofId, ctHandles, signatures[1], extraDataV0);

      // Consensus should not be reached at the second response since the first response is malicious
      // Check 2nd response event: it should not emit an event for proof verification
      await expect(txResponse2).to.not.emit(inputVerification, "VerifyProofResponse");

      // Trigger a second valid proof verification response with:
      // - the third coprocessor transaction sender
      // - the third coprocessor signer's signature
      const txResponse3 = inputVerification
        .connect(coprocessorTxSenders[2])
        .verifyProofResponse(zkProofId, ctHandles, signatures[2], extraDataV0);

      // Consensus should be reached at the third response
      // Check 3rd response event: it should only contain 2 valid signatures
      await expect(txResponse3)
        .to.emit(inputVerification, "VerifyProofResponse")
        .withArgs(zkProofId, ctHandles, signatures.slice(1, 3));
    });

    it("Should get all valid coprocessor transaction senders from proof verification consensus", async function () {
      // Trigger a valid proof verification response with the first coprocessor transaction sender
      await inputVerification
        .connect(coprocessorTxSenders[0])
        .verifyProofResponse(zkProofId, ctHandles, signatures[0], extraDataV0);

      // Check that the coprocessor transaction senders list is empty because consensus is not reached yet
      const proofVerificationConsensusTxSenders1 = await inputVerification.getVerifyProofConsensusTxSenders(zkProofId);
      expect(proofVerificationConsensusTxSenders1).to.deep.equal([]);

      // Trigger a second valid proof verification response with the second coprocessor transaction sender
      await inputVerification
        .connect(coprocessorTxSenders[1])
        .verifyProofResponse(zkProofId, ctHandles, signatures[1], extraDataV0);

      const expectedCoprocessorTxSenders2 = coprocessorTxSenders.slice(0, 2).map((s) => s.address);

      // Check that the coprocessor transaction senders that were involved in the consensus are the
      // 2 coprocessor transaction senders, at the moment the consensus is reached
      const proofVerificationConsensusTxSenders2 = await inputVerification.getVerifyProofConsensusTxSenders(zkProofId);
      expect(proofVerificationConsensusTxSenders2).to.deep.equal(expectedCoprocessorTxSenders2);

      // Trigger a third valid proof verification response with the third coprocessor transaction senders
      await inputVerification
        .connect(coprocessorTxSenders[2])
        .verifyProofResponse(zkProofId, ctHandles, signatures[2], extraDataV0);

      const expectedCoprocessorTxSenders3 = coprocessorTxSenders.map((s) => s.address);

      // Check that the coprocessor transaction senders that were involved in the consensus are the
      // 3 coprocessor transaction senders, after the consensus is reached
      const proofVerificationConsensusTxSenders3 = await inputVerification.getVerifyProofConsensusTxSenders(zkProofId);
      expect(proofVerificationConsensusTxSenders3).to.deep.equal(expectedCoprocessorTxSenders3);
    });

    it("Should get all valid coprocessor transaction senders from proof verification consensus and ignore malicious ones", async function () {
      // Trigger 2 valid proof verification responses
      await inputVerification
        .connect(coprocessorTxSenders[0])
        .verifyProofResponse(zkProofId, ctHandles, signatures[0], extraDataV0);
      await inputVerification
        .connect(coprocessorTxSenders[1])
        .verifyProofResponse(zkProofId, ctHandles, signatures[1], extraDataV0);

      // Create a malicious EIP712 message: the ctHandles are different from the expected ones
      // but the signature is valid (the new handles will be given to the response call)
      const fakeEip712Message = createEIP712ResponseZKPoK(
        hre.network.config.chainId!,
        inputVerificationAddress,
        newCtHandles,
        userAddress,
        contractAddress,
        contractChainId,
        extraDataV0,
      );

      // Get the EIP712 signatures
      const [fakeSignature] = await getSignaturesZKPoK(fakeEip712Message, coprocessorSigners.slice(2, 3));

      // Trigger a third invalid proof verification response
      await inputVerification
        .connect(coprocessorTxSenders[2])
        .verifyProofResponse(zkProofId, newCtHandles, fakeSignature, extraDataV0);

      const expectedCoprocessorTxSenders = coprocessorTxSenders.slice(0, 2).map((s) => s.address);

      // Check that the coprocessor transaction senders that were involved in the consensus are the
      // first 2 coprocessor transaction senders (the third one is ignored because the response is invalid)
      const proofVerificationConsensusTxSenders = await inputVerification.getVerifyProofConsensusTxSenders(zkProofId);
      expect(proofVerificationConsensusTxSenders).to.deep.equal(expectedCoprocessorTxSenders);
    });

    it("Should get all valid coprocessor transaction senders from proof verification consensus and ignore the one from proof rejection", async function () {
      // Trigger 2 valid proof verification responses
      await inputVerification
        .connect(coprocessorTxSenders[0])
        .verifyProofResponse(zkProofId, ctHandles, signatures[0], extraDataV0);
      await inputVerification
        .connect(coprocessorTxSenders[1])
        .verifyProofResponse(zkProofId, ctHandles, signatures[1], extraDataV0);

      // Trigger a valid proof rejection with the third coprocessor transaction sender
      await inputVerification.connect(coprocessorTxSenders[2]).rejectProofResponse(zkProofId, extraDataV0);

      const expectedCoprocessorTxSenders = coprocessorTxSenders.slice(0, 2).map((s) => s.address);

      // Check that the coprocessor transaction senders that were involved in the consensus for proof verification
      // are the first 2 coprocessor transaction senders (the third one is ignored because it rejected the proof)
      const proofVerificationConsensusTxSenders = await inputVerification.getVerifyProofConsensusTxSenders(zkProofId);
      expect(proofVerificationConsensusTxSenders).to.deep.equal(expectedCoprocessorTxSenders);
    });

    it("Should revert in case of invalid zkProofId in verify proof response", async function () {
      // Check that a verify proof response with null (invalid) zkProofId reverts
      await expect(
        inputVerification
          .connect(coprocessorTxSenders[0])
          .verifyProofResponse(nullZkProofId, ctHandles, signatures[0], extraDataV0),
      ).to.be.revertedWithCustomError(inputVerification, "VerifyProofNotRequested");

      // Check that a verify proof response with too high (not requested yet) zkProofId reverts
      await expect(
        inputVerification
          .connect(coprocessorTxSenders[0])
          .verifyProofResponse(tooHighZkProofId, ctHandles, signatures[0], extraDataV0),
      ).to.be.revertedWithCustomError(inputVerification, "VerifyProofNotRequested");
    });

    it("Should revert because of two responses with same signature for proof verification", async function () {
      // Trigger a first proof response with :
      // - the first coprocessor transaction sender
      // - the first coprocessor signer's signature
      await inputVerification
        .connect(coprocessorTxSenders[0])
        .verifyProofResponse(zkProofId, ctHandles, signatures[0], extraDataV0);

      // Check that a coprocessor signer cannot sign a second time for the same proof
      await expect(
        inputVerification
          .connect(coprocessorTxSenders[0])
          .verifyProofResponse(zkProofId, ctHandles, signatures[0], extraDataV0),
      )
        .revertedWithCustomError(inputVerification, "CoprocessorAlreadyVerified")
        .withArgs(zkProofId, coprocessorTxSenders[0].address, coprocessorSigners[0].address);
    });

    it("Should revert because same coprocessor first verifies then rejects a proof", async function () {
      // Trigger a proof verification response with:
      // - the first coprocessor transaction sender
      // - the first coprocessor signer's signature
      await inputVerification
        .connect(coprocessorTxSenders[0])
        .verifyProofResponse(zkProofId, ctHandles, signatures[0], extraDataV0);

      // Check that the coprocessor transaction sender representing the above coprocessor signer
      // cannot reject the same proof
      // The address in the error message is the coprocessor signer's address as we are checking
      // the coprocessor signer's address here, not the coprocessor transaction sender's address
      await expect(inputVerification.connect(coprocessorTxSenders[0]).rejectProofResponse(zkProofId, extraDataV0))
        .revertedWithCustomError(inputVerification, "CoprocessorAlreadyVerified")
        .withArgs(zkProofId, coprocessorTxSenders[0].address, coprocessorSigners[0].address);
    });

    it("Should revert because the signer is not a coprocessor", async function () {
      // Create a fake signature from a non-coprocessor signer
      const [fakeSignature] = await getSignaturesZKPoK(eip712Message, [fakeSigner]);

      // Check that triggering a proof response using a signature from a non-coprocessor signer reverts
      await expect(
        inputVerification
          .connect(coprocessorTxSenders[0])
          .verifyProofResponse(zkProofId, ctHandles, fakeSignature, extraDataV0),
      )
        .revertedWithCustomError(gatewayConfig, "NotCoprocessorSigner")
        .withArgs(fakeSigner.address);
    });

    it("Should revert because the transaction sender is not a coprocessor", async function () {
      await expect(
        inputVerification.connect(fakeTxSender).verifyProofResponse(zkProofId, ctHandles, signatures[0], extraDataV0),
      )
        .revertedWithCustomError(gatewayConfig, "NotCoprocessorTxSender")
        .withArgs(fakeTxSender.address);
    });

    it("Should check that a proof has been verified", async function () {
      // Trigger two valid proof verification responses
      for (let i = 0; i < coprocessorTxSenders.length; i++) {
        await inputVerification
          .connect(coprocessorTxSenders[i])
          .verifyProofResponse(zkProofId, ctHandles, signatures[i], extraDataV0);
      }

      await expect(inputVerification.checkProofVerified(zkProofId)).not.to.be.reverted;
    });

    it("Should check that a proof has not been verified", async function () {
      await expect(inputVerification.checkProofVerified(fakeZkProofId))
        .to.be.revertedWithCustomError(inputVerification, "ProofNotVerified")
        .withArgs(fakeZkProofId);
    });
  });

  describe("Proof rejection response", async function () {
    let gatewayConfig: GatewayConfig;
    let inputVerification: InputVerification;
    let coprocessorTxSenders: HardhatEthersSigner[];
    let coprocessorSigners: HardhatEthersSigner[];
    let contractChainId: number;
    let inputVerificationAddress: string;
    let pauser: HardhatEthersSigner;

    beforeEach(async function () {
      const fixture = await loadFixture(loadTestVariablesFixture);
      gatewayConfig = fixture.gatewayConfig;
      inputVerification = fixture.inputVerification;
      coprocessorTxSenders = fixture.coprocessorTxSenders;
      coprocessorSigners = fixture.coprocessorSigners;
      contractChainId = fixture.chainIds[0];
      pauser = fixture.pauser;

      inputVerificationAddress = await inputVerification.getAddress();

      // The ZK proof ID will always be 1 since we reset the state of the network before each test (using fixtures)
      await inputVerification.verifyProofRequest(
        contractChainId,
        contractAddress,
        userAddress,
        ciphertextWithZKProof,
        extraDataV0,
      );
    });

    it("Should reject a proof with 2 valid responses", async function () {
      // Trigger two valid proof rejection responses using different coprocessor transaction senders
      await inputVerification.connect(coprocessorTxSenders[0]).rejectProofResponse(zkProofId, extraDataV0);
      const txResponse = inputVerification.connect(coprocessorTxSenders[1]).rejectProofResponse(zkProofId, extraDataV0);

      // Consensus should be reached at the second response
      await expect(txResponse).to.emit(inputVerification, "RejectProofResponse").withArgs(zkProofId);
    });

    it("Should reject a proof with 2 valid responses and ignore the other valid one", async function () {
      // Trigger three valid proof rejection responses using different coprocessor transaction senders
      const txResponse1 = await inputVerification
        .connect(coprocessorTxSenders[0])
        .rejectProofResponse(zkProofId, extraDataV0);
      await inputVerification.connect(coprocessorTxSenders[1]).rejectProofResponse(zkProofId, extraDataV0);
      const txResponse3 = inputVerification
        .connect(coprocessorTxSenders[2])
        .rejectProofResponse(zkProofId, extraDataV0);

      // Check that the 1st and 3rd responses do not emit an event:
      // - 1st response is ignored because consensus is not reached yet
      // - 3rd response is ignored (not reverted) even though it is late
      await expect(txResponse1).to.not.emit(inputVerification, "RejectProofResponse");
      await expect(txResponse3).to.not.emit(inputVerification, "RejectProofResponse");
    });

    it("Should reject a proof with 2 valid responses and 1 valid proof verification response", async function () {
      // Create the EIP712 message
      const eip712Message = createEIP712ResponseZKPoK(
        hre.network.config.chainId!,
        inputVerificationAddress,
        ctHandles,
        userAddress,
        contractAddress,
        contractChainId,
        extraDataV0,
      );

      // Get the EIP712 signature
      const [signature] = await getSignaturesZKPoK(eip712Message, coprocessorSigners.slice(0, 1));

      // Trigger a valid proof verification response with:
      // - the first coprocessor transaction sender
      // - the first coprocessor signer's signature
      await inputVerification
        .connect(coprocessorTxSenders[0])
        .verifyProofResponse(zkProofId, ctHandles, signature, extraDataV0);

      // Trigger a valid proof rejection response with the second coprocessor transaction sender
      // representing the second coprocessor signer
      const txResponse2 = inputVerification
        .connect(coprocessorTxSenders[1])
        .rejectProofResponse(zkProofId, extraDataV0);

      // Consensus should not be reached at the second response since the first response is a proof verification
      // Check 2nd response event: it should not emit an event (either for proof verification or rejection)
      await expect(txResponse2)
        .to.not.emit(inputVerification, "RejectProofResponse")
        .to.not.emit(inputVerification, "VerifyProofResponse");

      // Trigger a second valid proof rejection response with the third coprocessor transaction sender
      // representing the third coprocessor signer
      const txResponse3 = inputVerification
        .connect(coprocessorTxSenders[2])
        .rejectProofResponse(zkProofId, extraDataV0);

      // Consensus should be reached at the third response
      await expect(txResponse3).to.emit(inputVerification, "RejectProofResponse").withArgs(zkProofId);
    });

    it("Should get all valid coprocessor transaction senders from proof rejection consensus", async function () {
      // Trigger a valid proof rejection responses using the first coprocessor transaction sender
      await inputVerification.connect(coprocessorTxSenders[0]).rejectProofResponse(zkProofId, extraDataV0);

      const expectedCoprocessorTxSenders1 = coprocessorTxSenders.slice(0, 1).map((s) => s.address);

      // Get the coprocessor transaction sender that answered first, before the consensus is reached
      // Since consensus only depends on the proof ID, the list represents the coprocessor
      // transaction sender that answered, and is accessible before the consensus is reached
      const proofRejectionConsensusTxSenders1 = await inputVerification.getRejectProofConsensusTxSenders(zkProofId);
      expect(proofRejectionConsensusTxSenders1).to.deep.equal(expectedCoprocessorTxSenders1);

      // Trigger a second valid proof rejection response using the second coprocessor transaction sender
      await inputVerification.connect(coprocessorTxSenders[1]).rejectProofResponse(zkProofId, extraDataV0);

      const expectedCoprocessorTxSenders2 = coprocessorTxSenders.slice(0, 2).map((s) => s.address);

      // Check that the coprocessor transaction senders that were involved in the consensus are the
      // first 2 coprocessor transaction senders, at the moment the consensus is reached
      const proofRejectionConsensusTxSenders2 = await inputVerification.getRejectProofConsensusTxSenders(zkProofId);
      expect(proofRejectionConsensusTxSenders2).to.deep.equal(expectedCoprocessorTxSenders2);

      // Trigger a third valid proof rejection response using the third coprocessor transaction sender
      await inputVerification.connect(coprocessorTxSenders[2]).rejectProofResponse(zkProofId, extraDataV0);

      const expectedCoprocessorTxSenders3 = coprocessorTxSenders.map((s) => s.address);

      // Check that the coprocessor transaction senders that were involved in the consensus are the 3
      // coprocessor transaction senders, after the consensus is reached
      const proofRejectionConsensusTxSenders3 = await inputVerification.getRejectProofConsensusTxSenders(zkProofId);
      expect(proofRejectionConsensusTxSenders3).to.deep.equal(expectedCoprocessorTxSenders3);
    });

    it("Should get all valid coprocessor transaction senders from proof rejection consensus and ignore the one from proof verification", async function () {
      // Trigger 2 valid proof rejection responses using different coprocessor transaction senders
      await inputVerification.connect(coprocessorTxSenders[0]).rejectProofResponse(zkProofId, extraDataV0);
      await inputVerification.connect(coprocessorTxSenders[1]).rejectProofResponse(zkProofId, extraDataV0);

      // Create the EIP712 message
      const eip712Message = createEIP712ResponseZKPoK(
        hre.network.config.chainId!,
        inputVerificationAddress,
        ctHandles,
        userAddress,
        contractAddress,
        contractChainId,
        extraDataV0,
      );

      // Get the EIP712 signature
      const [signature] = await getSignaturesZKPoK(eip712Message, coprocessorSigners.slice(2, 3));

      // Trigger a valid proof verification response with the third coprocessor transaction sender
      await inputVerification
        .connect(coprocessorTxSenders[2])
        .verifyProofResponse(zkProofId, ctHandles, signature, extraDataV0);

      const expectedCoprocessorTxSenders = coprocessorTxSenders.slice(0, 2).map((s) => s.address);

      // Check that the coprocessor transaction senders that were involved in the consensus for proof rejection
      // are the first 2 coprocessor transaction senders (the third one is ignored because it verified the proof)
      const proofRejectionConsensusTxSenders = await inputVerification.getRejectProofConsensusTxSenders(zkProofId);
      expect(proofRejectionConsensusTxSenders).to.deep.equal(expectedCoprocessorTxSenders);
    });

    it("Should revert in case of invalid zkProofId in reject proof response", async function () {
      // Check that a reject proof response with null (invalid) zkProofId reverts
      await expect(
        inputVerification.connect(coprocessorTxSenders[0]).rejectProofResponse(nullZkProofId, extraDataV0),
      ).to.be.revertedWithCustomError(inputVerification, "VerifyProofNotRequested");

      // Check that a reject proof response with too high (not requested yet) zkProofId reverts
      await expect(
        inputVerification.connect(coprocessorTxSenders[0]).rejectProofResponse(tooHighZkProofId, extraDataV0),
      ).to.be.revertedWithCustomError(inputVerification, "VerifyProofNotRequested");
    });

    it("Should revert because of two rejections from the same coprocessor", async function () {
      const coprocessorTxSender = coprocessorTxSenders[0];
      const coprocessorSigner = coprocessorSigners[0];

      // Trigger a first proof response
      await inputVerification.connect(coprocessorTxSender).rejectProofResponse(zkProofId, extraDataV0);

      // Check that a coprocessor transaction sender cannot send a second response for the same proof
      await expect(inputVerification.connect(coprocessorTxSender).rejectProofResponse(zkProofId, extraDataV0))
        .revertedWithCustomError(inputVerification, "CoprocessorAlreadyRejected")
        .withArgs(zkProofId, coprocessorTxSender.address, coprocessorSigner.address);
    });

    it("Should revert because same coprocessor first rejects then verifies a proof", async function () {
      const coprocessorTxSender = coprocessorTxSenders[0];
      const coprocessorSigner = coprocessorSigners[0];

      // Create the EIP712 message
      const eip712Message = createEIP712ResponseZKPoK(
        hre.network.config.chainId!,
        inputVerificationAddress,
        ctHandles,
        userAddress,
        contractAddress,
        contractChainId,
        extraDataV0,
      );

      // Get the EIP712 signatures
      const [signature1] = await getSignaturesZKPoK(eip712Message, coprocessorSigners);

      // Trigger a first proof response
      await inputVerification.connect(coprocessorTxSender).rejectProofResponse(zkProofId, extraDataV0);

      // Check that a Coprocessor transaction sender cannot send a second response for the same proof
      await expect(
        inputVerification
          .connect(coprocessorTxSender)
          .verifyProofResponse(zkProofId, ctHandles, signature1, extraDataV0),
      )
        .revertedWithCustomError(inputVerification, "CoprocessorAlreadyRejected")
        .withArgs(zkProofId, coprocessorTxSender.address, coprocessorSigner.address);
    });

    it("Should revert because the sender is not a coprocessor transaction sender", async function () {
      // Check that triggering a proof response with a non-coprocessor transaction sender reverts
      await expect(inputVerification.connect(fakeTxSender).rejectProofResponse(zkProofId, extraDataV0))
        .revertedWithCustomError(gatewayConfig, "NotCoprocessorTxSender")
        .withArgs(fakeTxSender.address);
    });

    it("Should check that a proof has been rejected", async function () {
      // Trigger two valid proof verification responses
      for (let i = 0; i < coprocessorTxSenders.length; i++) {
        await inputVerification.connect(coprocessorTxSenders[i]).rejectProofResponse(zkProofId, extraDataV0);
      }

      await expect(inputVerification.checkProofRejected(zkProofId)).to.not.be.reverted;
    });

    it("Should check that a proof has not been rejected", async function () {
      await expect(inputVerification.checkProofRejected(fakeZkProofId))
        .to.be.revertedWithCustomError(inputVerification, "ProofNotRejected")
        .withArgs(fakeZkProofId);
    });
  });

  describe("Pause", async function () {
    let inputVerification: InputVerification;
    let owner: Wallet;
    let pauser: HardhatEthersSigner;

    beforeEach(async function () {
      const fixtureData = await loadFixture(loadTestVariablesFixture);
      inputVerification = fixtureData.inputVerification;
      owner = fixtureData.owner;
      pauser = fixtureData.pauser;
    });

    it("Should pause the contract with the pauser and unpause with the owner", async function () {
      // Check that the contract is not paused
      expect(await inputVerification.paused()).to.be.false;

      // Pause the contract with the pauser address
      await expect(inputVerification.connect(pauser).pause()).to.emit(inputVerification, "Paused").withArgs(pauser);
      expect(await inputVerification.paused()).to.be.true;

      // Unpause the contract with the owner address (not the pauser)
      await expect(inputVerification.connect(owner).unpause()).to.emit(inputVerification, "Unpaused").withArgs(owner);
      expect(await inputVerification.paused()).to.be.false;
    });

    it("Should revert on pause because sender is not the pauser", async function () {
      const fakePauser = createRandomWallet();

      await expect(inputVerification.connect(fakePauser).pause())
        .to.be.revertedWithCustomError(inputVerification, "NotPauserOrGatewayConfig")
        .withArgs(fakePauser.address);
    });

    it("Should revert on unpause because sender is not the owner", async function () {
      // Pause the contract with the pauser address
      await inputVerification.connect(pauser).pause();

      const fakeOwner = createRandomWallet();

      await expect(inputVerification.connect(fakeOwner).unpause())
        .to.be.revertedWithCustomError(inputVerification, "NotOwnerOrGatewayConfig")
        .withArgs(fakeOwner.address);
    });
  });
});
