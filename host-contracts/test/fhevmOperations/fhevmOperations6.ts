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

describe('FHEVM operations 6', function () {
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

  it('test operator "ne" overload (euint64, euint32) => ebool test 1 (18444565548408164729, 332411040)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18444565548408164729n);
    input.add32(332411040n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 2 (332411036, 332411040)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(332411036n);
    input.add32(332411040n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 3 (332411040, 332411040)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(332411040n);
    input.add32(332411040n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 4 (332411040, 332411036)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(332411040n);
    input.add32(332411036n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 1 (18438656337905881701, 1409884702)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18438656337905881701n);
    input.add32(1409884702n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 2 (1409884698, 1409884702)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1409884698n);
    input.add32(1409884702n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 3 (1409884702, 1409884702)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1409884702n);
    input.add32(1409884702n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 4 (1409884702, 1409884698)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1409884702n);
    input.add32(1409884698n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 1 (18442698536289009329, 1123305342)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18442698536289009329n);
    input.add32(1123305342n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 2 (1123305338, 1123305342)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1123305338n);
    input.add32(1123305342n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 3 (1123305342, 1123305342)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1123305342n);
    input.add32(1123305342n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 4 (1123305342, 1123305338)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1123305342n);
    input.add32(1123305338n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 1 (18440734311689478421, 3383538917)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18440734311689478421n);
    input.add32(3383538917n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 2 (3383538913, 3383538917)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(3383538913n);
    input.add32(3383538917n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 3 (3383538917, 3383538917)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(3383538917n);
    input.add32(3383538917n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 4 (3383538917, 3383538913)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(3383538917n);
    input.add32(3383538913n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 1 (18439541790773932503, 3803842460)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18439541790773932503n);
    input.add32(3803842460n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 2 (3803842456, 3803842460)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(3803842456n);
    input.add32(3803842460n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 3 (3803842460, 3803842460)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(3803842460n);
    input.add32(3803842460n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 4 (3803842460, 3803842456)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(3803842460n);
    input.add32(3803842456n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 1 (18445407910931120527, 1114644634)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18445407910931120527n);
    input.add32(1114644634n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(1114644634n);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 2 (1114644630, 1114644634)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1114644630n);
    input.add32(1114644634n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(1114644630n);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 3 (1114644634, 1114644634)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1114644634n);
    input.add32(1114644634n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(1114644634n);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 4 (1114644634, 1114644630)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1114644634n);
    input.add32(1114644630n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(1114644630n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 1 (18439241143652599977, 3431527089)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18439241143652599977n);
    input.add32(3431527089n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18439241143652599977n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 2 (3431527085, 3431527089)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(3431527085n);
    input.add32(3431527089n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(3431527089n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 3 (3431527089, 3431527089)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(3431527089n);
    input.add32(3431527089n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(3431527089n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 4 (3431527089, 3431527085)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(3431527089n);
    input.add32(3431527085n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(3431527089n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 1 (9222015721118456611, 9221149604385772711)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(9222015721118456611n);
    input.add64(9221149604385772711n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18443165325504229322n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 2 (9221149604385772709, 9221149604385772711)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(9221149604385772709n);
    input.add64(9221149604385772711n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18442299208771545420n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 3 (9221149604385772711, 9221149604385772711)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(9221149604385772711n);
    input.add64(9221149604385772711n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18442299208771545422n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 4 (9221149604385772711, 9221149604385772709)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(9221149604385772711n);
    input.add64(9221149604385772709n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18442299208771545420n);
  });

  it('test operator "sub" overload (euint64, euint64) => euint64 test 1 (18444880651513884263, 18444880651513884263)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18444880651513884263n);
    input.add64(18444880651513884263n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint64) => euint64 test 2 (18444880651513884263, 18444880651513884259)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18444880651513884263n);
    input.add64(18444880651513884259n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 1 (4293509571, 4293907104)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(4293509571n);
    input.add64(4293907104n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18435931248008892384n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 2 (4293509571, 4293509571)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(4293509571n);
    input.add64(4293509571n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18434224436268604041n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 3 (4293509571, 4293509571)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(4293509571n);
    input.add64(4293509571n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18434224436268604041n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 4 (4293509571, 4293509571)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(4293509571n);
    input.add64(4293509571n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18434224436268604041n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 1 (18442806918536224481, 18445883338193704967)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442806918536224481n);
    input.add64(18445883338193704967n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18442241745934948353n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 2 (18442806918536224477, 18442806918536224481)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442806918536224477n);
    input.add64(18442806918536224481n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18442806918536224449n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 3 (18442806918536224481, 18442806918536224481)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442806918536224481n);
    input.add64(18442806918536224481n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18442806918536224481n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 4 (18442806918536224481, 18442806918536224477)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442806918536224481n);
    input.add64(18442806918536224477n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18442806918536224449n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 1 (18442460373850523907, 18439455727192157821)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442460373850523907n);
    input.add64(18439455727192157821n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18444175614410010495n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 2 (18439455727192157817, 18439455727192157821)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439455727192157817n);
    input.add64(18439455727192157821n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18439455727192157821n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 3 (18439455727192157821, 18439455727192157821)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439455727192157821n);
    input.add64(18439455727192157821n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18439455727192157821n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 4 (18439455727192157821, 18439455727192157817)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439455727192157821n);
    input.add64(18439455727192157817n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18439455727192157821n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 1 (18446689090961918239, 18441841593257065797)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18446689090961918239n);
    input.add64(18441841593257065797n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(4882682144601178n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 2 (18441841593257065793, 18441841593257065797)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441841593257065793n);
    input.add64(18441841593257065797n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 3 (18441841593257065797, 18441841593257065797)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441841593257065797n);
    input.add64(18441841593257065797n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 4 (18441841593257065797, 18441841593257065793)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441841593257065797n);
    input.add64(18441841593257065793n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 1 (18441018419551902661, 18438502160708961977)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441018419551902661n);
    input.add64(18438502160708961977n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 2 (18438502160708961973, 18438502160708961977)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438502160708961973n);
    input.add64(18438502160708961977n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 3 (18438502160708961977, 18438502160708961977)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438502160708961977n);
    input.add64(18438502160708961977n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 4 (18438502160708961977, 18438502160708961973)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438502160708961977n);
    input.add64(18438502160708961973n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 1 (18445073496792916493, 18445919609823520349)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445073496792916493n);
    input.add64(18445919609823520349n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 2 (18445073496792916489, 18445073496792916493)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445073496792916489n);
    input.add64(18445073496792916493n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 3 (18445073496792916493, 18445073496792916493)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445073496792916493n);
    input.add64(18445073496792916493n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 4 (18445073496792916493, 18445073496792916489)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445073496792916493n);
    input.add64(18445073496792916489n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 1 (18445142250206032657, 18443006402125177743)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445142250206032657n);
    input.add64(18443006402125177743n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 2 (18443006402125177739, 18443006402125177743)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443006402125177739n);
    input.add64(18443006402125177743n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 3 (18443006402125177743, 18443006402125177743)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443006402125177743n);
    input.add64(18443006402125177743n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 4 (18443006402125177743, 18443006402125177739)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443006402125177743n);
    input.add64(18443006402125177739n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 1 (18440157232511816769, 18443486324713450337)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440157232511816769n);
    input.add64(18443486324713450337n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 2 (18440157232511816765, 18440157232511816769)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440157232511816765n);
    input.add64(18440157232511816769n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 3 (18440157232511816769, 18440157232511816769)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440157232511816769n);
    input.add64(18440157232511816769n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 4 (18440157232511816769, 18440157232511816765)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440157232511816769n);
    input.add64(18440157232511816765n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 1 (18445113073087695607, 18442756910171648089)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445113073087695607n);
    input.add64(18442756910171648089n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 2 (18442756910171648085, 18442756910171648089)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442756910171648085n);
    input.add64(18442756910171648089n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 3 (18442756910171648089, 18442756910171648089)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442756910171648089n);
    input.add64(18442756910171648089n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 4 (18442756910171648089, 18442756910171648085)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442756910171648089n);
    input.add64(18442756910171648085n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 1 (18443181861435726807, 18437844054807684345)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443181861435726807n);
    input.add64(18437844054807684345n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 2 (18437844054807684341, 18437844054807684345)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18437844054807684341n);
    input.add64(18437844054807684345n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 3 (18437844054807684345, 18437844054807684345)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18437844054807684345n);
    input.add64(18437844054807684345n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 4 (18437844054807684345, 18437844054807684341)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18437844054807684345n);
    input.add64(18437844054807684341n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 1 (18441715559190543601, 18440213110422838799)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441715559190543601n);
    input.add64(18440213110422838799n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18440213110422838799n);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 2 (18440213110422838795, 18440213110422838799)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440213110422838795n);
    input.add64(18440213110422838799n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18440213110422838795n);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 3 (18440213110422838799, 18440213110422838799)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440213110422838799n);
    input.add64(18440213110422838799n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18440213110422838799n);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 4 (18440213110422838799, 18440213110422838795)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440213110422838799n);
    input.add64(18440213110422838795n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18440213110422838795n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 1 (18445666292498536589, 18445029194076592359)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445666292498536589n);
    input.add64(18445029194076592359n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18445666292498536589n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 2 (18445029194076592355, 18445029194076592359)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445029194076592355n);
    input.add64(18445029194076592359n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18445029194076592359n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 3 (18445029194076592359, 18445029194076592359)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445029194076592359n);
    input.add64(18445029194076592359n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18445029194076592359n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 4 (18445029194076592359, 18445029194076592355)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445029194076592359n);
    input.add64(18445029194076592355n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18445029194076592359n);
  });

  it('test operator "add" overload (euint64, euint128) => euint128 test 1 (2, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(2n);
    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(9223372036854775811n);
  });

  it('test operator "add" overload (euint64, euint128) => euint128 test 2 (9220385446038631008, 9220385446038631010)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(9220385446038631008n);
    input.add128(9220385446038631010n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18440770892077262018n);
  });

  it('test operator "add" overload (euint64, euint128) => euint128 test 3 (9220385446038631010, 9220385446038631010)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(9220385446038631010n);
    input.add128(9220385446038631010n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18440770892077262020n);
  });

  it('test operator "add" overload (euint64, euint128) => euint128 test 4 (9220385446038631010, 9220385446038631008)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(9220385446038631010n);
    input.add128(9220385446038631008n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18440770892077262018n);
  });

  it('test operator "sub" overload (euint64, euint128) => euint128 test 1 (18439658341746819099, 18439658341746819099)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439658341746819099n);
    input.add128(18439658341746819099n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint128) => euint128 test 2 (18439658341746819099, 18439658341746819095)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439658341746819099n);
    input.add128(18439658341746819095n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint128) => euint128 test 1 (2, 4611686018427387905)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(2n);
    input.add128(4611686018427387905n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(9223372036854775810n);
  });

  it('test operator "mul" overload (euint64, euint128) => euint128 test 2 (4294533286, 4294533286)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(4294533286n);
    input.add128(4294533286n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18443016144561957796n);
  });

  it('test operator "mul" overload (euint64, euint128) => euint128 test 3 (4294533286, 4294533286)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(4294533286n);
    input.add128(4294533286n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18443016144561957796n);
  });

  it('test operator "mul" overload (euint64, euint128) => euint128 test 4 (4294533286, 4294533286)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(4294533286n);
    input.add128(4294533286n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18443016144561957796n);
  });

  it('test operator "and" overload (euint64, euint128) => euint128 test 1 (18437910234671303343, 340282366920938463463369815604252648041)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18437910234671303343n);
    input.add128(340282366920938463463369815604252648041n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18437905699185623593n);
  });

  it('test operator "and" overload (euint64, euint128) => euint128 test 2 (18437910234671303339, 18437910234671303343)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18437910234671303339n);
    input.add128(18437910234671303343n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18437910234671303339n);
  });

  it('test operator "and" overload (euint64, euint128) => euint128 test 3 (18437910234671303343, 18437910234671303343)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18437910234671303343n);
    input.add128(18437910234671303343n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18437910234671303343n);
  });

  it('test operator "and" overload (euint64, euint128) => euint128 test 4 (18437910234671303343, 18437910234671303339)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18437910234671303343n);
    input.add128(18437910234671303339n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18437910234671303339n);
  });

  it('test operator "or" overload (euint64, euint128) => euint128 test 1 (18437770720875505123, 340282366920938463463371521953900116517)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18437770720875505123n);
    input.add128(340282366920938463463371521953900116517n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463371546178727837671n);
  });

  it('test operator "or" overload (euint64, euint128) => euint128 test 2 (18437770720875505119, 18437770720875505123)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18437770720875505119n);
    input.add128(18437770720875505123n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18437770720875505151n);
  });

  it('test operator "or" overload (euint64, euint128) => euint128 test 3 (18437770720875505123, 18437770720875505123)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18437770720875505123n);
    input.add128(18437770720875505123n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18437770720875505123n);
  });

  it('test operator "or" overload (euint64, euint128) => euint128 test 4 (18437770720875505123, 18437770720875505119)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18437770720875505123n);
    input.add128(18437770720875505119n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18437770720875505151n);
  });

  it('test operator "xor" overload (euint64, euint128) => euint128 test 1 (18445537742224280099, 340282366920938463463366077817924109071)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445537742224280099n);
    input.add128(340282366920938463463366077817924109071n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463444935186679223191852n);
  });

  it('test operator "xor" overload (euint64, euint128) => euint128 test 2 (18445537742224280095, 18445537742224280099)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445537742224280095n);
    input.add128(18445537742224280099n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(60n);
  });

  it('test operator "xor" overload (euint64, euint128) => euint128 test 3 (18445537742224280099, 18445537742224280099)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445537742224280099n);
    input.add128(18445537742224280099n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint128) => euint128 test 4 (18445537742224280099, 18445537742224280095)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445537742224280099n);
    input.add128(18445537742224280095n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(60n);
  });

  it('test operator "eq" overload (euint64, euint128) => ebool test 1 (18440279452511647831, 340282366920938463463372877547540946063)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440279452511647831n);
    input.add128(340282366920938463463372877547540946063n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint128) => ebool test 2 (18440279452511647827, 18440279452511647831)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440279452511647827n);
    input.add128(18440279452511647831n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint128) => ebool test 3 (18440279452511647831, 18440279452511647831)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440279452511647831n);
    input.add128(18440279452511647831n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint128) => ebool test 4 (18440279452511647831, 18440279452511647827)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440279452511647831n);
    input.add128(18440279452511647827n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint128) => ebool test 1 (18445830461373196895, 340282366920938463463371646197488822017)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445830461373196895n);
    input.add128(340282366920938463463371646197488822017n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint128) => ebool test 2 (18445830461373196891, 18445830461373196895)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445830461373196891n);
    input.add128(18445830461373196895n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint128) => ebool test 3 (18445830461373196895, 18445830461373196895)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445830461373196895n);
    input.add128(18445830461373196895n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint128) => ebool test 4 (18445830461373196895, 18445830461373196891)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445830461373196895n);
    input.add128(18445830461373196891n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint128) => ebool test 1 (18442893969514677185, 340282366920938463463367514096392336973)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442893969514677185n);
    input.add128(340282366920938463463367514096392336973n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint128) => ebool test 2 (18442893969514677181, 18442893969514677185)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442893969514677181n);
    input.add128(18442893969514677185n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint128) => ebool test 3 (18442893969514677185, 18442893969514677185)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442893969514677185n);
    input.add128(18442893969514677185n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint128) => ebool test 4 (18442893969514677185, 18442893969514677181)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442893969514677185n);
    input.add128(18442893969514677181n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint128) => ebool test 1 (18442800747554845031, 340282366920938463463371013086871642257)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442800747554845031n);
    input.add128(340282366920938463463371013086871642257n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint128) => ebool test 2 (18442800747554845027, 18442800747554845031)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442800747554845027n);
    input.add128(18442800747554845031n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint128) => ebool test 3 (18442800747554845031, 18442800747554845031)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442800747554845031n);
    input.add128(18442800747554845031n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint128) => ebool test 4 (18442800747554845031, 18442800747554845027)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442800747554845031n);
    input.add128(18442800747554845027n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint128) => ebool test 1 (18442114717348659435, 340282366920938463463367991149865832215)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442114717348659435n);
    input.add128(340282366920938463463367991149865832215n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint128) => ebool test 2 (18442114717348659431, 18442114717348659435)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442114717348659431n);
    input.add128(18442114717348659435n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint128) => ebool test 3 (18442114717348659435, 18442114717348659435)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442114717348659435n);
    input.add128(18442114717348659435n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint128) => ebool test 4 (18442114717348659435, 18442114717348659431)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442114717348659435n);
    input.add128(18442114717348659431n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint128) => ebool test 1 (18446018356203517667, 340282366920938463463368084467456188941)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18446018356203517667n);
    input.add128(340282366920938463463368084467456188941n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint128) => ebool test 2 (18446018356203517663, 18446018356203517667)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18446018356203517663n);
    input.add128(18446018356203517667n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint128) => ebool test 3 (18446018356203517667, 18446018356203517667)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18446018356203517667n);
    input.add128(18446018356203517667n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint128) => ebool test 4 (18446018356203517667, 18446018356203517663)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18446018356203517667n);
    input.add128(18446018356203517663n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint128) => euint128 test 1 (18444179762169981363, 340282366920938463463368224411849581449)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18444179762169981363n);
    input.add128(340282366920938463463368224411849581449n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18444179762169981363n);
  });

  it('test operator "min" overload (euint64, euint128) => euint128 test 2 (18444179762169981359, 18444179762169981363)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18444179762169981359n);
    input.add128(18444179762169981363n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18444179762169981359n);
  });

  it('test operator "min" overload (euint64, euint128) => euint128 test 3 (18444179762169981363, 18444179762169981363)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18444179762169981363n);
    input.add128(18444179762169981363n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18444179762169981363n);
  });

  it('test operator "min" overload (euint64, euint128) => euint128 test 4 (18444179762169981363, 18444179762169981359)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18444179762169981363n);
    input.add128(18444179762169981359n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18444179762169981359n);
  });

  it('test operator "max" overload (euint64, euint128) => euint128 test 1 (18438364178558425963, 340282366920938463463371982856487266675)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438364178558425963n);
    input.add128(340282366920938463463371982856487266675n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463371982856487266675n);
  });

  it('test operator "max" overload (euint64, euint128) => euint128 test 2 (18438364178558425959, 18438364178558425963)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438364178558425959n);
    input.add128(18438364178558425963n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18438364178558425963n);
  });

  it('test operator "max" overload (euint64, euint128) => euint128 test 3 (18438364178558425963, 18438364178558425963)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438364178558425963n);
    input.add128(18438364178558425963n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18438364178558425963n);
  });

  it('test operator "max" overload (euint64, euint128) => euint128 test 4 (18438364178558425963, 18438364178558425959)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438364178558425963n);
    input.add128(18438364178558425959n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18438364178558425963n);
  });

  it('test operator "and" overload (euint64, euint256) => euint256 test 1 (18438123442423711645, 115792089237316195423570985008687907853269984665640564039457583794571923914299)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438123442423711645n);
    input.add256(115792089237316195423570985008687907853269984665640564039457583794571923914299n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.resEuint256());
    expect(res).to.equal(18438050873074992665n);
  });

  it('test operator "and" overload (euint64, euint256) => euint256 test 2 (18438123442423711641, 18438123442423711645)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438123442423711641n);
    input.add256(18438123442423711645n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.resEuint256());
    expect(res).to.equal(18438123442423711641n);
  });

  it('test operator "and" overload (euint64, euint256) => euint256 test 3 (18438123442423711645, 18438123442423711645)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438123442423711645n);
    input.add256(18438123442423711645n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.resEuint256());
    expect(res).to.equal(18438123442423711645n);
  });

  it('test operator "and" overload (euint64, euint256) => euint256 test 4 (18438123442423711645, 18438123442423711641)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438123442423711645n);
    input.add256(18438123442423711641n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.resEuint256());
    expect(res).to.equal(18438123442423711641n);
  });

  it('test operator "or" overload (euint64, euint256) => euint256 test 1 (18440859675317395197, 115792089237316195423570985008687907853269984665640564039457583679111934127881)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440859675317395197n);
    input.add256(115792089237316195423570985008687907853269984665640564039457583679111934127881n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457583969429108981757n);
  });

  it('test operator "or" overload (euint64, euint256) => euint256 test 2 (18440859675317395193, 18440859675317395197)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440859675317395193n);
    input.add256(18440859675317395197n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.resEuint256());
    expect(res).to.equal(18440859675317395197n);
  });

  it('test operator "or" overload (euint64, euint256) => euint256 test 3 (18440859675317395197, 18440859675317395197)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440859675317395197n);
    input.add256(18440859675317395197n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.resEuint256());
    expect(res).to.equal(18440859675317395197n);
  });

  it('test operator "or" overload (euint64, euint256) => euint256 test 4 (18440859675317395197, 18440859675317395193)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440859675317395197n);
    input.add256(18440859675317395193n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.resEuint256());
    expect(res).to.equal(18440859675317395197n);
  });

  it('test operator "xor" overload (euint64, euint256) => euint256 test 1 (18443144507911915129, 115792089237316195423570985008687907853269984665640564039457576857539835346987)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443144507911915129n);
    input.add256(115792089237316195423570985008687907853269984665640564039457576857539835346987n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039439143367173646386770n);
  });

  it('test operator "xor" overload (euint64, euint256) => euint256 test 2 (18443144507911915125, 18443144507911915129)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443144507911915125n);
    input.add256(18443144507911915129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.resEuint256());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint64, euint256) => euint256 test 3 (18443144507911915129, 18443144507911915129)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443144507911915129n);
    input.add256(18443144507911915129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint256) => euint256 test 4 (18443144507911915129, 18443144507911915125)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443144507911915129n);
    input.add256(18443144507911915125n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.resEuint256());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint64, euint256) => ebool test 1 (18445450564170594745, 115792089237316195423570985008687907853269984665640564039457582438046507346451)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445450564170594745n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582438046507346451n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint256) => ebool test 2 (18445450564170594741, 18445450564170594745)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445450564170594741n);
    input.add256(18445450564170594745n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint256) => ebool test 3 (18445450564170594745, 18445450564170594745)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445450564170594745n);
    input.add256(18445450564170594745n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint256) => ebool test 4 (18445450564170594745, 18445450564170594741)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445450564170594745n);
    input.add256(18445450564170594741n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint256) => ebool test 1 (18444597976861499833, 115792089237316195423570985008687907853269984665640564039457575593896606645561)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18444597976861499833n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575593896606645561n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint256) => ebool test 2 (18444597976861499829, 18444597976861499833)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18444597976861499829n);
    input.add256(18444597976861499833n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint256) => ebool test 3 (18444597976861499833, 18444597976861499833)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18444597976861499833n);
    input.add256(18444597976861499833n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint256) => ebool test 4 (18444597976861499833, 18444597976861499829)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18444597976861499833n);
    input.add256(18444597976861499829n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "add" overload (euint128, euint8) => euint128 test 1 (129, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(129n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(131n);
  });

  it('test operator "add" overload (euint128, euint8) => euint128 test 2 (98, 100)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(98n);
    input.add8(100n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(198n);
  });

  it('test operator "add" overload (euint128, euint8) => euint128 test 3 (100, 100)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(100n);
    input.add8(100n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(200n);
  });

  it('test operator "add" overload (euint128, euint8) => euint128 test 4 (100, 98)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(100n);
    input.add8(98n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(198n);
  });

  it('test operator "sub" overload (euint128, euint8) => euint128 test 1 (53, 53)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(53n);
    input.add8(53n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint128, euint8) => euint128 test 2 (53, 49)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(53n);
    input.add8(49n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint128, euint8) => euint128 test 1 (65, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(65n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(130n);
  });

  it('test operator "mul" overload (euint128, euint8) => euint128 test 2 (12, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(12n);
    input.add8(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(144n);
  });

  it('test operator "mul" overload (euint128, euint8) => euint128 test 3 (12, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(12n);
    input.add8(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(144n);
  });

  it('test operator "mul" overload (euint128, euint8) => euint128 test 4 (12, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(12n);
    input.add8(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(144n);
  });

  it('test operator "and" overload (euint128, euint8) => euint128 test 1 (340282366920938463463366584011855602539, 199)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463366584011855602539n);
    input.add8(199n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(67n);
  });

  it('test operator "and" overload (euint128, euint8) => euint128 test 2 (195, 199)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(195n);
    input.add8(199n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(195n);
  });

  it('test operator "and" overload (euint128, euint8) => euint128 test 3 (199, 199)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(199n);
    input.add8(199n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(199n);
  });

  it('test operator "and" overload (euint128, euint8) => euint128 test 4 (199, 195)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(199n);
    input.add8(195n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(195n);
  });

  it('test operator "or" overload (euint128, euint8) => euint128 test 1 (340282366920938463463366114412714713449, 107)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463366114412714713449n);
    input.add8(107n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463366114412714713451n);
  });

  it('test operator "or" overload (euint128, euint8) => euint128 test 2 (103, 107)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(103n);
    input.add8(107n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(111n);
  });

  it('test operator "or" overload (euint128, euint8) => euint128 test 3 (107, 107)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(107n);
    input.add8(107n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(107n);
  });

  it('test operator "or" overload (euint128, euint8) => euint128 test 4 (107, 103)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(107n);
    input.add8(103n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(111n);
  });

  it('test operator "xor" overload (euint128, euint8) => euint128 test 1 (340282366920938463463374558473787998709, 61)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463374558473787998709n);
    input.add8(61n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463374558473787998664n);
  });

  it('test operator "xor" overload (euint128, euint8) => euint128 test 2 (57, 61)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(57n);
    input.add8(61n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint128, euint8) => euint128 test 3 (61, 61)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(61n);
    input.add8(61n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint128, euint8) => euint128 test 4 (61, 57)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(61n);
    input.add8(57n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint128, euint8) => ebool test 1 (340282366920938463463372565976692944677, 40)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463372565976692944677n);
    input.add8(40n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint8) => ebool test 2 (36, 40)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(36n);
    input.add8(40n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint8) => ebool test 3 (40, 40)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(40n);
    input.add8(40n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint128, euint8) => ebool test 4 (40, 36)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(40n);
    input.add8(36n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint8) => ebool test 1 (340282366920938463463366982527961317623, 89)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463366982527961317623n);
    input.add8(89n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint8) => ebool test 2 (85, 89)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(85n);
    input.add8(89n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint8) => ebool test 3 (89, 89)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(89n);
    input.add8(89n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint8) => ebool test 4 (89, 85)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(89n);
    input.add8(85n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint8) => ebool test 1 (340282366920938463463368919225377283387, 79)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463368919225377283387n);
    input.add8(79n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint8) => ebool test 2 (75, 79)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(75n);
    input.add8(79n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint128, euint8) => ebool test 3 (79, 79)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(79n);
    input.add8(79n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint8) => ebool test 4 (79, 75)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(79n);
    input.add8(75n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint8) => ebool test 1 (340282366920938463463371298021724846093, 78)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463371298021724846093n);
    input.add8(78n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint8) => ebool test 2 (74, 78)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(74n);
    input.add8(78n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint8) => ebool test 3 (78, 78)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(78n);
    input.add8(78n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint8) => ebool test 4 (78, 74)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(78n);
    input.add8(74n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint8) => ebool test 1 (340282366920938463463369973989121582221, 158)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463369973989121582221n);
    input.add8(158n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint128, euint8) => ebool test 2 (154, 158)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(154n);
    input.add8(158n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint8) => ebool test 3 (158, 158)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(158n);
    input.add8(158n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint8) => ebool test 4 (158, 154)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(158n);
    input.add8(154n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint8) => ebool test 1 (340282366920938463463374549319063928805, 84)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463374549319063928805n);
    input.add8(84n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint8) => ebool test 2 (80, 84)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(80n);
    input.add8(84n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint128, euint8) => ebool test 3 (84, 84)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(84n);
    input.add8(84n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint8) => ebool test 4 (84, 80)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(84n);
    input.add8(80n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });
});
