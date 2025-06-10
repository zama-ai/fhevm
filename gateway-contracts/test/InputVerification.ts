import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture, mine } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { Wallet } from "ethers";
import hre from "hardhat";

import { CoprocessorContexts, GatewayConfig, InputVerification, InputVerification__factory } from "../typechain-types";
import { CoprocessorContextBlockPeriodsStruct } from "../typechain-types/contracts/CoprocessorContexts";
import {
  ContextStatus,
  EIP712,
  addNewCoprocessorContext,
  createBytes32,
  createCtHandles,
  createEIP712ResponseZKPoK,
  createRandomAddress,
  createRandomWallet,
  getSignaturesZKPoK,
  loadTestVariablesFixture,
  refreshCoprocessorContextAfterBlockPeriod,
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

  // Define the first context ID
  const contextId = 1;

  // Define 3 new valid ctHandles
  const newCtHandles = createCtHandles(3);

  // Define fake values
  const fakeHostChainId = 123;
  const fakeTxSender = createRandomWallet();
  const fakeSigner = createRandomWallet();
  const fakeZkProofId = 2;

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
    let coprocessorContexts: CoprocessorContexts;
    let contractChainId: number;
    let owner: Wallet;

    beforeEach(async function () {
      const fixture = await loadFixture(loadTestVariablesFixture);
      gatewayConfig = fixture.gatewayConfig;
      coprocessorContexts = fixture.coprocessorContexts;
      inputVerification = fixture.inputVerification;
      contractChainId = fixture.chainIds[0];
      owner = fixture.owner;
    });

    it("Should request a proof verification", async function () {
      // Trigger a proof verification request
      const txResponse = inputVerification.verifyProofRequest(
        contractChainId,
        contractAddress,
        userAddress,
        ciphertextWithZKProof,
      );

      await expect(txResponse)
        .to.emit(inputVerification, "VerifyProofRequest")
        .withArgs(zkProofId, contextId, contractChainId, contractAddress, userAddress, ciphertextWithZKProof);
    });

    it("Should revert because the contract's chain ID does not correspond to a registered host chain", async function () {
      await expect(
        inputVerification.verifyProofRequest(fakeHostChainId, contractAddress, userAddress, ciphertextWithZKProof),
      )
        .revertedWithCustomError(gatewayConfig, "HostChainNotRegistered")
        .withArgs(fakeHostChainId);
    });

    it("Should revert because the contract is paused", async function () {
      // Pause the contract
      await inputVerification.connect(owner).pause();

      // Try calling paused verify proof request
      await expect(
        inputVerification
          .connect(owner)
          .verifyProofRequest(contractChainId, contractAddress, userAddress, ciphertextWithZKProof),
      ).to.be.revertedWithCustomError(gatewayConfig, "EnforcedPause");
    });

    describe("Context changes", async function () {
      let blockPeriods: CoprocessorContextBlockPeriodsStruct;

      // Define the new expected context ID
      const newContextId = 2;

      beforeEach(async function () {
        // Add a new coprocessor context
        const newCoprocessorContext = await addNewCoprocessorContext(3, coprocessorContexts, owner);
        blockPeriods = newCoprocessorContext.blockPeriods;
      });

      it("Should activate the new context and suspend the old one", async function () {
        // Mine the number of blocks required for the pre-activation period to pass
        await mine(blockPeriods.preActivationBlockPeriod);

        // Trigger a proof verification request to refresh the statuses of the coprocessor contexts
        await inputVerification.verifyProofRequest(
          contractChainId,
          contractAddress,
          userAddress,
          ciphertextWithZKProof,
        );

        // Make sure the old context has been suspended
        expect(await coprocessorContexts.getCoprocessorContextStatus(contextId)).to.equal(ContextStatus.Suspended);

        // Make sure the new context has been activated
        expect(await coprocessorContexts.getCoprocessorContextStatus(newContextId)).to.equal(ContextStatus.Active);
      });

      it("Should deactivate the suspended context", async function () {
        // Mine the number of blocks required for the pre-activation period to pass
        await mine(blockPeriods.preActivationBlockPeriod);

        // Trigger a proof verification request to refresh the status of the coprocessor context
        await inputVerification.verifyProofRequest(
          contractChainId,
          contractAddress,
          userAddress,
          ciphertextWithZKProof,
        );

        // Then mine the number of blocks required for the suspended period to pass
        await mine(blockPeriods.suspendedBlockPeriod);

        // Trigger an additional proof verification request to refresh the status of the coprocessor context
        await inputVerification.verifyProofRequest(
          contractChainId,
          contractAddress,
          userAddress,
          ciphertextWithZKProof,
        );

        // Make sure the old context has been deactivated
        expect(await coprocessorContexts.getCoprocessorContextStatus(contextId)).to.equal(ContextStatus.Deactivated);
      });
    });
  });

  describe("Proof verification response", async function () {
    let gatewayConfig: GatewayConfig;
    let coprocessorContexts: CoprocessorContexts;
    let inputVerification: InputVerification;
    let coprocessorTxSenders: HardhatEthersSigner[];
    let coprocessorSigners: HardhatEthersSigner[];
    let contractChainId: number;
    let inputVerificationAddress: string;
    let eip712Message: EIP712;
    let signatures: string[];
    let owner: Wallet;

    beforeEach(async function () {
      const fixture = await loadFixture(loadTestVariablesFixture);
      gatewayConfig = fixture.gatewayConfig;
      coprocessorContexts = fixture.coprocessorContexts;
      inputVerification = fixture.inputVerification;
      coprocessorTxSenders = fixture.coprocessorTxSenders;
      coprocessorSigners = fixture.coprocessorSigners;
      contractChainId = fixture.chainIds[0];
      owner = fixture.owner;

      inputVerificationAddress = await inputVerification.getAddress();

      // Create the EIP712 message
      eip712Message = createEIP712ResponseZKPoK(
        hre.network.config.chainId!,
        inputVerificationAddress,
        ctHandles,
        userAddress,
        contractAddress,
        contractChainId,
      );

      // Get the EIP712 signatures
      signatures = await getSignaturesZKPoK(eip712Message, coprocessorSigners);

      // The ZK proof ID will always be 1 since we reset the state of the network before each test (using fixtures)
      await inputVerification.verifyProofRequest(contractChainId, contractAddress, userAddress, ciphertextWithZKProof);
    });

    it("Should verify proof with 2 valid responses", async function () {
      // Trigger two valid proof verification responses
      await inputVerification.connect(coprocessorTxSenders[0]).verifyProofResponse(zkProofId, ctHandles, signatures[0]);
      let txResponse = await inputVerification
        .connect(coprocessorTxSenders[1])
        .verifyProofResponse(zkProofId, ctHandles, signatures[1]);

      // Consensus should be reached at the second response
      // Check 2nd response event: it should only contain 2 valid signatures
      await expect(txResponse)
        .to.emit(inputVerification, "VerifyProofResponse")
        .withArgs(zkProofId, ctHandles, signatures.slice(0, 2));
    });

    it("Should ignore other valid responses", async function () {
      // Trigger three valid proof verification responses
      let txResponse1 = await inputVerification
        .connect(coprocessorTxSenders[0])
        .verifyProofResponse(zkProofId, ctHandles, signatures[0]);
      await inputVerification.connect(coprocessorTxSenders[1]).verifyProofResponse(zkProofId, ctHandles, signatures[1]);
      let txResponse3 = inputVerification
        .connect(coprocessorTxSenders[2])
        .verifyProofResponse(zkProofId, ctHandles, signatures[2]);

      // Check that the 1st and 3rd responses do not emit an event:
      // - 1st response is ignored because consensus is not reached yet
      // - 3rd response is ignored (not reverted) even though it is late
      await expect(txResponse1).to.not.emit(inputVerification, "VerifyProofResponse");
      await expect(txResponse3).to.not.emit(inputVerification, "VerifyProofResponse");
    });

    it("Should verify a proof with 2 valid responses and 1 valid proof rejection response", async function () {
      // Trigger a valid proof rejection with the first coprocessor transaction sender
      await inputVerification.connect(coprocessorTxSenders[0]).rejectProofResponse(zkProofId);

      // Trigger a first valid proof verification response with:
      // - the second coprocessor transaction sender
      // - the second coprocessor signer's signature
      let txResponse2 = inputVerification
        .connect(coprocessorTxSenders[1])
        .verifyProofResponse(zkProofId, ctHandles, signatures[1]);

      // Consensus should not be reached at the second response since the first response is a proof rejection
      // Check 2nd response event: it should not emit an event (either for proof verification or rejection)
      await expect(txResponse2)
        .to.not.emit(inputVerification, "VerifyProofResponse")
        .to.not.emit(inputVerification, "RejectProofResponse");

      // Trigger a second valid proof verification response with:
      // - the third coprocessor transaction sender
      // - the third coprocessor signer's signature
      let txResponse3 = inputVerification
        .connect(coprocessorTxSenders[2])
        .verifyProofResponse(zkProofId, ctHandles, signatures[2]);

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
      );

      // Get the EIP712 signatures
      const [fakeSignature] = await getSignaturesZKPoK(fakeEip712Message, coprocessorSigners);

      // Trigger a malicious proof verification response with:
      // - the first coprocessor transaction sender (expected)
      // - a fake signature (unexpected)
      await inputVerification
        .connect(coprocessorTxSenders[0])
        .verifyProofResponse(zkProofId, newCtHandles, fakeSignature);

      // Trigger a first valid proof verification response with:
      // - the second coprocessor transaction sender
      // - the second coprocessor signer's signature
      let txResponse2 = inputVerification
        .connect(coprocessorTxSenders[1])
        .verifyProofResponse(zkProofId, ctHandles, signatures[1]);

      // Consensus should not be reached at the second response since the first response is malicious
      // Check 2nd response event: it should not emit an event for proof verification
      await expect(txResponse2).to.not.emit(inputVerification, "VerifyProofResponse");

      // Trigger a second valid proof verification response with:
      // - the third coprocessor transaction sender
      // - the third coprocessor signer's signature
      let txResponse3 = inputVerification
        .connect(coprocessorTxSenders[2])
        .verifyProofResponse(zkProofId, ctHandles, signatures[2]);

      // Consensus should be reached at the third response
      // Check 3rd response event: it should only contain 2 valid signatures
      await expect(txResponse3)
        .to.emit(inputVerification, "VerifyProofResponse")
        .withArgs(zkProofId, ctHandles, signatures.slice(1, 3));
    });

    it("Should revert because of two responses with same signature for proof verification", async function () {
      // Trigger a first proof response with :
      // - the first coprocessor transaction sender
      // - the first coprocessor signer's signature
      await inputVerification.connect(coprocessorTxSenders[0]).verifyProofResponse(zkProofId, ctHandles, signatures[0]);

      // Check that a coprocessor signer cannot sign a second time for the same proof
      await expect(
        inputVerification.connect(coprocessorTxSenders[0]).verifyProofResponse(zkProofId, ctHandles, signatures[0]),
      )
        .revertedWithCustomError(inputVerification, "CoprocessorAlreadyVerified")
        .withArgs(zkProofId, coprocessorTxSenders[0].address, coprocessorSigners[0].address);
    });

    it("Should revert because same coprocessor first verifies then rejects a proof", async function () {
      // Trigger a proof verification response with:
      // - the first coprocessor transaction sender
      // - the first coprocessor signer's signature
      await inputVerification.connect(coprocessorTxSenders[0]).verifyProofResponse(zkProofId, ctHandles, signatures[0]);

      // Check that the coprocessor transaction sender representing the above coprocessor signer
      // cannot reject the same proof
      // The address in the error message is the coprocessor signer's address as we are checking
      // the coprocessor signer's address here, not the coprocessor transaction sender's address
      await expect(inputVerification.connect(coprocessorTxSenders[0]).rejectProofResponse(zkProofId))
        .revertedWithCustomError(inputVerification, "CoprocessorAlreadyVerified")
        .withArgs(zkProofId, coprocessorTxSenders[0].address, coprocessorSigners[0].address);
    });

    it("Should revert because the signer is not a coprocessor", async function () {
      // Create a fake signature from a non-coprocessor signer
      const [fakeSignature] = await getSignaturesZKPoK(eip712Message, [fakeSigner]);

      // Check that triggering a proof response using a signature from a non-coprocessor signer reverts
      await expect(
        inputVerification.connect(coprocessorTxSenders[0]).verifyProofResponse(zkProofId, ctHandles, fakeSignature),
      )
        .revertedWithCustomError(coprocessorContexts, "NotCoprocessorSignerFromContext")
        .withArgs(contextId, fakeSigner.address);
    });

    it("Should revert because the transaction sender is not a coprocessor", async function () {
      await expect(inputVerification.connect(fakeTxSender).verifyProofResponse(zkProofId, ctHandles, signatures[0]))
        .revertedWithCustomError(coprocessorContexts, "NotCoprocessorTxSenderFromContext")
        .withArgs(contextId, fakeTxSender.address);
    });

    it("Should check that a proof has been verified", async function () {
      // Trigger two valid proof verification responses
      for (let i = 0; i < coprocessorTxSenders.length; i++) {
        await inputVerification
          .connect(coprocessorTxSenders[i])
          .verifyProofResponse(zkProofId, ctHandles, signatures[i]);
      }

      await expect(inputVerification.checkProofVerified(zkProofId)).not.to.be.reverted;
    });

    it("Should check that a proof has not been verified", async function () {
      await expect(inputVerification.checkProofVerified(fakeZkProofId))
        .to.be.revertedWithCustomError(inputVerification, "ProofNotVerified")
        .withArgs(fakeZkProofId);
    });

    it("Should revert because the contract is paused", async function () {
      // Pause the contract
      await inputVerification.connect(owner).pause();

      // Try calling paused verify proof response
      await expect(
        inputVerification.connect(coprocessorTxSenders[0]).verifyProofResponse(zkProofId, ctHandles, signatures[0]),
      ).to.be.revertedWithCustomError(gatewayConfig, "EnforcedPause");
    });

    describe("Context changes", async function () {
      let blockPeriods: CoprocessorContextBlockPeriodsStruct;

      // Define the new expected context ID
      const newContextId = 2;

      beforeEach(async function () {
        // Add a new coprocessor context
        const newCoprocessorContext = await addNewCoprocessorContext(3, coprocessorContexts, owner);
        blockPeriods = newCoprocessorContext.blockPeriods;
      });

      it("Should activate the new context and suspend the old one", async function () {
        // Mine the number of blocks required for the pre-activation period to pass
        await mine(blockPeriods.preActivationBlockPeriod);

        // Trigger a proof verification response to refresh the statuses of the coprocessor contexts
        await inputVerification
          .connect(coprocessorTxSenders[0])
          .verifyProofResponse(zkProofId, ctHandles, signatures[0]);

        // Make sure the old context has been suspended
        expect(await coprocessorContexts.getCoprocessorContextStatus(contextId)).to.equal(ContextStatus.Suspended);

        // Make sure the new context has been activated
        expect(await coprocessorContexts.getCoprocessorContextStatus(newContextId)).to.equal(ContextStatus.Active);
      });

      it("Should deactivate the suspended context", async function () {
        // Mine the number of blocks required for the pre-activation period to pass
        await mine(blockPeriods.preActivationBlockPeriod);

        // Trigger a proof verification response to refresh the status of the coprocessor context
        await inputVerification
          .connect(coprocessorTxSenders[0])
          .verifyProofResponse(zkProofId, ctHandles, signatures[0]);

        // Then mine the number of blocks required for the suspended period to pass and refresh the status
        // Here we cannot call the `verifyProofResponse` anymore because the context will be deactivated
        // at the beginning of it, making it invalid and thus reverting the transaction. Another option
        // would be to call another request, or a response based on a different zkProofId.
        await refreshCoprocessorContextAfterBlockPeriod(blockPeriods.suspendedBlockPeriod, coprocessorContexts);

        // Make sure the old context has been deactivated
        expect(await coprocessorContexts.getCoprocessorContextStatus(contextId)).to.equal(ContextStatus.Deactivated);
      });

      it("Should verify proof with suspended context", async function () {
        // Trigger a first valid proof verification response
        await inputVerification
          .connect(coprocessorTxSenders[0])
          .verifyProofResponse(zkProofId, ctHandles, signatures[0]);

        // Wait for the pre activation period to pass: this suspends the old context
        await refreshCoprocessorContextAfterBlockPeriod(blockPeriods.preActivationBlockPeriod, coprocessorContexts);

        // Trigger a second valid proof verification response
        let txResponse = await inputVerification
          .connect(coprocessorTxSenders[1])
          .verifyProofResponse(zkProofId, ctHandles, signatures[1]);

        // Consensus should be reached at the second response
        // This is because the consensus is reached amongst the suspended context (3 coprocessors)
        // and not the new one (1 coprocessor)
        await expect(txResponse)
          .to.emit(inputVerification, "VerifyProofResponse")
          .withArgs(zkProofId, ctHandles, signatures.slice(0, 2));
      });

      it("Should revert because the context is no longer valid", async function () {
        // Wait for the pre activation period to pass: this suspends the old context
        await refreshCoprocessorContextAfterBlockPeriod(blockPeriods.preActivationBlockPeriod, coprocessorContexts);

        // Wait for the suspended period to pass: this deactivates the old context
        await refreshCoprocessorContextAfterBlockPeriod(blockPeriods.suspendedBlockPeriod, coprocessorContexts);

        // Check that allow verifying a proof associated to a request that have been registered under
        // an active context reverts because this context is no longer valid
        await expect(
          inputVerification.connect(coprocessorTxSenders[0]).verifyProofResponse(zkProofId, ctHandles, signatures[0]),
        )
          .revertedWithCustomError(inputVerification, "InvalidCoprocessorContextProofVerification")
          .withArgs(zkProofId, contextId, ContextStatus.Deactivated);
      });
    });
  });

  describe("Proof rejection response", async function () {
    let gatewayConfig: GatewayConfig;
    let coprocessorContexts: CoprocessorContexts;
    let inputVerification: InputVerification;
    let coprocessorTxSenders: HardhatEthersSigner[];
    let coprocessorSigners: HardhatEthersSigner[];
    let contractChainId: number;
    let inputVerificationAddress: string;
    let owner: Wallet;

    beforeEach(async function () {
      const fixture = await loadFixture(loadTestVariablesFixture);
      gatewayConfig = fixture.gatewayConfig;
      coprocessorContexts = fixture.coprocessorContexts;
      inputVerification = fixture.inputVerification;
      coprocessorTxSenders = fixture.coprocessorTxSenders;
      coprocessorSigners = fixture.coprocessorSigners;
      contractChainId = fixture.chainIds[0];
      owner = fixture.owner;

      inputVerificationAddress = await inputVerification.getAddress();

      // The ZK proof ID will always be 1 since we reset the state of the network before each test (using fixtures)
      await inputVerification.verifyProofRequest(contractChainId, contractAddress, userAddress, ciphertextWithZKProof);
    });

    it("Should reject a proof with 2 valid responses", async function () {
      // Trigger two valid proof rejection responses using different coprocessor transaction senders
      await inputVerification.connect(coprocessorTxSenders[0]).rejectProofResponse(zkProofId);
      let txResponse = inputVerification.connect(coprocessorTxSenders[1]).rejectProofResponse(zkProofId);

      // Consensus should be reached at the second response
      await expect(txResponse).to.emit(inputVerification, "RejectProofResponse").withArgs(zkProofId);
    });

    it("Should ignore other valid responses", async function () {
      // Trigger three valid proof rejection responses using different coprocessor transaction senders
      let txResponse1 = await inputVerification.connect(coprocessorTxSenders[0]).rejectProofResponse(zkProofId);
      await inputVerification.connect(coprocessorTxSenders[1]).rejectProofResponse(zkProofId);
      let txResponse3 = inputVerification.connect(coprocessorTxSenders[2]).rejectProofResponse(zkProofId);

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
      );

      // Get the EIP712 signatures
      const [signature1] = await getSignaturesZKPoK(eip712Message, coprocessorSigners);

      // Trigger a valid proof verification response with:
      // - the first coprocessor transaction sender
      // - the first coprocessor signer's signature
      await inputVerification.connect(coprocessorTxSenders[0]).verifyProofResponse(zkProofId, ctHandles, signature1);

      // Trigger a valid proof rejection response with the second coprocessor transaction sender
      // representing the second coprocessor signer
      let txResponse2 = inputVerification.connect(coprocessorTxSenders[1]).rejectProofResponse(zkProofId);

      // Consensus should not be reached at the second response since the first response is a proof verification
      // Check 2nd response event: it should not emit an event (either for proof verification or rejection)
      await expect(txResponse2)
        .to.not.emit(inputVerification, "RejectProofResponse")
        .to.not.emit(inputVerification, "VerifyProofResponse");

      // Trigger a second valid proof rejection response with the third coprocessor transaction sender
      // representing the third coprocessor signer
      let txResponse3 = inputVerification.connect(coprocessorTxSenders[2]).rejectProofResponse(zkProofId);

      // Consensus should be reached at the third response
      await expect(txResponse3).to.emit(inputVerification, "RejectProofResponse").withArgs(zkProofId);
    });

    it("Should revert because of two rejections from the same coprocessor", async function () {
      const coprocessorTxSender = coprocessorTxSenders[0];
      const coprocessorSigner = coprocessorSigners[0];

      // Trigger a first proof response
      await inputVerification.connect(coprocessorTxSender).rejectProofResponse(zkProofId);

      // Check that a coprocessor transaction sender cannot send a second response for the same proof
      await expect(inputVerification.connect(coprocessorTxSender).rejectProofResponse(zkProofId))
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
      );

      // Get the EIP712 signatures
      const [signature1] = await getSignaturesZKPoK(eip712Message, coprocessorSigners);

      // Trigger a first proof response
      await inputVerification.connect(coprocessorTxSender).rejectProofResponse(zkProofId);

      // Check that a Coprocessor transaction sender cannot send a second response for the same proof
      await expect(inputVerification.connect(coprocessorTxSender).verifyProofResponse(zkProofId, ctHandles, signature1))
        .revertedWithCustomError(inputVerification, "CoprocessorAlreadyRejected")
        .withArgs(zkProofId, coprocessorTxSender.address, coprocessorSigner.address);
    });

    it("Should revert because the sender is not a coprocessor transaction sender", async function () {
      // Check that triggering a proof response with a non-coprocessor transaction sender reverts
      await expect(inputVerification.connect(fakeTxSender).rejectProofResponse(zkProofId))
        .revertedWithCustomError(coprocessorContexts, "NotCoprocessorTxSenderFromContext")
        .withArgs(contextId, fakeTxSender.address);
    });

    it("Should check that a proof has been rejected", async function () {
      // Trigger two valid proof verification responses
      for (let i = 0; i < coprocessorTxSenders.length; i++) {
        await inputVerification.connect(coprocessorTxSenders[i]).rejectProofResponse(zkProofId);
      }

      await expect(inputVerification.checkProofRejected(zkProofId)).to.not.be.reverted;
    });

    it("Should check that a proof has not been rejected", async function () {
      await expect(inputVerification.checkProofRejected(fakeZkProofId))
        .to.be.revertedWithCustomError(inputVerification, "ProofNotRejected")
        .withArgs(fakeZkProofId);
    });

    it("Should revert because the contract is paused", async function () {
      // Pause the contract
      await inputVerification.connect(owner).pause();

      // Try calling paused reject proof response
      await expect(
        inputVerification.connect(coprocessorTxSenders[0]).rejectProofResponse(zkProofId),
      ).to.be.revertedWithCustomError(gatewayConfig, "EnforcedPause");
    });

    describe("Context refresh", async function () {
      let blockPeriods: CoprocessorContextBlockPeriodsStruct;

      // Define the new expected context ID
      const newContextId = 2;

      beforeEach(async function () {
        // Add a new coprocessor context
        const newCoprocessorContext = await addNewCoprocessorContext(3, coprocessorContexts, owner);
        blockPeriods = newCoprocessorContext.blockPeriods;
      });

      it("Should activate the new context and suspend the old one", async function () {
        // Mine the number of blocks required for the pre-activation period to pass
        await mine(blockPeriods.preActivationBlockPeriod);

        // Trigger a proof rejection response to refresh the statuses of the coprocessor contexts
        await inputVerification.connect(coprocessorTxSenders[0]).rejectProofResponse(zkProofId);

        // Make sure the old context has been suspended
        expect(await coprocessorContexts.getCoprocessorContextStatus(contextId)).to.equal(ContextStatus.Suspended);

        // Make sure the new context has been activated
        expect(await coprocessorContexts.getCoprocessorContextStatus(newContextId)).to.equal(ContextStatus.Active);
      });

      it("Should deactivate the suspended context", async function () {
        // Mine the number of blocks required for the pre-activation period to pass
        await mine(blockPeriods.preActivationBlockPeriod);

        // Trigger a proof rejection response to refresh the status of the coprocessor context
        await inputVerification.connect(coprocessorTxSenders[0]).rejectProofResponse(zkProofId);

        // Then mine the number of blocks required for the suspended period to pass and refresh the status
        // Here we cannot call the `verifyProofResponse` anymore because the context will be deactivated
        // at the beginning of it, making it invalid and thus reverting the transaction. Another option
        // would be to call another request, or a response based on a different zkProofId.
        await refreshCoprocessorContextAfterBlockPeriod(blockPeriods.suspendedBlockPeriod, coprocessorContexts);

        // Make sure the old context has been deactivated
        expect(await coprocessorContexts.getCoprocessorContextStatus(contextId)).to.equal(ContextStatus.Deactivated);
      });

      it("Should reject proof with suspended context", async function () {
        // Trigger a first valid proof rejection response
        await inputVerification.connect(coprocessorTxSenders[0]).rejectProofResponse(zkProofId);

        // Wait for the pre activation period to pass: this suspends the old context
        await refreshCoprocessorContextAfterBlockPeriod(blockPeriods.preActivationBlockPeriod, coprocessorContexts);

        // Trigger a second valid proof rejection response
        let txResponse = await inputVerification.connect(coprocessorTxSenders[1]).rejectProofResponse(zkProofId);

        // Consensus should be reached at the second response
        // This is because the consensus is reached amongst the suspended context (3 coprocessors)
        // and not the new one (1 coprocessor)
        await expect(txResponse).to.emit(inputVerification, "RejectProofResponse").withArgs(zkProofId);
      });

      it("Should revert because the context is no longer valid", async function () {
        // Wait for the pre activation period to pass: this suspends the old context
        await refreshCoprocessorContextAfterBlockPeriod(blockPeriods.preActivationBlockPeriod, coprocessorContexts);

        // Wait for the suspended period to pass: this deactivates the old context
        await refreshCoprocessorContextAfterBlockPeriod(blockPeriods.suspendedBlockPeriod, coprocessorContexts);

        // Check that allow rejecting a proof associated to a request that have been registered under
        // an active context reverts because this context is no longer valid
        await expect(inputVerification.connect(coprocessorTxSenders[0]).rejectProofResponse(zkProofId))
          .revertedWithCustomError(inputVerification, "InvalidCoprocessorContextProofRejection")
          .withArgs(zkProofId, contextId, ContextStatus.Deactivated);
      });
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

    it("Should pause and unpause contract with owner address", async function () {
      // Check that the contract is not paused
      expect(await inputVerification.paused()).to.be.false;

      // Pause the contract with the owner address
      await expect(inputVerification.connect(owner).pause()).to.emit(inputVerification, "Paused").withArgs(owner);
      expect(await inputVerification.paused()).to.be.true;

      // Unpause the contract with the owner address
      await expect(inputVerification.connect(owner).unpause()).to.emit(inputVerification, "Unpaused").withArgs(owner);
      expect(await inputVerification.paused()).to.be.false;
    });

    it("Should pause contract with pauser address", async function () {
      // Check that the contract is not paused
      expect(await inputVerification.paused()).to.be.false;

      // Pause the contract with the pauser address
      await expect(inputVerification.connect(pauser).pause()).to.emit(inputVerification, "Paused").withArgs(pauser);
      expect(await inputVerification.paused()).to.be.true;
    });

    it("Should revert on pause because sender is not owner or pauser address", async function () {
      const notOwnerOrPauser = createRandomWallet();
      await expect(inputVerification.connect(notOwnerOrPauser).pause())
        .to.be.revertedWithCustomError(inputVerification, "NotOwnerOrPauser")
        .withArgs(notOwnerOrPauser.address);
    });
  });
});
