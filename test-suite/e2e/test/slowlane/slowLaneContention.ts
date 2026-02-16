import { expect } from 'chai';
import { ethers } from 'hardhat';

import { createInstance } from '../instance';
import { getSigner } from '../signers';

describe('Slow lane deterministic contention', function () {
  beforeEach(async function () {
    this.signer = await getSigner(119);

    const contractFactory = await ethers.getContractFactory('SlowLaneContention');
    const contract = await contractFactory.connect(this.signer).deploy();
    await contract.waitForDeployment();

    this.contractAddress = await contract.getAddress();
    this.contract = contract;
    this.instance = await createInstance();
  });

  it('creates one heavy chain and one light chain', async function () {
    const heavyInput = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    heavyInput.add64(1);
    const heavyEncryptedInput = await heavyInput.encrypt();

    const heavyTx = await this.contract.runAddChain(
      heavyEncryptedInput.handles[0],
      heavyEncryptedInput.inputProof,
      8,
    );
    const heavyReceipt = await heavyTx.wait();
    expect(heavyReceipt?.status).to.eq(1);

    const lightInput = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    lightInput.add64(1);
    const lightEncryptedInput = await lightInput.encrypt();

    const lightTx = await this.contract.runAddChain(
      lightEncryptedInput.handles[0],
      lightEncryptedInput.inputProof,
      1,
    );
    const lightReceipt = await lightTx.wait();
    expect(lightReceipt?.status).to.eq(1);
  });
});
