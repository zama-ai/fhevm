import { expect } from 'chai';
import { ethers } from 'hardhat';

import type { FHEVMCollectionSuite } from '../../typechain-types/examples/tests/FHEVMCollectionSuite';
import { createInstances, decrypt8, decrypt32, decrypt64, decryptBool } from '../instance';
import { getSigners, initSigners } from '../signers';

async function deployFHEVMCollectionSuiteFixture(): Promise<FHEVMCollectionSuite> {
  const signers = await getSigners();
  const contractFactory = await ethers.getContractFactory('FHEVMCollectionSuite');
  const contract = await contractFactory.connect(signers.alice).deploy();
  await contract.waitForDeployment();
  return contract as unknown as FHEVMCollectionSuite;
}

describe('Collection operators: isIn and sum', function () {
  this.timeout(120_000);

  before(async function () {
    await initSigners(1);
    this.signers = await getSigners();
    const contract = await deployFHEVMCollectionSuiteFixture();
    this.contractAddress = await contract.getAddress();
    this.contract = contract;
    this.instances = await createInstances(this.signers);
  });

  // -------------------------------------------------------------------------
  // isIn — euint8
  // -------------------------------------------------------------------------

  it('isIn(euint8): returns true when value is present', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.add8(42n);
    const enc = await input.encrypt();
    const tx = await this.contract.isIn_euint8_found(enc.handles[0], enc.inputProof, [1, 10, 42, 99]);
    await tx.wait();
    expect(await decryptBool(await this.contract.lastBool())).to.equal(true);
  });

  it('isIn(euint8): returns false when value is absent', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.add8(7n);
    const enc = await input.encrypt();
    const tx = await this.contract.isIn_euint8_found(enc.handles[0], enc.inputProof, [1, 10, 42, 99]);
    await tx.wait();
    expect(await decryptBool(await this.contract.lastBool())).to.equal(false);
  });

  it('isIn(euint8): n=1 found (short-circuit path)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.add8(5n);
    const enc = await input.encrypt();
    const tx = await this.contract.isIn_euint8_found(enc.handles[0], enc.inputProof, [5]);
    await tx.wait();
    expect(await decryptBool(await this.contract.lastBool())).to.equal(true);
  });

  it('isIn(euint8): n=1 not found (short-circuit path)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.add8(5n);
    const enc = await input.encrypt();
    const tx = await this.contract.isIn_euint8_found(enc.handles[0], enc.inputProof, [9]);
    await tx.wait();
    expect(await decryptBool(await this.contract.lastBool())).to.equal(false);
  });

  // -------------------------------------------------------------------------
  // isIn — euint32
  // -------------------------------------------------------------------------

  it('isIn(euint32): returns true when value is present', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.add32(1000n);
    const enc = await input.encrypt();
    const tx = await this.contract.isIn_euint32_found(enc.handles[0], enc.inputProof, [100, 500, 1000, 9999]);
    await tx.wait();
    expect(await decryptBool(await this.contract.lastBool())).to.equal(true);
  });

  it('isIn(euint32): returns false when value is absent', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.add32(123n);
    const enc = await input.encrypt();
    const tx = await this.contract.isIn_euint32_found(enc.handles[0], enc.inputProof, [100, 500, 1000, 9999]);
    await tx.wait();
    expect(await decryptBool(await this.contract.lastBool())).to.equal(false);
  });

  // -------------------------------------------------------------------------
  // isIn — euint64
  // -------------------------------------------------------------------------

  it('isIn(euint64): returns true when value is present', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.add64(999999999999n);
    const enc = await input.encrypt();
    const tx = await this.contract.isIn_euint64_found(enc.handles[0], enc.inputProof, [1n, 999999999999n, 42n]);
    await tx.wait();
    expect(await decryptBool(await this.contract.lastBool())).to.equal(true);
  });

  it('isIn(euint64): returns false when value is absent', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.add64(888888n);
    const enc = await input.encrypt();
    const tx = await this.contract.isIn_euint64_found(enc.handles[0], enc.inputProof, [1n, 999999999999n, 42n]);
    await tx.wait();
    expect(await decryptBool(await this.contract.lastBool())).to.equal(false);
  });

  // -------------------------------------------------------------------------
  // sum — euint8
  // -------------------------------------------------------------------------

  it('sum(euint8): sums a single element', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.add8(7n);
    const enc = await input.encrypt();
    const tx = await this.contract.sum_euint8(enc.handles, enc.inputProof);
    await tx.wait();
    expect(await decrypt8(await this.contract.lastUint8())).to.equal(7n);
  });

  it('sum(euint8): sums two elements', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.add8(3n);
    input.add8(5n);
    const enc = await input.encrypt();
    const tx = await this.contract.sum_euint8(enc.handles, enc.inputProof);
    await tx.wait();
    expect(await decrypt8(await this.contract.lastUint8())).to.equal(8n);
  });

  it('sum(euint8): sums an odd-length array (tree odd-carry path)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.add8(1n);
    input.add8(2n);
    input.add8(3n);
    const enc = await input.encrypt();
    const tx = await this.contract.sum_euint8(enc.handles, enc.inputProof);
    await tx.wait();
    expect(await decrypt8(await this.contract.lastUint8())).to.equal(6n);
  });

  it('sum(euint8): sums a power-of-two array', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.add8(10n);
    input.add8(20n);
    input.add8(30n);
    input.add8(40n);
    const enc = await input.encrypt();
    const tx = await this.contract.sum_euint8(enc.handles, enc.inputProof);
    await tx.wait();
    expect(await decrypt8(await this.contract.lastUint8())).to.equal(100n);
  });

  it('sum(euint8): wraps on overflow', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.add8(200n);
    input.add8(100n);
    const enc = await input.encrypt();
    const tx = await this.contract.sum_euint8(enc.handles, enc.inputProof);
    await tx.wait();
    // 200 + 100 = 300, wraps to 44 in uint8
    expect(await decrypt8(await this.contract.lastUint8())).to.equal(44n);
  });

  // -------------------------------------------------------------------------
  // sum — euint32
  // -------------------------------------------------------------------------

  it('sum(euint32): sums correctly', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.add32(1000n);
    input.add32(2000n);
    input.add32(3000n);
    const enc = await input.encrypt();
    const tx = await this.contract.sum_euint32(enc.handles, enc.inputProof);
    await tx.wait();
    expect(await decrypt32(await this.contract.lastUint32())).to.equal(6000n);
  });

  // -------------------------------------------------------------------------
  // sum — euint64
  // -------------------------------------------------------------------------

  it('sum(euint64): sums large values correctly', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.add64(1000000000n);
    input.add64(2000000000n);
    input.add64(3000000000n);
    const enc = await input.encrypt();
    const tx = await this.contract.sum_euint64(enc.handles, enc.inputProof);
    await tx.wait();
    expect(await decrypt64(await this.contract.lastUint64())).to.equal(6000000000n);
  });

  // -------------------------------------------------------------------------
  // sum: input array is not mutated
  // -------------------------------------------------------------------------

  it('sum(euint32): does not mutate the input array', async function () {
    const notMutated = await this.contract.sum_euint32_checkNoMutation.staticCall([10, 20, 30, 40]);
    expect(notMutated).to.equal(true);
  });
});
