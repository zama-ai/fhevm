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

describe('FHEVM operations 3', function () {
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

  it('test operator "sub" overload (euint16, euint32) => euint32 test 1 (56163, 56163)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(56163n);
    input.add32(56163n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint32) => euint32 test 2 (56163, 56159)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(56163n);
    input.add32(56159n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 1 (2, 19357)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(2n);
    input.add32(19357n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(38714n);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 2 (155, 155)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(155n);
    input.add32(155n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(24025n);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 3 (155, 155)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(155n);
    input.add32(155n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(24025n);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 4 (155, 155)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(155n);
    input.add32(155n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(24025n);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 1 (16747, 1340660547)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(16747n);
    input.add32(1340660547n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(16707n);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 2 (16743, 16747)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(16743n);
    input.add32(16747n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(16739n);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 3 (16747, 16747)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(16747n);
    input.add32(16747n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(16747n);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 4 (16747, 16743)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(16747n);
    input.add32(16743n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(16739n);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 1 (16375, 306327437)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(16375n);
    input.add32(306327437n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(306331647n);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 2 (16371, 16375)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(16371n);
    input.add32(16375n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(16375n);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 3 (16375, 16375)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(16375n);
    input.add32(16375n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(16375n);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 4 (16375, 16371)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(16375n);
    input.add32(16371n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(16375n);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 1 (27803, 4274534847)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(27803n);
    input.add32(4274534847n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(4274541860n);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 2 (27799, 27803)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(27799n);
    input.add32(27803n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 3 (27803, 27803)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(27803n);
    input.add32(27803n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 4 (27803, 27799)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(27803n);
    input.add32(27799n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 1 (20779, 138761085)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(20779n);
    input.add32(138761085n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 2 (20775, 20779)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(20775n);
    input.add32(20779n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 3 (20779, 20779)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(20779n);
    input.add32(20779n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 4 (20779, 20775)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(20779n);
    input.add32(20775n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 1 (60727, 2571221215)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(60727n);
    input.add32(2571221215n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 2 (60723, 60727)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(60723n);
    input.add32(60727n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 3 (60727, 60727)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(60727n);
    input.add32(60727n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 4 (60727, 60723)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(60727n);
    input.add32(60723n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 1 (35984, 4044824521)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(35984n);
    input.add32(4044824521n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 2 (35980, 35984)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(35980n);
    input.add32(35984n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 3 (35984, 35984)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(35984n);
    input.add32(35984n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 4 (35984, 35980)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(35984n);
    input.add32(35980n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 1 (60438, 3117553998)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(60438n);
    input.add32(3117553998n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 2 (60434, 60438)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(60434n);
    input.add32(60438n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 3 (60438, 60438)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(60438n);
    input.add32(60438n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 4 (60438, 60434)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(60438n);
    input.add32(60434n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 1 (17160, 2721029617)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(17160n);
    input.add32(2721029617n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 2 (17156, 17160)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(17156n);
    input.add32(17160n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 3 (17160, 17160)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(17160n);
    input.add32(17160n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 4 (17160, 17156)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(17160n);
    input.add32(17156n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 1 (59319, 1380090301)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(59319n);
    input.add32(1380090301n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 2 (59315, 59319)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(59315n);
    input.add32(59319n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 3 (59319, 59319)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(59319n);
    input.add32(59319n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 4 (59319, 59315)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(59319n);
    input.add32(59315n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 1 (60667, 3444042353)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(60667n);
    input.add32(3444042353n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(60667n);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 2 (60663, 60667)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(60663n);
    input.add32(60667n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(60663n);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 3 (60667, 60667)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(60667n);
    input.add32(60667n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(60667n);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 4 (60667, 60663)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(60667n);
    input.add32(60663n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(60663n);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 1 (4613, 3163233444)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(4613n);
    input.add32(3163233444n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(3163233444n);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 2 (4609, 4613)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(4609n);
    input.add32(4613n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(4613n);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 3 (4613, 4613)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(4613n);
    input.add32(4613n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(4613n);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 4 (4613, 4609)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(4613n);
    input.add32(4609n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(4613n);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 1 (2, 65519)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(2n);
    input.add64(65519n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(65521n);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 2 (27697, 27701)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(27697n);
    input.add64(27701n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(55398n);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 3 (27701, 27701)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(27701n);
    input.add64(27701n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(55402n);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 4 (27701, 27697)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(27701n);
    input.add64(27697n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(55398n);
  });

  it('test operator "sub" overload (euint16, euint64) => euint64 test 1 (44233, 44233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(44233n);
    input.add64(44233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint64) => euint64 test 2 (44233, 44229)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(44233n);
    input.add64(44229n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 1 (2, 32761)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(2n);
    input.add64(32761n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(65522n);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 2 (223, 223)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(223n);
    input.add64(223n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(49729n);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 3 (223, 223)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(223n);
    input.add64(223n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(49729n);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 4 (223, 223)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(223n);
    input.add64(223n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(49729n);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 1 (31182, 18446021716973436235)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(31182n);
    input.add64(18446021716973436235n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(14666n);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 2 (31178, 31182)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(31178n);
    input.add64(31182n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(31178n);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 3 (31182, 31182)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(31182n);
    input.add64(31182n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(31182n);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 4 (31182, 31178)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(31182n);
    input.add64(31178n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(31178n);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 1 (21079, 18440920994741407323)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(21079n);
    input.add64(18440920994741407323n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(18440920994741411423n);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 2 (21075, 21079)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(21075n);
    input.add64(21079n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(21079n);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 3 (21079, 21079)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(21079n);
    input.add64(21079n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(21079n);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 4 (21079, 21075)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(21079n);
    input.add64(21075n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(21079n);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 1 (4980, 18440064999794934595)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(4980n);
    input.add64(18440064999794934595n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(18440064999794937911n);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 2 (4976, 4980)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(4976n);
    input.add64(4980n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 3 (4980, 4980)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(4980n);
    input.add64(4980n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 4 (4980, 4976)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(4980n);
    input.add64(4976n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 1 (1469, 18438812144295057355)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(1469n);
    input.add64(18438812144295057355n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 2 (1465, 1469)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(1465n);
    input.add64(1469n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 3 (1469, 1469)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(1469n);
    input.add64(1469n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 4 (1469, 1465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(1469n);
    input.add64(1465n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 1 (6052, 18437995395556478611)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(6052n);
    input.add64(18437995395556478611n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 2 (6048, 6052)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(6048n);
    input.add64(6052n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 3 (6052, 6052)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(6052n);
    input.add64(6052n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 4 (6052, 6048)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(6052n);
    input.add64(6048n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 1 (21207, 18440474075290845155)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(21207n);
    input.add64(18440474075290845155n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 2 (21203, 21207)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(21203n);
    input.add64(21207n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 3 (21207, 21207)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(21207n);
    input.add64(21207n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 4 (21207, 21203)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(21207n);
    input.add64(21203n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 1 (20053, 18443525132434609807)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(20053n);
    input.add64(18443525132434609807n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 2 (20049, 20053)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(20049n);
    input.add64(20053n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 3 (20053, 20053)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(20053n);
    input.add64(20053n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 4 (20053, 20049)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(20053n);
    input.add64(20049n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 1 (39839, 18442689076661068589)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(39839n);
    input.add64(18442689076661068589n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 2 (39835, 39839)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(39835n);
    input.add64(39839n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 3 (39839, 39839)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(39839n);
    input.add64(39839n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 4 (39839, 39835)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(39839n);
    input.add64(39835n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 1 (27983, 18446321085788846223)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(27983n);
    input.add64(18446321085788846223n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 2 (27979, 27983)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(27979n);
    input.add64(27983n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 3 (27983, 27983)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(27983n);
    input.add64(27983n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 4 (27983, 27979)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(27983n);
    input.add64(27979n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 1 (7319, 18444595869661013569)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(7319n);
    input.add64(18444595869661013569n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(7319n);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 2 (7315, 7319)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(7315n);
    input.add64(7319n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(7315n);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 3 (7319, 7319)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(7319n);
    input.add64(7319n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(7319n);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 4 (7319, 7315)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(7319n);
    input.add64(7315n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(7315n);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 1 (32600, 18443188074164557729)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(32600n);
    input.add64(18443188074164557729n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(18443188074164557729n);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 2 (32596, 32600)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(32596n);
    input.add64(32600n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(32600n);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 3 (32600, 32600)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(32600n);
    input.add64(32600n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(32600n);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 4 (32600, 32596)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(32600n);
    input.add64(32596n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract2.resEuint64());
    expect(res).to.equal(32600n);
  });

  it('test operator "add" overload (euint16, euint128) => euint128 test 1 (2, 32769)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(2n);
    input.add128(32769n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(32771n);
  });

  it('test operator "add" overload (euint16, euint128) => euint128 test 2 (20450, 20454)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(20450n);
    input.add128(20454n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(40904n);
  });

  it('test operator "add" overload (euint16, euint128) => euint128 test 3 (20454, 20454)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(20454n);
    input.add128(20454n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(40908n);
  });

  it('test operator "add" overload (euint16, euint128) => euint128 test 4 (20454, 20450)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(20454n);
    input.add128(20450n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(40904n);
  });

  it('test operator "sub" overload (euint16, euint128) => euint128 test 1 (36684, 36684)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(36684n);
    input.add128(36684n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint128) => euint128 test 2 (36684, 36680)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(36684n);
    input.add128(36680n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint128) => euint128 test 1 (2, 16385)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(2n);
    input.add128(16385n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(32770n);
  });

  it('test operator "mul" overload (euint16, euint128) => euint128 test 2 (218, 218)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(218n);
    input.add128(218n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(47524n);
  });

  it('test operator "mul" overload (euint16, euint128) => euint128 test 3 (218, 218)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(218n);
    input.add128(218n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(47524n);
  });

  it('test operator "mul" overload (euint16, euint128) => euint128 test 4 (218, 218)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(218n);
    input.add128(218n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(47524n);
  });

  it('test operator "and" overload (euint16, euint128) => euint128 test 1 (20860, 340282366920938463463369339393260151087)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(20860n);
    input.add128(340282366920938463463369339393260151087n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(16684n);
  });

  it('test operator "and" overload (euint16, euint128) => euint128 test 2 (20856, 20860)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(20856n);
    input.add128(20860n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(20856n);
  });

  it('test operator "and" overload (euint16, euint128) => euint128 test 3 (20860, 20860)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(20860n);
    input.add128(20860n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(20860n);
  });

  it('test operator "and" overload (euint16, euint128) => euint128 test 4 (20860, 20856)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(20860n);
    input.add128(20856n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(20856n);
  });

  it('test operator "or" overload (euint16, euint128) => euint128 test 1 (12530, 340282366920938463463367364367079931363)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(12530n);
    input.add128(340282366920938463463367364367079931363n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(340282366920938463463367364367079931379n);
  });

  it('test operator "or" overload (euint16, euint128) => euint128 test 2 (12526, 12530)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(12526n);
    input.add128(12530n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(12542n);
  });

  it('test operator "or" overload (euint16, euint128) => euint128 test 3 (12530, 12530)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(12530n);
    input.add128(12530n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(12530n);
  });

  it('test operator "or" overload (euint16, euint128) => euint128 test 4 (12530, 12526)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(12530n);
    input.add128(12526n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(12542n);
  });

  it('test operator "xor" overload (euint16, euint128) => euint128 test 1 (15770, 340282366920938463463371308241757817591)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(15770n);
    input.add128(340282366920938463463371308241757817591n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(340282366920938463463371308241757828973n);
  });

  it('test operator "xor" overload (euint16, euint128) => euint128 test 2 (15766, 15770)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(15766n);
    input.add128(15770n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint16, euint128) => euint128 test 3 (15770, 15770)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(15770n);
    input.add128(15770n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint128) => euint128 test 4 (15770, 15766)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(15770n);
    input.add128(15766n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint16, euint128) => ebool test 1 (6479, 340282366920938463463366436603373356281)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(6479n);
    input.add128(340282366920938463463366436603373356281n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint128) => ebool test 2 (6475, 6479)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(6475n);
    input.add128(6479n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint128) => ebool test 3 (6479, 6479)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(6479n);
    input.add128(6479n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint128) => ebool test 4 (6479, 6475)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(6479n);
    input.add128(6475n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint128) => ebool test 1 (12291, 340282366920938463463370786110933271907)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(12291n);
    input.add128(340282366920938463463370786110933271907n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint128) => ebool test 2 (12287, 12291)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(12287n);
    input.add128(12291n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint128) => ebool test 3 (12291, 12291)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(12291n);
    input.add128(12291n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint128) => ebool test 4 (12291, 12287)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(12291n);
    input.add128(12287n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint128) => ebool test 1 (13971, 340282366920938463463369346041068104207)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(13971n);
    input.add128(340282366920938463463369346041068104207n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint128) => ebool test 2 (13967, 13971)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(13967n);
    input.add128(13971n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint128) => ebool test 3 (13971, 13971)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(13971n);
    input.add128(13971n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint128) => ebool test 4 (13971, 13967)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(13971n);
    input.add128(13967n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint128) => ebool test 1 (3782, 340282366920938463463370166535624829067)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(3782n);
    input.add128(340282366920938463463370166535624829067n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint128) => ebool test 2 (3778, 3782)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(3778n);
    input.add128(3782n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint128) => ebool test 3 (3782, 3782)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(3782n);
    input.add128(3782n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint128) => ebool test 4 (3782, 3778)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(3782n);
    input.add128(3778n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint128) => ebool test 1 (35352, 340282366920938463463374416701918746169)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(35352n);
    input.add128(340282366920938463463374416701918746169n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint128) => ebool test 2 (35348, 35352)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(35348n);
    input.add128(35352n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint128) => ebool test 3 (35352, 35352)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(35352n);
    input.add128(35352n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint128) => ebool test 4 (35352, 35348)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(35352n);
    input.add128(35348n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint128) => ebool test 1 (45838, 340282366920938463463366401290446630467)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(45838n);
    input.add128(340282366920938463463366401290446630467n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint128) => ebool test 2 (45834, 45838)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(45834n);
    input.add128(45838n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint128) => ebool test 3 (45838, 45838)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(45838n);
    input.add128(45838n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint128) => ebool test 4 (45838, 45834)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(45838n);
    input.add128(45834n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint128) => euint128 test 1 (51430, 340282366920938463463366481893816419789)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(51430n);
    input.add128(340282366920938463463366481893816419789n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(51430n);
  });

  it('test operator "min" overload (euint16, euint128) => euint128 test 2 (51426, 51430)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(51426n);
    input.add128(51430n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(51426n);
  });

  it('test operator "min" overload (euint16, euint128) => euint128 test 3 (51430, 51430)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(51430n);
    input.add128(51430n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(51430n);
  });

  it('test operator "min" overload (euint16, euint128) => euint128 test 4 (51430, 51426)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(51430n);
    input.add128(51426n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(51426n);
  });

  it('test operator "max" overload (euint16, euint128) => euint128 test 1 (63191, 340282366920938463463372825992852373929)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(63191n);
    input.add128(340282366920938463463372825992852373929n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(340282366920938463463372825992852373929n);
  });

  it('test operator "max" overload (euint16, euint128) => euint128 test 2 (63187, 63191)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(63187n);
    input.add128(63191n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(63191n);
  });

  it('test operator "max" overload (euint16, euint128) => euint128 test 3 (63191, 63191)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(63191n);
    input.add128(63191n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(63191n);
  });

  it('test operator "max" overload (euint16, euint128) => euint128 test 4 (63191, 63187)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(63191n);
    input.add128(63187n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract2.resEuint128());
    expect(res).to.equal(63191n);
  });

  it('test operator "and" overload (euint16, euint256) => euint256 test 1 (9039, 115792089237316195423570985008687907853269984665640564039457577533707407694673)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(9039n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577533707407694673n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract2.resEuint256());
    expect(res).to.equal(833n);
  });

  it('test operator "and" overload (euint16, euint256) => euint256 test 2 (9035, 9039)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(9035n);
    input.add256(9039n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract2.resEuint256());
    expect(res).to.equal(9035n);
  });

  it('test operator "and" overload (euint16, euint256) => euint256 test 3 (9039, 9039)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(9039n);
    input.add256(9039n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract2.resEuint256());
    expect(res).to.equal(9039n);
  });

  it('test operator "and" overload (euint16, euint256) => euint256 test 4 (9039, 9035)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(9039n);
    input.add256(9035n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract2.resEuint256());
    expect(res).to.equal(9035n);
  });

  it('test operator "or" overload (euint16, euint256) => euint256 test 1 (20656, 115792089237316195423570985008687907853269984665640564039457578925344449230427)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(20656n);
    input.add256(115792089237316195423570985008687907853269984665640564039457578925344449230427n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract2.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457578925344449230587n);
  });

  it('test operator "or" overload (euint16, euint256) => euint256 test 2 (20652, 20656)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(20652n);
    input.add256(20656n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract2.resEuint256());
    expect(res).to.equal(20668n);
  });

  it('test operator "or" overload (euint16, euint256) => euint256 test 3 (20656, 20656)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(20656n);
    input.add256(20656n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract2.resEuint256());
    expect(res).to.equal(20656n);
  });

  it('test operator "or" overload (euint16, euint256) => euint256 test 4 (20656, 20652)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(20656n);
    input.add256(20652n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract2.resEuint256());
    expect(res).to.equal(20668n);
  });

  it('test operator "xor" overload (euint16, euint256) => euint256 test 1 (836, 115792089237316195423570985008687907853269984665640564039457575437861740953305)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(836n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575437861740953305n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract2.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575437861740952989n);
  });

  it('test operator "xor" overload (euint16, euint256) => euint256 test 2 (832, 836)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(832n);
    input.add256(836n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract2.resEuint256());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint16, euint256) => euint256 test 3 (836, 836)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(836n);
    input.add256(836n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract2.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint256) => euint256 test 4 (836, 832)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(836n);
    input.add256(832n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract2.resEuint256());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint16, euint256) => ebool test 1 (54938, 115792089237316195423570985008687907853269984665640564039457580012666163516013)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(54938n);
    input.add256(115792089237316195423570985008687907853269984665640564039457580012666163516013n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint256) => ebool test 2 (54934, 54938)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(54934n);
    input.add256(54938n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint256) => ebool test 3 (54938, 54938)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(54938n);
    input.add256(54938n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint256) => ebool test 4 (54938, 54934)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(54938n);
    input.add256(54934n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint256) => ebool test 1 (57764, 115792089237316195423570985008687907853269984665640564039457576066884046555613)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(57764n);
    input.add256(115792089237316195423570985008687907853269984665640564039457576066884046555613n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint256) => ebool test 2 (57760, 57764)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(57760n);
    input.add256(57764n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint256) => ebool test 3 (57764, 57764)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(57764n);
    input.add256(57764n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint256) => ebool test 4 (57764, 57760)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(57764n);
    input.add256(57760n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 1 (220, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(220n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(222n);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 2 (30, 34)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(30n);
    input.add8(34n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(64n);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 3 (34, 34)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(34n);
    input.add8(34n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(68n);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 4 (34, 30)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(34n);
    input.add8(30n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(64n);
  });

  it('test operator "sub" overload (euint32, euint8) => euint32 test 1 (242, 242)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(242n);
    input.add8(242n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint8) => euint32 test 2 (242, 238)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(242n);
    input.add8(238n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 1 (123, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(123n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(246n);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 2 (11, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(11n);
    input.add8(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(132n);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 3 (12, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(12n);
    input.add8(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(144n);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 4 (12, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(12n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(132n);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 1 (415821561, 21)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(415821561n);
    input.add8(21n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(17n);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 2 (17, 21)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(17n);
    input.add8(21n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(17n);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 3 (21, 21)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(21n);
    input.add8(21n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(21n);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 4 (21, 17)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(21n);
    input.add8(17n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(17n);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 1 (2867682603, 131)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(2867682603n);
    input.add8(131n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(2867682731n);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 2 (127, 131)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(127n);
    input.add8(131n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(255n);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 3 (131, 131)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(131n);
    input.add8(131n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(131n);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 4 (131, 127)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(131n);
    input.add8(127n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(255n);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 1 (622312533, 225)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(622312533n);
    input.add8(225n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(622312628n);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 2 (221, 225)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(221n);
    input.add8(225n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(60n);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 3 (225, 225)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(225n);
    input.add8(225n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 4 (225, 221)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(225n);
    input.add8(221n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract2.resEuint32());
    expect(res).to.equal(60n);
  });
});
