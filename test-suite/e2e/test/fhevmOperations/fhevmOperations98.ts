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

describe('FHEVM operations 98', function () {
  before(async function () {
    this.signer = await getSigner(98);

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

  it('test operator "min" overload (uint64, euint64) => euint64 test 1 (18445601212142589175, 18438285349872957297)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add64(18438285349872957297n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_uint64_euint64(
      18445601212142589175n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438285349872957297n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 2 (18441651294918459839, 18441651294918459843)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add64(18441651294918459843n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_uint64_euint64(
      18441651294918459839n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18441651294918459839n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 3 (18441651294918459843, 18441651294918459843)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add64(18441651294918459843n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_uint64_euint64(
      18441651294918459843n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18441651294918459843n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 4 (18441651294918459843, 18441651294918459839)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add64(18441651294918459839n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_uint64_euint64(
      18441651294918459843n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18441651294918459839n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "not" overload (euint16) => euint16 test 1 (28750)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add16(28750n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.not_euint16(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 36785n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint8, euint8) => ebool test 1 (37, 246)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add8(37n);
    input.add8(246n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint8_euint8(
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

  it('test operator "le" overload (euint8, euint8) => ebool test 2 (33, 37)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add8(33n);
    input.add8(37n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint8_euint8(
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

  it('test operator "le" overload (euint8, euint8) => ebool test 3 (37, 37)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add8(37n);
    input.add8(37n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint8_euint8(
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

  it('test operator "le" overload (euint8, euint8) => ebool test 4 (37, 33)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add8(37n);
    input.add8(33n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint8_euint8(
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

  it('test operator "add" overload (uint8, euint8) => euint8 test 1 (25, 29)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add8(29n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_uint8_euint8(25n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 54n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 2 (12, 16)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add8(16n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_uint8_euint8(12n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 3 (16, 16)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add8(16n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_uint8_euint8(16n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 32n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 4 (16, 12)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add8(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_uint8_euint8(16n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 1 (89, 63)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add8(89n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint8_uint8(encryptedAmount.handles[0], 63n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 2 (85, 89)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add8(85n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint8_uint8(encryptedAmount.handles[0], 89n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 3 (89, 89)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add8(89n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint8_uint8(encryptedAmount.handles[0], 89n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 4 (89, 85)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add8(89n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint8_uint8(encryptedAmount.handles[0], 85n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 1 (3474197211, 1627200460)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(3474197211n);
    input.add32(1627200460n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3474197211n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 2 (1627200456, 1627200460)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(1627200456n);
    input.add32(1627200460n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1627200460n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 3 (1627200460, 1627200460)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(1627200460n);
    input.add32(1627200460n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1627200460n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 4 (1627200460, 1627200456)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(1627200460n);
    input.add32(1627200456n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1627200460n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
