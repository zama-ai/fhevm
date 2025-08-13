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

describe('FHEVM operations 70', function () {
  before(async function () {
    this.signer = await getSigner(70);

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

  it('test operator "min" overload (euint8, uint8) => euint8 test 1 (28, 160)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add8(28n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint8_uint8(encryptedAmount.handles[0], 160n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 2 (24, 28)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add8(24n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint8_uint8(encryptedAmount.handles[0], 28n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 24n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 3 (28, 28)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add8(28n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint8_uint8(encryptedAmount.handles[0], 28n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 4 (28, 24)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add8(28n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint8_uint8(encryptedAmount.handles[0], 24n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 24n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 1 (9219006391713233981, 9222924686015978976)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add64(9219006391713233981n);
    input.add64(9222924686015978976n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18441931077729212957n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 2 (9219006391713233979, 9219006391713233981)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add64(9219006391713233979n);
    input.add64(9219006391713233981n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438012783426467960n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 3 (9219006391713233981, 9219006391713233981)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add64(9219006391713233981n);
    input.add64(9219006391713233981n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438012783426467962n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 4 (9219006391713233981, 9219006391713233979)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add64(9219006391713233981n);
    input.add64(9219006391713233979n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438012783426467960n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint8, euint128) => euint128 test 1 (185, 340282366920938463463368750634998766107)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add8(185n);
    input.add128(340282366920938463463368750634998766107n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463368750634998766267n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint8, euint128) => euint128 test 2 (181, 185)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add8(181n);
    input.add128(185n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 189n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint8, euint128) => euint128 test 3 (185, 185)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add8(185n);
    input.add128(185n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 185n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint8, euint128) => euint128 test 4 (185, 181)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add8(185n);
    input.add128(181n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 189n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 1 (18441304055124096547, 2342841858)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add64(18441304055124096547n);
    input.add32(2342841858n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_euint32(
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

  it('test operator "le" overload (euint64, euint32) => ebool test 2 (2342841854, 2342841858)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add64(2342841854n);
    input.add32(2342841858n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_euint32(
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

  it('test operator "le" overload (euint64, euint32) => ebool test 3 (2342841858, 2342841858)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add64(2342841858n);
    input.add32(2342841858n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_euint32(
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

  it('test operator "le" overload (euint64, euint32) => ebool test 4 (2342841858, 2342841854)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add64(2342841858n);
    input.add32(2342841854n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_euint32(
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

  it('test operator "sub" overload (euint8, euint16) => euint16 test 1 (49, 49)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add8(49n);
    input.add16(49n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint8, euint16) => euint16 test 2 (49, 45)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add8(49n);
    input.add16(45n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint16, euint16) => euint16 test 1 (58883, 24202)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);

    input.add16(24202n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint16_euint16(58883n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 17922n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint16, euint16) => euint16 test 2 (25609, 25613)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);

    input.add16(25613n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint16_euint16(25609n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 25609n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint16, euint16) => euint16 test 3 (25613, 25613)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);

    input.add16(25613n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint16_euint16(25613n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 25613n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint16, euint16) => euint16 test 4 (25613, 25609)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);

    input.add16(25609n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint16_euint16(25613n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 25609n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
