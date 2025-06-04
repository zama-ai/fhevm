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

  it('test operator "mul" overload (euint32, euint64) => euint64 test 1 (2, 2147116975)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add32(2n);
    input.add64(2147116975n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4294233950n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 2 (57137, 57137)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add32(57137n);
    input.add64(57137n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3264636769n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 3 (57137, 57137)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add32(57137n);
    input.add64(57137n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3264636769n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 4 (57137, 57137)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add32(57137n);
    input.add64(57137n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3264636769n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 1 (2, 237)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(2n);
    input.add32(237n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 239n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 2 (31, 35)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(31n);
    input.add32(35n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 66n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 3 (35, 35)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(35n);
    input.add32(35n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 70n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 4 (35, 31)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(35n);
    input.add32(31n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 66n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint256, uint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457576475934570167893, 115792089237316195423570985008687907853269984665640564039457577032192106692389)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576475934570167893n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457577032192106692389n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457576466998529998341n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint256, uint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457576475934570167889, 115792089237316195423570985008687907853269984665640564039457576475934570167893)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576475934570167889n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457576475934570167893n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457576475934570167889n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint256, uint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457576475934570167893, 115792089237316195423570985008687907853269984665640564039457576475934570167893)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576475934570167893n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457576475934570167893n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457576475934570167893n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint256, uint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457576475934570167893, 115792089237316195423570985008687907853269984665640564039457576475934570167889)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576475934570167893n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457576475934570167889n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457576475934570167889n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 1 (103, 89)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(103n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint8_uint8(encryptedAmount.handles[0], 89n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 2 (99, 103)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(99n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint8_uint8(encryptedAmount.handles[0], 103n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 3 (103, 103)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(103n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint8_uint8(encryptedAmount.handles[0], 103n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 4 (103, 99)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(103n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint8_uint8(encryptedAmount.handles[0], 99n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint8, euint256) => euint256 test 1 (108, 115792089237316195423570985008687907853269984665640564039457582199083036821869)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(108n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582199083036821869n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457582199083036821869n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint8, euint256) => euint256 test 2 (104, 108)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(104n);
    input.add256(108n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 108n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint8, euint256) => euint256 test 3 (108, 108)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(108n);
    input.add256(108n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 108n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint8, euint256) => euint256 test 4 (108, 104)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(108n);
    input.add256(104n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 108n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 1 (12, 63000)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(12n);
    input.add16(63000n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 63004n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 2 (8, 12)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(8n);
    input.add16(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 12n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 3 (12, 12)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(12n);
    input.add16(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 12n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 4 (12, 8)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(12n);
    input.add16(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 12n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
