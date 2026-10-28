import { expect } from 'chai';
import { ethers } from 'hardhat';

import { getTxHCUFromTxReceipt } from '../coprocessorUtils';
import { createInstances } from '../instance';
import { initializeHCULimit } from '../paymentUtils';
import { getSigners, initSigners } from '../signers';
import { deployEncryptedERC20Fixture } from './EncryptedERC20.fixture';

const MAX_UINT48 = 2n ** 48n - 1n;

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
    this.hcuLimit = await initializeHCULimit();
  });

  afterEach(async function () {
    const ownerHcuLimit = this.hcuLimit.connect(this.signers.fred);
    await ownerHcuLimit.setQuota(0, MAX_UINT48, MAX_UINT48);
    await ownerHcuLimit.setCrossTxDepthLimit(0, 0);
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

  it('should charge FHE operations to the default quota when it has a cap', async function () {
    const transaction = await this.erc20.mint(10000);
    await transaction.wait();

    const ownerHcuLimit = this.hcuLimit.connect(this.signers.fred);
    await ownerHcuLimit.setQuota(0, 1_000_000_000, 1_000_000_000);

    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.add64(1337);
    const encryptedAmount = await input.encrypt();
    const tx = await this.erc20['transfer(address,bytes32,bytes)'](
      this.signers.bob.address,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    const receipt = await tx.wait();
    expect(receipt?.status).to.eq(1);

    const [, , usedHCU] = await this.hcuLimit.getQuota(0);
    expect(usedHCU).to.be.greaterThan(0n);
  });

  it('should not meter a contract in an uncapped quota', async function () {
    const transaction = await this.erc20.mint(10000);
    await transaction.wait();

    const ownerHcuLimit = this.hcuLimit.connect(this.signers.fred);
    await ownerHcuLimit.setQuota(0, 1_000_000_000, 1_000_000_000);
    await ownerHcuLimit.setQuota(1, MAX_UINT48, MAX_UINT48);
    await ownerHcuLimit.assignQuota(1, this.contractAddress);

    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.add64(1337);
    const encryptedAmount = await input.encrypt();
    const tx = await this.erc20['transfer(address,bytes32,bytes)'](
      this.signers.bob.address,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    const receipt = await tx.wait();
    expect(receipt?.status).to.eq(1);

    const [, , defaultUsedHCU] = await this.hcuLimit.getQuota(0);
    const [, , uncappedUsedHCU] = await this.hcuLimit.getQuota(1);
    expect(defaultUsedHCU).to.eq(0n);
    expect(uncappedUsedHCU).to.eq(0n);
  });

  it('should refuse dependent transfers at the cross-transaction depth limit until decay frees room', async function () {
    await (await this.hcuLimit.connect(this.signers.fred).setCrossTxDepthLimit(5_000_000, 1)).wait();
    await (await this.erc20.mint(10000)).wait();

    // Each transfer computes on the balances left by the previous one, so the chain deepens every time.
    const encryptAmount = async () => {
      const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
      input.add64(1);
      return input.encrypt();
    };
    const transfer = (amount: { handles: Uint8Array[]; inputProof: Uint8Array }) =>
      this.erc20['transfer(address,bytes32,bytes)'](this.signers.bob.address, amount.handles[0], amount.inputProof);
    // Returns the blocks to wait reported by the depth limit, or 0 when the transfer would pass.
    const simulate = async (amount: { handles: Uint8Array[]; inputProof: Uint8Array }) => {
      try {
        await this.erc20['transfer(address,bytes32,bytes)'].staticCall(
          this.signers.bob.address,
          amount.handles[0],
          amount.inputProof,
        );
        return 0n;
      } catch (error: any) {
        const decoded = this.hcuLimit.interface.parseError(error.data);
        expect(decoded?.name).to.eq('HCUCrossTxDepthLimitExceeded');
        return decoded!.args[0] as bigint;
      }
    };

    let refusals = 0;
    for (let sent = 0; sent < 40 && refusals === 0; sent++) {
      const amount = await encryptAmount();
      if ((await simulate(amount)) > 0n) {
        await expect(transfer(amount)).to.be.revertedWithCustomError(this.hcuLimit, 'HCUCrossTxDepthLimitExceeded');
        refusals++;
      } else {
        await (await transfer(amount)).wait();
      }
    }
    expect(refusals).to.eq(1, 'the chain should reach the limit');

    // The reported wait covers the first operation that does not fit; later operations of the same
    // transaction can need more, so a client waits, simulates again, and repeats.
    for (let round = 0; round < 5; round++) {
      const amount = await encryptAmount();
      const retryAfterBlocks = await simulate(amount);
      if (retryAfterBlocks === 0n) {
        const receipt = await (await transfer(amount)).wait();
        expect(receipt?.status).to.eq(1);
        return;
      }
      await ethers.provider.send('hardhat_mine', ['0x' + retryAfterBlocks.toString(16)]);
    }
    expect.fail('the transfer should pass once decay frees room');
  });
});
