import { expect } from "chai";
import type { ContractTransactionResponse } from "ethers";
import { ethers } from "hardhat";

import { aclAddress, createInstances } from "../instance";
import { getSigners, initSigners } from "../signers";
import { waitForBlock } from "../utils";

const NOT_ALLOWED_ON_HOST_ACL = "not_allowed_on_host_acl";
// Protocol v0.11 ACL enforces a 1-hour minimum (ExpirationDateBeforeOneHour).
// Use 1h+1m so the grant succeeds on both v0.11 and v0.13, then fast-forward
// with evm_increaseTime so the test doesn't wait 3660 real seconds.
const DELEGATION_EXPIRY_SECONDS = 3660;
const DELEGATION_EXPIRY_POLL_MS = 2_000;
// Default delegation lifetime for tests that don't intentionally expire mid-run.
const ONE_DAY_SECONDS = 24 * 60 * 60;
// Mocha timeout for tests that wait through one or more coprocessor propagation
// cycles (~6m37s observed on Sepolia for the existing per-contract revoke test).
const SLOW_TEST_TIMEOUT_MS = 10 * 60 * 1000;
// Host-chain blocks to wait for the coprocessor to absorb an ACL change.
const PROPAGATION_BLOCKS = 15;
// Encrypted token balance Alice transfers to the smart wallet in the outer
// `before` hook; reused as the expected value in delegated-decrypt assertions.
const SMART_WALLET_INITIAL_BALANCE = 500000n;

const relayerErrorLabel = (error: unknown): string | undefined => {
  if (typeof error !== "object" || error === null || !("relayerApiError" in error)) {
    return undefined;
  }
  return (error as { relayerApiError?: { label?: string } }).relayerApiError?.label;
};

const waitForDelegationExpiry = async (expirationTimestamp: number) => {
  while (true) {
    const latestBlock = await ethers.provider.getBlock("latest");
    if (latestBlock && latestBlock.timestamp > expirationTimestamp) {
      return;
    }
    await new Promise((resolve) => setTimeout(resolve, DELEGATION_EXPIRY_POLL_MS));
  }
};

describe("Delegated user decryption", function () {
  before(async function () {
    await initSigners(5);
    this.signers = await getSigners();
    this.instances = await createInstances(this.signers);

    // Deploy the EncryptedERC20 token contract.
    const tokenFactory = await ethers.getContractFactory("EncryptedERC20");
    this.token = await tokenFactory.connect(this.signers.alice).deploy("Zama Confidential Token", "ZAMA");
    await this.token.waitForDeployment();
    this.tokenAddress = await this.token.getAddress();

    // Deploy SmartWalletWithDelegation with Bob as the owner.
    const smartWalletFactory = await ethers.getContractFactory("SmartWalletWithDelegation");
    this.smartWallet = await smartWalletFactory.connect(this.signers.bob).deploy(this.signers.bob.address);
    await this.smartWallet.waitForDeployment();
    this.smartWalletAddress = await this.smartWallet.getAddress();

    // Alice mints tokens to herself.
    const mintAmount = 1000000n;
    const mintTx = await this.token.connect(this.signers.alice).mint(mintAmount);
    await mintTx.wait();

    // Alice transfers some tokens to the smartWallet contract.
    const transferAmount = SMART_WALLET_INITIAL_BALANCE;
    const encryptedTransferAmount = await this.instances.alice.encryptUint64({
      value: transferAmount,
      contractAddress: this.tokenAddress,
      userAddress: this.signers.alice.address,
    });

    const transferTx = await this.token
      .connect(this.signers.alice)
      [
        "transfer(address,bytes32,bytes)"
      ](this.smartWalletAddress, encryptedTransferAmount.handles[0], encryptedTransferAmount.inputProof);
    await transferTx.wait();
  });

  it("test delegated user decryption - smartWallet owner delegates his own EOA to decrypt the smartWallet balance", async function () {
    // Bob (smartWallet owner) delegates decryption rights to his own EOA.
    const expirationTimestamp = Math.floor(Date.now() / 1000) + ONE_DAY_SECONDS;
    const delegateTx = await this.smartWallet
      .connect(this.signers.bob)
      .delegateUserDecryption(this.signers.bob.address, this.tokenAddress, expirationTimestamp);
    await delegateTx.wait();

    // Wait for the coprocessor to absorb the ACL change.
    const currentBlock = await ethers.provider.getBlockNumber();
    await waitForBlock(currentBlock + PROPAGATION_BLOCKS);

    // Get the encrypted balance handle of the smartWallet.
    const balanceHandle = await this.token.balanceOf(this.smartWalletAddress);

    // Bob's EOA can now decrypt the smartWallet's confidential balance.
    const decryptedBalance = await this.instances.bob.delegatedUserDecryptSingleHandle({
      handle: balanceHandle,
      contractAddress: this.tokenAddress,
      delegatorAddress: this.smartWalletAddress,
      signer: this.signers.bob,
    });

    // Verify the decrypted balance matches what was transferred.
    expect(decryptedBalance).to.equal(SMART_WALLET_INITIAL_BALANCE);
  });

  it("test widlcard delegation - smartWallet owner delegates a third EOA to decrypt the smartWallet balance", async function () {
    if (!this.instances.carol.supportsWildcard) {
      return;
    }

    const WILDCARD_CONTRACT = "0xFFfFfFffFFfffFFfFFfFFFFFffFFFffffFfFFFfF";
    // Bob (smartWallet owner) delegates decryption rights to Carol's EOA.
    const expirationTimestamp = Math.floor(Date.now() / 1000) + ONE_DAY_SECONDS;
    const delegateTx = await this.smartWallet
      .connect(this.signers.bob)
      .delegateUserDecryption(this.signers.carol.address, WILDCARD_CONTRACT, expirationTimestamp);
    await delegateTx.wait();

    // Wait for the coprocessor to absorb the ACL change.
    const currentBlock = await ethers.provider.getBlockNumber();
    await waitForBlock(currentBlock + PROPAGATION_BLOCKS);

    // Get the encrypted balance handle of the smartWallet.
    const balanceHandle = await this.token.balanceOf(this.smartWalletAddress);

    // Carol's EOA can now decrypt the smartWallet's confidential balance.
    const decryptedBalance = await this.instances.carol.delegatedUserDecryptSingleHandle({
      handle: balanceHandle,
      contractAddress: WILDCARD_CONTRACT,
      delegatorAddress: this.smartWalletAddress,
      signer: this.signers.carol,
    });

    // Verify the decrypted balance matches what was transferred.
    expect(decryptedBalance).to.equal(SMART_WALLET_INITIAL_BALANCE);
  });

  it("test delegated user decryption - smartWallet can execute transference of funds to a third EOA", async function () {
    // First, Bob needs to delegate so the smartWallet can initiate transfers.
    const expirationTimestamp = Math.floor(Date.now() / 1000) + ONE_DAY_SECONDS;
    const delegateTx = await this.smartWallet
      .connect(this.signers.bob)
      .delegateUserDecryption(this.signers.bob.address, this.tokenAddress, expirationTimestamp);
    await delegateTx.wait();

    // Wait for the coprocessor to absorb the ACL change.
    let currentBlock = await ethers.provider.getBlockNumber();
    await waitForBlock(currentBlock + PROPAGATION_BLOCKS);

    // Get the current smartWallet balance before transfer
    const smartWalletBalanceBefore = await this.token.balanceOf(this.smartWalletAddress);
    const decryptedBalanceBefore = await this.instances.bob.delegatedUserDecryptSingleHandle({
      handle: smartWalletBalanceBefore,
      contractAddress: this.tokenAddress,
      delegatorAddress: this.smartWalletAddress,
      signer: this.signers.bob,
    });

    // Bob proposes a transaction from the smartWallet to transfer tokens to Carol.
    // The encrypted input must be created for the smartWallet address since it will be the msg.sender.
    const transferAmount = 100000n;
    const encryptedTransferAmount = await this.instances.bob.encryptUint64({
      value: transferAmount,
      contractAddress: this.tokenAddress,
      userAddress: this.smartWalletAddress,
    });

    // Encode the transfer function call with full signature to avoid ambiguity.
    const transferData = this.token.interface.encodeFunctionData("transfer(address,bytes32,bytes)", [
      this.signers.carol.address,
      encryptedTransferAmount.handles[0],
      encryptedTransferAmount.inputProof,
    ]);

    // Propose the transaction.
    const proposeTx = await this.smartWallet.connect(this.signers.bob).proposeTx(this.tokenAddress, transferData);
    await proposeTx.wait();

    // Get the transaction ID.
    const txId = await this.smartWallet.txCounter();

    // Execute the transaction.
    const executeTx = await this.smartWallet.connect(this.signers.bob).executeTx(txId);
    await executeTx.wait();

    // Verify the smartWallet balance decreased.
    const smartWalletBalanceAfter = await this.token.balanceOf(this.smartWalletAddress);
    const decryptedBalanceAfter = await this.instances.bob.delegatedUserDecryptSingleHandle({
      handle: smartWalletBalanceAfter,
      contractAddress: this.tokenAddress,
      delegatorAddress: this.smartWalletAddress,
      signer: this.signers.bob,
    });

    // The smartWallet balance should have decreased by the transfer amount.
    expect(Number(decryptedBalanceBefore) - Number(decryptedBalanceAfter)).to.equal(Number(transferAmount));
  });

  describe("negative-acl", function () {
    it("should reject when delegation has been revoked", async function () {
      // 10min — observed ~6m37s due to two 15-block waits for delegation propagation in sepolia
      this.timeout(SLOW_TEST_TIMEOUT_MS);
      // First, ensure Bob has delegation.
      const expirationTimestamp = Math.floor(Date.now() / 1000) + ONE_DAY_SECONDS;
      const delegateTx = await this.smartWallet
        .connect(this.signers.bob)
        .delegateUserDecryption(this.signers.bob.address, this.tokenAddress, expirationTimestamp);
      await delegateTx.wait();

      // Wait for the coprocessor to absorb the ACL change.
      const currentBlock1 = await ethers.provider.getBlockNumber();
      await waitForBlock(currentBlock1 + PROPAGATION_BLOCKS);

      // Revoke the delegation for Bob's EOA.
      const revokeTx = await this.smartWallet
        .connect(this.signers.bob)
        .revokeUserDecryptionDelegation(this.signers.bob.address, this.tokenAddress);
      await revokeTx.wait();

      // Wait for the coprocessor to absorb the revocation.
      const currentBlock2 = await ethers.provider.getBlockNumber();
      await waitForBlock(currentBlock2 + PROPAGATION_BLOCKS);

      // Try to decrypt the smartWallet balance with Bob's EOA, which should now fail.
      const balanceHandle = await this.token.balanceOf(this.smartWalletAddress);

      try {
        await this.instances.bob.delegatedUserDecryptSingleHandle({
          handle: balanceHandle,
          contractAddress: this.tokenAddress,
          delegatorAddress: this.smartWalletAddress,
          signer: this.signers.bob,
        });
        expect.fail("Expected delegated user decrypt to be rejected after revocation");
      } catch (error: unknown) {
        expect((error as { message: string }).message).contains(
          this.instances.bob.getDelegatedUserDecryptErrorMessage({
            contractAddress: this.tokenAddress,
            delegatorAddress: this.smartWalletAddress,
            handle: balanceHandle,
            signer: this.signers.bob,
            type: "revocation",
          }),
        );
      }
    });

    it("should reject when no delegation exists", async function () {
      const balanceHandle = await this.token.balanceOf(this.smartWalletAddress);

      try {
        await this.instances.dave.delegatedUserDecryptSingleHandle({
          handle: balanceHandle,
          contractAddress: this.tokenAddress,
          delegatorAddress: this.smartWalletAddress,
          signer: this.signers.dave,
        });
        expect.fail("Expected delegated user decrypt to be rejected without delegation");
      } catch (error: unknown) {
        expect((error as { message: string }).message).contains(
          this.instances.dave.getDelegatedUserDecryptErrorMessage({
            handle: balanceHandle,
            contractAddress: this.tokenAddress,
            delegatorAddress: this.smartWalletAddress,
            signer: this.signers.dave,
            type: "delegation-does-not-exist",
          }),
        );
      }
    });

    it("should reject when delegation is for wrong contract", async function () {
      const dummyFactory = await ethers.getContractFactory("UserDecrypt");
      const dummy = await dummyFactory.connect(this.signers.alice).deploy();
      await dummy.waitForDeployment();
      const wrongAddress = await dummy.getAddress();

      const expirationTimestamp = Math.floor(Date.now() / 1000) + ONE_DAY_SECONDS;
      const tx = await this.smartWallet
        .connect(this.signers.bob)
        .delegateUserDecryption(this.signers.eve.address, wrongAddress, expirationTimestamp);
      await tx.wait();
      const currentBlock = await ethers.provider.getBlockNumber();
      await waitForBlock(currentBlock + PROPAGATION_BLOCKS);

      const balanceHandle = await this.token.balanceOf(this.smartWalletAddress);

      try {
        await this.instances.eve.delegatedUserDecryptSingleHandle({
          handle: balanceHandle,
          contractAddress: this.tokenAddress,
          delegatorAddress: this.smartWalletAddress,
          signer: this.signers.eve,
        });
        expect.fail("Expected delegated user decrypt to be rejected for wrong contract");
      } catch (error: unknown) {
        //expect(relayerErrorLabel(error)).to.equal(NOT_ALLOWED_ON_HOST_ACL);
        expect((error as { message: string }).message).contains(
          this.instances.eve.getDelegatedUserDecryptErrorMessage({
            type: "contract-unauthorized",
            contractAddress: this.tokenAddress,
            delegatorAddress: this.smartWalletAddress,
            handle: balanceHandle,
            signer: this.signers.eve,
          }),
        );
      }
    });

    it("should reject when delegation has expired", async function () {
      const latestBlock = await ethers.provider.getBlock("latest");
      const expirationTimestamp = latestBlock!.timestamp + DELEGATION_EXPIRY_SECONDS;
      const tx = await this.smartWallet
        .connect(this.signers.bob)
        .delegateUserDecryption(this.signers.eve.address, this.tokenAddress, expirationTimestamp);
      await tx.wait();

      // Fast-forward blockchain time past the expiry instead of waiting in real time.
      await ethers.provider.send("evm_increaseTime", [DELEGATION_EXPIRY_SECONDS + 60]);
      await ethers.provider.send("evm_mine");
      await waitForDelegationExpiry(expirationTimestamp);

      const currentBlock = await ethers.provider.getBlockNumber();
      await waitForBlock(currentBlock + PROPAGATION_BLOCKS);

      const balanceHandle = await this.token.balanceOf(this.smartWalletAddress);

      try {
        await this.instances.eve.delegatedUserDecryptSingleHandle({
          handle: balanceHandle,
          contractAddress: this.tokenAddress,
          delegatorAddress: this.smartWalletAddress,
          signer: this.signers.eve,
        });
        expect.fail("Expected delegated user decrypt to be rejected for expired delegation");
      } catch (error: unknown) {
        expect((error as { message: string }).message).contains(
          this.instances.eve.getDelegatedUserDecryptErrorMessage({
            type: "contract-unauthorized",
            contractAddress: this.tokenAddress,
            delegatorAddress: this.smartWalletAddress,
            handle: balanceHandle,
            signer: this.signers.eve,
          }),
        );
      }
    });
  });

  describe("wildcard delegation", function () {
    // Wildcard delegation lets the delegator grant decryption rights for every
    // contract in one shot via the ACL's `WILDCARD_DELEGATION_ADDRESS()` sentinel.

    // Distinct handle values seeded into each fixture target so test sites
    // can refer to them by purpose rather than by raw numeral.
    const TARGET_B_VALUE = 424242n;
    const TARGET_C_VALUE = 777n;
    const TARGET_D_VALUE = 314159n;
    const ALICE_EXTRA_MINT = 123n;

    const ACL_WILDCARD_ABI = [
      "function WILDCARD_DELEGATION_ADDRESS() view returns (address)",
      "function delegateForUserDecryption(address delegate, address contractAddress, uint64 expirationDate)",
    ];

    const EXPECTED_WILDCARD_ADDRESS = ethers.getAddress(`0x${"f".repeat(40)}`);

    // Send an ACL-mutating tx (delegate / revoke), wait for the receipt, then
    // wait `PROPAGATION_BLOCKS` for the coprocessor to propagate the change.
    const txAndPropagate = async (call: () => Promise<ContractTransactionResponse>) => {
      const tx = await call();
      await tx.wait();
      await waitForBlock((await ethers.provider.getBlockNumber()) + PROPAGATION_BLOCKS);
    };

    // Expect a delegated user-decryption call to be rejected by the host ACL.
    const expectNotAllowed = async (call: () => Promise<unknown>, failMessage: string) => {
      try {
        await call();
        expect.fail(failMessage);
      } catch (error: unknown) {
        expect(relayerErrorLabel(error)).to.equal(NOT_ALLOWED_ON_HOST_ACL);
      }
    };

    before(async function () {
      // Read the wildcard sentinel from the deployed ACL once and reuse it
      // via `this.wildcardAddress` across scenarios.
      const acl = new ethers.Contract(aclAddress, ACL_WILDCARD_ABI, ethers.provider);
      this.wildcardAddress = await acl.WILDCARD_DELEGATION_ADDRESS();

      // Deploy a `WildcardDelegationTarget` so cross-contract coverage runs
      // against an address distinct from the outer `before`'s EncryptedERC20.
      const targetFactory = await ethers.getContractFactory("WildcardDelegationTarget");
      this.targetB = await targetFactory.connect(this.signers.alice).deploy();
      await this.targetB.waitForDeployment();
      this.targetBAddress = await this.targetB.getAddress();

      // Alice seeds the target with a handle owned (allow-granted) by the delegator.
      const encryptedForB = await this.instances.alice.encryptUint64({
        contractAddress: this.targetBAddress,
        userAddress: this.signers.alice.address,
        value: TARGET_B_VALUE,
      });

      await (
        await this.targetB
          .connect(this.signers.alice)
          .deposit(this.smartWalletAddress, encryptedForB.handles[0], encryptedForB.inputProof)
      ).wait();
      this.delegatorHandleOnB = await this.targetB.valueOf(this.smartWalletAddress);
    });

    it("exposes WILDCARD_DELEGATION_ADDRESS() at the expected sentinel value", async function () {
      expect(this.wildcardAddress).to.equal(EXPECTED_WILDCARD_ADDRESS);
    });

    describe("happy paths", function () {
      it("one wildcard delegation covers handles on two distinct contracts", async function () {
        const expirationTimestamp = Math.floor(Date.now() / 1000) + ONE_DAY_SECONDS;
        await txAndPropagate(() =>
          this.smartWallet
            .connect(this.signers.bob)
            .delegateUserDecryption(this.signers.bob.address, this.wildcardAddress, expirationTimestamp),
        );

        // Decrypt handle on the EncryptedERC20 token (A) — covered by wildcard.
        const balanceHandle = await this.token.balanceOf(this.smartWalletAddress);
        const balance = await this.instances.bob.delegatedUserDecryptSingleHandle({
          handle: balanceHandle,
          contractAddress: this.tokenAddress,
          delegatorAddress: this.smartWalletAddress,
          signer: this.signers.bob,
        });
        expect(balance).to.equal(SMART_WALLET_INITIAL_BALANCE);

        // Decrypt handle on the WildcardDelegationTarget (B) — also covered.
        const valueOnB = await this.instances.bob.delegatedUserDecryptSingleHandle({
          handle: this.delegatorHandleOnB,
          contractAddress: this.targetBAddress,
          delegatorAddress: this.smartWalletAddress,
          signer: this.signers.bob,
        });
        expect(valueOnB).to.equal(TARGET_B_VALUE);
      });

      it("wildcard covers contracts deployed after the delegation was set", async function () {
        const expirationTimestamp = Math.floor(Date.now() / 1000) + ONE_DAY_SECONDS;
        const tx = await this.smartWallet
          .connect(this.signers.bob)
          .delegateUserDecryption(this.signers.bob.address, this.wildcardAddress, expirationTimestamp);
        await tx.wait();

        // Deploy a brand-new app contract C, then seed it with a handle for the
        // smart wallet. The wildcard delegation must still cover the handle even
        // though contract C did not exist when the delegation was set.
        const targetFactory = await ethers.getContractFactory("WildcardDelegationTarget");
        const targetC = await targetFactory.connect(this.signers.alice).deploy();
        await targetC.waitForDeployment();
        const targetCAddress = await targetC.getAddress();

        const encryptedForC = await this.instances.alice.encryptUint64({
          contractAddress: targetCAddress,
          userAddress: this.signers.alice.address,
          value: TARGET_C_VALUE,
        });

        await (
          await targetC
            .connect(this.signers.alice)
            .deposit(this.smartWalletAddress, encryptedForC.handles[0], encryptedForC.inputProof)
        ).wait();
        const handleOnC = await targetC.valueOf(this.smartWalletAddress);
        await waitForBlock((await ethers.provider.getBlockNumber()) + PROPAGATION_BLOCKS);

        const value = await this.instances.bob.delegatedUserDecryptSingleHandle({
          handle: handleOnC,
          contractAddress: targetCAddress,
          delegatorAddress: this.smartWalletAddress,
          signer: this.signers.bob,
        });
        expect(value).to.equal(TARGET_C_VALUE);
      });

      it("wildcard and per-contract delegation can coexist for the same delegate", async function () {
        const expirationTimestamp = Math.floor(Date.now() / 1000) + ONE_DAY_SECONDS;
        await txAndPropagate(() =>
          this.smartWallet
            .connect(this.signers.bob)
            .delegateUserDecryption(this.signers.bob.address, this.tokenAddress, expirationTimestamp),
        );
        await txAndPropagate(() =>
          this.smartWallet
            .connect(this.signers.bob)
            .delegateUserDecryption(this.signers.bob.address, this.wildcardAddress, expirationTimestamp),
        );

        const balanceHandle = await this.token.balanceOf(this.smartWalletAddress);
        const balance = await this.instances.bob.delegatedUserDecryptSingleHandle({
          handle: balanceHandle,
          contractAddress: this.tokenAddress,
          signer: this.signers.bob,
          delegatorAddress: this.smartWalletAddress,
        });
        expect(balance).to.equal(SMART_WALLET_INITIAL_BALANCE);
      });
    });

    describe("negative paths", function () {
      it("rejects after the wildcard delegation expires", async function () {
        this.timeout(SLOW_TEST_TIMEOUT_MS);
        const latestBlock = await ethers.provider.getBlock("latest");
        const expirationTimestamp = latestBlock!.timestamp + DELEGATION_EXPIRY_SECONDS;
        const tx = await this.smartWallet
          .connect(this.signers.bob)
          .delegateUserDecryption(this.signers.eve.address, this.wildcardAddress, expirationTimestamp);
        await tx.wait();

        // Fast-forward blockchain time past the expiry instead of waiting in real time.
        await ethers.provider.send("evm_increaseTime", [DELEGATION_EXPIRY_SECONDS + 60]);
        await ethers.provider.send("evm_mine");
        await waitForDelegationExpiry(expirationTimestamp);
        await waitForBlock((await ethers.provider.getBlockNumber()) + PROPAGATION_BLOCKS);

        await expectNotAllowed(
          () =>
            this.instances.eve.delegatedUserDecryptSingleHandle({
              handle: this.delegatorHandleOnB,
              contractAddress: this.targetBAddress,
              delegatorAddress: this.smartWalletAddress,
              signer: this.signers.eve,
            }),
          "Expected delegated user decrypt to be rejected after wildcard expiry",
        );
      });

      it("rejects when the requesting EOA is not the registered wildcard delegate", async function () {
        // The wildcard is granted to Eve. Carol — a different EOA — tries
        // to use it. Delegations are recorded per (delegator, delegate)
        // pair, so a wildcard issued to Eve does not authorize Carol.
        const expirationTimestamp = Math.floor(Date.now() / 1000) + ONE_DAY_SECONDS;
        await txAndPropagate(() =>
          this.smartWallet
            .connect(this.signers.bob)
            .delegateUserDecryption(this.signers.eve.address, this.wildcardAddress, expirationTimestamp),
        );

        const unauthorizedDelegate = this.signers.carol;
        await expectNotAllowed(
          () =>
            this.instances.carol.delegatedUserDecryptSingleHandle({
              handle: this.delegatorHandleOnB,
              contractAddress: this.targetBAddress,
              delegatorAddress: this.smartWalletAddress,
              signer: unauthorizedDelegate,
            }),
          "Expected delegated user decrypt to be rejected for non-delegate caller",
        );
      });

      it("does not bypass ownership: rejects when the delegator is not allowed on the handle", async function () {
        // The smart wallet holds a wildcard delegation to Bob. Alice then
        // mints a fresh handle she keeps for herself (no transfer), so the
        // handle is owned by Alice — not the smart wallet. Wildcard
        // delegation does not bypass the requirement that the delegator
        // actually owns the handle.
        const expirationTimestamp = Math.floor(Date.now() / 1000) + ONE_DAY_SECONDS;
        const tx = await this.smartWallet
          .connect(this.signers.bob)
          .delegateUserDecryption(this.signers.bob.address, this.wildcardAddress, expirationTimestamp);
        await tx.wait();

        await (await this.token.connect(this.signers.alice).mint(ALICE_EXTRA_MINT)).wait();
        const aliceOnlyHandle = await this.token.balanceOf(this.signers.alice.address);
        await waitForBlock((await ethers.provider.getBlockNumber()) + PROPAGATION_BLOCKS);

        await expectNotAllowed(
          () =>
            this.instances.bob.delegatedUserDecryptSingleHandle({
              handle: aliceOnlyHandle,
              contractAddress: this.tokenAddress,
              delegatorAddress: this.smartWalletAddress,
              signer: this.signers.bob,
            }),
          "Expected rejection: smart wallet is not allowed on Alice-owned handle",
        );
      });

      it("does not bypass ownership: rejects when the app contract is not allowed on the handle", async function () {
        // The smart wallet wildcard-delegates to Bob. Bob signs a request
        // claiming the token-balance handle lives on `targetB`, but the
        // handle was issued by the EncryptedERC20 token — `targetB` never
        // had access to it. Wildcard authorizes the delegate, not the app
        // contract's access to the handle.
        const expirationTimestamp = Math.floor(Date.now() / 1000) + ONE_DAY_SECONDS;
        await txAndPropagate(() =>
          this.smartWallet
            .connect(this.signers.bob)
            .delegateUserDecryption(this.signers.bob.address, this.wildcardAddress, expirationTimestamp),
        );

        const tokenBalanceHandle = await this.token.balanceOf(this.smartWalletAddress);
        const appContractWithoutAccess = this.targetBAddress;
        await expectNotAllowed(
          () =>
            this.instances.bob.delegatedUserDecryptSingleHandle({
              handle: tokenBalanceHandle,
              contractAddress: appContractWithoutAccess,
              delegatorAddress: this.smartWalletAddress,
              signer: this.signers.bob,
            }),
          "Expected rejection: targetB is not allowed on the token-balance handle",
        );
      });

      it("does not allow transitive delegation: a wildcard recipient cannot re-grant onward", async function () {
        // Bob's smart wallet wildcard-delegates to Carol. Carol then tries to
        // wildcard-delegate to Dave from her own EOA via a direct ACL call.
        // The (Carol, Dave) entry has no bearing on handles owned by the smart
        // wallet — Dave's request as delegate of the smart wallet must still be
        // rejected.
        const expirationTimestamp = Math.floor(Date.now() / 1000) + ONE_DAY_SECONDS;
        await txAndPropagate(() =>
          this.smartWallet
            .connect(this.signers.bob)
            .delegateUserDecryption(this.signers.carol.address, this.wildcardAddress, expirationTimestamp),
        );

        const acl = new ethers.Contract(aclAddress, ACL_WILDCARD_ABI, this.signers.carol);
        await txAndPropagate(() =>
          acl.delegateForUserDecryption(this.signers.dave.address, this.wildcardAddress, BigInt(expirationTimestamp)),
        );

        await expectNotAllowed(
          () =>
            this.instances.dave.delegatedUserDecryptSingleHandle({
              handle: this.delegatorHandleOnB,
              contractAddress: this.targetBAddress,
              delegatorAddress: this.smartWalletAddress,
              signer: this.signers.dave,
            }),
          "Expected rejection: Carol's onward delegation must not grant Dave access to smart-wallet handles",
        );
      });
    });

    describe("revocation matrix", function () {
      it("revoking the wildcard leaves the per-contract entry active", async function () {
        this.timeout(SLOW_TEST_TIMEOUT_MS);
        const expirationTimestamp = Math.floor(Date.now() / 1000) + ONE_DAY_SECONDS;
        await txAndPropagate(() =>
          this.smartWallet
            .connect(this.signers.bob)
            .delegateUserDecryption(this.signers.carol.address, this.wildcardAddress, expirationTimestamp),
        );
        await txAndPropagate(() =>
          this.smartWallet
            .connect(this.signers.bob)
            .delegateUserDecryption(this.signers.carol.address, this.tokenAddress, expirationTimestamp),
        );
        await txAndPropagate(() =>
          this.smartWallet
            .connect(this.signers.bob)
            .revokeUserDecryptionDelegation(this.signers.carol.address, this.wildcardAddress),
        );

        // appA still works via the surviving per-contract entry.
        const balanceHandle = await this.token.balanceOf(this.smartWalletAddress);
        const balance = await this.instances.carol.delegatedUserDecryptSingleHandle({
          handle: balanceHandle,
          contractAddress: this.tokenAddress,
          delegatorAddress: this.smartWalletAddress,
          signer: this.signers.carol,
        });
        expect(balance).to.equal(SMART_WALLET_INITIAL_BALANCE);

        // appB rejects — wildcard is gone and there is no per-contract entry for B.
        await expectNotAllowed(
          () =>
            this.instances.carol.delegatedUserDecryptSingleHandle({
              handle: this.delegatorHandleOnB,
              contractAddress: this.targetBAddress,
              delegatorAddress: this.smartWalletAddress,
              signer: this.signers.carol,
            }),
          "Expected rejection on appB after wildcard revocation",
        );
      });

      it("revoking the per-contract entry leaves the wildcard active", async function () {
        const expirationTimestamp = Math.floor(Date.now() / 1000) + ONE_DAY_SECONDS;
        await txAndPropagate(() =>
          this.smartWallet
            .connect(this.signers.bob)
            .delegateUserDecryption(this.signers.carol.address, this.wildcardAddress, expirationTimestamp),
        );
        await txAndPropagate(() =>
          this.smartWallet
            .connect(this.signers.bob)
            .delegateUserDecryption(this.signers.carol.address, this.tokenAddress, expirationTimestamp),
        );
        await txAndPropagate(() =>
          this.smartWallet
            .connect(this.signers.bob)
            .revokeUserDecryptionDelegation(this.signers.carol.address, this.tokenAddress),
        );

        // Both A and B continue to work via the surviving wildcard.
        const balanceHandle = await this.token.balanceOf(this.smartWalletAddress);
        const balance = await this.instances.carol.delegatedUserDecryptSingleHandle({
          handle: balanceHandle,
          contractAddress: this.tokenAddress,
          delegatorAddress: this.smartWalletAddress,
          signer: this.signers.carol,
        });
        expect(balance).to.equal(SMART_WALLET_INITIAL_BALANCE);

        const valueOnB = await this.instances.carol.delegatedUserDecryptSingleHandle({
          handle: this.delegatorHandleOnB,
          contractAddress: this.targetBAddress,
          delegatorAddress: this.smartWalletAddress,
          signer: this.signers.carol,
        });
        expect(valueOnB).to.equal(TARGET_B_VALUE);
      });

      it("revoking both entries rejects on every contract", async function () {
        this.timeout(SLOW_TEST_TIMEOUT_MS);
        const expirationTimestamp = Math.floor(Date.now() / 1000) + ONE_DAY_SECONDS;
        await txAndPropagate(() =>
          this.smartWallet
            .connect(this.signers.bob)
            .delegateUserDecryption(this.signers.carol.address, this.wildcardAddress, expirationTimestamp),
        );
        await txAndPropagate(() =>
          this.smartWallet
            .connect(this.signers.bob)
            .delegateUserDecryption(this.signers.carol.address, this.tokenAddress, expirationTimestamp),
        );
        await txAndPropagate(() =>
          this.smartWallet
            .connect(this.signers.bob)
            .revokeUserDecryptionDelegation(this.signers.carol.address, this.wildcardAddress),
        );
        await txAndPropagate(() =>
          this.smartWallet
            .connect(this.signers.bob)
            .revokeUserDecryptionDelegation(this.signers.carol.address, this.tokenAddress),
        );

        const balanceHandle = await this.token.balanceOf(this.smartWalletAddress);
        await expectNotAllowed(
          () =>
            this.instances.carol.delegatedUserDecryptSingleHandle({
              handle: balanceHandle,
              contractAddress: this.tokenAddress,
              delegatorAddress: this.smartWalletAddress,
              signer: this.signers.carol,
            }),
          "Expected rejection on appA after revoking both entries",
        );

        await expectNotAllowed(
          () =>
            this.instances.carol.delegatedUserDecryptSingleHandle({
              handle: this.delegatorHandleOnB,
              contractAddress: this.targetBAddress,
              delegatorAddress: this.smartWalletAddress,
              signer: this.signers.carol,
            }),
          "Expected rejection on appB after revoking both entries",
        );
      });
    });

    describe("independence", function () {
      it("revoking one delegator's wildcard does not affect another delegator's", async function () {
        this.timeout(SLOW_TEST_TIMEOUT_MS);

        // Stand up a second smart wallet (Y) owned by Dave, with its own handle
        // on a fresh app contract. Both wallets wildcard-delegate to Carol.
        // Revoking Bob's wildcard must leave Dave's wildcard untouched.
        const smartWalletYFactory = await ethers.getContractFactory("SmartWalletWithDelegation");
        const smartWalletY = await smartWalletYFactory.connect(this.signers.dave).deploy(this.signers.dave.address);
        await smartWalletY.waitForDeployment();
        const smartWalletYAddress = await smartWalletY.getAddress();

        const targetFactory = await ethers.getContractFactory("WildcardDelegationTarget");
        const targetD = await targetFactory.connect(this.signers.alice).deploy();
        await targetD.waitForDeployment();
        const targetDAddress = await targetD.getAddress();

        const encryptedForD = await this.instances.alice.encryptUint64({
          contractAddress: targetDAddress,
          userAddress: this.signers.alice.address,
          value: TARGET_D_VALUE,
        });

        await (
          await targetD
            .connect(this.signers.alice)
            .deposit(smartWalletYAddress, encryptedForD.handles[0], encryptedForD.inputProof)
        ).wait();
        const handleForY = await targetD.valueOf(smartWalletYAddress);

        const expirationTimestamp = Math.floor(Date.now() / 1000) + ONE_DAY_SECONDS;
        await txAndPropagate(() =>
          this.smartWallet
            .connect(this.signers.bob)
            .delegateUserDecryption(this.signers.carol.address, this.wildcardAddress, expirationTimestamp),
        );
        await txAndPropagate(() =>
          smartWalletY
            .connect(this.signers.dave)
            .delegateUserDecryption(this.signers.carol.address, this.wildcardAddress, expirationTimestamp),
        );
        await txAndPropagate(() =>
          this.smartWallet
            .connect(this.signers.bob)
            .revokeUserDecryptionDelegation(this.signers.carol.address, this.wildcardAddress),
        );

        // Y's wildcard is untouched: Carol can still decrypt Y's handle.
        const valueOnD = await this.instances.carol.delegatedUserDecryptSingleHandle({
          handle: handleForY,
          contractAddress: targetDAddress,
          delegatorAddress: smartWalletYAddress,
          signer: this.signers.carol,
        });
        expect(valueOnD).to.equal(TARGET_D_VALUE);
      });
    });
  });
});
