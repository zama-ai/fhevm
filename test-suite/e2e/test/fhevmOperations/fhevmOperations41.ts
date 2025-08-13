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

describe('FHEVM operations 41', function () {
  before(async function () {
    this.signer = await getSigner(41);

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

  it('test operator "eq" overload (euint256, euint16) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457583972799541144779, 46649)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457583972799541144779n);
    input.add16(46649n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint256_euint16(
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

  it('test operator "eq" overload (euint256, euint16) => ebool test 2 (46645, 46649)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add256(46645n);
    input.add16(46649n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint256_euint16(
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

  it('test operator "eq" overload (euint256, euint16) => ebool test 3 (46649, 46649)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add256(46649n);
    input.add16(46649n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint256_euint16(
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

  it('test operator "eq" overload (euint256, euint16) => ebool test 4 (46649, 46645)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add256(46649n);
    input.add16(46645n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint256_euint16(
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

  it('test operator "xor" overload (euint8, euint128) => euint128 test 1 (128, 340282366920938463463366713208224548189)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(128n);
    input.add128(340282366920938463463366713208224548189n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463366713208224548317n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint8, euint128) => euint128 test 2 (124, 128)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(124n);
    input.add128(128n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 252n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint8, euint128) => euint128 test 3 (128, 128)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(128n);
    input.add128(128n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint8, euint128) => euint128 test 4 (128, 124)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(128n);
    input.add128(124n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 252n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint128, uint8) => euint128 test 1 (340282366920938463463372539450663984667, 5)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(340282366920938463463372539450663984667n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.shl_euint128_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463308432036432954208n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint128, uint8) => euint128 test 2 (1, 5)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(1n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.shl_euint128_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 32n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint128, uint8) => euint128 test 3 (5, 5)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.shl_euint128_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 160n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint128, uint8) => euint128 test 4 (5, 1)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.shl_euint128_uint8(encryptedAmount.handles[0], 1n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 10n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 1 (169, 504173202)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(169n);
    input.add32(504173202n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint8_euint32(
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

  it('test operator "ge" overload (euint8, euint32) => ebool test 2 (165, 169)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(165n);
    input.add32(169n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint8_euint32(
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

  it('test operator "ge" overload (euint8, euint32) => ebool test 3 (169, 169)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(169n);
    input.add32(169n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint8_euint32(
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

  it('test operator "ge" overload (euint8, euint32) => ebool test 4 (169, 165)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(169n);
    input.add32(165n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint8_euint32(
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

  it('test operator "eq" overload (euint8, euint32) => ebool test 1 (221, 253233828)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(221n);
    input.add32(253233828n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint8_euint32(
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

  it('test operator "eq" overload (euint8, euint32) => ebool test 2 (217, 221)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(217n);
    input.add32(221n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint8_euint32(
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

  it('test operator "eq" overload (euint8, euint32) => ebool test 3 (221, 221)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(221n);
    input.add32(221n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint8_euint32(
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

  it('test operator "eq" overload (euint8, euint32) => ebool test 4 (221, 217)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(221n);
    input.add32(217n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint8_euint32(
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

  it('test operator "ne" overload (euint16, euint16) => ebool test 1 (38952, 2639)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add16(38952n);
    input.add16(2639n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint16_euint16(
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

  it('test operator "ne" overload (euint16, euint16) => ebool test 2 (2635, 2639)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add16(2635n);
    input.add16(2639n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint16_euint16(
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

  it('test operator "ne" overload (euint16, euint16) => ebool test 3 (2639, 2639)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add16(2639n);
    input.add16(2639n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint16_euint16(
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

  it('test operator "ne" overload (euint16, euint16) => ebool test 4 (2639, 2635)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add16(2639n);
    input.add16(2635n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint16_euint16(
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
});
