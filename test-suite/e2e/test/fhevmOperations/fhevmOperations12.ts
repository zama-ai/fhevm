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

describe('FHEVM operations 12', function () {
  before(async function () {
    this.signer = await getSigner(12);

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

  it('test operator "lt" overload (euint32, euint16) => ebool test 1 (3072374768, 43891)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(3072374768n);
    input.add16(43891n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 2 (43887, 43891)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(43887n);
    input.add16(43891n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 3 (43891, 43891)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(43891n);
    input.add16(43891n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 4 (43891, 43887)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(43891n);
    input.add16(43887n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 1 (1067111389, 18445362393245300625)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(1067111389n);
    input.add64(18445362393245300625n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18445362393245300625n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 2 (1067111385, 1067111389)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(1067111385n);
    input.add64(1067111389n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1067111389n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 3 (1067111389, 1067111389)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(1067111389n);
    input.add64(1067111389n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1067111389n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 4 (1067111389, 1067111385)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(1067111389n);
    input.add64(1067111385n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1067111389n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 1 (52016, 2)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(52016n);
    input.add16(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 52018n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 2 (14454, 14458)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(14454n);
    input.add16(14458n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28912n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 3 (14458, 14458)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(14458n);
    input.add16(14458n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28916n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 4 (14458, 14454)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(14458n);
    input.add16(14454n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28912n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 1 (18439541079614675395, 859381937)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add64(18439541079614675395n);
    input.add32(859381937n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18439541079616256499n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 2 (859381933, 859381937)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add64(859381933n);
    input.add32(859381937n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 859381949n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 3 (859381937, 859381937)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add64(859381937n);
    input.add32(859381937n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 859381937n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 4 (859381937, 859381933)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add64(859381937n);
    input.add32(859381933n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 859381949n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 1 (3857873472, 3300)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(3857873472n);
    input.add16(3300n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3300n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 2 (3296, 3300)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(3296n);
    input.add16(3300n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3296n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 3 (3300, 3300)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(3300n);
    input.add16(3300n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3300n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 4 (3300, 3296)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(3300n);
    input.add16(3296n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3296n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 1 (27495, 37)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(27495n);
    input.add8(37n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 37n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 2 (33, 37)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(33n);
    input.add8(37n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 33n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 3 (37, 37)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(37n);
    input.add8(37n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 37n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 4 (37, 33)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(37n);
    input.add8(33n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 33n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
