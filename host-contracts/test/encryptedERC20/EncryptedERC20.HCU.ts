import { expect } from 'chai';

import { getTxHCUFromTxReceipt } from '../coprocessorUtils';
import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';
import { deployEncryptedERC20Fixture } from './EncryptedERC20.fixture';

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
});
