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

describe('FHEVM operations 1', function () {
  before(async function () {
    this.signer = await getSigner(1);

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

  it('test operator "mul" overload (euint32, euint64) => euint64 test 1 (2, 2146758085)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(2n);
    input.add64(2146758085n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4293516170n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 2 (36490, 36490)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(36490n);
    input.add64(36490n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1331520100n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 3 (36490, 36490)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(36490n);
    input.add64(36490n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1331520100n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 4 (36490, 36490)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(36490n);
    input.add64(36490n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1331520100n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint8, euint128) => ebool test 1 (234, 340282366920938463463368393475851372281)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(234n);
    input.add128(340282366920938463463368393475851372281n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint128(
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

  it('test operator "le" overload (euint8, euint128) => ebool test 2 (230, 234)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(230n);
    input.add128(234n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint128(
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

  it('test operator "le" overload (euint8, euint128) => ebool test 3 (234, 234)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(234n);
    input.add128(234n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint128(
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

  it('test operator "le" overload (euint8, euint128) => ebool test 4 (234, 230)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(234n);
    input.add128(230n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint128(
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

  it('test operator "xor" overload (euint32, euint256) => euint256 test 1 (2192414678, 115792089237316195423570985008687907853269984665640564039457583936362768174331)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(2192414678n);
    input.add256(115792089237316195423570985008687907853269984665640564039457583936362768174331n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457583936360596806445n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint256) => euint256 test 2 (2192414674, 2192414678)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(2192414674n);
    input.add256(2192414678n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint256) => euint256 test 3 (2192414678, 2192414678)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(2192414678n);
    input.add256(2192414678n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint256) => euint256 test 4 (2192414678, 2192414674)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(2192414678n);
    input.add256(2192414674n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint8, euint256) => ebool test 1 (246, 115792089237316195423570985008687907853269984665640564039457581298041017534679)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(246n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581298041017534679n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint256(
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

  it('test operator "ne" overload (euint8, euint256) => ebool test 2 (242, 246)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(242n);
    input.add256(246n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint256(
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

  it('test operator "ne" overload (euint8, euint256) => ebool test 3 (246, 246)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(246n);
    input.add256(246n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint256(
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

  it('test operator "ne" overload (euint8, euint256) => ebool test 4 (246, 242)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(246n);
    input.add256(242n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint256(
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

  it('test operator "eq" overload (euint128, euint128) => ebool test 1 (340282366920938463463369847520313727655, 340282366920938463463369457203716781689)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add128(340282366920938463463369847520313727655n);
    input.add128(340282366920938463463369457203716781689n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint128_euint128(
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

  it('test operator "eq" overload (euint128, euint128) => ebool test 2 (340282366920938463463369457203716781685, 340282366920938463463369457203716781689)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add128(340282366920938463463369457203716781685n);
    input.add128(340282366920938463463369457203716781689n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint128_euint128(
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

  it('test operator "eq" overload (euint128, euint128) => ebool test 3 (340282366920938463463369457203716781689, 340282366920938463463369457203716781689)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add128(340282366920938463463369457203716781689n);
    input.add128(340282366920938463463369457203716781689n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint128_euint128(
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

  it('test operator "eq" overload (euint128, euint128) => ebool test 4 (340282366920938463463369457203716781689, 340282366920938463463369457203716781685)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add128(340282366920938463463369457203716781689n);
    input.add128(340282366920938463463369457203716781685n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint128_euint128(
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

  it('test operator "eq" overload (euint256, euint64) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457578480827121145153, 18438656479126686989)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578480827121145153n);
    input.add64(18438656479126686989n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint256_euint64(
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

  it('test operator "eq" overload (euint256, euint64) => ebool test 2 (18438656479126686985, 18438656479126686989)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add256(18438656479126686985n);
    input.add64(18438656479126686989n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint256_euint64(
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

  it('test operator "eq" overload (euint256, euint64) => ebool test 3 (18438656479126686989, 18438656479126686989)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add256(18438656479126686989n);
    input.add64(18438656479126686989n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint256_euint64(
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

  it('test operator "eq" overload (euint256, euint64) => ebool test 4 (18438656479126686989, 18438656479126686985)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add256(18438656479126686989n);
    input.add64(18438656479126686985n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint256_euint64(
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
});
