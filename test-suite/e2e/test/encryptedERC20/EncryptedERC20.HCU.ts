import { expect } from 'chai';
import { ethers } from 'hardhat';

import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';
import { getTxHCUFromTxReceipt, mineNBlocks } from '../utils';
import { deployEncryptedERC20Fixture } from './EncryptedERC20.fixture';

// Minimal ABI for HCULimit — the contract is deployed by the host-sc stack
// but not compiled in the E2E test suite.
const HCU_LIMIT_ABI = [
  'function getBlockMeter() view returns (uint48, uint48)',
  'function getGlobalHCUCapPerBlock() view returns (uint48)',
  'function getMaxHCUPerTx() view returns (uint48)',
  'function getMaxHCUDepthPerTx() view returns (uint48)',
  'function setHCUPerBlock(uint48)',
  'function setMaxHCUPerTx(uint48)',
  'function setMaxHCUDepthPerTx(uint48)',
  'function addToBlockHCUWhitelist(address)',
  'function removeFromBlockHCUWhitelist(address)',
  'function isBlockHCUWhitelisted(address) view returns (bool)',
];

describe('EncryptedERC20:HCU', function () {
  before(async function () {
    await initSigners(2);
    this.signers = await getSigners();
  });

  beforeEach(async function () {
    const contract = await deployEncryptedERC20Fixture();
    this.contractAddress = await contract.getAddress();
    this.erc20 = contract;
    this.instances = await createInstances(this.signers);
  });

  it('should transfer tokens between two users', async function () {
    const transaction = await this.erc20.mint(10000);
    const t1 = await transaction.wait();
    expect(t1?.status).to.eq(1);

    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.add64(1337);
    const encryptedTransferAmount = await input.encrypt();
    const tx = await this.erc20['transfer(address,bytes32,bytes)'](
      this.signers.bob.address,
      encryptedTransferAmount.handles[0],
      encryptedTransferAmount.inputProof,
    );
    const t2 = await tx.wait();
    expect(t2?.status).to.eq(1);

    const { globalTxHCU: HCUTransfer, maxTxHCUDepth: HCUMaxDepthTransfer } = getTxHCUFromTxReceipt(t2);
    console.log('Total HCU in transfer', HCUTransfer);
    console.log('HCU Depth in transfer', HCUMaxDepthTransfer);
    console.log('Native Gas Consumed in transfer', t2.gasUsed);

    // Le euint64 (149000) +  TrivialEncrypt euint64 (32) + Select euint64 (55000) + Add euint64 (162000)
    /// + TrivialEncrypt euint64(32) (Initialize balance to 0) + Sub euint euint64 (162000)
    expect(HCUTransfer).to.eq(528_064, 'HCU incorrect');

    /// Le euint64 (149000) + Select euint64 (55000) + Sub euint64 (162000)
    expect(HCUMaxDepthTransfer).to.eq(366_000, 'HCU Depth incorrect');
  });

  it('should be able to transferFrom only if allowance is sufficient', async function () {
    const transaction = await this.erc20.mint(10000);
    await transaction.wait();

    const inputAlice = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    inputAlice.add64(1337);
    const encryptedAllowanceAmount = await inputAlice.encrypt();
    const tx = await this.erc20['approve(address,bytes32,bytes)'](
      this.signers.bob.address,
      encryptedAllowanceAmount.handles[0],
      encryptedAllowanceAmount.inputProof,
    );
    await tx.wait();

    const bobErc20 = this.erc20.connect(this.signers.bob);
    const inputBob2 = this.instances.bob.createEncryptedInput(this.contractAddress, this.signers.bob.address);
    inputBob2.add64(1337); // below allowance so next tx should send token
    const encryptedTransferAmount2 = await inputBob2.encrypt();
    const tx3 = await bobErc20['transferFrom(address,address,bytes32,bytes)'](
      this.signers.alice.address,
      this.signers.bob.address,
      encryptedTransferAmount2.handles[0],
      encryptedTransferAmount2.inputProof,
    );

    const t3 = await tx3.wait();

    const { globalTxHCU: HCUTransferFrom, maxTxHCUDepth: HCUMaxDepthTransferFrom } = getTxHCUFromTxReceipt(t3);
    console.log('Total HCU in transferFrom', HCUTransferFrom);
    console.log('HCU Depth in transferFrom', HCUMaxDepthTransferFrom);
    console.log('Native Gas Consumed in transferFrom', t3.gasUsed);

    // Le euint64 (149000) + Le euint64 (149000) + And ebool (34000) + Sub euint64 (162000) + TrivialEncrypt (32) + Select euint64 (55000) +
    // Select euint64 (55000) + Add ebool (25000) + TrivialEncrypt (Initialize balance to 0) (32) + Sub euint64 (162000)
    expect(HCUTransferFrom).to.eq(919_064, 'HCU incorrect');

    // Le euint64 (149000) + And ebool (25000) + Select euint64 (55000) + Sub euint64 (162000)
    expect(HCUMaxDepthTransferFrom).to.eq(391_000, 'HCU Depth incorrect');
  });

  describe('block cap scenarios', function () {
    const TIGHT_DEPTH_PER_TX = 400_000;
    const TIGHT_MAX_PER_TX = 600_000;
    const TIGHT_PER_BLOCK = 600_000;

    let savedHCUPerBlock: bigint;
    let savedMaxHCUPerTx: bigint;
    let savedMaxHCUDepthPerTx: bigint;

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    async function sendEncryptedTransfer(ctx: any, sender: string, recipient: string, amount: number) {
      const erc20 = ctx.erc20.connect(ctx.signers[sender]);
      const input = ctx.instances[sender].createEncryptedInput(ctx.contractAddress, ctx.signers[sender].address);
      input.add64(amount);
      const enc = await input.encrypt();
      return erc20['transfer(address,bytes32,bytes)'](recipient, enc.handles[0], enc.inputProof);
    }

    // Narrowest-first when lowering to satisfy: hcuPerBlock >= maxHCUPerTx >= maxHCUDepthPerTx
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    async function lowerHCULimits(ctx: any) {
      const ownerHcuLimit = ctx.hcuLimit.connect(ctx.deployer);
      await ownerHcuLimit.setMaxHCUDepthPerTx(TIGHT_DEPTH_PER_TX);
      await ownerHcuLimit.setMaxHCUPerTx(TIGHT_MAX_PER_TX);
      await ownerHcuLimit.setHCUPerBlock(TIGHT_PER_BLOCK);
    }

    before(async function () {
      const hcuLimitAddress = process.env.HCU_LIMIT_CONTRACT_ADDRESS;
      if (!hcuLimitAddress) {
        throw new Error('HCU_LIMIT_CONTRACT_ADDRESS env var is required for block cap tests');
      }
      this.hcuLimit = new ethers.Contract(hcuLimitAddress, HCU_LIMIT_ABI, ethers.provider);

      const deployerKey = process.env.DEPLOYER_PRIVATE_KEY;
      if (!deployerKey) {
        throw new Error('DEPLOYER_PRIVATE_KEY env var is required for block cap tests');
      }
      this.deployer = new ethers.Wallet(deployerKey, ethers.provider);
    });

    beforeEach(async function () {
      savedHCUPerBlock = await this.hcuLimit.getGlobalHCUCapPerBlock();
      savedMaxHCUPerTx = await this.hcuLimit.getMaxHCUPerTx();
      savedMaxHCUDepthPerTx = await this.hcuLimit.getMaxHCUDepthPerTx();
    });

    afterEach(async function () {
      await ethers.provider.send('evm_setAutomine', [true]);
      // Widest-first when restoring to satisfy: hcuPerBlock >= maxHCUPerTx >= maxHCUDepthPerTx
      const ownerHcuLimit = this.hcuLimit.connect(this.deployer);
      await ownerHcuLimit.setHCUPerBlock(savedHCUPerBlock);
      await ownerHcuLimit.setMaxHCUPerTx(savedMaxHCUPerTx);
      await ownerHcuLimit.setMaxHCUDepthPerTx(savedMaxHCUDepthPerTx);

      if (await this.hcuLimit.isBlockHCUWhitelisted(this.contractAddress)) {
        await ownerHcuLimit.removeFromBlockHCUWhitelist(this.contractAddress);
      }
    });

    it('should accumulate HCU from multiple users in the same block', async function () {
      const mintTx = await this.erc20.mint(10000);
      await mintTx.wait();

      const setupTx = await sendEncryptedTransfer(this, 'alice', this.signers.bob.address, 5000);
      await setupTx.wait();

      await mineNBlocks(1);
      await ethers.provider.send('evm_setAutomine', [false]);

      const txAlice = await sendEncryptedTransfer(this, 'alice', this.signers.carol.address, 1000);
      const txBob = await sendEncryptedTransfer(this, 'bob', this.signers.carol.address, 1000);

      await ethers.provider.send('evm_mine');
      await ethers.provider.send('evm_setAutomine', [true]);

      const receiptAlice = await txAlice.wait();
      const receiptBob = await txBob.wait();

      expect(receiptAlice?.status).to.eq(1);
      expect(receiptBob?.status).to.eq(1);
      expect(receiptAlice?.blockNumber).to.eq(receiptBob?.blockNumber);

      const { globalTxHCU: hcuAlice } = getTxHCUFromTxReceipt(receiptAlice);
      const { globalTxHCU: hcuBob } = getTxHCUFromTxReceipt(receiptBob);

      const [, usedHCU] = await this.hcuLimit.getBlockMeter();
      expect(usedHCU).to.eq(BigInt(hcuAlice + hcuBob));
    });

    it('should revert when block HCU cap is exhausted', async function () {
      await lowerHCULimits(this);

      const mintTx = await this.erc20.mint(10000);
      await mintTx.wait();

      const setupTx = await sendEncryptedTransfer(this, 'alice', this.signers.bob.address, 5000);
      await setupTx.wait();

      await mineNBlocks(1);
      await ethers.provider.send('evm_setAutomine', [false]);

      // Alice ~528K HCU (under 600K cap), Bob ~528K would push block total > 600K
      const tx1 = await sendEncryptedTransfer(this, 'alice', this.signers.carol.address, 100);
      const tx2 = await sendEncryptedTransfer(this, 'bob', this.signers.carol.address, 100);

      await ethers.provider.send('evm_mine');
      await ethers.provider.send('evm_setAutomine', [true]);

      const receipt1 = await tx1.wait();
      expect(receipt1?.status).to.eq(1, 'First transfer should succeed');

      // Use getTransactionReceipt to avoid ethers throwing on reverted tx
      const receipt2 = await ethers.provider.getTransactionReceipt(tx2.hash);
      expect(receipt2?.status).to.eq(0, 'Second transfer should revert (block cap exceeded)');
      expect(receipt1?.blockNumber).to.eq(receipt2?.blockNumber);
    });

    it('should allow previously blocked caller to succeed after block rollover', async function () {
      await lowerHCULimits(this);

      const mintTx = await this.erc20.mint(10000);
      await mintTx.wait();

      const setupTx = await sendEncryptedTransfer(this, 'alice', this.signers.bob.address, 5000);
      await setupTx.wait();

      // Block N: alice fills the cap, bob gets blocked
      await mineNBlocks(1);
      await ethers.provider.send('evm_setAutomine', [false]);

      const txAlice = await sendEncryptedTransfer(this, 'alice', this.signers.carol.address, 100);
      const txBob = await sendEncryptedTransfer(this, 'bob', this.signers.carol.address, 100);

      await ethers.provider.send('evm_mine');
      await ethers.provider.send('evm_setAutomine', [true]);

      const receiptAlice = await txAlice.wait();
      expect(receiptAlice?.status).to.eq(1, 'Alice should succeed');

      const receiptBob = await ethers.provider.getTransactionReceipt(txBob.hash);
      expect(receiptBob?.status).to.eq(0, 'Bob should be blocked in block N');

      // Block N+1: meter resets, bob retries and succeeds
      await mineNBlocks(1);

      const [, usedHCUAfterReset] = await this.hcuLimit.getBlockMeter();
      expect(usedHCUAfterReset).to.eq(0n, 'Meter should reset after new block');

      const retryBob = await sendEncryptedTransfer(this, 'bob', this.signers.carol.address, 100);
      const receiptRetry = await retryBob.wait();
      expect(receiptRetry?.status).to.eq(1, 'Bob should succeed after rollover');
    });

    it('should count HCU after whitelist removal', async function () {
      const ownerHcuLimit = this.hcuLimit.connect(this.deployer);

      const mintTx = await this.erc20.mint(10000);
      await mintTx.wait();

      await ownerHcuLimit.addToBlockHCUWhitelist(this.contractAddress);
      await mineNBlocks(1);

      // Transfer while whitelisted — meter stays at 0
      const tx1 = await sendEncryptedTransfer(this, 'alice', this.signers.bob.address, 100);
      await tx1.wait();

      const [, usedHCUWhitelisted] = await this.hcuLimit.getBlockMeter();
      expect(usedHCUWhitelisted).to.eq(0n, 'Whitelisted contract should not count HCU');

      await ownerHcuLimit.removeFromBlockHCUWhitelist(this.contractAddress);

      // Transfer after removal — meter should count HCU
      const tx2 = await sendEncryptedTransfer(this, 'alice', this.signers.bob.address, 100);
      await tx2.wait();

      const [, usedHCUAfterRemoval] = await this.hcuLimit.getBlockMeter();
      expect(usedHCUAfterRemoval).to.be.greaterThan(0n, 'Should count HCU after whitelist removal');
    });

    it('should reject setHCUPerBlock from non-owner', async function () {
      const aliceHcuLimit = this.hcuLimit.connect(this.signers.alice);
      await expect(aliceHcuLimit.setHCUPerBlock(1_000_000)).to.be.reverted;
    });
  });
});
