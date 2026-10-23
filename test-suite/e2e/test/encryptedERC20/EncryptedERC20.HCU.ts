import { expect } from 'chai';
import type {
  ContractRunner,
  ContractTransactionResponse,
  TransactionReceipt,
  TransactionRequest,
  Wallet,
} from 'ethers';
import { ethers } from 'hardhat';

import type { EncryptedERC20 } from '../../types';
import { createInstances } from '../instance';
import { isLiveNetwork } from '../network';
import type { Signers } from '../signers';
import { getSigners, initSigners } from '../signers';
import type { FhevmInstances } from '../types';
import { getTxHCUFromTxReceipt, mineNBlocks, waitForPendingTransactions, waitForTransactionReceipt } from '../utils';
import { deployEncryptedERC20Fixture } from './EncryptedERC20.fixture';

// Minimal ABI for HCULimit — the contract is deployed by the host-sc stack
// but not compiled in the E2E test suite.
const HCU_LIMIT_ABI = [
  'function getVersion() view returns (string)',
  'function getQuota(uint16) view returns (uint48, uint48, uint48)',
  'function getContractQuota(address) view returns (uint16)',
  'function getMaxHCUPerTx() view returns (uint48)',
  'function getMaxHCUDepthPerTx() view returns (uint48)',
  'function setQuota(uint16, uint48, uint48)',
  'function assignQuota(address, uint16)',
  'function lowerQuota(uint16, uint48, uint48)',
  'function setMaxHCUPerTx(uint48)',
  'function setMaxHCUDepthPerTx(uint48)',
  'error NotPauser(address)',
];

type HcuLimitContract = {
  connect(runner: ContractRunner): HcuLimitContract;
  getVersion(): Promise<string>;
  getQuota(quotaId: number, overrides?: ReadAtBlock): Promise<[bigint, bigint, bigint]>;
  getContractQuota(account: string): Promise<bigint>;
  getMaxHCUPerTx(): Promise<bigint>;
  getMaxHCUDepthPerTx(): Promise<bigint>;
  setQuota(
    quotaId: number,
    maxHCU: bigint | number,
    decayPerBlock: bigint | number,
  ): Promise<ContractTransactionResponse>;
  assignQuota(account: string, quotaId: number): Promise<ContractTransactionResponse>;
  lowerQuota(
    quotaId: number,
    maxHCU: bigint | number,
    decayPerBlock: bigint | number,
  ): Promise<ContractTransactionResponse>;
  setMaxHCUPerTx(value: bigint | number): Promise<ContractTransactionResponse>;
  setMaxHCUDepthPerTx(value: bigint | number): Promise<ContractTransactionResponse>;
};

const DEFAULT_QUOTA = 0;
const UNCAPPED_QUOTA = 1;
const NO_CAP = 2n ** 48n - 1n;

// Quotas replaced the global block cap in HCULimit v0.5.0; getVersion() returns e.g. "HCULimit v0.5.0".
const supportsQuotas = async (hcuLimit: HcuLimitContract): Promise<boolean> => {
  const [, major, minor] = (await hcuLimit.getVersion()).match(/v(\d+)\.(\d+)\./) ?? [];
  return Number(major) > 0 || Number(minor) >= 5;
};

type ReadAtBlock = {
  blockTag?: number | string;
};

type TransferSender = 'alice' | 'bob';

type HcuTransferContext = {
  contractAddress: string;
  erc20: EncryptedERC20;
  instances: FhevmInstances;
  signers: Signers;
};

const LIVE_HCU_LIMIT_ERROR = 'HCU_LIMIT_CONTRACT_ADDRESS env var is required for live HCU block-cap coverage';
const LOCAL_HCU_LIMIT_ERROR = 'HCU_LIMIT_CONTRACT_ADDRESS env var is required for block cap tests';

const requireHcuLimit = (hcuLimit: HcuLimitContract | null, message: string): HcuLimitContract => {
  if (!hcuLimit) {
    throw new Error(message);
  }
  return hcuLimit;
};

const requireDeployer = (deployer: Wallet | undefined): Wallet => {
  if (!deployer) {
    throw new Error('DEPLOYER_PRIVATE_KEY env var is required for block cap tests');
  }
  return deployer;
};

const requireReceipt = (receipt: TransactionReceipt | null, label: string): TransactionReceipt => {
  if (!receipt) {
    throw new Error(`${label} receipt is missing`);
  }
  return receipt;
};

describe('EncryptedERC20:HCU', function () {
  async function assertWithinConfiguredCaps(hcuLimit: HcuLimitContract, receipt: TransactionReceipt, label: string) {
    const { globalTxHCU, maxTxHCUDepth } = getTxHCUFromTxReceipt(receipt);
    const [maxPerTx, maxDepth] = await Promise.all([hcuLimit.getMaxHCUPerTx(), hcuLimit.getMaxHCUDepthPerTx()]);
    expect(BigInt(globalTxHCU) <= maxPerTx).to.eq(true, `${label} should stay within the deployed per-tx cap`);
    expect(BigInt(maxTxHCUDepth) <= maxDepth).to.eq(true, `${label} should stay within the deployed depth cap`);
  }

  async function assertQuotaIncludesTransaction(
    hcuLimit: HcuLimitContract,
    contractAddress: string,
    receipt: TransactionReceipt,
    label: string,
  ) {
    if (!(await supportsQuotas(hcuLimit))) {
      return;
    }
    const { globalTxHCU } = getTxHCUFromTxReceipt(receipt);
    const quotaId = Number(await hcuLimit.getContractQuota(contractAddress));
    const [cap, , usedHCU] = await hcuLimit.getQuota(quotaId, { blockTag: receipt.blockNumber });
    if (cap === NO_CAP) {
      expect(usedHCU).to.eq(0n, `${label} should not be metered by an uncapped quota`);
      return;
    }
    expect(usedHCU >= BigInt(globalTxHCU)).to.eq(true, `${label} quota usage should include this tx HCU`);
    expect(usedHCU <= cap).to.eq(true, `${label} quota usage should stay within the quota cap`);
  }

  before(async function () {
    await initSigners(2);
    this.signers = await getSigners();
  });

  beforeEach(async function () {
    const contract = await deployEncryptedERC20Fixture();
    this.contractAddress = await contract.getAddress();
    this.erc20 = contract;
    this.instances = await createInstances(this.signers);
    const hcuLimitAddress = process.env.HCU_LIMIT_CONTRACT_ADDRESS;
    this.hcuLimit = hcuLimitAddress
      ? (new ethers.Contract(hcuLimitAddress, HCU_LIMIT_ABI, ethers.provider) as unknown as HcuLimitContract)
      : null;
  });

  it('should transfer tokens between two users', async function () {
    const transaction = await this.erc20.mint(10000);
    const t1 = await transaction.wait();
    expect(t1?.status).to.eq(1);

    const encryptedTransferAmount = await this.instances.alice.encryptUint64({
      value: 1337,
      contractAddress: this.contractAddress,
      userAddress: this.signers.alice.address,
    });

    const tx = await this.erc20['transfer(address,bytes32,bytes)'](
      this.signers.bob.address,
      encryptedTransferAmount.handles[0],
      encryptedTransferAmount.inputProof,
    );
    const t2 = requireReceipt(await tx.wait(), 'transfer');
    expect(t2.status).to.eq(1);

    const { globalTxHCU: HCUTransfer, maxTxHCUDepth: HCUMaxDepthTransfer } = getTxHCUFromTxReceipt(t2);
    console.log('Total HCU in transfer', HCUTransfer);
    console.log('HCU Depth in transfer', HCUMaxDepthTransfer);
    console.log('Native Gas Consumed in transfer', t2.gasUsed);

    if (isLiveNetwork()) {
      return;
    }

    // Le euint64 (149000) +  TrivialEncrypt euint64 (32) + Select euint64 (55000) + Add euint64 (162000)
    /// + TrivialEncrypt euint64(32) (Initialize balance to 0) + Sub euint euint64 (162000)
    expect(HCUTransfer).to.eq(528_064, 'HCU incorrect');

    /// Le euint64 (149000) + Select euint64 (55000) + Sub euint64 (162000)
    expect(HCUMaxDepthTransfer).to.eq(366_000, 'HCU Depth incorrect');
  });

  it('should be able to transferFrom only if allowance is sufficient', async function () {
    const transaction = await this.erc20.mint(10000);
    await transaction.wait();

    const encryptedAllowanceAmount = await this.instances.alice.encryptUint64({
      value: 1337,
      contractAddress: this.contractAddress,
      userAddress: this.signers.alice.address,
    });

    const tx = await this.erc20['approve(address,bytes32,bytes)'](
      this.signers.bob.address,
      encryptedAllowanceAmount.handles[0],
      encryptedAllowanceAmount.inputProof,
    );
    await tx.wait();

    const bobErc20 = this.erc20.connect(this.signers.bob);

    const encryptedTransferAmount2 = await this.instances.bob.encryptUint64({
      value: 1337, // below allowance so next tx should send token
      contractAddress: this.contractAddress,
      userAddress: this.signers.bob.address,
    });

    const tx3 = await bobErc20['transferFrom(address,address,bytes32,bytes)'](
      this.signers.alice.address,
      this.signers.bob.address,
      encryptedTransferAmount2.handles[0],
      encryptedTransferAmount2.inputProof,
    );

    const t3 = requireReceipt(await tx3.wait(), 'transferFrom');

    const { globalTxHCU: HCUTransferFrom, maxTxHCUDepth: HCUMaxDepthTransferFrom } = getTxHCUFromTxReceipt(t3);
    console.log('Total HCU in transferFrom', HCUTransferFrom);
    console.log('HCU Depth in transferFrom', HCUMaxDepthTransferFrom);
    console.log('Native Gas Consumed in transferFrom', t3.gasUsed);

    if (isLiveNetwork()) {
      return;
    }

    // Le euint64 (149000) + Le euint64 (149000) + And ebool (34000) + Sub euint64 (162000) + TrivialEncrypt (32) + Select euint64 (55000) +
    // Select euint64 (55000) + Add ebool (25000) + TrivialEncrypt (Initialize balance to 0) (32) + Sub euint64 (162000)
    expect(HCUTransferFrom).to.eq(919_064, 'HCU incorrect');

    // Le euint64 (149000) + And ebool (25000) + Select euint64 (55000) + Sub euint64 (162000)
    expect(HCUMaxDepthTransferFrom).to.eq(391_000, 'HCU Depth incorrect');
  });

  describe('block cap scenarios', function () {
    const BATCHED_TRANSFER_GAS_LIMIT = 1_000_000;

    async function sendEncryptedTransfer(
      ctx: HcuTransferContext,
      sender: TransferSender,
      recipient: string,
      amount: number,
      overrides?: TransactionRequest,
    ) {
      const erc20 = ctx.erc20.connect(ctx.signers[sender]);
      const enc = await ctx.instances[sender].encryptUint64({
        value: amount,
        contractAddress: ctx.contractAddress,
        userAddress: ctx.signers[sender].address,
      });
      return erc20['transfer(address,bytes32,bytes)'](recipient, enc.handles[0], enc.inputProof, overrides ?? {});
    }

    async function mintAndDistribute(ctx: HcuTransferContext) {
      const mintTx = await ctx.erc20.mint(10000);
      await mintTx.wait();
      const setupTx = await sendEncryptedTransfer(ctx, 'alice', ctx.signers.bob.address, 5000);
      await setupTx.wait();
    }

    before(async function () {
      const deployerKey = process.env.DEPLOYER_PRIVATE_KEY;
      if (deployerKey) {
        this.deployer = new ethers.Wallet(deployerKey, ethers.provider);
      }
    });

    describe('live read-only coverage', function () {
      beforeEach(function () {
        if (!isLiveNetwork()) {
          this.skip();
        }
      });

      it('should keep transfer HCU within deployed caps', async function () {
        const mintTx = await this.erc20.mint(10000);
        await mintTx.wait();

        const encryptedTransferAmount = await this.instances.alice.encryptUint64({
          value: 1337,
          contractAddress: this.contractAddress,
          userAddress: this.signers.alice.address,
        });
        const tx = await this.erc20['transfer(address,bytes32,bytes)'](
          this.signers.bob.address,
          encryptedTransferAmount.handles[0],
          encryptedTransferAmount.inputProof,
        );
        const receipt = requireReceipt(await tx.wait(), 'live transfer');
        const hcuLimit = requireHcuLimit(this.hcuLimit, LIVE_HCU_LIMIT_ERROR);
        await assertWithinConfiguredCaps(hcuLimit, receipt, 'transfer');
        await assertQuotaIncludesTransaction(hcuLimit, this.contractAddress, receipt, 'transfer');
      });

      it('should keep transferFrom HCU within deployed caps', async function () {
        const mintTx = await this.erc20.mint(10000);
        await mintTx.wait();

        const encryptedAllowanceAmount = await this.instances.alice.encryptUint64({
          value: 1337,
          contractAddress: this.contractAddress,
          userAddress: this.signers.alice.address,
        });
        const approveTx = await this.erc20['approve(address,bytes32,bytes)'](
          this.signers.bob.address,
          encryptedAllowanceAmount.handles[0],
          encryptedAllowanceAmount.inputProof,
        );
        await approveTx.wait();

        const bobErc20 = this.erc20.connect(this.signers.bob);

        const encryptedTransferAmount = await this.instances.bob.encryptUint64({
          value: 1337,
          contractAddress: this.contractAddress,
          userAddress: this.signers.bob.address,
        });

        const tx = await bobErc20['transferFrom(address,address,bytes32,bytes)'](
          this.signers.alice.address,
          this.signers.bob.address,
          encryptedTransferAmount.handles[0],
          encryptedTransferAmount.inputProof,
        );
        const receipt = requireReceipt(await tx.wait(), 'live transferFrom');
        await assertWithinConfiguredCaps(requireHcuLimit(this.hcuLimit, LIVE_HCU_LIMIT_ERROR), receipt, 'transferFrom');
      });
    });

    describe('local deterministic coverage', function () {
      const TIGHT_DEPTH_PER_TX = 400_000;
      const TIGHT_MAX_PER_TX = 600_000;
      const TIGHT_PER_BLOCK = 600_000;
      let savedDefaultCap: bigint;
      let savedDefaultDecay: bigint;
      let savedUncappedCap: bigint;
      let savedUncappedDecay: bigint;
      let savedMaxHCUPerTx: bigint;
      let savedMaxHCUDepthPerTx: bigint;

      beforeEach(async function () {
        if (isLiveNetwork()) {
          this.skip();
        }
        const hcuLimit = requireHcuLimit(this.hcuLimit, LOCAL_HCU_LIMIT_ERROR);
        if (!(await supportsQuotas(hcuLimit))) {
          this.skip();
        }
        const deployer = requireDeployer(this.deployer);
        [
          [savedDefaultCap, savedDefaultDecay],
          [savedUncappedCap, savedUncappedDecay],
          savedMaxHCUPerTx,
          savedMaxHCUDepthPerTx,
        ] = await Promise.all([
          hcuLimit.getQuota(DEFAULT_QUOTA),
          hcuLimit.getQuota(UNCAPPED_QUOTA),
          hcuLimit.getMaxHCUPerTx(),
          hcuLimit.getMaxHCUDepthPerTx(),
        ]);
        // Narrowest-first when lowering: maxHCUPerTx >= maxHCUDepthPerTx
        const ownerHcuLimit = hcuLimit.connect(deployer);
        await (await ownerHcuLimit.setMaxHCUDepthPerTx(TIGHT_DEPTH_PER_TX)).wait();
        await (await ownerHcuLimit.setMaxHCUPerTx(TIGHT_MAX_PER_TX)).wait();
        // Maximum and decay both TIGHT_PER_BLOCK: the quota allows TIGHT_PER_BLOCK HCU per block.
        await (await ownerHcuLimit.setQuota(DEFAULT_QUOTA, TIGHT_PER_BLOCK, TIGHT_PER_BLOCK)).wait();
      });

      afterEach(async function () {
        if (isLiveNetwork()) {
          return;
        }
        // beforeEach skipped (live network or HCULimit v0.1.x) — nothing to restore.
        if (savedDefaultCap === undefined) {
          return;
        }
        await ethers.provider.send('evm_setAutomine', [true]);
        await ethers.provider.send('evm_setIntervalMining', [1]);

        const hcuLimit = requireHcuLimit(this.hcuLimit, LOCAL_HCU_LIMIT_ERROR);
        const ownerHcuLimit = hcuLimit.connect(requireDeployer(this.deployer));
        await (await ownerHcuLimit.setQuota(DEFAULT_QUOTA, savedDefaultCap, savedDefaultDecay)).wait();
        await (await ownerHcuLimit.setQuota(UNCAPPED_QUOTA, savedUncappedCap, savedUncappedDecay)).wait();
        await (await ownerHcuLimit.setMaxHCUPerTx(savedMaxHCUPerTx)).wait();
        await (await ownerHcuLimit.setMaxHCUDepthPerTx(savedMaxHCUDepthPerTx)).wait();
      });

      it('should accumulate HCU across users until the block cap is exhausted', async function () {
        await mintAndDistribute(this);

        await mineNBlocks(1);
        await ethers.provider.send('evm_setIntervalMining', [0]);
        await ethers.provider.send('evm_setAutomine', [false]);

        // Alice fills the cap, Bob would push block total over — use fixed gasLimit
        // to bypass estimateGas (which reverts against pending state)
        const tx1 = await sendEncryptedTransfer(this, 'alice', this.signers.carol.address, 100, {
          gasLimit: BATCHED_TRANSFER_GAS_LIMIT,
        });
        const tx2 = await sendEncryptedTransfer(this, 'bob', this.signers.carol.address, 100, {
          gasLimit: BATCHED_TRANSFER_GAS_LIMIT,
        });
        await waitForPendingTransactions([tx1.hash, tx2.hash]);

        await ethers.provider.send('evm_mine');
        await ethers.provider.send('evm_setAutomine', [true]);
        await ethers.provider.send('evm_setIntervalMining', [1]);

        const receipt1 = await waitForTransactionReceipt(tx1.hash);
        expect(receipt1.status).to.eq(1, 'First transfer should succeed');

        // Use getTransactionReceipt to avoid ethers throwing on reverted tx
        const receipt2 = await ethers.provider.getTransactionReceipt(tx2.hash);
        expect(receipt2?.status).to.eq(0, 'Second transfer should revert (block cap exceeded)');
        expect(receipt1?.blockNumber).to.eq(receipt2?.blockNumber);
      });

      it('should allow previously blocked caller to succeed after block rollover', async function () {
        await mintAndDistribute(this);
        const hcuLimit = requireHcuLimit(this.hcuLimit, LOCAL_HCU_LIMIT_ERROR);

        // Block N: alice fills the cap, bob gets blocked
        await mineNBlocks(1);
        await ethers.provider.send('evm_setIntervalMining', [0]);
        await ethers.provider.send('evm_setAutomine', [false]);

        const txAlice = await sendEncryptedTransfer(this, 'alice', this.signers.carol.address, 100, {
          gasLimit: BATCHED_TRANSFER_GAS_LIMIT,
        });
        const txBob = await sendEncryptedTransfer(this, 'bob', this.signers.carol.address, 100, {
          gasLimit: BATCHED_TRANSFER_GAS_LIMIT,
        });
        await waitForPendingTransactions([txAlice.hash, txBob.hash]);

        await ethers.provider.send('evm_mine');
        await ethers.provider.send('evm_setAutomine', [true]);
        await ethers.provider.send('evm_setIntervalMining', [1]);

        const receiptAlice = await waitForTransactionReceipt(txAlice.hash);
        expect(receiptAlice.status).to.eq(1, 'Alice should succeed');

        const receiptBob = await ethers.provider.getTransactionReceipt(txBob.hash);
        expect(receiptBob?.status).to.eq(0, 'Bob should be blocked in block N');

        // Block N+1: quota usage has decayed by a full block, bob retries and succeeds
        await mineNBlocks(1);

        const [, , usedHCUAfterReset] = await hcuLimit.getQuota(DEFAULT_QUOTA);
        expect(usedHCUAfterReset).to.eq(0n, 'Quota usage should decay to 0 after a block');

        const retryBob = await sendEncryptedTransfer(this, 'bob', this.signers.carol.address, 100);
        const receiptRetry = requireReceipt(await retryBob.wait(), 'retry transfer');
        expect(receiptRetry.status).to.eq(1, 'Bob should succeed after rollover');
      });

      it('should count HCU after leaving an uncapped quota', async function () {
        const hcuLimit = requireHcuLimit(this.hcuLimit, LOCAL_HCU_LIMIT_ERROR);
        const ownerHcuLimit = hcuLimit.connect(requireDeployer(this.deployer));

        // Use manual mining (automine=false + explicit evm_mine) to avoid
        // the unreliable automine+intervalMining(0) combo that hangs in CI.
        await ethers.provider.send('evm_setIntervalMining', [0]);
        await ethers.provider.send('evm_setAutomine', [false]);

        const mintTx = await this.erc20.mint(10000);
        await ethers.provider.send('evm_mine');
        const mintReceipt = await waitForTransactionReceipt(mintTx.hash);
        expect(mintReceipt.status).to.eq(1, 'Mint should succeed');

        const uncapTx = await ownerHcuLimit.setQuota(UNCAPPED_QUOTA, NO_CAP, NO_CAP);
        const assignTx = await ownerHcuLimit.assignQuota(this.contractAddress, UNCAPPED_QUOTA);
        await ethers.provider.send('evm_mine');
        await waitForTransactionReceipt(uncapTx.hash);
        await waitForTransactionReceipt(assignTx.hash);

        // Advance to a fresh block so the transfer starts with clean quota usage
        await mineNBlocks(1);

        // Transfer while uncapped — the default quota stays at 0
        const tx1 = await sendEncryptedTransfer(this, 'alice', this.signers.bob.address, 100, {
          gasLimit: BATCHED_TRANSFER_GAS_LIMIT,
        });
        await ethers.provider.send('evm_mine');
        const receiptUncapped = await waitForTransactionReceipt(tx1.hash);
        expect(receiptUncapped.status).to.eq(1, 'Transfer in an uncapped quota should succeed');

        const [, , usedHCUUncapped] = await hcuLimit.getQuota(DEFAULT_QUOTA);
        expect(usedHCUUncapped).to.eq(0n, 'A contract in an uncapped quota should not count HCU');

        const reassignTx = await ownerHcuLimit.assignQuota(this.contractAddress, DEFAULT_QUOTA);
        await ethers.provider.send('evm_mine');
        await waitForTransactionReceipt(reassignTx.hash);

        // Transfer back in the default quota — usage should count HCU
        const tx2 = await sendEncryptedTransfer(this, 'alice', this.signers.bob.address, 100, {
          gasLimit: BATCHED_TRANSFER_GAS_LIMIT,
        });
        await ethers.provider.send('evm_mine');
        await waitForTransactionReceipt(tx2.hash);

        const [, , usedHCUDefault] = await hcuLimit.getQuota(DEFAULT_QUOTA);
        expect(usedHCUDefault).to.be.greaterThan(0n, 'Should count HCU in the default quota');
      });

      it('should reject lowerQuota from a non-owner non-pauser', async function () {
        const hcuLimit = requireHcuLimit(this.hcuLimit, LOCAL_HCU_LIMIT_ERROR);
        const aliceHcuLimit = hcuLimit.connect(this.signers.alice);
        await expect(aliceHcuLimit.lowerQuota(DEFAULT_QUOTA, 1_000_000, 1_000_000)).to.be.revertedWithCustomError(
          hcuLimit as unknown as { interface: any },
          'NotPauser',
        );
      });
    });
  });
});
