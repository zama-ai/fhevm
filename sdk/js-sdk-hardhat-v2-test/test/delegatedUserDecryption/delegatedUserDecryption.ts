import { expect } from 'chai';
import { ethers, network } from 'hardhat';

import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';
import { delegatedUserDecryptSingleHandle, waitForBlock } from '../utils';
import { Contract } from 'ethers';
import type { Signer } from 'ethers';

const NOT_ALLOWED_ON_HOST_ACL = 'not_allowed_on_host_acl';
const DELEGATION_EXPIRY_SECONDS = network.name === 'localhost' ? 5 : 75;
const DELEGATION_EXPIRY_POLL_MS = 2_000;

const relayerErrorLabel = (error: unknown): string | undefined => {
  if (typeof error !== 'object' || error === null || !('relayerApiError' in error)) {
    return undefined;
  }
  return (error as { relayerApiError?: { label?: string } }).relayerApiError?.label;
};

const DELEGATION_EXPIRY_BLOCKS_PER_POLL = 3;

const waitForDelegationExpiry = async (expirationTimestamp: number) => {
  while (true) {
    const latestBlock = await ethers.provider.getBlock('latest');
    console.log(`latestBlock?.timestamp=${latestBlock?.timestamp} : ${expirationTimestamp}`);
    if (latestBlock && latestBlock.timestamp > expirationTimestamp) {
      return;
    }
    if (network.name === 'localhost') {
      // External node (anvil/hardhat-node) won't progress on its own when idle;
      // mine a few blocks so the next `latestBlock.timestamp` reflects current
      // wall time before re-polling.
      for (let i = 0; i < DELEGATION_EXPIRY_BLOCKS_PER_POLL; i++) {
        await ethers.provider.send('evm_mine', []);
      }
    }
    await new Promise((resolve) => setTimeout(resolve, DELEGATION_EXPIRY_POLL_MS));
  }
};

async function safeDelegateUserDecryption(parameters: {
  smartWallet: Contract;
  signer: Signer;
  delegate: string;
  contractAddress: string;
  expirationTimestamp?: number | undefined;
}) {
  const { smartWallet, signer, contractAddress, expirationTimestamp, delegate } = parameters;

  const expirationDate = expirationTimestamp ?? Math.floor(Date.now() / 1000) + 86400;

  const currentExpirationDate = await smartWallet.getDelegatedUserDecryptionExpirationDate(delegate, contractAddress);
  if (currentExpirationDate === BigInt(expirationDate)) {
    console.log("Skip call 'delegateUserDecryption': same expiration date");
    return;
  }

  const tx = await (smartWallet.connect(signer) as Contract).delegateUserDecryption(
    delegate,
    contractAddress,
    expirationDate,
  );
  await tx.wait();
}

describe('Delegated user decryption', function () {
  before(async function () {
    await initSigners(5);
    this.signers = await getSigners();
    this.instances = await createInstances(this.signers);

    // Deploy the EncryptedERC20 token contract.
    const tokenFactory = await ethers.getContractFactory('EncryptedERC20');
    this.token = await tokenFactory.connect(this.signers.alice).deploy('Zama Confidential Token', 'ZAMA');
    await this.token.waitForDeployment();
    this.tokenAddress = await this.token.getAddress();

    // Deploy SmartWalletWithDelegation with Bob as the owner.
    const smartWalletFactory = await ethers.getContractFactory('SmartWalletWithDelegation');
    this.smartWallet = await smartWalletFactory.connect(this.signers.bob).deploy(this.signers.bob.address);
    await this.smartWallet.waitForDeployment();
    this.smartWalletAddress = await this.smartWallet.getAddress();

    // Alice mints tokens to herself.
    const mintAmount = 1000000n;
    const mintTx = await this.token.connect(this.signers.alice).mint(mintAmount);
    await mintTx.wait();

    // Alice transfers some tokens to the smartWallet contract.
    const transferAmount = 500000n;
    const input = this.instances.alice.createEncryptedInput(this.tokenAddress, this.signers.alice.address);
    input.add64(transferAmount);
    const encryptedTransferAmount = await input.encrypt();

    const transferTx = await this.token
      .connect(this.signers.alice)
      [
        'transfer(address,bytes32,bytes)'
      ](this.smartWalletAddress, encryptedTransferAmount.handles[0], encryptedTransferAmount.inputProof);
    await transferTx.wait();
  });

  it('test delegated user decryption - smartWallet owner delegates his own EOA to decrypt the smartWallet balance', async function () {
    // Bob (smartWallet owner) delegates decryption rights to his own EOA.
    const expirationTimestamp = Math.floor(Date.now() / 1000) + 86400; // 24 hours from now
    await safeDelegateUserDecryption({
      signer: this.signers.bob,
      delegate: this.signers.bob.address,
      smartWallet: this.smartWallet,
      contractAddress: this.tokenAddress,
      expirationTimestamp,
    });
    // const delegateTx = await this.smartWallet
    //   .connect(this.signers.bob)
    //   .delegateUserDecryption(this.signers.bob.address, this.tokenAddress, expirationTimestamp);
    // await delegateTx.wait();

    // Wait for 15 blocks to ensure delegation is propagated by the coprocessor.
    const currentBlock = await ethers.provider.getBlockNumber();
    await waitForBlock(currentBlock + 15);

    // Get the encrypted balance handle of the smartWallet.
    const balanceHandle = await this.token.balanceOf(this.smartWalletAddress);

    // Bob's EOA can now decrypt the smartWallet's confidential balance.
    const { publicKey, privateKey } = await this.instances.bob.generateKeypairAsync();

    const decryptedBalance = await delegatedUserDecryptSingleHandle(
      this.instances.bob,
      balanceHandle,
      this.tokenAddress,
      this.smartWalletAddress,
      this.signers.bob.address,
      this.signers.bob,
      privateKey,
      publicKey,
    );

    // Verify the decrypted balance matches what was transferred.
    expect(decryptedBalance).to.equal(500000n);
  });

  it('test delegated user decryption - smartWallet owner delegates a third EOA to decrypt the smartWallet balance', async function () {
    // Bob (smartWallet owner) delegates decryption rights to Carol's EOA.
    const expirationTimestamp = Math.floor(Date.now() / 1000) + 86400; // 24 hours from now
    await safeDelegateUserDecryption({
      smartWallet: this.smartWallet,
      signer: this.signers.bob,
      delegate: this.signers.carol.address,
      contractAddress: this.tokenAddress,
      expirationTimestamp,
    });
    // const delegateTx = await this.smartWallet
    //   .connect(this.signers.bob)
    //   .delegateUserDecryption(this.signers.carol.address, this.tokenAddress, expirationTimestamp);
    // await delegateTx.wait();

    // Wait for 15 blocks to ensure delegation is propagated by the coprocessor.
    const currentBlock = await ethers.provider.getBlockNumber();
    await waitForBlock(currentBlock + 15);

    // Get the encrypted balance handle of the smartWallet.
    const balanceHandle = await this.token.balanceOf(this.smartWalletAddress);

    // Carol's EOA can now decrypt the smartWallet's confidential balance.
    const { publicKey, privateKey } = await this.instances.carol.generateKeypairAsync();

    const decryptedBalance = await delegatedUserDecryptSingleHandle(
      this.instances.carol,
      balanceHandle,
      this.tokenAddress,
      this.smartWalletAddress,
      this.signers.carol.address,
      this.signers.carol,
      privateKey,
      publicKey,
    );

    // Verify the decrypted balance matches what was transferred.
    expect(decryptedBalance).to.equal(500000n);
  });

  it('test delegated user decryption - smartWallet can execute transference of funds to a third EOA', async function () {
    // First, Bob needs to delegate so the smartWallet can initiate transfers.
    const expirationTimestamp = Math.floor(Date.now() / 1000) + 86400;
    await safeDelegateUserDecryption({
      smartWallet: this.smartWallet,
      signer: this.signers.bob,
      delegate: this.signers.bob.address,
      contractAddress: this.tokenAddress,
      expirationTimestamp,
    });
    // const delegateTx = await this.smartWallet
    //   .connect(this.signers.bob)
    //   .delegateUserDecryption(this.signers.bob.address, this.tokenAddress, expirationTimestamp);
    // await delegateTx.wait();

    // Wait for 15 blocks to ensure delegation is propagated by the coprocessor.
    let currentBlock = await ethers.provider.getBlockNumber();
    await waitForBlock(currentBlock + 15);

    // Get the current smartWallet balance before transfer
    const smartWalletBalanceBefore = await this.token.balanceOf(this.smartWalletAddress);
    const { publicKey: pkBefore, privateKey: skBefore } = await this.instances.bob.generateKeypairAsync();
    const decryptedBalanceBefore = await delegatedUserDecryptSingleHandle(
      this.instances.bob,
      smartWalletBalanceBefore,
      this.tokenAddress,
      this.smartWalletAddress,
      this.signers.bob.address,
      this.signers.bob,
      skBefore,
      pkBefore,
    );

    // Bob proposes a transaction from the smartWallet to transfer tokens to Carol.
    // The encrypted input must be created for the smartWallet address since it will be the msg.sender.
    const transferAmount = 100000n;
    const input = this.instances.bob.createEncryptedInput(this.tokenAddress, this.smartWalletAddress);
    input.add64(transferAmount);
    const encryptedTransferAmount = await input.encrypt();

    // Encode the transfer function call with full signature to avoid ambiguity.
    const transferData = this.token.interface.encodeFunctionData('transfer(address,bytes32,bytes)', [
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
    const { publicKey: pkAfter, privateKey: skAfter } = await this.instances.bob.generateKeypairAsync();
    const decryptedBalanceAfter = await delegatedUserDecryptSingleHandle(
      this.instances.bob,
      smartWalletBalanceAfter,
      this.tokenAddress,
      this.smartWalletAddress,
      this.signers.bob.address,
      this.signers.bob,
      skAfter,
      pkAfter,
    );

    // The smartWallet balance should have decreased by the transfer amount.
    expect(Number(decryptedBalanceBefore) - Number(decryptedBalanceAfter)).to.equal(Number(transferAmount));
  });

  describe('negative-acl', function () {
    it('should reject when delegation has been revoked', async function () {
      // 10min — observed ~6m37s due to two 15-block waits for delegation propagation in sepolia
      this.timeout(600000);
      // First, ensure Bob has delegation.
      const expirationTimestamp = Math.floor(Date.now() / 1000) + 86400;
      await safeDelegateUserDecryption({
        smartWallet: this.smartWallet,
        signer: this.signers.bob,
        delegate: this.signers.bob.address,
        contractAddress: this.tokenAddress,
        expirationTimestamp,
      });
      // const delegateTx = await this.smartWallet
      //   .connect(this.signers.bob)
      //   .delegateUserDecryption(this.signers.bob.address, this.tokenAddress, expirationTimestamp);
      // await delegateTx.wait();

      // Wait for 15 blocks to ensure delegation is propagated by the coprocessor.
      const currentBlock1 = await ethers.provider.getBlockNumber();
      await waitForBlock(currentBlock1 + 15);

      // Revoke the delegation for Bob's EOA.
      const revokeTx = await this.smartWallet
        .connect(this.signers.bob)
        .revokeUserDecryptionDelegation(this.signers.bob.address, this.tokenAddress);
      await revokeTx.wait();

      // Wait for 15 blocks to ensure revocation is propagated by the coprocessor.
      const currentBlock2 = await ethers.provider.getBlockNumber();
      await waitForBlock(currentBlock2 + 15);

      // Try to decrypt the smartWallet balance with Bob's EOA, which should now fail.
      const balanceHandle = await this.token.balanceOf(this.smartWalletAddress);
      const { publicKey, privateKey } = await this.instances.bob.generateKeypairAsync();

      try {
        await delegatedUserDecryptSingleHandle(
          this.instances.bob,
          balanceHandle,
          this.tokenAddress,
          this.smartWalletAddress, //delegator
          this.signers.bob.address, //delegate
          this.signers.bob,
          privateKey,
          publicKey,
        );
        expect.fail('Expected delegated user decrypt to be rejected after revocation');
      } catch (error: unknown) {
        expect((error as { message: string }).message).contains(
          `Delegate ${this.signers.bob.address} is not delegated by ${this.smartWalletAddress} to user decrypt handle ${balanceHandle} on contract ${this.tokenAddress}`,
        );
      }
    });

    it('should reject when no delegation exists', async function () {
      const balanceHandle = await this.token.balanceOf(this.smartWalletAddress);
      const { publicKey, privateKey } = await this.instances.dave.generateKeypairAsync();

      try {
        await delegatedUserDecryptSingleHandle(
          this.instances.dave,
          balanceHandle,
          this.tokenAddress,
          this.smartWalletAddress, // delegator
          this.signers.dave.address, // delegate
          this.signers.dave,
          privateKey,
          publicKey,
        );
        expect.fail('Expected delegated user decrypt to be rejected without delegation');
      } catch (error: unknown) {
        console.log('=================== A2 ======================');
        console.log((error as { message: string }).message);
        console.log('=========================================');
        expect((error as { message: string }).message).contains(
          `Delegate ${this.signers.dave.address} is not delegated by ${this.smartWalletAddress} to user decrypt handle ${balanceHandle} on contract ${this.tokenAddress}`,
        );
      }
    });

    it('should reject when delegation is for wrong contract', async function () {
      const dummyFactory = await ethers.getContractFactory('UserDecrypt');
      const dummy = await dummyFactory.connect(this.signers.alice).deploy();
      await dummy.waitForDeployment();
      const wrongAddress = await dummy.getAddress();

      const expirationTimestamp = Math.floor(Date.now() / 1000) + 86400;
      await safeDelegateUserDecryption({
        smartWallet: this.smartWallet,
        signer: this.signers.bob,
        delegate: this.signers.eve.address,
        contractAddress: wrongAddress,
        expirationTimestamp,
      });
      // const tx = await this.smartWallet
      //   .connect(this.signers.bob)
      //   .delegateUserDecryption(this.signers.eve.address, wrongAddress, expirationTimestamp);
      // await tx.wait();
      const currentBlock = await ethers.provider.getBlockNumber();
      await waitForBlock(currentBlock + 15);

      const balanceHandle = await this.token.balanceOf(this.smartWalletAddress);
      const { publicKey, privateKey } = await this.instances.eve.generateKeypairAsync();

      try {
        await delegatedUserDecryptSingleHandle(
          this.instances.eve,
          balanceHandle,
          this.tokenAddress,
          this.smartWalletAddress, // delegator
          this.signers.eve.address, // delegate
          this.signers.eve, //signer
          privateKey,
          publicKey,
        );
        expect.fail('Expected delegated user decrypt to be rejected for wrong contract');
      } catch (error: unknown) {
        expect((error as { message: string }).message).contains(
          `Delegate ${this.signers.eve.address} is not delegated by ${this.smartWalletAddress} to user decrypt handle ${balanceHandle} on contract ${this.tokenAddress}`,
        );
      }
    });

    it('should reject when delegation has expired', async function () {
      const latestBlock = await ethers.provider.getBlock('latest');
      const expirationTimestamp = latestBlock!.timestamp + DELEGATION_EXPIRY_SECONDS;
      // const tx = await this.smartWallet
      //   .connect(this.signers.bob)
      //   .delegateUserDecryption(this.signers.eve.address, this.tokenAddress, expirationTimestamp);
      // await tx.wait();
      await safeDelegateUserDecryption({
        smartWallet: this.smartWallet,
        signer: this.signers.bob,
        delegate: this.signers.eve.address,
        contractAddress: this.tokenAddress,
        expirationTimestamp,
      });

      await waitForDelegationExpiry(expirationTimestamp);

      const currentBlock = await ethers.provider.getBlockNumber();
      await waitForBlock(currentBlock + 15);

      const balanceHandle = await this.token.balanceOf(this.smartWalletAddress);
      const { publicKey, privateKey } = await this.instances.eve.generateKeypairAsync();

      try {
        await delegatedUserDecryptSingleHandle(
          this.instances.eve,
          balanceHandle,
          this.tokenAddress,
          this.smartWalletAddress,
          this.signers.eve.address,
          this.signers.eve,
          privateKey,
          publicKey,
        );
        expect.fail('Expected delegated user decrypt to be rejected for expired delegation');
      } catch (error: unknown) {
        expect((error as { message: string }).message).contains(
          `Delegate ${this.signers.eve.address} is not delegated by ${this.smartWalletAddress} to user decrypt handle ${balanceHandle} on contract ${this.tokenAddress}`,
        );
      }
    });
  });
});
