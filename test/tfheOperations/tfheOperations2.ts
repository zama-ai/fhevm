import { expect } from 'chai';
import { ethers } from 'hardhat';

import type { TFHETestSuite1 } from '../../types/contracts/tests/TFHETestSuite1';
import type { TFHETestSuite2 } from '../../types/contracts/tests/TFHETestSuite2';
import type { TFHETestSuite3 } from '../../types/contracts/tests/TFHETestSuite3';
import type { TFHETestSuite4 } from '../../types/contracts/tests/TFHETestSuite4';
import type { TFHETestSuite5 } from '../../types/contracts/tests/TFHETestSuite5';
import type { TFHETestSuite6 } from '../../types/contracts/tests/TFHETestSuite6';
import { createInstances, decrypt4, decrypt8, decrypt16, decrypt32, decrypt64, decryptBool } from '../instance';
import { getSigners, initSigners } from '../signers';

async function deployTfheTestFixture1(): Promise<TFHETestSuite1> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite1');
  const contract = await contractFactory.connect(admin).deploy({
    value: ethers.parseEther('0.001'),
  });
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture2(): Promise<TFHETestSuite2> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite2');
  const contract = await contractFactory.connect(admin).deploy({
    value: ethers.parseEther('0.001'),
  });
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture3(): Promise<TFHETestSuite3> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite3');
  const contract = await contractFactory.connect(admin).deploy({
    value: ethers.parseEther('0.001'),
  });
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture4(): Promise<TFHETestSuite4> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite4');
  const contract = await contractFactory.connect(admin).deploy({
    value: ethers.parseEther('0.001'),
  });
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture5(): Promise<TFHETestSuite5> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite5');
  const contract = await contractFactory.connect(admin).deploy({
    value: ethers.parseEther('0.001'),
  });
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture6(): Promise<TFHETestSuite6> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite6');
  const contract = await contractFactory.connect(admin).deploy({
    value: ethers.parseEther('0.001'),
  });
  await contract.waitForDeployment();

  return contract;
}

describe('TFHE operations 2', function () {
  before(async function () {
    await initSigners(1);
    this.signers = await getSigners();

    const contract1 = await deployTfheTestFixture1();
    this.contract1Address = await contract1.getAddress();
    this.contract1 = contract1;

    const contract2 = await deployTfheTestFixture2();
    this.contract2Address = await contract2.getAddress();
    this.contract2 = contract2;

    const contract3 = await deployTfheTestFixture3();
    this.contract3Address = await contract3.getAddress();
    this.contract3 = contract3;

    const contract4 = await deployTfheTestFixture4();
    this.contract4Address = await contract4.getAddress();
    this.contract4 = contract4;

    const contract5 = await deployTfheTestFixture5();
    this.contract5Address = await contract5.getAddress();
    this.contract5 = contract5;

    const contract6 = await deployTfheTestFixture6();
    this.contract6Address = await contract6.getAddress();
    this.contract6 = contract6;

    const instances = await createInstances(this.signers);
    this.instances = instances;
  });

  it('test operator "sub" overload (euint4, euint32) => euint32 test 1 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add32(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint4, euint32) => euint32 test 2 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add32(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint4, euint32) => euint32 test 1 (2, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(2n);
    input.add32(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(10n);
  });

  it('test operator "mul" overload (euint4, euint32) => euint32 test 2 (3, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(3n);
    input.add32(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, euint32) => euint32 test 3 (3, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(3n);
    input.add32(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, euint32) => euint32 test 4 (3, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(3n);
    input.add32(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(9n);
  });

  it('test operator "and" overload (euint4, euint32) => euint32 test 1 (1, 1067388092)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(1n);
    input.add32(1067388092n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint4, euint32) => euint32 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add32(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint4, euint32) => euint32 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add32(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint4, euint32) => euint32 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add32(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint4, euint32) => euint32 test 1 (12, 3045135587)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(12n);
    input.add32(3045135587n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(3045135599n);
  });

  it('test operator "or" overload (euint4, euint32) => euint32 test 2 (8, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add32(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint4, euint32) => euint32 test 3 (12, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(12n);
    input.add32(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint4, euint32) => euint32 test 4 (12, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(12n);
    input.add32(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint4, euint32) => euint32 test 1 (6, 1855370868)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(6n);
    input.add32(1855370868n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(1855370866n);
  });

  it('test operator "xor" overload (euint4, euint32) => euint32 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add32(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint4, euint32) => euint32 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add32(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint4, euint32) => euint32 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add32(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint4, euint32) => ebool test 1 (2, 183972195)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(2n);
    input.add32(183972195n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint32) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add32(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint32) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add32(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, euint32) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add32(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint32) => ebool test 1 (14, 1312265251)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(14n);
    input.add32(1312265251n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint32) => ebool test 2 (10, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add32(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint32) => ebool test 3 (14, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(14n);
    input.add32(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint32) => ebool test 4 (14, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(14n);
    input.add32(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint32) => ebool test 1 (3, 735253791)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(3n);
    input.add32(735253791n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint32) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add32(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint32) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add32(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint32) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add32(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint4, euint32) => ebool test 1 (1, 2112514560)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(1n);
    input.add32(2112514560n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint32) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add32(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint32) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add32(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint32) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add32(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint32) => ebool test 1 (5, 3704560179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(5n);
    input.add32(3704560179n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint32) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add32(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint32) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add32(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint32) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add32(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint32) => ebool test 1 (6, 2034411051)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(6n);
    input.add32(2034411051n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint32) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add32(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint32) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add32(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint32) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add32(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint4, euint32) => euint32 test 1 (10, 421626494)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add32(421626494n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(10n);
  });

  it('test operator "min" overload (euint4, euint32) => euint32 test 2 (6, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(6n);
    input.add32(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(6n);
  });

  it('test operator "min" overload (euint4, euint32) => euint32 test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add32(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(10n);
  });

  it('test operator "min" overload (euint4, euint32) => euint32 test 4 (10, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add32(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(6n);
  });

  it('test operator "max" overload (euint4, euint32) => euint32 test 1 (12, 3887686486)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(12n);
    input.add32(3887686486n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(3887686486n);
  });

  it('test operator "max" overload (euint4, euint32) => euint32 test 2 (8, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add32(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(12n);
  });

  it('test operator "max" overload (euint4, euint32) => euint32 test 3 (12, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(12n);
    input.add32(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(12n);
  });

  it('test operator "max" overload (euint4, euint32) => euint32 test 4 (12, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(12n);
    input.add32(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint4, euint64) => euint64 test 1 (2, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(2n);
    input.add64(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(11n);
  });

  it('test operator "add" overload (euint4, euint64) => euint64 test 2 (6, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(6n);
    input.add64(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(14n);
  });

  it('test operator "add" overload (euint4, euint64) => euint64 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(5n);
    input.add64(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint4, euint64) => euint64 test 4 (8, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add64(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(14n);
  });

  it('test operator "sub" overload (euint4, euint64) => euint64 test 1 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add64(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint4, euint64) => euint64 test 2 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add64(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint4, euint64) => euint64 test 1 (2, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(2n);
    input.add64(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(10n);
  });

  it('test operator "mul" overload (euint4, euint64) => euint64 test 2 (3, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(3n);
    input.add64(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(15n);
  });

  it('test operator "mul" overload (euint4, euint64) => euint64 test 3 (3, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(3n);
    input.add64(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, euint64) => euint64 test 4 (5, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(5n);
    input.add64(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(15n);
  });

  it('test operator "and" overload (euint4, euint64) => euint64 test 1 (11, 18444970625118669221)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(11n);
    input.add64(18444970625118669221n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(1n);
  });

  it('test operator "and" overload (euint4, euint64) => euint64 test 2 (7, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(7n);
    input.add64(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(3n);
  });

  it('test operator "and" overload (euint4, euint64) => euint64 test 3 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(11n);
    input.add64(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(11n);
  });

  it('test operator "and" overload (euint4, euint64) => euint64 test 4 (11, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(11n);
    input.add64(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(3n);
  });

  it('test operator "or" overload (euint4, euint64) => euint64 test 1 (5, 18439458059788568419)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(5n);
    input.add64(18439458059788568419n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(18439458059788568423n);
  });

  it('test operator "or" overload (euint4, euint64) => euint64 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add64(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint4, euint64) => euint64 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add64(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint4, euint64) => euint64 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add64(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint4, euint64) => euint64 test 1 (2, 18446366816185499095)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(2n);
    input.add64(18446366816185499095n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(18446366816185499093n);
  });

  it('test operator "xor" overload (euint4, euint64) => euint64 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add64(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint4, euint64) => euint64 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add64(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint4, euint64) => euint64 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add64(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint4, euint64) => ebool test 1 (9, 18438414163462572393)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);
    input.add64(18438414163462572393n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint64) => ebool test 2 (5, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(5n);
    input.add64(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint64) => ebool test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);
    input.add64(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, euint64) => ebool test 4 (9, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);
    input.add64(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint64) => ebool test 1 (7, 18441024342807508949)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(7n);
    input.add64(18441024342807508949n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint64) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add64(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint64) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add64(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint64) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add64(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint64) => ebool test 1 (7, 18446430329213146395)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(7n);
    input.add64(18446430329213146395n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint64) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add64(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint64) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add64(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint64) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add64(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint4, euint64) => ebool test 1 (10, 18442574847292216251)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add64(18442574847292216251n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint64) => ebool test 2 (6, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(6n);
    input.add64(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint64) => ebool test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add64(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint64) => ebool test 4 (10, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add64(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint64) => ebool test 1 (9, 18445100928210525947)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);
    input.add64(18445100928210525947n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint64) => ebool test 2 (5, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(5n);
    input.add64(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint64) => ebool test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);
    input.add64(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint64) => ebool test 4 (9, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);
    input.add64(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint64) => ebool test 1 (7, 18445391148064274615)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(7n);
    input.add64(18445391148064274615n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint64) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add64(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint64) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add64(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint64) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add64(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint4, euint64) => euint64 test 1 (12, 18438956406288597713)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(12n);
    input.add64(18438956406288597713n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(12n);
  });

  it('test operator "min" overload (euint4, euint64) => euint64 test 2 (8, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add64(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint4, euint64) => euint64 test 3 (12, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(12n);
    input.add64(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(12n);
  });

  it('test operator "min" overload (euint4, euint64) => euint64 test 4 (12, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(12n);
    input.add64(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint4, euint64) => euint64 test 1 (5, 18446478225770660877)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(5n);
    input.add64(18446478225770660877n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(18446478225770660877n);
  });

  it('test operator "max" overload (euint4, euint64) => euint64 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add64(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint4, euint64) => euint64 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add64(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint4, euint64) => euint64 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add64(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint4, uint8) => euint4 test 1 (1, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(1n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_uint8(encryptedAmount.handles[0], 1n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(2n);
  });

  it('test operator "add" overload (euint4, uint8) => euint4 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint4, uint8) => euint4 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint4, uint8) => euint4 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (uint8, euint4) => euint4 test 1 (1, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_uint8_euint4(1n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(3n);
  });

  it('test operator "add" overload (uint8, euint4) => euint4 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_uint8_euint4(4n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (uint8, euint4) => euint4 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_uint8_euint4(5n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (uint8, euint4) => euint4 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_uint8_euint4(8n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint4, uint8) => euint4 test 1 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint4_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint4, uint8) => euint4 test 2 (10, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint4_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint8, euint4) => euint4 test 1 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_uint8_euint4(9n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint8, euint4) => euint4 test 2 (9, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_uint8_euint4(9n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint4, uint8) => euint4 test 1 (5, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_uint8(encryptedAmount.handles[0], 2n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(10n);
  });

  it('test operator "mul" overload (euint4, uint8) => euint4 test 2 (3, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(3n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_uint8(encryptedAmount.handles[0], 3n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, uint8) => euint4 test 3 (3, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(3n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_uint8(encryptedAmount.handles[0], 3n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, uint8) => euint4 test 4 (3, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(3n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_uint8(encryptedAmount.handles[0], 3n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (uint8, euint4) => euint4 test 1 (3, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_uint8_euint4(3n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (uint8, euint4) => euint4 test 2 (3, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_uint8_euint4(3n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(15n);
  });

  it('test operator "mul" overload (uint8, euint4) => euint4 test 3 (3, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_uint8_euint4(3n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (uint8, euint4) => euint4 test 4 (5, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_uint8_euint4(5n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(15n);
  });

  it('test operator "div" overload (euint4, uint8) => euint4 test 1 (14, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(14n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.div_euint4_uint8(encryptedAmount.handles[0], 14n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint4, uint8) => euint4 test 2 (10, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.div_euint4_uint8(encryptedAmount.handles[0], 14n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint4, uint8) => euint4 test 3 (14, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(14n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.div_euint4_uint8(encryptedAmount.handles[0], 14n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint4, uint8) => euint4 test 4 (14, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(14n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.div_euint4_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint4, uint8) => euint4 test 1 (9, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.rem_euint4_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(4n);
  });

  it('test operator "rem" overload (euint4, uint8) => euint4 test 2 (5, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.rem_euint4_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(5n);
  });

  it('test operator "rem" overload (euint4, uint8) => euint4 test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.rem_euint4_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint4, uint8) => euint4 test 4 (9, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.rem_euint4_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint4, uint8) => ebool test 1 (4, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_uint8(encryptedAmount.handles[0], 1n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, uint8) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, uint8) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, uint8) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint4) => ebool test 1 (2, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_uint8_euint4(2n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint4) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_uint8_euint4(4n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint4) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_uint8_euint4(8n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint8, euint4) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_uint8_euint4(8n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, uint8) => ebool test 1 (4, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, uint8) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, uint8) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, uint8) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint4) => ebool test 1 (3, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_uint8_euint4(3n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint4) => ebool test 2 (10, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_uint8_euint4(10n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint4) => ebool test 3 (14, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_uint8_euint4(14n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint8, euint4) => ebool test 4 (14, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_uint8_euint4(14n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, uint8) => ebool test 1 (13, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_uint8(encryptedAmount.handles[0], 2n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, uint8) => ebool test 2 (9, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_uint8(encryptedAmount.handles[0], 13n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, uint8) => ebool test 3 (13, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_uint8(encryptedAmount.handles[0], 13n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, uint8) => ebool test 4 (13, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint8, euint4) => ebool test 1 (1, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_uint8_euint4(1n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint8, euint4) => ebool test 2 (8, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_uint8_euint4(8n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint8, euint4) => ebool test 3 (12, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_uint8_euint4(12n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint8, euint4) => ebool test 4 (12, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_uint8_euint4(12n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint4, uint8) => ebool test 1 (14, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(14n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_uint8(encryptedAmount.handles[0], 13n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint4, uint8) => ebool test 2 (10, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_uint8(encryptedAmount.handles[0], 14n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, uint8) => ebool test 3 (14, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(14n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_uint8(encryptedAmount.handles[0], 14n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, uint8) => ebool test 4 (14, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(14n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint8, euint4) => ebool test 1 (1, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_uint8_euint4(1n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint4) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_uint8_euint4(4n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint4) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_uint8_euint4(8n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint4) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);

    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_uint8_euint4(8n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });
});
