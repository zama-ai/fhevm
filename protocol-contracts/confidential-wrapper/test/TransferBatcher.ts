import { ethers, fhevm } from "hardhat";
import { RegulatedERC7984Upgradeable, ERC7984TransferBatcher, AdminProvider } from "../types";
import { expect } from "chai";
import { getBatchTransferEvent, getConfidentialBalance, getRetryTransferEvent, getBatchTransferFee } from "./utils";
import { getSigners, Signers } from "./signers";
import { deployConfidentialErc20Fixture, deployTransferBatcherFixture } from "./fixtures";


describe("TransferBatcher", function () {
  let signers: Signers;
  let cErc20: RegulatedERC7984Upgradeable;
  let cErc20Address: string;
  let transferBatcher: ERC7984TransferBatcher;
  let transferBatcherAddress: string;
  let adminProvider: AdminProvider;

  before(async function () {
    signers = await getSigners();
  });

  beforeEach(async () => {
    ({ cErc20, cErc20Address, adminProvider } = await deployConfidentialErc20Fixture(signers));
    await cErc20.grantRole(await cErc20.WRAPPER_ROLE(), signers.deployer);
    ({transferBatcher, transferBatcherAddress} = await deployTransferBatcherFixture(signers.deployer, adminProvider));
  });

  describe("confidentialBatchTransfer", function () {
    it("should batch transfer without fees", async function () {
      const mint = await cErc20.mint(signers.alice, 100_000_000);
      await mint.wait();

      const aliceBalanceBefore = await getConfidentialBalance(cErc20, signers.alice);

      const setOperator = await cErc20.connect(signers.alice).setOperator(transferBatcherAddress, Math.floor(Date.now() / 1000) + 6000);
      await setOperator.wait();

      const initTxId = await cErc20.nextTxId();

      const transferAmount = 1_000;
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(transferBatcherAddress, signers.alice.address)
        .add64(transferAmount)
        .add64(transferAmount * 2)
        .add64(transferAmount * 3)
        .encrypt();

      const transferData = [
        {
          "to": signers.bob.address,
          "encryptedAmount": encryptedTransferAmount.handles[0],
          "inputProof": encryptedTransferAmount.inputProof,
          "retryFor": 0,
        },
        {
          "to": signers.charlie.address,
          "encryptedAmount": encryptedTransferAmount.handles[1],
          "inputProof": encryptedTransferAmount.inputProof,
          "retryFor": 0,
        },
        {
          "to": signers.delta.address,
          "encryptedAmount": encryptedTransferAmount.handles[2],
          "inputProof": encryptedTransferAmount.inputProof,
          "retryFor": 0,
        }
      ]
      const nTransfers = BigInt(transferData.length);
      const batchTransferFee = await getBatchTransferFee(adminProvider);
      const batchTransfer = await transferBatcher.connect(signers.alice).confidentialBatchTransfer(cErc20Address, signers.alice.address, transferData, { value: batchTransferFee });
      const receipt = await batchTransfer.wait();

      const batchTransferEvents = getBatchTransferEvent(receipt);
      expect(batchTransferEvents.length).to.equal(1);
      const batchTransferEvent = batchTransferEvents[0];
      expect(batchTransferEvent.args[0]).to.equal(cErc20Address);
      expect(batchTransferEvent.args[1]).to.equal(signers.alice.address);
      expect(batchTransferEvent.args[2]).to.equal(initTxId);
      expect(batchTransferEvent.args[3]).to.equal(initTxId + nTransfers - BigInt(1));

      expect(await getConfidentialBalance(cErc20, signers.alice)).to.equal(aliceBalanceBefore - (BigInt(transferAmount * 6)));
      expect(await getConfidentialBalance(cErc20, signers.bob)).to.equal(transferAmount);
      expect(await getConfidentialBalance(cErc20, signers.charlie)).to.equal(transferAmount * 2);
      expect(await getConfidentialBalance(cErc20, signers.delta)).to.equal(transferAmount * 3);
    });

    it("should allow retry by original sender", async function () {
      const mint = await cErc20.mint(signers.alice, 100_000_000);
      await mint.wait();

      const setOperator = await cErc20.connect(signers.alice).setOperator(transferBatcherAddress, Math.floor(Date.now() / 1000) + 6000);
      await setOperator.wait();

      const transferAmount = 1_000;
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(transferBatcherAddress, signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      // Original transfer
      const originalTransferData = [
        {
          "to": signers.bob.address,
          "encryptedAmount": encryptedTransferAmount.handles[0],
          "inputProof": encryptedTransferAmount.inputProof,
          "retryFor": 0,
        }
      ];

      const originalTxId = await cErc20.nextTxId();
      const batchTransferFee = await getBatchTransferFee(adminProvider);
      const originalBatchTransfer = await transferBatcher.connect(signers.alice).confidentialBatchTransfer(cErc20Address, signers.alice.address, originalTransferData, { value: batchTransferFee });
      await originalBatchTransfer.wait();

      // Verify original transaction was recorded
      expect(await transferBatcher.txIdToSender(cErc20Address, originalTxId)).to.equal(signers.alice.address);

      // Retry the same transfer
      const encryptedRetryAmount = await fhevm
        .createEncryptedInput(transferBatcherAddress, signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      const retryTransferData = [
        {
          "to": signers.bob.address,
          "encryptedAmount": encryptedRetryAmount.handles[0],
          "inputProof": encryptedRetryAmount.inputProof,
          "retryFor": Number(originalTxId),
        }
      ];

      const retryTxId = await cErc20.nextTxId();
      const retryBatchTransfer = await transferBatcher.connect(signers.alice).confidentialBatchTransfer(cErc20Address, signers.alice.address, retryTransferData, { value: batchTransferFee });
      const retryReceipt = await retryBatchTransfer.wait();

      // Check retry event was emitted
      const retryEvents = getRetryTransferEvent(retryReceipt);
      expect(retryEvents.length).to.equal(1);
      const retryEvent = retryEvents[0];
      expect(retryEvent.args[0]).to.equal(cErc20Address); // cToken
      expect(retryEvent.args[1]).to.equal(signers.alice.address); // sender
      expect(retryEvent.args[2]).to.equal(originalTxId); // originalTxId
      expect(retryEvent.args[3]).to.equal(retryTxId); // retryTxId

      // Verify retry transaction was also recorded
      expect(await transferBatcher.txIdToSender(cErc20Address, retryTxId)).to.equal(signers.alice.address);

      // Verify both transfers went through (bob should have double the amount)
      expect(await getConfidentialBalance(cErc20, signers.bob)).to.equal(transferAmount * 2);
    });

    it("should reject retry by non-original sender", async function () {
      const mint = await cErc20.mint(signers.alice, 100_000_000);
      await mint.wait();

      const setOperatorAlice = await cErc20.connect(signers.alice).setOperator(transferBatcherAddress, Math.floor(Date.now() / 1000) + 6000);
      await setOperatorAlice.wait();

      const setOperatorBob = await cErc20.connect(signers.bob).setOperator(transferBatcherAddress, Math.floor(Date.now() / 1000) + 6000);
      await setOperatorBob.wait();

      const transferAmount = 1_000;
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(transferBatcherAddress, signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      // Original transfer by Alice
      const originalTransferData = [
        {
          "to": signers.charlie.address,
          "encryptedAmount": encryptedTransferAmount.handles[0],
          "inputProof": encryptedTransferAmount.inputProof,
          "retryFor": 0,
        }
      ];

      const originalTxId = await cErc20.nextTxId();
      const batchTransferFee = await getBatchTransferFee(adminProvider);
      const originalBatchTransfer = await transferBatcher.connect(signers.alice).confidentialBatchTransfer(cErc20Address, signers.alice.address, originalTransferData, { value: batchTransferFee });
      await originalBatchTransfer.wait();

      // Try to retry as Bob (should fail)
      const encryptedRetryAmount = await fhevm
        .createEncryptedInput(transferBatcherAddress, signers.bob.address)
        .add64(transferAmount)
        .encrypt();

      const retryTransferData = [
        {
          "to": signers.charlie.address,
          "encryptedAmount": encryptedRetryAmount.handles[0],
          "inputProof": encryptedRetryAmount.inputProof,
          "retryFor": Number(originalTxId),
        }
      ];

      await expect(
        transferBatcher.connect(signers.bob).confidentialBatchTransfer(cErc20Address, signers.bob.address, retryTransferData, { value: batchTransferFee })
      ).to.be.revertedWithCustomError(transferBatcher, "OnlyOriginalSenderCanRetry");
    });

    it("should handle multiple retries in same batch", async function () {
      const mint = await cErc20.mint(signers.alice, 100_000_000);
      await mint.wait();

      const setOperator = await cErc20.connect(signers.alice).setOperator(transferBatcherAddress, Math.floor(Date.now() / 1000) + 6000);
      await setOperator.wait();

      const transferAmount = 1_000;
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(transferBatcherAddress, signers.alice.address)
        .add64(transferAmount)
        .add64(transferAmount * 2)
        .encrypt();

      // First batch of original transfers
      const originalTransferData = [
        {
          "to": signers.bob.address,
          "encryptedAmount": encryptedTransferAmount.handles[0],
          "inputProof": encryptedTransferAmount.inputProof,
          "retryFor": 0,
        },
        {
          "to": signers.charlie.address,
          "encryptedAmount": encryptedTransferAmount.handles[1],
          "inputProof": encryptedTransferAmount.inputProof,
          "retryFor": 0,
        }
      ];

      const firstTxId = await cErc20.nextTxId();
      const batchTransferFee = await getBatchTransferFee(adminProvider);
      const originalBatchTransfer = await transferBatcher.connect(signers.alice).confidentialBatchTransfer(cErc20Address, signers.alice.address, originalTransferData, { value: batchTransferFee });
      await originalBatchTransfer.wait();

      const secondTxId = firstTxId + BigInt(1);

      // Retry both transfers in a single batch
      const encryptedRetryAmount = await fhevm
        .createEncryptedInput(transferBatcherAddress, signers.alice.address)
        .add64(transferAmount)
        .add64(transferAmount * 2)
        .encrypt();

      const retryTransferData = [
        {
          "to": signers.bob.address,
          "encryptedAmount": encryptedRetryAmount.handles[0],
          "inputProof": encryptedRetryAmount.inputProof,
          "retryFor": Number(firstTxId),
        },
        {
          "to": signers.charlie.address,
          "encryptedAmount": encryptedRetryAmount.handles[1],
          "inputProof": encryptedRetryAmount.inputProof,
          "retryFor": Number(secondTxId),
        }
      ];

      const retryBatchTransfer = await transferBatcher.connect(signers.alice).confidentialBatchTransfer(cErc20Address, signers.alice.address, retryTransferData, { value: batchTransferFee });
      const retryReceipt = await retryBatchTransfer.wait();

      // Check that two retry events were emitted
      const retryEvents = getRetryTransferEvent(retryReceipt);
      expect(retryEvents.length).to.equal(2);

      // Verify the events contain correct information
      expect(retryEvents[0].args[2]).to.equal(firstTxId); // originalTxId
      expect(retryEvents[1].args[2]).to.equal(secondTxId); // originalTxId

      // Verify balances show both original and retry transfers went through
      expect(await getConfidentialBalance(cErc20, signers.bob)).to.equal(transferAmount * 2);
      expect(await getConfidentialBalance(cErc20, signers.charlie)).to.equal(transferAmount * 4);
    });

    it("should reject retry for non-existent transaction", async function () {
      const mint = await cErc20.mint(signers.alice, 100_000_000);
      await mint.wait();

      const setOperator = await cErc20.connect(signers.alice).setOperator(transferBatcherAddress, Math.floor(Date.now() / 1000) + 6000);
      await setOperator.wait();

      const transferAmount = 1_000;
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(transferBatcherAddress, signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      const nonExistentTxId = 999999;
      const retryTransferData = [
        {
          "to": signers.bob.address,
          "encryptedAmount": encryptedTransferAmount.handles[0],
          "inputProof": encryptedTransferAmount.inputProof,
          "retryFor": nonExistentTxId,
        }
      ];

      const batchTransferFee = await getBatchTransferFee(adminProvider);
      await expect(
        transferBatcher.connect(signers.alice).confidentialBatchTransfer(cErc20Address, signers.alice.address, retryTransferData, { value: batchTransferFee })
      ).to.be.revertedWithCustomError(transferBatcher, "OnlyOriginalSenderCanRetry");
    });

    it("should reject calls with insufficient fee", async function () {
      const mint = await cErc20.mint(signers.alice, 100_000_000);
      await mint.wait();

      const setOperator = await cErc20.connect(signers.alice).setOperator(transferBatcherAddress, Math.floor(Date.now() / 1000) + 6000);
      await setOperator.wait();

      const transferAmount = 1_000;
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(transferBatcherAddress, signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      const transferData = [
        {
          "to": signers.bob.address,
          "encryptedAmount": encryptedTransferAmount.handles[0],
          "inputProof": encryptedTransferAmount.inputProof,
          "retryFor": 0,
        }
      ];

      const batchTransferFee = await getBatchTransferFee(adminProvider);
      const insufficientFee = batchTransferFee - BigInt(1);

      await expect(
        transferBatcher.connect(signers.alice).confidentialBatchTransfer(cErc20Address, signers.alice.address, transferData, { value: insufficientFee })
      ).to.be.revertedWithCustomError(transferBatcher, "InsufficientFee");
    });

    it("should reject calls with excess fee", async function () {
      const mint = await cErc20.mint(signers.alice, 100_000_000);
      await mint.wait();

      const setOperator = await cErc20.connect(signers.alice).setOperator(transferBatcherAddress, Math.floor(Date.now() / 1000) + 6000);
      await setOperator.wait();

      const transferAmount = 1_000;
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(transferBatcherAddress, signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      const transferData = [
        {
          "to": signers.bob.address,
          "encryptedAmount": encryptedTransferAmount.handles[0],
          "inputProof": encryptedTransferAmount.inputProof,
          "retryFor": 0,
        }
      ];

      const batchTransferFee = await getBatchTransferFee(adminProvider);
      const excessFee = batchTransferFee + BigInt(1);

      await expect(
        transferBatcher.connect(signers.alice).confidentialBatchTransfer(cErc20Address, signers.alice.address, transferData, { value: excessFee })
      ).to.be.revertedWithCustomError(transferBatcher, "InsufficientFee");
    });

    it("should prevent cross-token transaction ID collision", async function () {
      // Deploy a second confidential token
      const { cErc20: cErc20B, cErc20Address: cErc20AddressB } = await deployConfidentialErc20Fixture(signers);
      await cErc20B.grantRole(await cErc20B.WRAPPER_ROLE(), signers.deployer);

      // Alias for clarity
      const cErc20A = cErc20;
      const cErc20AddressA = cErc20Address;

      // Mint tokens for alice in token A only
      await cErc20A.mint(signers.alice, 100_000_000);

      // Setup operator for token A
      await cErc20A.connect(signers.alice).setOperator(transferBatcherAddress, Math.floor(Date.now() / 1000) + 6000);

      const transferAmount = 1_000;

      // Transfer with token A - this will use txId = 1
      const encryptedAmountA = await fhevm
        .createEncryptedInput(transferBatcherAddress, signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      const transferDataA = [
        {
          "to": signers.bob.address,
          "encryptedAmount": encryptedAmountA.handles[0],
          "inputProof": encryptedAmountA.inputProof,
          "retryFor": 0,
        }
      ];

      const txIdA = await cErc20A.nextTxId();
      const batchTransferFee = await getBatchTransferFee(adminProvider);
      await transferBatcher.connect(signers.alice).confidentialBatchTransfer(cErc20AddressA, signers.alice.address, transferDataA, { value: batchTransferFee });

      // Verify alice is recorded as sender for token A's txId
      expect(await transferBatcher.txIdToSender(cErc20AddressA, txIdA)).to.equal(signers.alice.address);

      // Now bob transfers with token B - this will also use txId = 1 (both start fresh)
      await cErc20B.mint(signers.bob, 100_000_000);
      await cErc20B.connect(signers.bob).setOperator(transferBatcherAddress, Math.floor(Date.now() / 1000) + 6000);

      const encryptedAmountB = await fhevm
        .createEncryptedInput(transferBatcherAddress, signers.bob.address)
        .add64(transferAmount)
        .encrypt();

      const transferDataB = [
        {
          "to": signers.charlie.address,
          "encryptedAmount": encryptedAmountB.handles[0],
          "inputProof": encryptedAmountB.inputProof,
          "retryFor": 0,
        }
      ];

      const txIdB = await cErc20B.nextTxId();
      expect(txIdB).to.equal(txIdA); // Both tokens have the same txId = 1

      await transferBatcher.connect(signers.bob).confidentialBatchTransfer(cErc20AddressB, signers.bob.address, transferDataB, { value: batchTransferFee });

      // Verify bob is recorded as sender for token B's txId
      expect(await transferBatcher.txIdToSender(cErc20AddressB, txIdB)).to.equal(signers.bob.address);

      // CRITICAL: Verify alice's entry for token A was NOT overwritten
      expect(await transferBatcher.txIdToSender(cErc20AddressA, txIdA)).to.equal(signers.alice.address);

      // Verify alice can still retry her original transaction on token A
      const encryptedRetryA = await fhevm
        .createEncryptedInput(transferBatcherAddress, signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      const retryDataA = [
        {
          "to": signers.bob.address,
          "encryptedAmount": encryptedRetryA.handles[0],
          "inputProof": encryptedRetryA.inputProof,
          "retryFor": Number(txIdA),
        }
      ];

      // This should succeed - alice should still be able to retry her transaction
      await expect(
        transferBatcher.connect(signers.alice).confidentialBatchTransfer(cErc20AddressA, signers.alice.address, retryDataA, { value: batchTransferFee })
      ).to.not.be.reverted;

      // Verify bob can retry his transaction on token B
      const encryptedRetryB = await fhevm
        .createEncryptedInput(transferBatcherAddress, signers.bob.address)
        .add64(transferAmount)
        .encrypt();

      const retryDataB = [
        {
          "to": signers.charlie.address,
          "encryptedAmount": encryptedRetryB.handles[0],
          "inputProof": encryptedRetryB.inputProof,
          "retryFor": Number(txIdB),
        }
      ];

      await expect(
        transferBatcher.connect(signers.bob).confidentialBatchTransfer(cErc20AddressB, signers.bob.address, retryDataB, { value: batchTransferFee })
      ).to.not.be.reverted;

      // Verify alice CANNOT retry bob's transaction on token B (cross-token protection)
      const encryptedAliceRetryB = await fhevm
        .createEncryptedInput(transferBatcherAddress, signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      const aliceRetryDataB = [
        {
          "to": signers.charlie.address,
          "encryptedAmount": encryptedAliceRetryB.handles[0],
          "inputProof": encryptedAliceRetryB.inputProof,
          "retryFor": Number(txIdB),
        }
      ];

      await expect(
        transferBatcher.connect(signers.alice).confidentialBatchTransfer(cErc20AddressB, signers.alice.address, aliceRetryDataB, { value: batchTransferFee })
      ).to.be.revertedWithCustomError(transferBatcher, "OnlyOriginalSenderCanRetry");
    });

    it("should revert with FeeTransferFailed when fee transfer fails", async function () {
      const mint = await cErc20.mint(signers.alice, 100_000_000);
      await mint.wait();

      const setOperator = await cErc20.connect(signers.alice).setOperator(transferBatcherAddress, Math.floor(Date.now() / 1000) + 6000);
      await setOperator.wait();

      // Deploy RejectEth contract
      const RejectEth = await ethers.getContractFactory("RejectEth");
      const rejectEth = await RejectEth.deploy();
      await rejectEth.waitForDeployment();
      const rejectEthAddress = await rejectEth.getAddress();

      // Set fee recipient to RejectEth contract
      const feeManagerAddress = await adminProvider.feeManager();
      const feeManager = await ethers.getContractAt("FeeManager", feeManagerAddress);
      const originalFeeRecipient = await feeManager.getFeeRecipient();
      await feeManager.setFeeRecipient(rejectEthAddress);

      const transferAmount = 1_000;
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(transferBatcherAddress, signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      const transferData = [
        {
          "to": signers.bob.address,
          "encryptedAmount": encryptedTransferAmount.handles[0],
          "inputProof": encryptedTransferAmount.inputProof,
          "retryFor": 0,
        }
      ];

      const batchTransferFee = await getBatchTransferFee(adminProvider);

      // Should revert with FeeTransferFailed error
      await expect(
        transferBatcher.connect(signers.alice).confidentialBatchTransfer(cErc20Address, signers.alice.address, transferData, { value: batchTransferFee })
      ).to.be.revertedWithCustomError(transferBatcher, "FeeTransferFailed");

      // Restore original fee recipient
      await feeManager.setFeeRecipient(originalFeeRecipient);
    });

    it("should revert when transfers array is empty", async function () {
      const mint = await cErc20.mint(signers.alice, 100_000_000);
      await mint.wait();

      const setOperator = await cErc20.connect(signers.alice).setOperator(transferBatcherAddress, Math.floor(Date.now() / 1000) + 6000);
      await setOperator.wait();

      // Empty transfer array
      const emptyTransferData: any[] = [];

      const batchTransferFee = await getBatchTransferFee(adminProvider);

      // Should revert with EmptyTransferArray error
      await expect(
        transferBatcher.connect(signers.alice).confidentialBatchTransfer(cErc20Address, signers.alice.address, emptyTransferData, { value: batchTransferFee })
      ).to.be.revertedWithCustomError(transferBatcher, "EmptyTransferArray");
    });

    it("should emit correct txId range for single transfer", async function () {
      const mint = await cErc20.mint(signers.alice, 100_000_000);
      await mint.wait();

      const setOperator = await cErc20.connect(signers.alice).setOperator(transferBatcherAddress, Math.floor(Date.now() / 1000) + 6000);
      await setOperator.wait();

      const transferAmount = 1_000;
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(transferBatcherAddress, signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      const transferData = [
        {
          "to": signers.bob.address,
          "encryptedAmount": encryptedTransferAmount.handles[0],
          "inputProof": encryptedTransferAmount.inputProof,
          "retryFor": 0,
        }
      ];

      const expectedStartTxId = await cErc20.nextTxId();
      const batchTransferFee = await getBatchTransferFee(adminProvider);
      const batchTransfer = await transferBatcher.connect(signers.alice).confidentialBatchTransfer(cErc20Address, signers.alice.address, transferData, { value: batchTransferFee });
      const receipt = await batchTransfer.wait();

      const batchTransferEvents = getBatchTransferEvent(receipt);
      expect(batchTransferEvents.length).to.equal(1);

      const batchTransferEvent = batchTransferEvents[0];
      expect(batchTransferEvent.args[2]).to.equal(expectedStartTxId); // startTxId
      expect(batchTransferEvent.args[3]).to.equal(expectedStartTxId); // endTxId should equal startTxId for single transfer
    });

    it("should emit correct txId range for multiple transfers", async function () {
      const mint = await cErc20.mint(signers.alice, 100_000_000);
      await mint.wait();

      const setOperator = await cErc20.connect(signers.alice).setOperator(transferBatcherAddress, Math.floor(Date.now() / 1000) + 6000);
      await setOperator.wait();

      const transferAmount = 1_000;
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(transferBatcherAddress, signers.alice.address)
        .add64(transferAmount)
        .add64(transferAmount * 2)
        .add64(transferAmount * 3)
        .encrypt();

      const transferData = [
        {
          "to": signers.bob.address,
          "encryptedAmount": encryptedTransferAmount.handles[0],
          "inputProof": encryptedTransferAmount.inputProof,
          "retryFor": 0,
        },
        {
          "to": signers.charlie.address,
          "encryptedAmount": encryptedTransferAmount.handles[1],
          "inputProof": encryptedTransferAmount.inputProof,
          "retryFor": 0,
        },
        {
          "to": signers.delta.address,
          "encryptedAmount": encryptedTransferAmount.handles[2],
          "inputProof": encryptedTransferAmount.inputProof,
          "retryFor": 0,
        }
      ];

      const expectedStartTxId = await cErc20.nextTxId();
      const expectedEndTxId = expectedStartTxId + BigInt(2); // 3 transfers, so endTxId = startTxId + 2
      const batchTransferFee = await getBatchTransferFee(adminProvider);
      const batchTransfer = await transferBatcher.connect(signers.alice).confidentialBatchTransfer(cErc20Address, signers.alice.address, transferData, { value: batchTransferFee });
      const receipt = await batchTransfer.wait();

      const batchTransferEvents = getBatchTransferEvent(receipt);
      expect(batchTransferEvents.length).to.equal(1);

      const batchTransferEvent = batchTransferEvents[0];
      expect(batchTransferEvent.args[2]).to.equal(expectedStartTxId); // startTxId
      expect(batchTransferEvent.args[3]).to.equal(expectedEndTxId); // endTxId

      // Verify range is valid: endTxId >= startTxId
      expect(batchTransferEvent.args[3]).to.be.gte(batchTransferEvent.args[2]);
    });
  });

  describe("Operator functionality", function () {
    it("should allow operator to perform batch transfer on behalf of token holder", async function () {
      // Mint tokens to Bob
      const mint = await cErc20.mint(signers.bob, 100_000_000);
      await mint.wait();

      const bobBalanceBefore = await getConfidentialBalance(cErc20, signers.bob);

      // Bob approves Alice as operator
      const setOperator = await cErc20.connect(signers.bob).setOperator(signers.alice.address, Math.floor(Date.now() / 1000) + 6000);
      await setOperator.wait();

      // Alice also needs to be an operator for the batcher to perform transfers
      const setBatcherOperator = await cErc20.connect(signers.bob).setOperator(transferBatcherAddress, Math.floor(Date.now() / 1000) + 6000);
      await setBatcherOperator.wait();

      const transferAmount = 1_000;
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(transferBatcherAddress, signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      const transferData = [
        {
          "to": signers.charlie.address,
          "encryptedAmount": encryptedTransferAmount.handles[0],
          "inputProof": encryptedTransferAmount.inputProof,
          "retryFor": 0,
        }
      ];

      const initTxId = await cErc20.nextTxId();
      const batchTransferFee = await getBatchTransferFee(adminProvider);

      // Alice (operator) performs batch transfer on behalf of Bob
      const batchTransfer = await transferBatcher.connect(signers.alice).confidentialBatchTransfer(
        cErc20Address,
        signers.bob.address,  // Transfer from Bob's account
        transferData,
        { value: batchTransferFee }
      );
      const receipt = await batchTransfer.wait();

      // Verify the event shows Bob as the sender (not Alice)
      const batchTransferEvents = getBatchTransferEvent(receipt);
      expect(batchTransferEvents.length).to.equal(1);
      const batchTransferEvent = batchTransferEvents[0];
      expect(batchTransferEvent.args[0]).to.equal(cErc20Address);
      expect(batchTransferEvent.args[1]).to.equal(signers.bob.address); // Bob is the sender
      expect(batchTransferEvent.args[2]).to.equal(initTxId);

      // Verify txIdToSender tracks Bob (token owner), not Alice (operator)
      expect(await transferBatcher.txIdToSender(cErc20Address, initTxId)).to.equal(signers.bob.address);

      // Verify balances: Bob's balance decreased, Charlie received tokens
      expect(await getConfidentialBalance(cErc20, signers.bob)).to.equal(bobBalanceBefore - BigInt(transferAmount));
      expect(await getConfidentialBalance(cErc20, signers.charlie)).to.equal(transferAmount);
    });

    it("should reject batch transfer when caller is not an approved operator", async function () {
      // Mint tokens to Bob
      const mint = await cErc20.mint(signers.bob, 100_000_000);
      await mint.wait();

      // Alice is NOT approved as operator for Bob
      const setOperator = await cErc20.connect(signers.bob).setOperator(signers.alice.address, Math.floor(Date.now() / 1000) - 6000);
      await setOperator.wait();

      // Bob still needs to approve batcher
      const setBatcherOperator = await cErc20.connect(signers.bob).setOperator(transferBatcherAddress, Math.floor(Date.now() / 1000) + 6000);
      await setBatcherOperator.wait();

      const transferAmount = 1_000;
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(transferBatcherAddress, signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      const transferData = [
        {
          "to": signers.charlie.address,
          "encryptedAmount": encryptedTransferAmount.handles[0],
          "inputProof": encryptedTransferAmount.inputProof,
          "retryFor": 0,
        }
      ];

      const batchTransferFee = await getBatchTransferFee(adminProvider);

      // Alice tries to perform batch transfer on behalf of Bob (should fail)
      await expect(
        transferBatcher.connect(signers.alice).confidentialBatchTransfer(
          cErc20Address,
          signers.bob.address,
          transferData,
          { value: batchTransferFee }
        )
      )
        .to.be.revertedWithCustomError(transferBatcher, "ERC7984UnauthorizedSpender")
        .withArgs(signers.bob.address, signers.alice.address);
    });

    it("should allow operator to retry transactions on behalf of token holder", async function () {
      // Mint tokens to Bob
      const mint = await cErc20.mint(signers.bob, 100_000_000);
      await mint.wait();

      // Bob approves Alice as operator
      const setOperator = await cErc20.connect(signers.bob).setOperator(signers.alice.address, Math.floor(Date.now() / 1000) + 6000);
      await setOperator.wait();

      const setBatcherOperator = await cErc20.connect(signers.bob).setOperator(transferBatcherAddress, Math.floor(Date.now() / 1000) + 6000);
      await setBatcherOperator.wait();

      const transferAmount = 1_000;
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(transferBatcherAddress, signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      // Original transfer by Alice (as Bob's operator)
      const originalTransferData = [
        {
          "to": signers.charlie.address,
          "encryptedAmount": encryptedTransferAmount.handles[0],
          "inputProof": encryptedTransferAmount.inputProof,
          "retryFor": 0,
        }
      ];

      const originalTxId = await cErc20.nextTxId();
      const batchTransferFee = await getBatchTransferFee(adminProvider);
      const originalBatchTransfer = await transferBatcher.connect(signers.alice).confidentialBatchTransfer(
        cErc20Address,
        signers.bob.address,
        originalTransferData,
        { value: batchTransferFee }
      );
      await originalBatchTransfer.wait();

      // Verify original transaction was recorded with Bob as sender
      expect(await transferBatcher.txIdToSender(cErc20Address, originalTxId)).to.equal(signers.bob.address);

      // Retry the same transfer (Alice acting as Bob's operator)
      const encryptedRetryAmount = await fhevm
        .createEncryptedInput(transferBatcherAddress, signers.alice.address)
        .add64(transferAmount)
        .encrypt();

      const retryTransferData = [
        {
          "to": signers.charlie.address,
          "encryptedAmount": encryptedRetryAmount.handles[0],
          "inputProof": encryptedRetryAmount.inputProof,
          "retryFor": Number(originalTxId),
        }
      ];

      const retryTxId = await cErc20.nextTxId();
      const retryBatchTransfer = await transferBatcher.connect(signers.alice).confidentialBatchTransfer(
        cErc20Address,
        signers.bob.address,
        retryTransferData,
        { value: batchTransferFee }
      );
      const retryReceipt = await retryBatchTransfer.wait();

      // Check retry event was emitted with Bob as sender
      const retryEvents = getRetryTransferEvent(retryReceipt);
      expect(retryEvents.length).to.equal(1);
      const retryEvent = retryEvents[0];
      expect(retryEvent.args[0]).to.equal(cErc20Address);
      expect(retryEvent.args[1]).to.equal(signers.bob.address); // Bob is the sender
      expect(retryEvent.args[2]).to.equal(originalTxId);
      expect(retryEvent.args[3]).to.equal(retryTxId);

      // Verify both transfers went through (charlie should have double the amount)
      expect(await getConfidentialBalance(cErc20, signers.charlie)).to.equal(transferAmount * 2);
    });

    it("should reject retry when operator tries to retry another user's transaction", async function () {
      // Mint tokens to both Bob and Alice
      await cErc20.mint(signers.bob, 100_000_000);
      await cErc20.mint(signers.alice, 100_000_000);

      // Bob approves batcher
      await cErc20.connect(signers.bob).setOperator(transferBatcherAddress, Math.floor(Date.now() / 1000) + 6000);

      // Alice approves batcher and Charlie as operator
      await cErc20.connect(signers.alice).setOperator(transferBatcherAddress, Math.floor(Date.now() / 1000) + 6000);
      await cErc20.connect(signers.alice).setOperator(signers.charlie.address, Math.floor(Date.now() / 1000) + 6000);

      const transferAmount = 1_000;

      // Bob performs an original transfer
      const encryptedBobAmount = await fhevm
        .createEncryptedInput(transferBatcherAddress, signers.bob.address)
        .add64(transferAmount)
        .encrypt();

      const bobTransferData = [
        {
          "to": signers.delta.address,
          "encryptedAmount": encryptedBobAmount.handles[0],
          "inputProof": encryptedBobAmount.inputProof,
          "retryFor": 0,
        }
      ];

      const bobTxId = await cErc20.nextTxId();
      const batchTransferFee = await getBatchTransferFee(adminProvider);
      await transferBatcher.connect(signers.bob).confidentialBatchTransfer(
        cErc20Address,
        signers.bob.address,
        bobTransferData,
        { value: batchTransferFee }
      );

      // Charlie (Alice's operator) tries to retry Bob's transaction (should fail)
      const encryptedRetryAmount = await fhevm
        .createEncryptedInput(transferBatcherAddress, signers.charlie.address)
        .add64(transferAmount)
        .encrypt();

      const retryTransferData = [
        {
          "to": signers.delta.address,
          "encryptedAmount": encryptedRetryAmount.handles[0],
          "inputProof": encryptedRetryAmount.inputProof,
          "retryFor": Number(bobTxId),
        }
      ];

      await expect(
        transferBatcher.connect(signers.charlie).confidentialBatchTransfer(
          cErc20Address,
          signers.alice.address, // Charlie is trying to retry using Alice's tokens
          retryTransferData,
          { value: batchTransferFee }
        )
      ).to.be.revertedWithCustomError(transferBatcher, "OnlyOriginalSenderCanRetry");
    });

    it("should allow operator to perform batch transfer with multiple recipients", async function () {
      // Mint tokens to Bob
      const mint = await cErc20.mint(signers.bob, 100_000_000);
      await mint.wait();

      const bobBalanceBefore = await getConfidentialBalance(cErc20, signers.bob);

      // Bob approves Alice as operator
      await cErc20.connect(signers.bob).setOperator(signers.alice.address, Math.floor(Date.now() / 1000) + 6000);
      await cErc20.connect(signers.bob).setOperator(transferBatcherAddress, Math.floor(Date.now() / 1000) + 6000);

      const transferAmount = 1_000;
      const encryptedTransferAmount = await fhevm
        .createEncryptedInput(transferBatcherAddress, signers.alice.address)
        .add64(transferAmount)
        .add64(transferAmount * 2)
        .add64(transferAmount * 3)
        .encrypt();

      const transferData = [
        {
          "to": signers.charlie.address,
          "encryptedAmount": encryptedTransferAmount.handles[0],
          "inputProof": encryptedTransferAmount.inputProof,
          "retryFor": 0,
        },
        {
          "to": signers.delta.address,
          "encryptedAmount": encryptedTransferAmount.handles[1],
          "inputProof": encryptedTransferAmount.inputProof,
          "retryFor": 0,
        },
        {
          "to": signers.alice.address, // Alice can receive tokens while being the operator
          "encryptedAmount": encryptedTransferAmount.handles[2],
          "inputProof": encryptedTransferAmount.inputProof,
          "retryFor": 0,
        }
      ];

      const batchTransferFee = await getBatchTransferFee(adminProvider);
      const batchTransfer = await transferBatcher.connect(signers.alice).confidentialBatchTransfer(
        cErc20Address,
        signers.bob.address,
        transferData,
        { value: batchTransferFee }
      );
      await batchTransfer.wait();

      // Verify all transfers went through
      const totalTransferred = BigInt(transferAmount * 6);
      expect(await getConfidentialBalance(cErc20, signers.bob)).to.equal(bobBalanceBefore - totalTransferred);
      expect(await getConfidentialBalance(cErc20, signers.charlie)).to.equal(transferAmount);
      expect(await getConfidentialBalance(cErc20, signers.delta)).to.equal(transferAmount * 2);
      expect(await getConfidentialBalance(cErc20, signers.alice)).to.equal(transferAmount * 3);
    });
  });
});
