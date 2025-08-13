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

describe('FHEVM operations 80', function () {
  before(async function () {
    this.signer = await getSigner(80);

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

  it('test operator "eq" overload (euint8, euint256) => ebool test 1 (185, 115792089237316195423570985008687907853269984665640564039457578136954620775025)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add8(185n);
    input.add256(115792089237316195423570985008687907853269984665640564039457578136954620775025n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint8, euint256) => ebool test 2 (181, 185)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add8(181n);
    input.add256(185n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint8, euint256) => ebool test 3 (185, 185)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add8(185n);
    input.add256(185n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint8, euint256) => ebool test 4 (185, 181)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add8(185n);
    input.add256(181n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint8, euint32) => euint32 test 1 (178, 178)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add8(178n);
    input.add32(178n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint8, euint32) => euint32 test 2 (178, 174)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add8(178n);
    input.add32(174n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 1 (18439400236042486555, 1693746903)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(18439400236042486555n);
    input.add32(1693746903n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1693746903n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 2 (1693746899, 1693746903)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(1693746899n);
    input.add32(1693746903n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1693746899n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 3 (1693746903, 1693746903)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(1693746903n);
    input.add32(1693746903n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1693746903n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 4 (1693746903, 1693746899)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(1693746903n);
    input.add32(1693746899n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1693746899n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 1 (1713718531, 897115282)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(1713718531n);
    input.add32(897115282n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1398645137n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 2 (897115278, 897115282)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(897115278n);
    input.add32(897115282n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 3 (897115282, 897115282)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(897115282n);
    input.add32(897115282n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 4 (897115282, 897115278)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(897115282n);
    input.add32(897115278n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 1 (93, 47)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add8(93n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint8_uint8(encryptedAmount.handles[0], 47n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 2 (89, 93)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add8(89n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint8_uint8(encryptedAmount.handles[0], 93n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 3 (93, 93)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add8(93n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint8_uint8(encryptedAmount.handles[0], 93n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 4 (93, 89)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add8(93n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint8_uint8(encryptedAmount.handles[0], 89n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint128, uint128) => ebool test 1 (340282366920938463463374044506934573231, 340282366920938463463371332918142558491)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(340282366920938463463374044506934573231n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463371332918142558491n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint128, uint128) => ebool test 2 (340282366920938463463373233245058954597, 340282366920938463463373233245058954601)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(340282366920938463463373233245058954597n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463373233245058954601n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint128, uint128) => ebool test 3 (340282366920938463463373233245058954601, 340282366920938463463373233245058954601)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(340282366920938463463373233245058954601n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463373233245058954601n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint128, uint128) => ebool test 4 (340282366920938463463373233245058954601, 340282366920938463463373233245058954597)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(340282366920938463463373233245058954601n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463373233245058954597n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });
});
