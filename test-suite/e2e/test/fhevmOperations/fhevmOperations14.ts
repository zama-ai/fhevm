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

describe('FHEVM operations 14', function () {
  before(async function () {
    this.signer = await getSigner(14);

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

  it('test operator "xor" overload (euint128, euint256) => euint256 test 1 (340282366920938463463371237790176916739, 115792089237316195423570985008687907853269984665640564039457580664441319097921)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add128(340282366920938463463371237790176916739n);
    input.add256(115792089237316195423570985008687907853269984665640564039457580664441319097921n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907852929702298719625575994209427373821309762n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint128, euint256) => euint256 test 2 (340282366920938463463371237790176916735, 340282366920938463463371237790176916739)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add128(340282366920938463463371237790176916735n);
    input.add256(340282366920938463463371237790176916739n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 508n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint128, euint256) => euint256 test 3 (340282366920938463463371237790176916739, 340282366920938463463371237790176916739)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add128(340282366920938463463371237790176916739n);
    input.add256(340282366920938463463371237790176916739n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint128_euint256(
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

  it('test operator "xor" overload (euint128, euint256) => euint256 test 4 (340282366920938463463371237790176916739, 340282366920938463463371237790176916735)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add128(340282366920938463463371237790176916739n);
    input.add256(340282366920938463463371237790176916735n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 508n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 1 (14177, 636623515)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(14177n);
    input.add32(636623515n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint16_euint32(
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

  it('test operator "eq" overload (euint16, euint32) => ebool test 2 (14173, 14177)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(14173n);
    input.add32(14177n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint16_euint32(
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

  it('test operator "eq" overload (euint16, euint32) => ebool test 3 (14177, 14177)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(14177n);
    input.add32(14177n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint16_euint32(
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

  it('test operator "eq" overload (euint16, euint32) => ebool test 4 (14177, 14173)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(14177n);
    input.add32(14173n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint16_euint32(
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

  it('test operator "ne" overload (euint128, euint16) => ebool test 1 (340282366920938463463368727312198655913, 63575)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add128(340282366920938463463368727312198655913n);
    input.add16(63575n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint128_euint16(
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

  it('test operator "ne" overload (euint128, euint16) => ebool test 2 (63571, 63575)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add128(63571n);
    input.add16(63575n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint128_euint16(
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

  it('test operator "ne" overload (euint128, euint16) => ebool test 3 (63575, 63575)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add128(63575n);
    input.add16(63575n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint128_euint16(
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

  it('test operator "ne" overload (euint128, euint16) => ebool test 4 (63575, 63571)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add128(63575n);
    input.add16(63571n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint128_euint16(
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

  it('test operator "ne" overload (euint16, euint256) => ebool test 1 (24695, 115792089237316195423570985008687907853269984665640564039457582882047377189715)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(24695n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582882047377189715n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint16_euint256(
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

  it('test operator "ne" overload (euint16, euint256) => ebool test 2 (24691, 24695)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(24691n);
    input.add256(24695n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint16_euint256(
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

  it('test operator "ne" overload (euint16, euint256) => ebool test 3 (24695, 24695)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(24695n);
    input.add256(24695n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint16_euint256(
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

  it('test operator "ne" overload (euint16, euint256) => ebool test 4 (24695, 24691)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(24695n);
    input.add256(24691n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint16_euint256(
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

  it('test operator "lt" overload (euint8, euint128) => ebool test 1 (47, 340282366920938463463366945922352977735)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(47n);
    input.add128(340282366920938463463366945922352977735n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint128(
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

  it('test operator "lt" overload (euint8, euint128) => ebool test 2 (43, 47)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(43n);
    input.add128(47n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint128(
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

  it('test operator "lt" overload (euint8, euint128) => ebool test 3 (47, 47)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(47n);
    input.add128(47n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint128(
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

  it('test operator "lt" overload (euint8, euint128) => ebool test 4 (47, 43)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(47n);
    input.add128(43n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint128(
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

  it('test operator "xor" overload (euint64, uint64) => euint64 test 1 (18438919973362278249, 18437988145485024287)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add64(18438919973362278249n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint64_uint64(
      encryptedAmount.handles[0],
      18437988145485024287n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1355157583598454n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint64, uint64) => euint64 test 2 (18438919973362278245, 18438919973362278249)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add64(18438919973362278245n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint64_uint64(
      encryptedAmount.handles[0],
      18438919973362278249n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 12n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint64, uint64) => euint64 test 3 (18438919973362278249, 18438919973362278249)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add64(18438919973362278249n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint64_uint64(
      encryptedAmount.handles[0],
      18438919973362278249n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint64, uint64) => euint64 test 4 (18438919973362278249, 18438919973362278245)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add64(18438919973362278249n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint64_uint64(
      encryptedAmount.handles[0],
      18438919973362278245n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 12n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
