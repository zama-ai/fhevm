import { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/signers';
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

describe('FHEVM operations 76', function () {
  before(async function () {
    this.signer = await getSigner(76);

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

  it('test operator "gt" overload (euint64, euint64) => ebool test 1 (18441518909520051975, 18446109705828114357)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(18441518909520051975n);
    input.add64(18446109705828114357n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint64_euint64(
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 2 (18441518909520051971, 18441518909520051975)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(18441518909520051971n);
    input.add64(18441518909520051975n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint64_euint64(
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 3 (18441518909520051975, 18441518909520051975)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(18441518909520051975n);
    input.add64(18441518909520051975n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint64_euint64(
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 4 (18441518909520051975, 18441518909520051971)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(18441518909520051975n);
    input.add64(18441518909520051971n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint64_euint64(
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "xor" overload (euint32, uint32) => euint32 test 1 (948248716, 1756703468)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(948248716n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint32_uint32(
      encryptedAmount.handles[0],
      1756703468n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1345325664n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "xor" overload (euint32, uint32) => euint32 test 2 (640520663, 640520667)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(640520663n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint32_uint32(
      encryptedAmount.handles[0],
      640520667n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 12n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "xor" overload (euint32, uint32) => euint32 test 3 (640520667, 640520667)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(640520667n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint32_uint32(
      encryptedAmount.handles[0],
      640520667n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "xor" overload (euint32, uint32) => euint32 test 4 (640520667, 640520663)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(640520667n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint32_uint32(
      encryptedAmount.handles[0],
      640520663n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 12n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 1 (18729, 1591888937)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add16(18729n);
    input.add32(1591888937n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint16_euint32(
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 2 (18725, 18729)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add16(18725n);
    input.add32(18729n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint16_euint32(
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 3 (18729, 18729)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add16(18729n);
    input.add32(18729n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint16_euint32(
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 4 (18729, 18725)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add16(18729n);
    input.add32(18725n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint16_euint32(
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 1 (209, 149)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add16(209n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint16_uint16(encryptedAmount.handles[0], 149n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 31141n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 2 (246, 246)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add16(246n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint16_uint16(encryptedAmount.handles[0], 246n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 60516n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 3 (246, 246)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add16(246n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint16_uint16(encryptedAmount.handles[0], 246n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 60516n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 4 (246, 246)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add16(246n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint16_uint16(encryptedAmount.handles[0], 246n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 60516n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "sub" overload (euint64, euint64) => euint64 test 1 (18443553360840341423, 18443553360840341423)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(18443553360840341423n);
    input.add64(18443553360840341423n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "sub" overload (euint64, euint64) => euint64 test 2 (18443553360840341423, 18443553360840341419)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(18443553360840341423n);
    input.add64(18443553360840341419n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 1 (195, 97)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add8(97n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint8_euint8(195n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 2 (48, 52)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add8(52n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint8_euint8(48n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 3 (52, 52)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add8(52n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint8_euint8(52n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 4 (52, 48)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add8(48n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint8_euint8(52n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });
});
