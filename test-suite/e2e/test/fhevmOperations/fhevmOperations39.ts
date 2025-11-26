import { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/signers';
import { assert } from 'chai';
import { ethers } from 'hardhat';

import type { FHEVMTestSuite1 } from '../../types/contracts/tests/FHEVMTestSuite1';
import type { FHEVMTestSuite2 } from '../../types/contracts/tests/FHEVMTestSuite2';
import type { FHEVMTestSuite3 } from '../../types/contracts/tests/FHEVMTestSuite3';
import type { FHEVMTestSuite4 } from '../../types/contracts/tests/FHEVMTestSuite4';
import type { FHEVMTestSuite5 } from '../../types/contracts/tests/FHEVMTestSuite5';
import type { FHEVMTestSuite6 } from '../../types/contracts/tests/FHEVMTestSuite6';
import type { FHEVMTestSuite7 } from '../../types/contracts/tests/FHEVMTestSuite7';
import { createInstance } from '../instance';
import { getSigner, getSigners, initSigners } from '../signers';

async function deployFHEVMTestFixture1(signer: HardhatEthersSigner): Promise<FHEVMTestSuite1> {
  const admin = signer;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite1');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture2(signer: HardhatEthersSigner): Promise<FHEVMTestSuite2> {
  const admin = signer;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite2');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture3(signer: HardhatEthersSigner): Promise<FHEVMTestSuite3> {
  const admin = signer;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite3');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture4(signer: HardhatEthersSigner): Promise<FHEVMTestSuite4> {
  const admin = signer;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite4');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture5(signer: HardhatEthersSigner): Promise<FHEVMTestSuite5> {
  const admin = signer;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite5');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture6(signer: HardhatEthersSigner): Promise<FHEVMTestSuite6> {
  const admin = signer;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite6');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture7(signer: HardhatEthersSigner): Promise<FHEVMTestSuite7> {
  const admin = signer;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite7');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

describe('FHEVM operations 39', function () {
  before(async function () {
    this.signer = await getSigner(39);

    const contract1 = await deployFHEVMTestFixture1(this.signer);
    this.contract1Address = await contract1.getAddress();
    this.contract1 = contract1;

    const contract2 = await deployFHEVMTestFixture2(this.signer);
    this.contract2Address = await contract2.getAddress();
    this.contract2 = contract2;

    const contract3 = await deployFHEVMTestFixture3(this.signer);
    this.contract3Address = await contract3.getAddress();
    this.contract3 = contract3;

    const contract4 = await deployFHEVMTestFixture4(this.signer);
    this.contract4Address = await contract4.getAddress();
    this.contract4 = contract4;

    const contract5 = await deployFHEVMTestFixture5(this.signer);
    this.contract5Address = await contract5.getAddress();
    this.contract5 = contract5;

    const contract6 = await deployFHEVMTestFixture6(this.signer);
    this.contract6Address = await contract6.getAddress();
    this.contract6 = contract6;

    const contract7 = await deployFHEVMTestFixture7(this.signer);
    this.contract7Address = await contract7.getAddress();
    this.contract7 = contract7;

    const instance = await createInstance();
    this.instance = instance;
  });

  it('test operator "eq" overload (euint128, euint64) => ebool test 1 (340282366920938463463369957482416745355, 18443626937865458463)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(340282366920938463463369957482416745355n);
    input.add64(18443626937865458463n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "eq" overload (euint128, euint64) => ebool test 2 (18443626937865458459, 18443626937865458463)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(18443626937865458459n);
    input.add64(18443626937865458463n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "eq" overload (euint128, euint64) => ebool test 3 (18443626937865458463, 18443626937865458463)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(18443626937865458463n);
    input.add64(18443626937865458463n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "eq" overload (euint128, euint64) => ebool test 4 (18443626937865458463, 18443626937865458459)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(18443626937865458463n);
    input.add64(18443626937865458459n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 1 (15, 18439609981023544053)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(15n);
    input.add64(18439609981023544053n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18439609981023544053n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 2 (11, 15)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(11n);
    input.add64(15n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 15n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 3 (15, 15)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(15n);
    input.add64(15n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 15n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 4 (15, 11)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(15n);
    input.add64(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 15n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "mul" overload (euint128, euint16) => euint128 test 1 (16385, 2)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(16385n);
    input.add16(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 32770n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "mul" overload (euint128, euint16) => euint128 test 2 (199, 199)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(199n);
    input.add16(199n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 39601n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "mul" overload (euint128, euint16) => euint128 test 3 (199, 199)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(199n);
    input.add16(199n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 39601n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "mul" overload (euint128, euint16) => euint128 test 4 (199, 199)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(199n);
    input.add16(199n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 39601n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "add" overload (euint128, euint32) => euint128 test 1 (2147483649, 2)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(2147483649n);
    input.add32(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2147483651n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "add" overload (euint128, euint32) => euint128 test 2 (468914479, 468914483)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(468914479n);
    input.add32(468914483n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 937828962n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "add" overload (euint128, euint32) => euint128 test 3 (468914483, 468914483)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(468914483n);
    input.add32(468914483n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 937828966n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "add" overload (euint128, euint32) => euint128 test 4 (468914483, 468914479)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(468914483n);
    input.add32(468914479n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 937828962n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 1 (42893, 26118)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add16(42893n);
    input.add16(26118n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 9732n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 2 (26114, 26118)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add16(26114n);
    input.add16(26118n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 26114n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 3 (26118, 26118)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add16(26118n);
    input.add16(26118n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 26118n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 4 (26118, 26114)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add16(26118n);
    input.add16(26114n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 26114n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "rotr" overload (euint32, uint8) => euint32 test 1 (14421585, 11)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add32(14421585n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.rotr_euint32_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3391101825n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "rotr" overload (euint32, uint8) => euint32 test 2 (7, 11)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add32(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.rotr_euint32_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 14680064n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "rotr" overload (euint32, uint8) => euint32 test 3 (11, 11)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add32(11n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.rotr_euint32_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 23068672n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "rotr" overload (euint32, uint8) => euint32 test 4 (11, 7)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add32(11n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.rotr_euint32_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 369098752n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });
});
