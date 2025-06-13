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

describe('FHEVM operations 44', function () {
  before(async function () {
    this.signer = await getSigner(44);

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

  it('test operator "max" overload (euint8, uint8) => euint8 test 1 (170, 50)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(170n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint8_uint8(encryptedAmount.handles[0], 50n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 170n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 2 (38, 42)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(38n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint8_uint8(encryptedAmount.handles[0], 42n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 42n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 3 (42, 42)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(42n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint8_uint8(encryptedAmount.handles[0], 42n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 42n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 4 (42, 38)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(42n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint8_uint8(encryptedAmount.handles[0], 38n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 42n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint8, uint8) => euint8 test 1 (8, 173)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint8_uint8(encryptedAmount.handles[0], 173n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 173n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint8, uint8) => euint8 test 2 (4, 8)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint8_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 12n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint8, uint8) => euint8 test 3 (8, 8)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint8_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 8n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint8, uint8) => euint8 test 4 (8, 4)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint8_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 12n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint64, uint64) => euint64 test 1 (18446090106203399041, 18438644060879307043)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(18446090106203399041n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint64_uint64(
      encryptedAmount.handles[0],
      18438644060879307043n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438063499202642177n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint64, uint64) => euint64 test 2 (18444736153789829021, 18444736153789829025)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(18444736153789829021n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint64_uint64(
      encryptedAmount.handles[0],
      18444736153789829025n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18444736153789828993n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint64, uint64) => euint64 test 3 (18444736153789829025, 18444736153789829025)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(18444736153789829025n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint64_uint64(
      encryptedAmount.handles[0],
      18444736153789829025n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18444736153789829025n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint64, uint64) => euint64 test 4 (18444736153789829025, 18444736153789829021)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(18444736153789829025n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint64_uint64(
      encryptedAmount.handles[0],
      18444736153789829021n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18444736153789828993n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint128, euint16) => ebool test 1 (340282366920938463463373077290914922863, 32708)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(340282366920938463463373077290914922863n);
    input.add16(32708n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint128_euint16(
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
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint128, euint16) => ebool test 2 (32704, 32708)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(32704n);
    input.add16(32708n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint128_euint16(
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
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint128, euint16) => ebool test 3 (32708, 32708)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(32708n);
    input.add16(32708n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint128_euint16(
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
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint128, euint16) => ebool test 4 (32708, 32704)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(32708n);
    input.add16(32704n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint128_euint16(
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
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint128, uint8) => euint128 test 1 (340282366920938463463366647627634768153, 7)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(340282366920938463463366647627634768153n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.rotl_euint128_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463462355752502687468799n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint128, uint8) => euint128 test 2 (3, 7)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(3n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.rotl_euint128_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 384n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint128, uint8) => euint128 test 3 (7, 7)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.rotl_euint128_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 896n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint128, uint8) => euint128 test 4 (7, 3)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.rotl_euint128_uint8(encryptedAmount.handles[0], 3n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 56n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (uint16, euint16) => euint16 test 1 (4656, 4656)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);

    input.add16(4656n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_uint16_euint16(4656n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (uint16, euint16) => euint16 test 2 (4656, 4652)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);

    input.add16(4652n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_uint16_euint16(4656n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
