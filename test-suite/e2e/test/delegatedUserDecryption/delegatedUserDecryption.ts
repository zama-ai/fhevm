import { expect } from 'chai';
import { ethers } from 'hardhat';

import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';
import { delegatedUserDecryptSingleHandle, waitForBlock } from '../utils';

const USER_DECRYPTION_NOT_DELEGATED_SELECTOR = '0x0190c506';

describe('Delegated user decryption', function () {
  before(async function () {
    await initSigners(3);
    this.signers = await getSigners();
    this.instances = await createInstances(this.signers);

    // Deploy the EncryptedERC20 token contract.
    const tokenFactory = await ethers.getContractFactory('EncryptedERC20');
    this.token = await tokenFactory.connect(this.signers.alice).deploy('Zama Confidential Token', 'ZAMA');
    await this.token.waitForDeployment();
    this.tokenAddress = await this.token.getAddress();

    // Deploy SimpleMultiSigWithDelegation with Bob as the owner.
    const multisigFactory = await ethers.getContractFactory('SimpleMultiSigWithDelegation');
    this.multisig = await multisigFactory.connect(this.signers.bob).deploy(this.signers.bob.address);
    await this.multisig.waitForDeployment();
    this.multisigAddress = await this.multisig.getAddress();

    // Alice mints tokens to herself.
    const mintAmount = 1000000n;
    const mintTx = await this.token.connect(this.signers.alice).mint(mintAmount);
    await mintTx.wait();

    // Alice transfers some tokens to the multisig contract.
    const transferAmount = 500000n;
    const input = this.instances.alice.createEncryptedInput(this.tokenAddress, this.signers.alice.address);
    input.add64(transferAmount);
    const encryptedTransferAmount = await input.encrypt();

    const transferTx = await this.token
      .connect(this.signers.alice)
    ['transfer(address,bytes32,bytes)'](
      this.multisigAddress,
      encryptedTransferAmount.handles[0],
      encryptedTransferAmount.inputProof,
    );
    await transferTx.wait();
  });

  it('test delegated user decryption - multisig owner delegates his own EOA to decrypt the multisig balance', async function () {
    // Bob (multisig owner) delegates decryption rights to his own EOA.
    const expirationTimestamp = Math.floor(Date.now() / 1000) + 86400; // 24 hours from now
    const delegateTx = await this.multisig
      .connect(this.signers.bob)
      .delegateUserDecryption(
        this.signers.bob.address,
        this.tokenAddress,
        expirationTimestamp,
      );
    await delegateTx.wait();

    // Wait for 15 blocks to ensure delegation is propagated by the coprocessor.
    const currentBlock = await ethers.provider.getBlockNumber();
    await waitForBlock(currentBlock + 15);

    // Get the encrypted balance handle of the multisig.
    const balanceHandle = await this.token.balanceOf(this.multisigAddress);

    // Bob's EOA can now decrypt the multisig's confidential balance.
    const { publicKey, privateKey } = this.instances.bob.generateKeypair();

    const decryptedBalance = await delegatedUserDecryptSingleHandle(
      this.instances.bob,
      balanceHandle,
      this.tokenAddress,
      this.multisigAddress,
      this.signers.bob.address,
      this.signers.bob,
      privateKey,
      publicKey,
    );

    // Verify the decrypted balance matches what was transferred.
    expect(decryptedBalance).to.equal(500000n);
  });

  it('test delegated user decryption - multisig owner delegates a third EOA to decrypt the multisig balance', async function () {
    // Bob (multisig owner) delegates decryption rights to Carol's EOA.
    const expirationTimestamp = Math.floor(Date.now() / 1000) + 86400; // 24 hours from now
    const delegateTx = await this.multisig
      .connect(this.signers.bob)
      .delegateUserDecryption(
        this.signers.carol.address,
        this.tokenAddress,
        expirationTimestamp,
      );
    await delegateTx.wait();

    // Wait for 15 blocks to ensure delegation is propagated by the coprocessor.
    const currentBlock = await ethers.provider.getBlockNumber();
    await waitForBlock(currentBlock + 15);

    // Get the encrypted balance handle of the multisig.
    const balanceHandle = await this.token.balanceOf(this.multisigAddress);

    // Carol's EOA can now decrypt the multisig's confidential balance.
    const { publicKey, privateKey } = this.instances.carol.generateKeypair();

    const decryptedBalance = await delegatedUserDecryptSingleHandle(
      this.instances.carol,
      balanceHandle,
      this.tokenAddress,
      this.multisigAddress,
      this.signers.carol.address,
      this.signers.carol,
      privateKey,
      publicKey,
    );

    // Verify the decrypted balance matches what was transferred.
    expect(decryptedBalance).to.equal(500000n);
  });

  it('test delegated user decryption - multisig can execute transference of funds to a third EOA', async function () {
    // First, Bob needs to delegate so the multisig can initiate transfers.
    const expirationTimestamp = Math.floor(Date.now() / 1000) + 86400;
    const delegateTx = await this.multisig
      .connect(this.signers.bob)
      .delegateUserDecryption(
        this.signers.bob.address,
        this.tokenAddress,
        expirationTimestamp,
      );
    await delegateTx.wait();

    // Wait for 15 blocks to ensure delegation is propagated by the coprocessor.
    let currentBlock = await ethers.provider.getBlockNumber();
    await waitForBlock(currentBlock + 15);

    // Get the current multisig balance before transfer
    const multisigBalanceBefore = await this.token.balanceOf(this.multisigAddress);
    const { publicKey: pkBefore, privateKey: skBefore } = this.instances.bob.generateKeypair();
    const decryptedBalanceBefore = await delegatedUserDecryptSingleHandle(
      this.instances.bob,
      multisigBalanceBefore,
      this.tokenAddress,
      this.multisigAddress,
      this.signers.bob.address,
      this.signers.bob,
      skBefore,
      pkBefore,
    );

    // Bob proposes a transaction from the multisig to transfer tokens to Carol.
    // The encrypted input must be created for the multisig address since it will be the msg.sender.
    const transferAmount = 100000n;
    const input = this.instances.bob.createEncryptedInput(this.tokenAddress, this.multisigAddress);
    input.add64(transferAmount);
    const encryptedTransferAmount = await input.encrypt();

    // Encode the transfer function call with full signature to avoid ambiguity.
    const transferData = this.token.interface.encodeFunctionData(
      'transfer(address,bytes32,bytes)',
      [
        this.signers.carol.address,
        encryptedTransferAmount.handles[0],
        encryptedTransferAmount.inputProof,
      ]
    );

    // Propose the transaction.
    const proposeTx = await this.multisig
      .connect(this.signers.bob)
      .proposeTx(this.tokenAddress, transferData);
    await proposeTx.wait();

    // Get the transaction ID.
    const txId = await this.multisig.txCounter();

    // Execute the transaction.
    const executeTx = await this.multisig
      .connect(this.signers.bob)
      .executeTx(txId);
    await executeTx.wait();

    // Verify the multisig balance decreased.
    const multisigBalanceAfter = await this.token.balanceOf(this.multisigAddress);
    const { publicKey: pkAfter, privateKey: skAfter } = this.instances.bob.generateKeypair();
    const decryptedBalanceAfter = await delegatedUserDecryptSingleHandle(
      this.instances.bob,
      multisigBalanceAfter,
      this.tokenAddress,
      this.multisigAddress,
      this.signers.bob.address,
      this.signers.bob,
      skAfter,
      pkAfter,
    );

    // The multisig balance should have decreased by the transfer amount.
    expect(Number(decryptedBalanceBefore) - Number(decryptedBalanceAfter)).to.equal(Number(transferAmount));
  });

  it('test delegated user decryption - multisig revokes the delegation of user decryption to an EOA', async function () {
    // First, ensure Bob has delegation.
    const expirationTimestamp = Math.floor(Date.now() / 1000) + 86400;
    const delegateTx = await this.multisig
      .connect(this.signers.bob)
      .delegateUserDecryption(
        this.signers.bob.address,
        this.tokenAddress,
        expirationTimestamp,
      );
    await delegateTx.wait();

    // Wait for 15 blocks to ensure delegation is propagated by the coprocessor.
    const currentBlock1 = await ethers.provider.getBlockNumber();
    await waitForBlock(currentBlock1 + 15);

    // Revoke the delegation for Bob's EOA.
    const revokeTx = await this.multisig
      .connect(this.signers.bob)
      .revokeUserDecryptionDelegation(
        this.signers.bob.address,
        this.tokenAddress,
      );
    await revokeTx.wait();

    // Wait for 15 blocks to ensure revocation is propagated by the coprocessor.
    const currentBlock2 = await ethers.provider.getBlockNumber();
    await waitForBlock(currentBlock2 + 15);

    // Try to decrypt the multisig balance with Bob's EOA, which should now fail.
    const balanceHandle = await this.token.balanceOf(this.multisigAddress);
    const { publicKey, privateKey } = this.instances.bob.generateKeypair();

    await expect(
      delegatedUserDecryptSingleHandle(
        this.instances.bob,
        balanceHandle,
        this.tokenAddress,
        this.multisigAddress,
        this.signers.bob.address,
        this.signers.bob,
        privateKey,
        publicKey,
      )
    ).to.be.rejectedWith(
      new RegExp(USER_DECRYPTION_NOT_DELEGATED_SELECTOR),
    );
  });
});
