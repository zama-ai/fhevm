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

describe('FHEVM operations 96', function () {
  before(async function () {
    this.signer = await getSigner(96);

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

  it('test operator "and" overload (euint16, euint32) => euint32 test 1 (11024, 30354842)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add16(11024n);
    input.add32(30354842n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 10512n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 2 (11020, 11024)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add16(11020n);
    input.add32(11024n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 11008n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 3 (11024, 11024)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add16(11024n);
    input.add32(11024n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 11024n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 4 (11024, 11020)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add16(11024n);
    input.add32(11020n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 11008n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 1 (18443078227457327121, 18442642039167027169)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add64(18442642039167027169n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_uint64_euint64(
      18443078227457327121n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18443078227457327121n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 2 (18443029152777533209, 18443029152777533213)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add64(18443029152777533213n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_uint64_euint64(
      18443029152777533209n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18443029152777533213n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 3 (18443029152777533213, 18443029152777533213)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add64(18443029152777533213n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_uint64_euint64(
      18443029152777533213n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18443029152777533213n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 4 (18443029152777533213, 18443029152777533209)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add64(18443029152777533209n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_uint64_euint64(
      18443029152777533213n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18443029152777533213n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint32, euint8) => euint32 test 1 (2194366101, 11)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(2194366101n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3534772587n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint32, euint8) => euint32 test 2 (7, 11)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(7n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 14680064n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint32, euint8) => euint32 test 3 (11, 11)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(11n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 23068672n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint32, euint8) => euint32 test 4 (11, 7)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(11n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 369098752n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 1 (65, 2)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add64(65n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 130n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 2 (12, 12)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add64(12n);
    input.add8(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 144n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 3 (12, 12)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add64(12n);
    input.add8(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 144n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 4 (12, 12)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add64(12n);
    input.add8(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 144n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 1 (16105, 10259)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add16(16105n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint16_uint16(encryptedAmount.handles[0], 10259n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 26364n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 2 (16101, 16105)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add16(16101n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint16_uint16(encryptedAmount.handles[0], 16105n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 32206n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 3 (16105, 16105)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add16(16105n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint16_uint16(encryptedAmount.handles[0], 16105n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 32210n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 4 (16105, 16101)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add16(16105n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint16_uint16(encryptedAmount.handles[0], 16101n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 32206n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 1 (364745651, 35582)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(364745651n);
    input.add16(35582n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 2 (35578, 35582)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(35578n);
    input.add16(35582n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 3 (35582, 35582)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(35582n);
    input.add16(35582n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 4 (35582, 35578)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(35582n);
    input.add16(35578n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });
});
