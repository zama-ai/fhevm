import { assert, expect } from 'chai';
import { ethers } from 'hardhat';

import type { FHEVMTestSuite1 } from '../../types/contracts/tests/FHEVMTestSuite1';
import type { FHEVMTestSuite2 } from '../../types/contracts/tests/FHEVMTestSuite2';
import type { FHEVMTestSuite3 } from '../../types/contracts/tests/FHEVMTestSuite3';
import type { FHEVMTestSuite4 } from '../../types/contracts/tests/FHEVMTestSuite4';
import type { FHEVMTestSuite5 } from '../../types/contracts/tests/FHEVMTestSuite5';
import type { FHEVMTestSuite6 } from '../../types/contracts/tests/FHEVMTestSuite6';
import type { FHEVMTestSuite7 } from '../../types/contracts/tests/FHEVMTestSuite7';
import { createInstances } from '../instance';
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

  it('test operator "sub" overload (euint16, euint32) => euint32 test 1 (12976, 12976)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(12976n);
    input.add32(12976n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint16, euint32) => euint32 test 2 (12976, 12972)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(12976n);
    input.add32(12972n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 1 (2, 19678)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(2n);
    input.add32(19678n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 39356n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 2 (203, 203)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(203n);
    input.add32(203n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 41209n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 3 (203, 203)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(203n);
    input.add32(203n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 41209n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 4 (203, 203)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(203n);
    input.add32(203n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 41209n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 1 (49991, 4050691006)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(49991n);
    input.add32(4050691006n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 33542n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 2 (49987, 49991)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(49987n);
    input.add32(49991n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 49987n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 3 (49991, 49991)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(49991n);
    input.add32(49991n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 49991n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 4 (49991, 49987)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(49991n);
    input.add32(49987n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 49987n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 1 (36796, 1809890343)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(36796n);
    input.add32(1809890343n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1809891263n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 2 (36792, 36796)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(36792n);
    input.add32(36796n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 36796n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 3 (36796, 36796)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(36796n);
    input.add32(36796n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 36796n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 4 (36796, 36792)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(36796n);
    input.add32(36792n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 36796n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 1 (61729, 2189484165)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(61729n);
    input.add32(2189484165n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2189431204n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 2 (61725, 61729)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(61725n);
    input.add32(61729n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 60n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 3 (61729, 61729)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(61729n);
    input.add32(61729n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 4 (61729, 61725)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(61729n);
    input.add32(61725n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 60n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 1 (35285, 1502986069)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(35285n);
    input.add32(1502986069n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 2 (35281, 35285)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(35281n);
    input.add32(35285n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 3 (35285, 35285)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(35285n);
    input.add32(35285n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 4 (35285, 35281)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(35285n);
    input.add32(35281n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 1 (42019, 944213577)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(42019n);
    input.add32(944213577n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 2 (42015, 42019)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(42015n);
    input.add32(42019n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 3 (42019, 42019)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(42019n);
    input.add32(42019n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 4 (42019, 42015)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(42019n);
    input.add32(42015n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 1 (39788, 2716305431)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(39788n);
    input.add32(2716305431n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 2 (39784, 39788)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(39784n);
    input.add32(39788n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 3 (39788, 39788)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(39788n);
    input.add32(39788n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 4 (39788, 39784)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(39788n);
    input.add32(39784n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 1 (41531, 797089470)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(41531n);
    input.add32(797089470n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 2 (41527, 41531)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(41527n);
    input.add32(41531n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 3 (41531, 41531)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(41531n);
    input.add32(41531n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 4 (41531, 41527)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(41531n);
    input.add32(41527n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 1 (10016, 988187666)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(10016n);
    input.add32(988187666n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 2 (10012, 10016)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(10012n);
    input.add32(10016n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 3 (10016, 10016)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(10016n);
    input.add32(10016n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 4 (10016, 10012)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(10016n);
    input.add32(10012n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 1 (6389, 1108237960)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(6389n);
    input.add32(1108237960n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 2 (6385, 6389)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(6385n);
    input.add32(6389n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 3 (6389, 6389)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(6389n);
    input.add32(6389n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 4 (6389, 6385)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(6389n);
    input.add32(6385n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 1 (9312, 4025637667)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(9312n);
    input.add32(4025637667n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 9312n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 2 (9308, 9312)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(9308n);
    input.add32(9312n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 9308n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 3 (9312, 9312)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(9312n);
    input.add32(9312n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 9312n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 4 (9312, 9308)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(9312n);
    input.add32(9308n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 9308n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 1 (20265, 998291896)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(20265n);
    input.add32(998291896n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 998291896n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 2 (20261, 20265)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(20261n);
    input.add32(20265n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 20265n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 3 (20265, 20265)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(20265n);
    input.add32(20265n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 20265n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 4 (20265, 20261)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(20265n);
    input.add32(20261n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 20265n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 1 (2, 65515)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(2n);
    input.add64(65515n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 65517n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 2 (16658, 16660)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(16658n);
    input.add64(16660n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 33318n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 3 (16660, 16660)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(16660n);
    input.add64(16660n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 33320n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 4 (16660, 16658)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(16660n);
    input.add64(16658n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 33318n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint16, euint64) => euint64 test 1 (40311, 40311)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(40311n);
    input.add64(40311n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint16, euint64) => euint64 test 2 (40311, 40307)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(40311n);
    input.add64(40307n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 1 (2, 32766)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(2n);
    input.add64(32766n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 65532n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 2 (233, 233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(233n);
    input.add64(233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 54289n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 3 (233, 233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(233n);
    input.add64(233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 54289n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 4 (233, 233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(233n);
    input.add64(233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 54289n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 1 (2629, 18441525174729174229)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(2629n);
    input.add64(18441525174729174229n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2117n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 2 (2625, 2629)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(2625n);
    input.add64(2629n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2625n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 3 (2629, 2629)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(2629n);
    input.add64(2629n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2629n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 4 (2629, 2625)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(2629n);
    input.add64(2625n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2625n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 1 (32722, 18443586327528883093)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(32722n);
    input.add64(18443586327528883093n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18443586327528898519n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 2 (32718, 32722)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(32718n);
    input.add64(32722n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 32734n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 3 (32722, 32722)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(32722n);
    input.add64(32722n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 32722n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 4 (32722, 32718)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(32722n);
    input.add64(32718n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 32734n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 1 (11106, 18442262658835085227)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(11106n);
    input.add64(18442262658835085227n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18442262658835078345n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 2 (11102, 11106)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(11102n);
    input.add64(11106n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 60n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 3 (11106, 11106)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(11106n);
    input.add64(11106n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 4 (11106, 11102)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(11106n);
    input.add64(11102n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 60n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 1 (30381, 18444803087693187229)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(30381n);
    input.add64(18444803087693187229n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 2 (30377, 30381)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(30377n);
    input.add64(30381n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 3 (30381, 30381)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(30381n);
    input.add64(30381n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 4 (30381, 30377)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(30381n);
    input.add64(30377n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 1 (64718, 18441768932533868139)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(64718n);
    input.add64(18441768932533868139n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 2 (64714, 64718)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(64714n);
    input.add64(64718n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 3 (64718, 64718)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(64718n);
    input.add64(64718n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 4 (64718, 64714)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(64718n);
    input.add64(64714n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 1 (49187, 18441297787837225539)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(49187n);
    input.add64(18441297787837225539n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 2 (49183, 49187)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(49183n);
    input.add64(49187n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 3 (49187, 49187)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(49187n);
    input.add64(49187n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 4 (49187, 49183)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(49187n);
    input.add64(49183n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 1 (51196, 18438552412505833467)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(51196n);
    input.add64(18438552412505833467n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 2 (51192, 51196)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(51192n);
    input.add64(51196n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 3 (51196, 51196)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(51196n);
    input.add64(51196n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 4 (51196, 51192)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(51196n);
    input.add64(51192n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 1 (36070, 18445165115306659259)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(36070n);
    input.add64(18445165115306659259n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 2 (36066, 36070)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(36066n);
    input.add64(36070n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 3 (36070, 36070)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(36070n);
    input.add64(36070n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 4 (36070, 36066)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(36070n);
    input.add64(36066n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 1 (3502, 18442435815411527191)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(3502n);
    input.add64(18442435815411527191n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 2 (3498, 3502)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(3498n);
    input.add64(3502n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 3 (3502, 3502)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(3502n);
    input.add64(3502n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 4 (3502, 3498)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(3502n);
    input.add64(3498n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 1 (36662, 18444768028676885417)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(36662n);
    input.add64(18444768028676885417n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 36662n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 2 (36658, 36662)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(36658n);
    input.add64(36662n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 36658n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 3 (36662, 36662)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(36662n);
    input.add64(36662n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 36662n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 4 (36662, 36658)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(36662n);
    input.add64(36658n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 36658n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 1 (5517, 18445505006727661549)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(5517n);
    input.add64(18445505006727661549n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18445505006727661549n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 2 (5513, 5517)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(5513n);
    input.add64(5517n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 5517n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 3 (5517, 5517)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(5517n);
    input.add64(5517n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 5517n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 4 (5517, 5513)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(5517n);
    input.add64(5513n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 5517n,
    };
    assert.deepEqual(res, expectedRes);
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
    const handle = await this.contract2.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 32771n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint16, euint128) => euint128 test 2 (18468, 18470)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(18468n);
    input.add128(18470n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 36938n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint16, euint128) => euint128 test 3 (18470, 18470)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(18470n);
    input.add128(18470n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 36940n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint16, euint128) => euint128 test 4 (18470, 18468)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(18470n);
    input.add128(18468n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 36938n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint16, euint128) => euint128 test 1 (25690, 25690)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(25690n);
    input.add128(25690n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint16, euint128) => euint128 test 2 (25690, 25686)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(25690n);
    input.add128(25686n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
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
    const handle = await this.contract2.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 32770n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint16, euint128) => euint128 test 2 (179, 179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(179n);
    input.add128(179n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 32041n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint16, euint128) => euint128 test 3 (179, 179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(179n);
    input.add128(179n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 32041n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint16, euint128) => euint128 test 4 (179, 179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(179n);
    input.add128(179n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 32041n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint16, euint128) => euint128 test 1 (28006, 340282366920938463463370382873486427233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(28006n);
    input.add128(340282366920938463463370382873486427233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1120n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint16, euint128) => euint128 test 2 (28002, 28006)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(28002n);
    input.add128(28006n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28002n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint16, euint128) => euint128 test 3 (28006, 28006)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(28006n);
    input.add128(28006n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28006n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint16, euint128) => euint128 test 4 (28006, 28002)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(28006n);
    input.add128(28002n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28002n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, euint128) => euint128 test 1 (13222, 340282366920938463463372009894753494031)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(13222n);
    input.add128(340282366920938463463372009894753494031n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463372009894753499055n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, euint128) => euint128 test 2 (13218, 13222)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(13218n);
    input.add128(13222n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 13222n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, euint128) => euint128 test 3 (13222, 13222)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(13222n);
    input.add128(13222n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 13222n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, euint128) => euint128 test 4 (13222, 13218)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(13222n);
    input.add128(13218n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 13222n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint128) => euint128 test 1 (49232, 340282366920938463463366661442020706709)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(49232n);
    input.add128(340282366920938463463366661442020706709n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463366661442020755909n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint128) => euint128 test 2 (49228, 49232)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(49228n);
    input.add128(49232n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint128) => euint128 test 3 (49232, 49232)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(49232n);
    input.add128(49232n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint128) => euint128 test 4 (49232, 49228)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(49232n);
    input.add128(49228n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint16, euint128) => ebool test 1 (29558, 340282366920938463463367885539494905623)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(29558n);
    input.add128(340282366920938463463367885539494905623n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint16, euint128) => ebool test 2 (29554, 29558)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(29554n);
    input.add128(29558n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint16, euint128) => ebool test 3 (29558, 29558)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(29558n);
    input.add128(29558n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint16, euint128) => ebool test 4 (29558, 29554)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(29558n);
    input.add128(29554n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint16, euint128) => ebool test 1 (50256, 340282366920938463463366433427854841079)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(50256n);
    input.add128(340282366920938463463366433427854841079n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint16, euint128) => ebool test 2 (50252, 50256)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(50252n);
    input.add128(50256n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint16, euint128) => ebool test 3 (50256, 50256)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(50256n);
    input.add128(50256n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint16, euint128) => ebool test 4 (50256, 50252)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(50256n);
    input.add128(50252n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint16, euint128) => ebool test 1 (45191, 340282366920938463463366515860282611551)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(45191n);
    input.add128(340282366920938463463366515860282611551n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint16, euint128) => ebool test 2 (45187, 45191)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(45187n);
    input.add128(45191n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint16, euint128) => ebool test 3 (45191, 45191)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(45191n);
    input.add128(45191n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint16, euint128) => ebool test 4 (45191, 45187)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(45191n);
    input.add128(45187n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint16, euint128) => ebool test 1 (26617, 340282366920938463463374482159973264755)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(26617n);
    input.add128(340282366920938463463374482159973264755n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint16, euint128) => ebool test 2 (26613, 26617)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(26613n);
    input.add128(26617n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint16, euint128) => ebool test 3 (26617, 26617)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(26617n);
    input.add128(26617n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint16, euint128) => ebool test 4 (26617, 26613)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(26617n);
    input.add128(26613n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint16, euint128) => ebool test 1 (53520, 340282366920938463463373390080059789019)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(53520n);
    input.add128(340282366920938463463373390080059789019n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint16, euint128) => ebool test 2 (53516, 53520)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(53516n);
    input.add128(53520n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint16, euint128) => ebool test 3 (53520, 53520)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(53520n);
    input.add128(53520n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint16, euint128) => ebool test 4 (53520, 53516)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(53520n);
    input.add128(53516n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint16, euint128) => ebool test 1 (53343, 340282366920938463463371082231924074137)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(53343n);
    input.add128(340282366920938463463371082231924074137n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint16, euint128) => ebool test 2 (53339, 53343)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(53339n);
    input.add128(53343n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint16, euint128) => ebool test 3 (53343, 53343)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(53343n);
    input.add128(53343n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint16, euint128) => ebool test 4 (53343, 53339)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(53343n);
    input.add128(53339n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint16, euint128) => euint128 test 1 (24029, 340282366920938463463370856702391593879)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(24029n);
    input.add128(340282366920938463463370856702391593879n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 24029n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint16, euint128) => euint128 test 2 (24025, 24029)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(24025n);
    input.add128(24029n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 24025n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint16, euint128) => euint128 test 3 (24029, 24029)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(24029n);
    input.add128(24029n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 24029n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint16, euint128) => euint128 test 4 (24029, 24025)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(24029n);
    input.add128(24025n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 24025n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint16, euint128) => euint128 test 1 (26902, 340282366920938463463372909889962896373)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(26902n);
    input.add128(340282366920938463463372909889962896373n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463372909889962896373n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint16, euint128) => euint128 test 2 (26898, 26902)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(26898n);
    input.add128(26902n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 26902n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint16, euint128) => euint128 test 3 (26902, 26902)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(26902n);
    input.add128(26902n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 26902n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint16, euint128) => euint128 test 4 (26902, 26898)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(26902n);
    input.add128(26898n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 26902n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint16, euint256) => euint256 test 1 (8026, 115792089237316195423570985008687907853269984665640564039457577431767225979983)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(8026n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577431767225979983n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 5194n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint16, euint256) => euint256 test 2 (8022, 8026)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(8022n);
    input.add256(8026n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 8018n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint16, euint256) => euint256 test 3 (8026, 8026)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(8026n);
    input.add256(8026n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 8026n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint16, euint256) => euint256 test 4 (8026, 8022)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(8026n);
    input.add256(8022n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 8018n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, euint256) => euint256 test 1 (55607, 115792089237316195423570985008687907853269984665640564039457576188215754675963)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(55607n);
    input.add256(115792089237316195423570985008687907853269984665640564039457576188215754675963n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457576188215754678271n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, euint256) => euint256 test 2 (55603, 55607)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(55603n);
    input.add256(55607n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 55607n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, euint256) => euint256 test 3 (55607, 55607)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(55607n);
    input.add256(55607n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 55607n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, euint256) => euint256 test 4 (55607, 55603)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(55607n);
    input.add256(55603n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 55607n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint256) => euint256 test 1 (27727, 115792089237316195423570985008687907853269984665640564039457580363331565483209)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(27727n);
    input.add256(115792089237316195423570985008687907853269984665640564039457580363331565483209n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457580363331565459590n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint256) => euint256 test 2 (27723, 27727)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(27723n);
    input.add256(27727n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint256) => euint256 test 3 (27727, 27727)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(27727n);
    input.add256(27727n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint256) => euint256 test 4 (27727, 27723)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(27727n);
    input.add256(27723n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint16, euint256) => ebool test 1 (32008, 115792089237316195423570985008687907853269984665640564039457577676372678011735)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(32008n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577676372678011735n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint16, euint256) => ebool test 2 (32004, 32008)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(32004n);
    input.add256(32008n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint16, euint256) => ebool test 3 (32008, 32008)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(32008n);
    input.add256(32008n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint16, euint256) => ebool test 4 (32008, 32004)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(32008n);
    input.add256(32004n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint16, euint256) => ebool test 1 (50100, 115792089237316195423570985008687907853269984665640564039457580245064398618657)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(50100n);
    input.add256(115792089237316195423570985008687907853269984665640564039457580245064398618657n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint16, euint256) => ebool test 2 (50096, 50100)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(50096n);
    input.add256(50100n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint16, euint256) => ebool test 3 (50100, 50100)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(50100n);
    input.add256(50100n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint16, euint256) => ebool test 4 (50100, 50096)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(50100n);
    input.add256(50096n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 1 (169, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(169n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 171n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 2 (101, 105)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(101n);
    input.add8(105n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 206n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 3 (105, 105)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(105n);
    input.add8(105n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 210n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 4 (105, 101)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(105n);
    input.add8(101n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 206n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint32, euint8) => euint32 test 1 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(5n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint32, euint8) => euint32 test 2 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(5n);
    input.add8(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 1 (96, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(96n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 192n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 2 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(9n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 81n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(9n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 81n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 4 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(9n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 81n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 1 (3160815052, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(3160815052n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(1n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(5n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 5n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(5n);
    input.add8(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 1 (3078982303, 88)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(3078982303n);
    input.add8(88n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3078982367n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 2 (84, 88)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(84n);
    input.add8(88n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 92n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 3 (88, 88)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(88n);
    input.add8(88n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 88n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 4 (88, 84)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(88n);
    input.add8(84n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 92n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 1 (2193894638, 121)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(2193894638n);
    input.add8(121n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2193894551n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 2 (117, 121)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(117n);
    input.add8(121n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 12n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 3 (121, 121)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(121n);
    input.add8(121n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 4 (121, 117)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(121n);
    input.add8(117n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 12n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
