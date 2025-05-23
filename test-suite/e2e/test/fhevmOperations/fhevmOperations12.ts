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

describe('FHEVM operations 12', function () {
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

  it('test operator "rotr" overload (euint8, uint8) => euint8 test 1 (118, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(118n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint8_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 118n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint8, uint8) => euint8 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint8_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint8, uint8) => euint8 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint8_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 8n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint8, uint8) => euint8 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint8_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 128n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 1 (48345, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(48345n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 55552n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1024n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2048n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 128n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 1 (48345, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(48345n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint16_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 55552n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint16_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1024n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint16_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2048n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint16_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 128n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint16, euint8) => euint16 test 1 (42969, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(42969n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 83n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint16, euint8) => euint16 test 2 (5, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(5n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint16, euint8) => euint16 test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(9n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint16, euint8) => euint16 test 4 (9, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(9n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 1 (42969, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(42969n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint16_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 83n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 2 (5, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint16_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(9n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint16_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 4 (9, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(9n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint16_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint16, euint8) => euint16 test 1 (9652, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(9652n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 53398n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint16, euint8) => euint16 test 2 (6, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(6n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 6144n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint16, euint8) => euint16 test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(10n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 10240n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint16, euint8) => euint16 test 4 (10, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(10n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 640n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint16, uint8) => euint16 test 1 (9652, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(9652n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint16_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 53398n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint16, uint8) => euint16 test 2 (6, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint16_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 6144n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint16, uint8) => euint16 test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(10n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint16_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 10240n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint16, uint8) => euint16 test 4 (10, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(10n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint16_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 640n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint16, euint8) => euint16 test 1 (9941, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(9941n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 55972n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint16, euint8) => euint16 test 2 (7, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(7n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 224n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint16, euint8) => euint16 test 3 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(11n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 352n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint16, euint8) => euint16 test 4 (11, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(11n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 5632n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint16, uint8) => euint16 test 1 (9941, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(9941n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint16_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 55972n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint16, uint8) => euint16 test 2 (7, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint16_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 224n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint16, uint8) => euint16 test 3 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(11n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint16_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 352n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint16, uint8) => euint16 test 4 (11, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(11n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint16_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 5632n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 1 (341149861, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(341149861n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 358754624n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 2 (2, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(2n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 128n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 3 (6, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(6n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 384n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 4 (6, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(6n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 24n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 1 (341149861, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(341149861n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint32_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 358754624n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 2 (2, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(2n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint32_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 128n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 3 (6, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint32_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 384n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 4 (6, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint32_uint8(encryptedAmount.handles[0], 2n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 24n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint32, euint8) => euint32 test 1 (1687773184, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(1687773184n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 13185728n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint32, euint8) => euint32 test 2 (3, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(3n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint32, euint8) => euint32 test 3 (7, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(7n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint32, euint8) => euint32 test 4 (7, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(7n);
    input.add8(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint32, uint8) => euint32 test 1 (1687773184, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(1687773184n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint32_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 13185728n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint32, uint8) => euint32 test 2 (3, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(3n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint32_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint32, uint8) => euint32 test 3 (7, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint32_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint32, uint8) => euint32 test 4 (7, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint32_uint8(encryptedAmount.handles[0], 3n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint32, euint8) => euint32 test 1 (3232617220, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(3232617220n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1855464965n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint32, euint8) => euint32 test 2 (7, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(7n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 14336n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint32, euint8) => euint32 test 3 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(11n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 22528n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint32, euint8) => euint32 test 4 (11, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(11n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1408n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint32, uint8) => euint32 test 1 (3232617220, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(3232617220n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint32_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1855464965n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint32, uint8) => euint32 test 2 (7, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint32_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 14336n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint32, uint8) => euint32 test 3 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(11n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint32_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 22528n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint32, uint8) => euint32 test 4 (11, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(11n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint32_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1408n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint32, euint8) => euint32 test 1 (328740886, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(328740886n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1481531584n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint32, euint8) => euint32 test 2 (2, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(2n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 134217728n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint32, euint8) => euint32 test 3 (6, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(6n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 402653184n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint32, euint8) => euint32 test 4 (6, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(6n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2147483649n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint32, uint8) => euint32 test 1 (328740886, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(328740886n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint32_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1481531584n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint32, uint8) => euint32 test 2 (2, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(2n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint32_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 134217728n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint32, uint8) => euint32 test 3 (6, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint32_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 402653184n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint32, uint8) => euint32 test 4 (6, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint32_uint8(encryptedAmount.handles[0], 2n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2147483649n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 1 (18443380118524278997, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443380118524278997n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 17585571546279761152n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1024n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2048n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 128n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 1 (18443380118524278997, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443380118524278997n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint64_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 17585571546279761152n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint64_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1024n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint64_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2048n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint64_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 128n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 1 (18444962543737090041, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18444962543737090041n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 288202539745892031n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 2 (2, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(2n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 3 (6, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(6n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 4 (6, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(6n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 1 (18444962543737090041, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18444962543737090041n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint64_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 288202539745892031n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 2 (2, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(2n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint64_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 3 (6, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint64_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 4 (6, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint64_uint8(encryptedAmount.handles[0], 2n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint64, euint8) => euint64 test 1 (18438964491562395283, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18438964491562395283n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 16455171044037530623n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint64, euint8) => euint64 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1024n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint64, euint8) => euint64 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2048n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint64, euint8) => euint64 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 128n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint64, uint8) => euint64 test 1 (18438964491562395283, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18438964491562395283n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint64_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 16455171044037530623n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint64, uint8) => euint64 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint64_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1024n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint64, uint8) => euint64 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint64_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2048n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint64, uint8) => euint64 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint64_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 128n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint64, euint8) => euint64 test 1 (18443879493471006695, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443879493471006695n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 11529170287002242495n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint64, euint8) => euint64 test 2 (2, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(2n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 576460752303423488n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint64, euint8) => euint64 test 3 (6, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(6n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1729382256910270464n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint64, euint8) => euint64 test 4 (6, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(6n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 9223372036854775809n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint64, uint8) => euint64 test 1 (18443879493471006695, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18443879493471006695n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint64_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 11529170287002242495n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint64, uint8) => euint64 test 2 (2, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(2n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint64_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 576460752303423488n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint64, uint8) => euint64 test 3 (6, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint64_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1729382256910270464n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint64, uint8) => euint64 test 4 (6, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint64_uint8(encryptedAmount.handles[0], 2n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 9223372036854775809n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint128, euint8) => euint128 test 1 (340282366920938463463367314747546687795, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463367314747546687795n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463455906898788927982592n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint128, euint8) => euint128 test 2 (6, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(6n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 6144n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint128, euint8) => euint128 test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(10n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 10240n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint128, euint8) => euint128 test 4 (10, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(10n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 640n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint128, uint8) => euint128 test 1 (340282366920938463463367314747546687795, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463367314747546687795n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint128_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463455906898788927982592n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint128, uint8) => euint128 test 2 (6, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint128_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 6144n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint128, uint8) => euint128 test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(10n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint128_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 10240n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint128, uint8) => euint128 test 4 (10, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(10n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint128_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 640n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint128, euint8) => euint128 test 1 (340282366920938463463373465622374264757, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463373465622374264757n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 664613997892457936451901300043699735n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint128, euint8) => euint128 test 2 (5, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(5n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint128, euint8) => euint128 test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(9n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint128, euint8) => euint128 test 4 (9, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(9n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint128, uint8) => euint128 test 1 (340282366920938463463373465622374264757, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463373465622374264757n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint128_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 664613997892457936451901300043699735n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint128, uint8) => euint128 test 2 (5, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint128_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint128, uint8) => euint128 test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(9n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint128_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint128, uint8) => euint128 test 4 (9, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(9n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint128_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint128, euint8) => euint128 test 1 (340282366920938463463371326193757672417, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463371326193757672417n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463269607815430962239n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint128, euint8) => euint128 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(1n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 32n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint128, euint8) => euint128 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(5n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 160n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint128, euint8) => euint128 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(5n);
    input.add8(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 10n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint128, uint8) => euint128 test 1 (340282366920938463463371326193757672417, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463371326193757672417n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint128_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463269607815430962239n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint128, uint8) => euint128 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(1n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint128_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 32n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint128, uint8) => euint128 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint128_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 160n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint128, uint8) => euint128 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint128_uint8(encryptedAmount.handles[0], 1n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 10n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint128, euint8) => euint128 test 1 (340282366920938463463369358650468868711, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463369358650468868711n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 69119855780815625390997956883051942673n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint128, euint8) => euint128 test 2 (5, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(5n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3323069989462289682259517650700861440n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint128, euint8) => euint128 test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(9n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 5981525981032121428067131771261550592n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint128, euint8) => euint128 test 4 (9, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(9n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 95704415696513942849074108340184809472n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint128, uint8) => euint128 test 1 (340282366920938463463369358650468868711, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463369358650468868711n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint128_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 69119855780815625390997956883051942673n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint128, uint8) => euint128 test 2 (5, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint128_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3323069989462289682259517650700861440n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint128, uint8) => euint128 test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(9n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint128_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 5981525981032121428067131771261550592n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint128, uint8) => euint128 test 4 (9, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(9n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint128_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 95704415696513942849074108340184809472n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457577809131235388115, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577809131235388115n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039444888902593701910528n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint256, euint8) => euint256 test 2 (7, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(7n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 14336n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint256, euint8) => euint256 test 3 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(11n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 22528n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint256, euint8) => euint256 test 4 (11, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(11n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1408n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint256, uint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457577809131235388115, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577809131235388115n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint256_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039444888902593701910528n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint256, uint8) => euint256 test 2 (7, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint256_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 14336n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint256, uint8) => euint256 test 3 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(11n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint256_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 22528n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint256, uint8) => euint256 test 4 (11, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(11n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint256_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1408n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457583166697072744393, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457583166697072744393n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3618502788666131106986593281521497120414687020801267626233049473959283523262n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint256, euint8) => euint256 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(1n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint256, euint8) => euint256 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(5n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint256, euint8) => euint256 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(5n);
    input.add8(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint256, uint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457583166697072744393, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457583166697072744393n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint256_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3618502788666131106986593281521497120414687020801267626233049473959283523262n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint256, uint8) => euint256 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(1n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint256_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint256, uint8) => euint256 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint256_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint256, uint8) => euint256 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint256_uint8(encryptedAmount.handles[0], 1n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457577336843463310585, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577336843463310585n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039443921657236487131135n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint256, euint8) => euint256 test 2 (7, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(7n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 14336n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint256, euint8) => euint256 test 3 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(11n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 22528n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint256, euint8) => euint256 test 4 (11, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(11n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1408n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint256, uint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457577336843463310585, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577336843463310585n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint256_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039443921657236487131135n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint256, uint8) => euint256 test 2 (7, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint256_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 14336n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint256, uint8) => euint256 test 3 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(11n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint256_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 22528n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint256, uint8) => euint256 test 4 (11, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(11n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint256_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1408n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457582863909982017621, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582863909982017621n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 39803530675327442176852526096736468324561557228813943888563544484845089132129n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint256, euint8) => euint256 test 2 (2, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(2n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3618502788666131106986593281521497120414687020801267626233049500247285301248n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint256, euint8) => euint256 test 3 (6, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(6n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 10855508365998393320959779844564491361244061062403802878699148500741855903744n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint256, euint8) => euint256 test 4 (6, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(6n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 57896044618658097711785492504343953926634992332820282019728792003956564819969n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint256, uint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457582863909982017621, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582863909982017621n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint256_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 39803530675327442176852526096736468324561557228813943888563544484845089132129n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint256, uint8) => euint256 test 2 (2, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(2n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint256_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3618502788666131106986593281521497120414687020801267626233049500247285301248n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint256, uint8) => euint256 test 3 (6, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint256_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 10855508365998393320959779844564491361244061062403802878699148500741855903744n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint256, uint8) => euint256 test 4 (6, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint256_uint8(encryptedAmount.handles[0], 2n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 57896044618658097711785492504343953926634992332820282019728792003956564819969n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "neg" overload (euint8) => euint8 test 1 (36)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(36n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.neg_euint8(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 220n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "not" overload (euint8) => euint8 test 1 (215)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(215n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.not_euint8(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 40n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "neg" overload (euint16) => euint16 test 1 (51125)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(51125n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.neg_euint16(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 14411n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "not" overload (euint16) => euint16 test 1 (44654)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add16(44654n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.not_euint16(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 20881n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "neg" overload (euint32) => euint32 test 1 (2813394568)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(2813394568n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.neg_euint32(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1481572728n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "not" overload (euint32) => euint32 test 1 (1393126997)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add32(1393126997n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.not_euint32(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2901840298n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "neg" overload (euint64) => euint64 test 1 (18438825483840604361)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18438825483840604361n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.neg_euint64(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 7918589868947255n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "not" overload (euint64) => euint64 test 1 (18445742971723367971)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add64(18445742971723367971n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.not_euint64(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1001101986183644n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "neg" overload (euint128) => euint128 test 1 (340282366920938463463368219347540256709)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368219347540256709n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.neg_euint128(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 6388084227954747n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "not" overload (euint128) => euint128 test 1 (340282366920938463463366575739489952737)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366575739489952737n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.not_euint128(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 8031692278258718n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "neg" overload (euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457576295030657801447)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576295030657801447n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.neg_euint256(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 7712882471838489n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
