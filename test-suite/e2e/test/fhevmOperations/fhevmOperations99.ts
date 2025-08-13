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

describe('FHEVM operations 99', function () {
  before(async function () {
    this.signer = await getSigner(99);

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

  it('test operator "and" overload (euint128, euint8) => euint128 test 1 (340282366920938463463366902983863640519, 212)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(340282366920938463463366902983863640519n);
    input.add8(212n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 196n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint128, euint8) => euint128 test 2 (208, 212)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(208n);
    input.add8(212n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 208n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint128, euint8) => euint128 test 3 (212, 212)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(212n);
    input.add8(212n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 212n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint128, euint8) => euint128 test 4 (212, 208)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(212n);
    input.add8(208n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 208n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 1 (3747531142, 3694787920)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(3747531142n);
    input.add32(3694787920n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint32_euint32(
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

  it('test operator "lt" overload (euint32, euint32) => ebool test 2 (3694787916, 3694787920)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(3694787916n);
    input.add32(3694787920n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint32_euint32(
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

  it('test operator "lt" overload (euint32, euint32) => ebool test 3 (3694787920, 3694787920)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(3694787920n);
    input.add32(3694787920n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint32_euint32(
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

  it('test operator "lt" overload (euint32, euint32) => ebool test 4 (3694787920, 3694787916)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(3694787920n);
    input.add32(3694787916n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint32_euint32(
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

  it('test operator "shr" overload (euint256, uint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457582465378165701305, 11)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582465378165701305n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint256_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 56539106072908298546665520023773392506479484700019806659891397688172932471n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint256, uint8) => euint256 test 2 (7, 11)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add256(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint256_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint256, uint8) => euint256 test 3 (11, 11)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add256(11n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint256_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint256, uint8) => euint256 test 4 (11, 7)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add256(11n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint256_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 1 (129, 2)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add64(129n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 131n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 2 (107, 109)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add64(107n);
    input.add8(109n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 216n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 3 (109, 109)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add64(109n);
    input.add8(109n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 218n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 4 (109, 107)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add64(109n);
    input.add8(107n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 216n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, uint16) => euint16 test 1 (41842, 58736)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add16(41842n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint16_uint16(encryptedAmount.handles[0], 58736n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 17922n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, uint16) => euint16 test 2 (30005, 30009)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add16(30005n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint16_uint16(encryptedAmount.handles[0], 30009n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 12n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, uint16) => euint16 test 3 (30009, 30009)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add16(30009n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint16_uint16(encryptedAmount.handles[0], 30009n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, uint16) => euint16 test 4 (30009, 30005)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add16(30009n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint16_uint16(encryptedAmount.handles[0], 30005n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 12n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "neg" overload (euint16) => euint16 test 1 (42455)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add16(42455n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.neg_euint16(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 23081n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
