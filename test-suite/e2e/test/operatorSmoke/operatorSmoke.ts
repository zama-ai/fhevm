import { expect } from 'chai';
import { ethers } from 'hardhat';

import { createInstance } from '../instance';
import { getSigner } from '../signers';

describe('Operator smoke', function () {
  before(async function () {
    this.signer = await getSigner(1);
    this.instance = await createInstance();

    const factory = await ethers.getContractFactory('OperatorSmoke');
    this.contract = await factory.connect(this.signer).deploy();
    await this.contract.waitForDeployment();
    this.contractAddress = await this.contract.getAddress();
  });

  it('representative operator smoke: encrypted/encrypted add computes 17 + 25 = 42', async function () {
    const input = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint64', value: 17n },
        { type: 'uint64', value: 25n },
      ],
      contractAddress: this.contractAddress,
      userAddress: this.signer.address,
    });

    const tx = await this.contract.add(input.handles[0], input.handles[1], input.inputProof);
    expect((await tx.wait()).status).to.equal(1);

    const handle = await this.contract.uint64Result();
    const decrypted = await this.instance.publicDecrypt([handle]);
    expect(decrypted.clearValues[handle]).to.equal(42n);
  });

  it('representative operator smoke: encrypted/scalar div computes 84 / 2 = 42', async function () {
    const input = await this.instance.encryptTypedValues({
      values: [{ type: 'uint64', value: 84n }],
      contractAddress: this.contractAddress,
      userAddress: this.signer.address,
    });

    const tx = await this.contract.div(input.handles[0], 2n, input.inputProof);
    expect((await tx.wait()).status).to.equal(1);

    const handle = await this.contract.uint64Result();
    const decrypted = await this.instance.publicDecrypt([handle]);
    expect(decrypted.clearValues[handle]).to.equal(42n);
  });

  it('representative operator smoke: encrypted comparison computes 17 < 25 = true', async function () {
    const input = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint64', value: 17n },
        { type: 'uint64', value: 25n },
      ],
      contractAddress: this.contractAddress,
      userAddress: this.signer.address,
    });

    const tx = await this.contract.lt(input.handles[0], input.handles[1], input.inputProof);
    expect((await tx.wait()).status).to.equal(1);

    const handle = await this.contract.boolResult();
    const decrypted = await this.instance.publicDecrypt([handle]);
    expect(decrypted.clearValues[handle]).to.equal(true);
  });

  it('representative operator smoke: unary negation computes -42 modulo 2^32', async function () {
    const input = await this.instance.encryptTypedValues({
      values: [{ type: 'uint32', value: 42n }],
      contractAddress: this.contractAddress,
      userAddress: this.signer.address,
    });

    const tx = await this.contract.neg(input.handles[0], input.inputProof);
    expect((await tx.wait()).status).to.equal(1);

    const handle = await this.contract.uint32Result();
    const decrypted = await this.instance.publicDecrypt([handle]);
    expect(decrypted.clearValues[handle]).to.equal(2n ** 32n - 42n);
  });
});
