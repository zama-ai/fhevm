import { expect } from 'chai';
import { ethers } from 'hardhat';

import type { FHEVMTestSuite1 } from '../../types/contracts/tests/FHEVMTestSuite1';
import type { FHEVMTestSuite2 } from '../../types/contracts/tests/FHEVMTestSuite2';
import type { FHEVMTestSuite3 } from '../../types/contracts/tests/FHEVMTestSuite3';
import type { FHEVMTestSuite4 } from '../../types/contracts/tests/FHEVMTestSuite4';
import type { FHEVMTestSuite5 } from '../../types/contracts/tests/FHEVMTestSuite5';
import type { FHEVMTestSuite6 } from '../../types/contracts/tests/FHEVMTestSuite6';
import type { FHEVMTestSuite7 } from '../../types/contracts/tests/FHEVMTestSuite7';
import {
  createInstances,
  decrypt8,
  decrypt16,
  decrypt32,
  decrypt64,
  decrypt128,
  decrypt256,
  decryptBool,
} from '../instance';
import { getSigners, initSigners } from '../signers';

async function deployFHEVMTestFixture1(): Promise<FHEVMTestSuite1> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite1');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture2(): Promise<FHEVMTestSuite2> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite2');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture3(): Promise<FHEVMTestSuite3> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite3');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture4(): Promise<FHEVMTestSuite4> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite4');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture5(): Promise<FHEVMTestSuite5> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite5');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture6(): Promise<FHEVMTestSuite6> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite6');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture7(): Promise<FHEVMTestSuite7> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite7');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

describe('FHEVM operations 4', function () {
  before(async function () {
    await initSigners(1);
    this.signers = await getSigners();

    const contract1 = await deployFHEVMTestFixture1();
    this.contract1Address = await contract1.getAddress();
    this.contract1 = contract1;

    const contract2 = await deployFHEVMTestFixture2();
    this.contract2Address = await contract2.getAddress();
    this.contract2 = contract2;

    const contract3 = await deployFHEVMTestFixture3();
    this.contract3Address = await contract3.getAddress();
    this.contract3 = contract3;

    const contract4 = await deployFHEVMTestFixture4();
    this.contract4Address = await contract4.getAddress();
    this.contract4 = contract4;

    const contract5 = await deployFHEVMTestFixture5();
    this.contract5Address = await contract5.getAddress();
    this.contract5 = contract5;

    const contract6 = await deployFHEVMTestFixture6();
    this.contract6Address = await contract6.getAddress();
    this.contract6 = contract6;

    const contract7 = await deployFHEVMTestFixture7();
    this.contract7Address = await contract7.getAddress();
    this.contract7 = contract7;

    const instances = await createInstances(this.signers);
    this.instances = instances;
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 1 (928574664, 45)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(928574664n);
    input.add8(45n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 2 (41, 45)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(41n);
    input.add8(45n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 3 (45, 45)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(45n);
    input.add8(45n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 4 (45, 41)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(45n);
    input.add8(41n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 1 (3408180882, 47)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(3408180882n);
    input.add8(47n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 2 (43, 47)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(43n);
    input.add8(47n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 3 (47, 47)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(47n);
    input.add8(47n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 4 (47, 43)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(47n);
    input.add8(43n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 1 (1348427722, 49)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(1348427722n);
    input.add8(49n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 2 (45, 49)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(45n);
    input.add8(49n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 3 (49, 49)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(49n);
    input.add8(49n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 4 (49, 45)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(49n);
    input.add8(45n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 1 (2743784517, 92)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(2743784517n);
    input.add8(92n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 2 (88, 92)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(88n);
    input.add8(92n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 3 (92, 92)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(92n);
    input.add8(92n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 4 (92, 88)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(92n);
    input.add8(88n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 1 (157572641, 83)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(157572641n);
    input.add8(83n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 2 (79, 83)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(79n);
    input.add8(83n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 3 (83, 83)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(83n);
    input.add8(83n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 4 (83, 79)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(83n);
    input.add8(79n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 1 (3940452686, 80)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(3940452686n);
    input.add8(80n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 2 (76, 80)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(76n);
    input.add8(80n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 3 (80, 80)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(80n);
    input.add8(80n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 4 (80, 76)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(80n);
    input.add8(76n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 1 (3113779294, 69)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(3113779294n);
    input.add8(69n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(69n);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 2 (65, 69)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(65n);
    input.add8(69n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(65n);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 3 (69, 69)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(69n);
    input.add8(69n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(69n);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 4 (69, 65)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(69n);
    input.add8(65n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(65n);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 1 (3087058654, 42)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(3087058654n);
    input.add8(42n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(3087058654n);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 2 (38, 42)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(38n);
    input.add8(42n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(42n);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 3 (42, 42)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(42n);
    input.add8(42n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(42n);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 4 (42, 38)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(42n);
    input.add8(38n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(42n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 1 (60753, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(60753n);
    input.add16(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(60756n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 2 (27186, 27190)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(27186n);
    input.add16(27190n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(54376n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 3 (27190, 27190)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(27190n);
    input.add16(27190n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(54380n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 4 (27190, 27186)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(27190n);
    input.add16(27186n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(54376n);
  });

  it('test operator "sub" overload (euint32, euint16) => euint32 test 1 (48306, 48306)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(48306n);
    input.add16(48306n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint16) => euint32 test 2 (48306, 48302)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(48306n);
    input.add16(48302n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 1 (20203, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(20203n);
    input.add16(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(40406n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 2 (190, 190)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(190n);
    input.add16(190n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(36100n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 3 (190, 190)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(190n);
    input.add16(190n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(36100n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 4 (190, 190)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(190n);
    input.add16(190n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(36100n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 1 (1834165412, 40108)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(1834165412n);
    input.add16(40108n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(1188n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 2 (40104, 40108)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(40104n);
    input.add16(40108n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(40104n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 3 (40108, 40108)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(40108n);
    input.add16(40108n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(40108n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 4 (40108, 40104)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(40108n);
    input.add16(40104n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(40104n);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 1 (2622445754, 22817)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(2622445754n);
    input.add16(22817n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(2622446011n);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 2 (22813, 22817)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(22813n);
    input.add16(22817n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(22845n);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 3 (22817, 22817)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(22817n);
    input.add16(22817n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(22817n);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 4 (22817, 22813)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(22817n);
    input.add16(22813n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(22845n);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 1 (718911449, 60491)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(718911449n);
    input.add16(60491n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(718887826n);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 2 (60487, 60491)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(60487n);
    input.add16(60491n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 3 (60491, 60491)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(60491n);
    input.add16(60491n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 4 (60491, 60487)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(60491n);
    input.add16(60487n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 1 (524855818, 2572)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(524855818n);
    input.add16(2572n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 2 (2568, 2572)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(2568n);
    input.add16(2572n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 3 (2572, 2572)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(2572n);
    input.add16(2572n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 4 (2572, 2568)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(2572n);
    input.add16(2568n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 1 (1951074594, 35540)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(1951074594n);
    input.add16(35540n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 2 (35536, 35540)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(35536n);
    input.add16(35540n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 3 (35540, 35540)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(35540n);
    input.add16(35540n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 4 (35540, 35536)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(35540n);
    input.add16(35536n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 1 (329344743, 19762)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(329344743n);
    input.add16(19762n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 2 (19758, 19762)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(19758n);
    input.add16(19762n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 3 (19762, 19762)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(19762n);
    input.add16(19762n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 4 (19762, 19758)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(19762n);
    input.add16(19758n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 1 (2286634721, 51712)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(2286634721n);
    input.add16(51712n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 2 (51708, 51712)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(51708n);
    input.add16(51712n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 3 (51712, 51712)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(51712n);
    input.add16(51712n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 4 (51712, 51708)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(51712n);
    input.add16(51708n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 1 (232161966, 45175)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(232161966n);
    input.add16(45175n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 2 (45171, 45175)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(45171n);
    input.add16(45175n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 3 (45175, 45175)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(45175n);
    input.add16(45175n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 4 (45175, 45171)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(45175n);
    input.add16(45171n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 1 (1490052603, 13245)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(1490052603n);
    input.add16(13245n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 2 (13241, 13245)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(13241n);
    input.add16(13245n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 3 (13245, 13245)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(13245n);
    input.add16(13245n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 4 (13245, 13241)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(13245n);
    input.add16(13241n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 1 (3033866947, 9185)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(3033866947n);
    input.add16(9185n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(9185n);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 2 (9181, 9185)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(9181n);
    input.add16(9185n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(9181n);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 3 (9185, 9185)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(9185n);
    input.add16(9185n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(9185n);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 4 (9185, 9181)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(9185n);
    input.add16(9181n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(9181n);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 1 (2362640443, 30129)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(2362640443n);
    input.add16(30129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(2362640443n);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 2 (30125, 30129)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(30125n);
    input.add16(30129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(30129n);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 3 (30129, 30129)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(30129n);
    input.add16(30129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(30129n);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 4 (30129, 30125)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(30129n);
    input.add16(30125n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(30129n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 1 (2078870183, 387745205)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(2078870183n);
    input.add32(387745205n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(2466615388n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 2 (775490404, 775490408)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(775490404n);
    input.add32(775490408n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(1550980812n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 3 (775490408, 775490408)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(775490408n);
    input.add32(775490408n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(1550980816n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 4 (775490408, 775490404)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(775490408n);
    input.add32(775490404n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(1550980812n);
  });

  it('test operator "sub" overload (euint32, euint32) => euint32 test 1 (15443261, 15443261)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(15443261n);
    input.add32(15443261n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint32) => euint32 test 2 (15443261, 15443257)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(15443261n);
    input.add32(15443257n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 1 (22470, 182136)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(22470n);
    input.add32(182136n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(4092595920n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 2 (44938, 44938)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(44938n);
    input.add32(44938n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(2019423844n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 3 (44938, 44938)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(44938n);
    input.add32(44938n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(2019423844n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 4 (44938, 44938)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(44938n);
    input.add32(44938n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(2019423844n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 1 (4243271706, 1169099340)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4243271706n);
    input.add32(1169099340n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(1152057352n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 2 (1169099336, 1169099340)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1169099336n);
    input.add32(1169099340n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(1169099336n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 3 (1169099340, 1169099340)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1169099340n);
    input.add32(1169099340n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(1169099340n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 4 (1169099340, 1169099336)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1169099340n);
    input.add32(1169099336n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(1169099336n);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 1 (1160502017, 1483702409)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1160502017n);
    input.add32(1483702409n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(1567611785n);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 2 (1160502013, 1160502017)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1160502013n);
    input.add32(1160502017n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(1160502269n);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 3 (1160502017, 1160502017)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1160502017n);
    input.add32(1160502017n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(1160502017n);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 4 (1160502017, 1160502013)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1160502017n);
    input.add32(1160502013n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(1160502269n);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 1 (37108106, 3772872733)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(37108106n);
    input.add32(3772872733n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(3805757847n);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 2 (37108102, 37108106)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(37108102n);
    input.add32(37108106n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 3 (37108106, 37108106)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(37108106n);
    input.add32(37108106n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 4 (37108106, 37108102)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(37108106n);
    input.add32(37108102n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 1 (1987927751, 3299870577)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1987927751n);
    input.add32(3299870577n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 2 (1987927747, 1987927751)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1987927747n);
    input.add32(1987927751n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 3 (1987927751, 1987927751)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1987927751n);
    input.add32(1987927751n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 4 (1987927751, 1987927747)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1987927751n);
    input.add32(1987927747n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 1 (984863625, 3431412474)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(984863625n);
    input.add32(3431412474n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 2 (984863621, 984863625)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(984863621n);
    input.add32(984863625n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 3 (984863625, 984863625)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(984863625n);
    input.add32(984863625n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 4 (984863625, 984863621)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(984863625n);
    input.add32(984863621n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 1 (4026144610, 3109238839)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4026144610n);
    input.add32(3109238839n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 2 (3109238835, 3109238839)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3109238835n);
    input.add32(3109238839n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 3 (3109238839, 3109238839)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3109238839n);
    input.add32(3109238839n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 4 (3109238839, 3109238835)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3109238839n);
    input.add32(3109238835n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 1 (319754915, 2254215359)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(319754915n);
    input.add32(2254215359n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 2 (319754911, 319754915)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(319754911n);
    input.add32(319754915n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 3 (319754915, 319754915)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(319754915n);
    input.add32(319754915n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 4 (319754915, 319754911)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(319754915n);
    input.add32(319754911n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 1 (3813081277, 1147958831)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3813081277n);
    input.add32(1147958831n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 2 (1147958827, 1147958831)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1147958827n);
    input.add32(1147958831n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 3 (1147958831, 1147958831)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1147958831n);
    input.add32(1147958831n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 4 (1147958831, 1147958827)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1147958831n);
    input.add32(1147958827n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 1 (2748635537, 958311251)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2748635537n);
    input.add32(958311251n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 2 (958311247, 958311251)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(958311247n);
    input.add32(958311251n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 3 (958311251, 958311251)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(958311251n);
    input.add32(958311251n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 4 (958311251, 958311247)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(958311251n);
    input.add32(958311247n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 1 (287613709, 1268459763)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(287613709n);
    input.add32(1268459763n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(287613709n);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 2 (287613705, 287613709)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(287613705n);
    input.add32(287613709n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(287613705n);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 3 (287613709, 287613709)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(287613709n);
    input.add32(287613709n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(287613709n);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 4 (287613709, 287613705)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(287613709n);
    input.add32(287613705n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(287613705n);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 1 (1500560192, 3872979860)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1500560192n);
    input.add32(3872979860n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(3872979860n);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 2 (1500560188, 1500560192)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1500560188n);
    input.add32(1500560192n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(1500560192n);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 3 (1500560192, 1500560192)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1500560192n);
    input.add32(1500560192n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(1500560192n);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 4 (1500560192, 1500560188)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1500560192n);
    input.add32(1500560188n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.resEuint32());
    expect(res).to.equal(1500560192n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 1 (2, 4293221806)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2n);
    input.add64(4293221806n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(4293221808n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 2 (2129264170, 2129264172)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2129264170n);
    input.add64(2129264172n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(4258528342n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 3 (2129264172, 2129264172)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2129264172n);
    input.add64(2129264172n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(4258528344n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 4 (2129264172, 2129264170)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2129264172n);
    input.add64(2129264170n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(4258528342n);
  });

  it('test operator "sub" overload (euint32, euint64) => euint64 test 1 (2514238580, 2514238580)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2514238580n);
    input.add64(2514238580n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint64) => euint64 test 2 (2514238580, 2514238576)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2514238580n);
    input.add64(2514238576n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 1 (2, 2147246739)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2n);
    input.add64(2147246739n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(4294493478n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 2 (54565, 54565)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(54565n);
    input.add64(54565n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(2977339225n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 3 (54565, 54565)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(54565n);
    input.add64(54565n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(2977339225n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 4 (54565, 54565)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(54565n);
    input.add64(54565n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(2977339225n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 1 (3530491980, 18439210276688399821)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3530491980n);
    input.add64(18439210276688399821n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(271452236n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 2 (3530491976, 3530491980)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3530491976n);
    input.add64(3530491980n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(3530491976n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 3 (3530491980, 3530491980)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3530491980n);
    input.add64(3530491980n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(3530491980n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 4 (3530491980, 3530491976)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3530491980n);
    input.add64(3530491976n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(3530491976n);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 1 (1265695443, 18446434545317482349)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1265695443n);
    input.add64(18446434545317482349n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18446434545339531263n);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 2 (1265695439, 1265695443)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1265695439n);
    input.add64(1265695443n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(1265695455n);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 3 (1265695443, 1265695443)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1265695443n);
    input.add64(1265695443n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(1265695443n);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 4 (1265695443, 1265695439)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1265695443n);
    input.add64(1265695439n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(1265695455n);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 1 (2058901138, 18444969346358607889)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2058901138n);
    input.add64(18444969346358607889n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18444969347791509123n);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 2 (2058901134, 2058901138)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2058901134n);
    input.add64(2058901138n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(28n);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 3 (2058901138, 2058901138)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2058901138n);
    input.add64(2058901138n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 4 (2058901138, 2058901134)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2058901138n);
    input.add64(2058901134n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(28n);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 1 (3353602442, 18444404744449370475)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3353602442n);
    input.add64(18444404744449370475n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 2 (3353602438, 3353602442)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3353602438n);
    input.add64(3353602442n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 3 (3353602442, 3353602442)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3353602442n);
    input.add64(3353602442n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 4 (3353602442, 3353602438)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3353602442n);
    input.add64(3353602438n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 1 (2348689264, 18442770455288909879)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2348689264n);
    input.add64(18442770455288909879n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 2 (2348689260, 2348689264)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2348689260n);
    input.add64(2348689264n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 3 (2348689264, 2348689264)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2348689264n);
    input.add64(2348689264n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 4 (2348689264, 2348689260)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2348689264n);
    input.add64(2348689260n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 1 (681513244, 18439956019762848847)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(681513244n);
    input.add64(18439956019762848847n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 2 (681513240, 681513244)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(681513240n);
    input.add64(681513244n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 3 (681513244, 681513244)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(681513244n);
    input.add64(681513244n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 4 (681513244, 681513240)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(681513244n);
    input.add64(681513240n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 1 (1663748873, 18438722170294938271)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1663748873n);
    input.add64(18438722170294938271n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 2 (1663748869, 1663748873)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1663748869n);
    input.add64(1663748873n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 3 (1663748873, 1663748873)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1663748873n);
    input.add64(1663748873n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 4 (1663748873, 1663748869)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1663748873n);
    input.add64(1663748869n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 1 (4007150973, 18440487152408523999)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4007150973n);
    input.add64(18440487152408523999n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 2 (4007150969, 4007150973)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4007150969n);
    input.add64(4007150973n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 3 (4007150973, 4007150973)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4007150973n);
    input.add64(4007150973n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 4 (4007150973, 4007150969)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4007150973n);
    input.add64(4007150969n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 1 (3941115237, 18441281291192038419)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3941115237n);
    input.add64(18441281291192038419n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 2 (3941115233, 3941115237)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3941115233n);
    input.add64(3941115237n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 3 (3941115237, 3941115237)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3941115237n);
    input.add64(3941115237n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 4 (3941115237, 3941115233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3941115237n);
    input.add64(3941115233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 1 (3308075156, 18441702414813729607)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3308075156n);
    input.add64(18441702414813729607n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(3308075156n);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 2 (3308075152, 3308075156)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3308075152n);
    input.add64(3308075156n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(3308075152n);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 3 (3308075156, 3308075156)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3308075156n);
    input.add64(3308075156n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(3308075156n);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 4 (3308075156, 3308075152)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3308075156n);
    input.add64(3308075152n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(3308075152n);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 1 (529176878, 18440957268015307015)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(529176878n);
    input.add64(18440957268015307015n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18440957268015307015n);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 2 (529176874, 529176878)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(529176874n);
    input.add64(529176878n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(529176878n);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 3 (529176878, 529176878)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(529176878n);
    input.add64(529176878n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(529176878n);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 4 (529176878, 529176874)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(529176878n);
    input.add64(529176874n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(529176878n);
  });

  it('test operator "add" overload (euint32, euint128) => euint128 test 1 (2, 2147483649)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2n);
    input.add128(2147483649n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(2147483651n);
  });

  it('test operator "add" overload (euint32, euint128) => euint128 test 2 (1269503551, 1269503553)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1269503551n);
    input.add128(1269503553n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(2539007104n);
  });

  it('test operator "add" overload (euint32, euint128) => euint128 test 3 (1269503553, 1269503553)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1269503553n);
    input.add128(1269503553n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(2539007106n);
  });

  it('test operator "add" overload (euint32, euint128) => euint128 test 4 (1269503553, 1269503551)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1269503553n);
    input.add128(1269503551n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(2539007104n);
  });

  it('test operator "sub" overload (euint32, euint128) => euint128 test 1 (1541154845, 1541154845)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1541154845n);
    input.add128(1541154845n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint128) => euint128 test 2 (1541154845, 1541154841)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1541154845n);
    input.add128(1541154841n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(4n);
  });
});
