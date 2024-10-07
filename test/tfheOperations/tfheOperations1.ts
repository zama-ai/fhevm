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
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture2(): Promise<TFHETestSuite2> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite2');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture3(): Promise<TFHETestSuite3> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite3');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture4(): Promise<TFHETestSuite4> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite4');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture5(): Promise<TFHETestSuite5> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite5');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture6(): Promise<TFHETestSuite6> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite6');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

describe('TFHE operations 1', function () {
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

  it('test operator "add" overload (euint4, euint4) => euint4 test 1 (3, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(3n);
    input.add4(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(5n);
  });

  it('test operator "add" overload (euint4, euint4) => euint4 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint4, euint4) => euint4 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(5n);
    input.add4(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint4, euint4) => euint4 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint4, euint4) => euint4 test 1 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint4, euint4) => euint4 test 2 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint4, euint4) => euint4 test 1 (3, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(3n);
    input.add4(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, euint4) => euint4 test 2 (3, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(3n);
    input.add4(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(15n);
  });

  it('test operator "mul" overload (euint4, euint4) => euint4 test 3 (3, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(3n);
    input.add4(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, euint4) => euint4 test 4 (5, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(5n);
    input.add4(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(15n);
  });

  it('test operator "and" overload (euint4, euint4) => euint4 test 1 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint4, euint4) => euint4 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint4, euint4) => euint4 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint4, euint4) => euint4 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint4, euint4) => euint4 test 1 (7, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(7n);
    input.add4(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(15n);
  });

  it('test operator "or" overload (euint4, euint4) => euint4 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint4, euint4) => euint4 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint4, euint4) => euint4 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint4, euint4) => euint4 test 1 (13, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);
    input.add4(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(3n);
  });

  it('test operator "xor" overload (euint4, euint4) => euint4 test 2 (9, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);
    input.add4(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint4, euint4) => euint4 test 3 (13, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);
    input.add4(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint4, euint4) => euint4 test 4 (13, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);
    input.add4(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint4, euint4) => ebool test 1 (6, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(6n);
    input.add4(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint4) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint4) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, euint4) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint4) => ebool test 1 (11, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(11n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint4) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint4) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint4) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint4) => ebool test 1 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);
    input.add4(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint4) => ebool test 2 (5, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(5n);
    input.add4(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint4) => ebool test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);
    input.add4(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint4) => ebool test 4 (9, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);
    input.add4(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint4, euint4) => ebool test 1 (4, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add4(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint4) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint4) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint4) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint4) => ebool test 1 (10, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add4(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint4, euint4) => ebool test 2 (5, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(5n);
    input.add4(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint4) => ebool test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);
    input.add4(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint4) => ebool test 4 (9, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);
    input.add4(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint4) => ebool test 1 (4, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add4(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint4) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint4) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint4) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint4, euint4) => euint4 test 1 (7, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(7n);
    input.add4(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(7n);
  });

  it('test operator "min" overload (euint4, euint4) => euint4 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint4, euint4) => euint4 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint4, euint4) => euint4 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint4, euint4) => euint4 test 1 (3, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(3n);
    input.add4(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(3n);
  });

  it('test operator "max" overload (euint4, euint4) => euint4 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint4, euint4) => euint4 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint4, euint4) => euint4 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint4, euint8) => euint8 test 1 (1, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(1n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint4, euint8) => euint8 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint4, euint8) => euint8 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(5n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint4, euint8) => euint8 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint4, euint8) => euint8 test 1 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint4, euint8) => euint8 test 2 (10, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint4, euint8) => euint8 test 1 (2, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(2n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(10n);
  });

  it('test operator "mul" overload (euint4, euint8) => euint8 test 2 (3, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(3n);
    input.add8(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, euint8) => euint8 test 3 (3, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(3n);
    input.add8(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, euint8) => euint8 test 4 (3, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(3n);
    input.add8(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(9n);
  });

  it('test operator "and" overload (euint4, euint8) => euint8 test 1 (10, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(2n);
  });

  it('test operator "and" overload (euint4, euint8) => euint8 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint4, euint8) => euint8 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint4, euint8) => euint8 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint4, euint8) => euint8 test 1 (12, 18)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(12n);
    input.add8(18n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(30n);
  });

  it('test operator "or" overload (euint4, euint8) => euint8 test 2 (8, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add8(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint4, euint8) => euint8 test 3 (12, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(12n);
    input.add8(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint4, euint8) => euint8 test 4 (12, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(12n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint4, euint8) => euint8 test 1 (5, 19)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(5n);
    input.add8(19n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(22n);
  });

  it('test operator "xor" overload (euint4, euint8) => euint8 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint4, euint8) => euint8 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint4, euint8) => euint8 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint4, euint8) => ebool test 1 (4, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint8) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint8) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, euint8) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint8) => ebool test 1 (4, 62)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add8(62n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint8) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint8) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint8) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint8) => ebool test 1 (13, 91)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);
    input.add8(91n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint8) => ebool test 2 (9, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);
    input.add8(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint8) => ebool test 3 (13, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);
    input.add8(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint8) => ebool test 4 (13, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint4, euint8) => ebool test 1 (14, 199)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(14n);
    input.add8(199n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint8) => ebool test 2 (10, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add8(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint8) => ebool test 3 (14, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(14n);
    input.add8(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint8) => ebool test 4 (14, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(14n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint8) => ebool test 1 (13, 94)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);
    input.add8(94n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint8) => ebool test 2 (9, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);
    input.add8(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint8) => ebool test 3 (13, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);
    input.add8(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint8) => ebool test 4 (13, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint8) => ebool test 1 (13, 128)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);
    input.add8(128n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint8) => ebool test 2 (9, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);
    input.add8(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint8) => ebool test 3 (13, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);
    input.add8(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint8) => ebool test 4 (13, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint4, euint8) => euint8 test 1 (8, 124)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add8(124n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint4, euint8) => euint8 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint4, euint8) => euint8 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint4, euint8) => euint8 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint4, euint8) => euint8 test 1 (1, 157)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(1n);
    input.add8(157n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(157n);
  });

  it('test operator "max" overload (euint4, euint8) => euint8 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint4, euint8) => euint8 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint4, euint8) => euint8 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint4, euint16) => euint16 test 1 (2, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(2n);
    input.add16(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(15n);
  });

  it('test operator "add" overload (euint4, euint16) => euint16 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add16(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint4, euint16) => euint16 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(5n);
    input.add16(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint4, euint16) => euint16 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add16(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint4, euint16) => euint16 test 1 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(11n);
    input.add16(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint4, euint16) => euint16 test 2 (11, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(11n);
    input.add16(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint4, euint16) => euint16 test 1 (2, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(2n);
    input.add16(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(10n);
  });

  it('test operator "mul" overload (euint4, euint16) => euint16 test 2 (3, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(3n);
    input.add16(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(15n);
  });

  it('test operator "mul" overload (euint4, euint16) => euint16 test 3 (3, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(3n);
    input.add16(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, euint16) => euint16 test 4 (5, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(5n);
    input.add16(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(15n);
  });

  it('test operator "and" overload (euint4, euint16) => euint16 test 1 (10, 28165)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add16(28165n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint4, euint16) => euint16 test 2 (6, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(6n);
    input.add16(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(2n);
  });

  it('test operator "and" overload (euint4, euint16) => euint16 test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add16(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(10n);
  });

  it('test operator "and" overload (euint4, euint16) => euint16 test 4 (10, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add16(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(2n);
  });

  it('test operator "or" overload (euint4, euint16) => euint16 test 1 (12, 197)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(12n);
    input.add16(197n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(205n);
  });

  it('test operator "or" overload (euint4, euint16) => euint16 test 2 (8, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add16(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint4, euint16) => euint16 test 3 (12, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(12n);
    input.add16(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint4, euint16) => euint16 test 4 (12, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(12n);
    input.add16(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint4, euint16) => euint16 test 1 (10, 13907)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add16(13907n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(13913n);
  });

  it('test operator "xor" overload (euint4, euint16) => euint16 test 2 (6, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(6n);
    input.add16(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint4, euint16) => euint16 test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add16(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint4, euint16) => euint16 test 4 (10, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add16(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint4, euint16) => ebool test 1 (10, 41899)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add16(41899n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint16) => ebool test 2 (6, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(6n);
    input.add16(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint16) => ebool test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add16(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, euint16) => ebool test 4 (10, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add16(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint16) => ebool test 1 (3, 41708)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(3n);
    input.add16(41708n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint16) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add16(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint16) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add16(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint16) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add16(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint16) => ebool test 1 (5, 34397)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(5n);
    input.add16(34397n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint16) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add16(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint16) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add16(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint16) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add16(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint4, euint16) => ebool test 1 (10, 34348)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add16(34348n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint16) => ebool test 2 (6, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(6n);
    input.add16(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint16) => ebool test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add16(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint16) => ebool test 4 (10, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add16(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint16) => ebool test 1 (3, 38645)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(3n);
    input.add16(38645n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint16) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add16(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint16) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add16(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint16) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add16(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint16) => ebool test 1 (10, 15096)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add16(15096n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint16) => ebool test 2 (6, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(6n);
    input.add16(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint16) => ebool test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add16(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint16) => ebool test 4 (10, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add16(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint4, euint16) => euint16 test 1 (12, 47994)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(12n);
    input.add16(47994n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(12n);
  });

  it('test operator "min" overload (euint4, euint16) => euint16 test 2 (8, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add16(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint4, euint16) => euint16 test 3 (12, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(12n);
    input.add16(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(12n);
  });

  it('test operator "min" overload (euint4, euint16) => euint16 test 4 (12, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(12n);
    input.add16(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint4, euint16) => euint16 test 1 (6, 62382)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(6n);
    input.add16(62382n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(62382n);
  });

  it('test operator "max" overload (euint4, euint16) => euint16 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add16(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint4, euint16) => euint16 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add16(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint4, euint16) => euint16 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add16(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint4, euint32) => euint32 test 1 (2, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(2n);
    input.add32(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint4, euint32) => euint32 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add32(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint4, euint32) => euint32 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(5n);
    input.add32(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint4, euint32) => euint32 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add32(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(12n);
  });
});
