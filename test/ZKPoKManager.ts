import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { expect } from "chai";
import hre from "hardhat";

import { HTTPZ, ZKPoKManager } from "../typechain-types";
import { EIP712, createEIP712ResponseZKPoK, deployZKPoKManagerFixture, getSignaturesZKPoK } from "./utils";

describe("ZKPoKManager", function () {
  const contractAddress = hre.ethers.getAddress("0x1234567890AbcdEF1234567890aBcdef12345678");
  const userAddress = hre.ethers.getAddress("0xabcdef1234567890abcdef1234567890abcdef12");
  const ciphertextWithZKProof = hre.ethers.randomBytes(32);
  const ctHandles = [hre.ethers.randomBytes(32), hre.ethers.randomBytes(32)];

  // Expected ZK proof id (after first request)
  const zkProofId = 1;
  const fakeCtHandles = [hre.ethers.randomBytes(32)];

  // Fake values
  const fakeChainId = 123;

  describe("Verify proof request", async function () {
    let httpz: HTTPZ;
    let zkpokManager: ZKPoKManager;
    let contractChainId: number;
    before(async function () {
      const fixture = await loadFixture(deployZKPoKManagerFixture);
      httpz = fixture.httpz;
      zkpokManager = fixture.zkpokManager;
      contractChainId = fixture.chainIds[0];
    });

    it("Should request a proof verification", async function () {
      // Trigger a proof verification request
      const txResponse = zkpokManager.verifyProofRequest(
        contractChainId,
        contractAddress,
        userAddress,
        ciphertextWithZKProof,
      );

      // Check that the event is emitted
      await expect(txResponse)
        .to.emit(zkpokManager, "VerifyProofRequest")
        .withArgs(zkProofId, contractChainId, contractAddress, userAddress, ciphertextWithZKProof);
    });

    it("Should revert with NetworkNotRegistered", async function () {
      // Check that sending a proof verification request with a fake chain id reverts
      await expect(zkpokManager.verifyProofRequest(fakeChainId, contractAddress, userAddress, ciphertextWithZKProof))
        .revertedWithCustomError(httpz, "NetworkNotRegistered")
        .withArgs(fakeChainId);
    });
  });

  describe("Proof verification response", async function () {
    let httpz: HTTPZ;
    let zkpokManager: ZKPoKManager;
    let coprocessorSigners: HardhatEthersSigner[];
    let fakeSigner: HardhatEthersSigner;
    let contractChainId: number;
    let zkpokManagerAddress: string;
    let eip712Message: EIP712;
    let signature1: string;
    let signature2: string;
    let signature3: string;

    beforeEach(async function () {
      const fixture = await loadFixture(deployZKPoKManagerFixture);
      httpz = fixture.httpz;
      zkpokManager = fixture.zkpokManager;
      coprocessorSigners = fixture.coprocessorSigners;
      fakeSigner = fixture.signers[0];
      contractChainId = fixture.chainIds[0];

      zkpokManagerAddress = await zkpokManager.getAddress();

      // Create the EIP712 message
      eip712Message = createEIP712ResponseZKPoK(
        hre.network.config.chainId!,
        zkpokManagerAddress,
        ctHandles,
        userAddress,
        contractAddress,
        contractChainId,
      );

      // Get the EIP712 signatures
      const signatures = await getSignaturesZKPoK(eip712Message, coprocessorSigners);
      signature1 = signatures[0];
      signature2 = signatures[1];
      signature3 = signatures[2];

      // The ZK proof ID will always be 1 since we deploy a new contract before each test
      await zkpokManager.verifyProofRequest(contractChainId, contractAddress, userAddress, ciphertextWithZKProof);
    });

    it("Should verify proof with 2 valid responses", async function () {
      // Trigger two valid proof verification responses
      await zkpokManager.verifyProofResponse(zkProofId, ctHandles, signature1);
      let txResponse = zkpokManager.verifyProofResponse(zkProofId, ctHandles, signature2);

      // Consensus should be reached at the second response
      // Check 2nd response event: it should only contain 2 valid signatures
      await expect(txResponse)
        .to.emit(zkpokManager, "VerifyProofResponse")
        .withArgs(zkProofId, ctHandles, [signature1, signature2]);

      // Check that the proof is verified
      expect(await zkpokManager.isProofVerified(zkProofId)).to.be.true;
    });

    it("Should ignore other valid responses", async function () {
      // Trigger three valid proof verification responses
      let txResponse1 = await zkpokManager.verifyProofResponse(zkProofId, ctHandles, signature1);
      await zkpokManager.verifyProofResponse(zkProofId, ctHandles, signature2);
      let txResponse3 = zkpokManager.verifyProofResponse(zkProofId, ctHandles, signature3);

      // Check that the 1st and 3rd responses do not emit an event:
      // - 1st response is ignored because consensus is not reached yet
      // - 3rd response is ignored (not reverted) even though it is late
      await expect(txResponse1).to.not.emit(zkpokManager, "VerifyProofResponse");
      await expect(txResponse3).to.not.emit(zkpokManager, "VerifyProofResponse");
    });

    it("Should verify a proof with 2 valid responses and 1 valid proof rejection response", async function () {
      // Trigger a valid proof rejection with the first coprocessor
      await zkpokManager.connect(coprocessorSigners[0]).rejectProofResponse(zkProofId);

      // Trigger two valid proof verification responses with the 2 other coprocessors' signatures
      let txResponse2 = zkpokManager.verifyProofResponse(zkProofId, ctHandles, signature2);
      let txResponse3 = zkpokManager.verifyProofResponse(zkProofId, ctHandles, signature3);

      // Consensus should not be reached at the second response since the first response is a proof rejection
      // Check 2nd response event: it should not emit an event (either for proof verification or rejection)
      await expect(txResponse2)
        .to.not.emit(zkpokManager, "VerifyProofResponse")
        .to.not.emit(zkpokManager, "RejectProofResponse");

      // Consensus should be reached at the third response
      // Check 3rd response event: it should only contain 2 valid signatures
      await expect(txResponse3)
        .to.emit(zkpokManager, "VerifyProofResponse")
        .withArgs(zkProofId, ctHandles, [signature2, signature3]);

      // Check that the proof is verified
      expect(await zkpokManager.isProofVerified(zkProofId)).to.be.true;
    });

    it("Should verify a proof with 2 valid responses and 1 malicious response", async function () {
      // Create a malicious EIP712 message: the ctHandles are different from the expected ones
      // but the signature is valid (the fake handles will be given to the response call )
      const fakeEip712Message = createEIP712ResponseZKPoK(
        hre.network.config.chainId!,
        zkpokManagerAddress,
        fakeCtHandles,
        userAddress,
        contractAddress,
        contractChainId,
      );

      // Get the EIP712 signatures
      const [fakeSignature] = await getSignaturesZKPoK(fakeEip712Message, coprocessorSigners);

      // Trigger a malicious proof verification response with the first coprocessor
      await zkpokManager.verifyProofResponse(zkProofId, fakeCtHandles, fakeSignature);

      // Trigger two valid proof verification responses with the 2 other coprocessors' signatures
      let txResponse2 = zkpokManager.verifyProofResponse(zkProofId, ctHandles, signature2);
      let txResponse3 = zkpokManager.verifyProofResponse(zkProofId, ctHandles, signature3);

      // Consensus should not be reached at the second response since the first response is malicious
      // Check 2nd response event: it should not emit an event for proof verification
      await expect(txResponse2).to.not.emit(zkpokManager, "VerifyProofResponse");

      // Consensus should be reached at the third response
      // Check 3rd response event: it should only contain 2 valid signatures
      await expect(txResponse3)
        .to.emit(zkpokManager, "VerifyProofResponse")
        .withArgs(zkProofId, ctHandles, [signature2, signature3]);

      // Check that the proof is verified
      expect(await zkpokManager.isProofVerified(zkProofId)).to.be.true;
    });

    it("Should revert because of two responses with same signature for proof verification", async function () {
      // Trigger a first proof response
      await zkpokManager.verifyProofResponse(zkProofId, ctHandles, signature1);

      // Check that a coprocessor cannot sign a second time for the same proof
      await expect(zkpokManager.verifyProofResponse(zkProofId, ctHandles, signature1))
        .revertedWithCustomError(zkpokManager, "CoprocessorSignerAlreadyResponded")
        .withArgs(zkProofId, coprocessorSigners[0].address);
    });

    it("Should revert because same coprocessor both verifies and rejects a proof", async function () {
      // Trigger a proof verification response with the first coprocessor's signature
      await zkpokManager.verifyProofResponse(zkProofId, ctHandles, signature1);

      // Check that the same coprocessor cannot send a second response for the same proof with a
      // proof rejection
      await expect(zkpokManager.connect(coprocessorSigners[0]).rejectProofResponse(zkProofId))
        .revertedWithCustomError(zkpokManager, "CoprocessorSignerAlreadyResponded")
        .withArgs(zkProofId, coprocessorSigners[0].address);
    });

    it("Should revert because the signer is not a coprocessor", async function () {
      // Create a fake signature from a non-coprocessor signer
      const [fakeSignature] = await getSignaturesZKPoK(eip712Message, [fakeSigner]);

      // Check that triggering a proof response using a signature from a non-coprocessor signer reverts
      await expect(zkpokManager.verifyProofResponse(zkProofId, ctHandles, fakeSignature))
        .revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(fakeSigner.address, httpz.COPROCESSOR_ROLE());
    });
  });

  describe("Proof rejection response", async function () {
    let httpz: HTTPZ;
    let zkpokManager: ZKPoKManager;
    let coprocessorSigners: HardhatEthersSigner[];
    let fakeSigner: HardhatEthersSigner;
    let contractChainId: number;
    let zkpokManagerAddress: string;

    beforeEach(async function () {
      const fixture = await loadFixture(deployZKPoKManagerFixture);
      httpz = fixture.httpz;
      zkpokManager = fixture.zkpokManager;
      coprocessorSigners = fixture.coprocessorSigners;
      fakeSigner = fixture.signers[0];
      contractChainId = fixture.chainIds[0];

      zkpokManagerAddress = await zkpokManager.getAddress();

      // The ZK proof ID will always be 1 since we deploy a new contract before each test
      await zkpokManager.verifyProofRequest(contractChainId, contractAddress, userAddress, ciphertextWithZKProof);
    });

    it("Should reject a proof with 2 valid responses", async function () {
      // Trigger two valid proof rejection responses using different coprocessors
      await zkpokManager.connect(coprocessorSigners[0]).rejectProofResponse(zkProofId);
      let txResponse = zkpokManager.connect(coprocessorSigners[1]).rejectProofResponse(zkProofId);

      // Consensus should be reached at the second response
      await expect(txResponse).to.emit(zkpokManager, "RejectProofResponse").withArgs(zkProofId);

      // Check that the proof is verified
      expect(await zkpokManager.isProofRejected(zkProofId)).to.be.true;
    });

    it("Should ignore other valid responses", async function () {
      // Trigger three valid proof rejection responses using different coprocessors
      let txResponse1 = await zkpokManager.connect(coprocessorSigners[0]).rejectProofResponse(zkProofId);
      await zkpokManager.connect(coprocessorSigners[1]).rejectProofResponse(zkProofId);
      let txResponse3 = zkpokManager.connect(coprocessorSigners[2]).rejectProofResponse(zkProofId);

      // Check that the 1st and 3rd responses do not emit an event:
      // - 1st response is ignored because consensus is not reached yet
      // - 3rd response is ignored (not reverted) even though it is late
      await expect(txResponse1).to.not.emit(zkpokManager, "RejectProofResponse");
      await expect(txResponse3).to.not.emit(zkpokManager, "RejectProofResponse");
    });

    it("Should reject a proof with 2 valid responses and 1 valid proof verification response", async function () {
      // Create the EIP712 message
      const eip712Message = createEIP712ResponseZKPoK(
        hre.network.config.chainId!,
        zkpokManagerAddress,
        ctHandles,
        userAddress,
        contractAddress,
        contractChainId,
      );

      // Get the EIP712 signatures
      const [signature1] = await getSignaturesZKPoK(eip712Message, coprocessorSigners);

      // Trigger a valid proof verification response with the first coprocessor's signature
      await zkpokManager.verifyProofResponse(zkProofId, ctHandles, signature1);

      // Trigger two valid proof rejection responses with the 2 other coprocessors
      let txResponse2 = zkpokManager.connect(coprocessorSigners[1]).rejectProofResponse(zkProofId);
      let txResponse3 = zkpokManager.connect(coprocessorSigners[2]).rejectProofResponse(zkProofId);

      // Consensus should not be reached at the second response since the first response is a proof verification
      // Check 2nd response event: it should not emit an event (either for proof verification or rejection)
      await expect(txResponse2)
        .to.not.emit(zkpokManager, "RejectProofResponse")
        .to.not.emit(zkpokManager, "VerifyProofResponse");

      // Consensus should be reached at the third response
      await expect(txResponse3).to.emit(zkpokManager, "RejectProofResponse").withArgs(zkProofId);

      // Check that the proof is verified
      expect(await zkpokManager.isProofRejected(zkProofId)).to.be.true;
    });

    it("Should revert because of two responses from the same coprocessor", async function () {
      const coprocessorSigner = coprocessorSigners[0];

      // Trigger a first proof response
      await zkpokManager.connect(coprocessorSigner).rejectProofResponse(zkProofId);

      // Check that a coprocessor cannot send a second response for the same proof
      await expect(zkpokManager.connect(coprocessorSigner).rejectProofResponse(zkProofId))
        .revertedWithCustomError(zkpokManager, "CoprocessorSignerAlreadyResponded")
        .withArgs(zkProofId, coprocessorSigner.address);
    });

    it("Should revert because the sender is not a coprocessor", async function () {
      // Check that triggering a proof response with a non-coprocessor sender reverts
      await expect(zkpokManager.connect(fakeSigner).rejectProofResponse(zkProofId))
        .revertedWithCustomError(httpz, "AccessControlUnauthorizedAccount")
        .withArgs(fakeSigner.address, httpz.COPROCESSOR_ROLE());
    });
  });
});
