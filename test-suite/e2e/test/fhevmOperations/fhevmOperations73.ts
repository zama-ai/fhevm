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

describe('FHEVM operations 73', function () {
  before(async function () {
    this.signer = await getSigner(73);

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

  it('test operator "mul" overload (uint128, euint128) => euint128 test 1 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);

    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_uint128_euint128(
      9223372036854775809n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 85070591730234615884290395931651604481n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (uint128, euint128) => euint128 test 2 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);

    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_uint128_euint128(
      9223372036854775809n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 85070591730234615884290395931651604481n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (uint128, euint128) => euint128 test 3 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);

    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_uint128_euint128(
      9223372036854775809n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 85070591730234615884290395931651604481n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (uint128, euint128) => euint128 test 4 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);

    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_uint128_euint128(
      9223372036854775809n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 85070591730234615884290395931651604481n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "not" overload (euint64) => euint64 test 1 (18444189886461807351)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add64(18444189886461807351n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.not_euint64(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2554187247744264n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint128, euint8) => euint128 test 1 (340282366920938463463367679758005690219, 9)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463367679758005690219n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 241919495232854688868492871440409895407n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint128, euint8) => euint128 test 2 (5, 9)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(5n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3323069989462289682259517650700861440n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint128, euint8) => euint128 test 3 (9, 9)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(9n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 5981525981032121428067131771261550592n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint128, euint8) => euint128 test 4 (9, 5)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(9n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 95704415696513942849074108340184809472n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 1 (29292, 4049428489)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(29292n);
    input.add32(4049428489n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4049428489n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 2 (29288, 29292)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(29288n);
    input.add32(29292n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 29292n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 3 (29292, 29292)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(29292n);
    input.add32(29292n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 29292n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 4 (29292, 29288)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(29292n);
    input.add32(29288n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 29292n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint256, euint128) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457576571406317754359, 340282366920938463463366796611846006713)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576571406317754359n);
    input.add128(340282366920938463463366796611846006713n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint256, euint128) => ebool test 2 (340282366920938463463366796611846006709, 340282366920938463463366796611846006713)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add256(340282366920938463463366796611846006709n);
    input.add128(340282366920938463463366796611846006713n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint256, euint128) => ebool test 3 (340282366920938463463366796611846006713, 340282366920938463463366796611846006713)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add256(340282366920938463463366796611846006713n);
    input.add128(340282366920938463463366796611846006713n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint256, euint128) => ebool test 4 (340282366920938463463366796611846006713, 340282366920938463463366796611846006709)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add256(340282366920938463463366796611846006713n);
    input.add128(340282366920938463463366796611846006709n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 1 (6240, 61)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(6240n);
    input.add8(61n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 6240n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 2 (57, 61)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(57n);
    input.add8(61n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 61n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 3 (61, 61)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(61n);
    input.add8(61n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 61n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 4 (61, 57)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(61n);
    input.add8(57n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 61n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
