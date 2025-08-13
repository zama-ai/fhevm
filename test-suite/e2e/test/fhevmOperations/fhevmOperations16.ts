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

describe('FHEVM operations 16', function () {
  before(async function () {
    this.signer = await getSigner(16);

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

  it('test operator "gt" overload (euint8, euint16) => ebool test 1 (154, 9069)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add8(154n);
    input.add16(9069n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 2 (150, 154)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add8(150n);
    input.add16(154n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 3 (154, 154)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add8(154n);
    input.add16(154n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 4 (154, 150)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add8(154n);
    input.add16(150n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 1 (9219006391713233981, 9222153868357440028)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add64(9219006391713233981n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint64_uint64(
      encryptedAmount.handles[0],
      9222153868357440028n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18441160260070674009n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 2 (9219006391713233979, 9219006391713233981)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add64(9219006391713233979n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint64_uint64(
      encryptedAmount.handles[0],
      9219006391713233981n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438012783426467960n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 3 (9219006391713233981, 9219006391713233981)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add64(9219006391713233981n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint64_uint64(
      encryptedAmount.handles[0],
      9219006391713233981n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438012783426467962n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 4 (9219006391713233981, 9219006391713233979)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add64(9219006391713233981n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint64_uint64(
      encryptedAmount.handles[0],
      9219006391713233979n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438012783426467960n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint16, euint64) => euint64 test 1 (16992, 16992)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(16992n);
    input.add64(16992n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint16, euint64) => euint64 test 2 (16992, 16988)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(16992n);
    input.add64(16988n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint256, euint32) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457579108058068580343, 1752296538)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579108058068580343n);
    input.add32(1752296538n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457579108059818754047n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint256, euint32) => euint256 test 2 (1752296534, 1752296538)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add256(1752296534n);
    input.add32(1752296538n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1752296542n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint256, euint32) => euint256 test 3 (1752296538, 1752296538)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add256(1752296538n);
    input.add32(1752296538n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1752296538n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint256, euint32) => euint256 test 4 (1752296538, 1752296534)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add256(1752296538n);
    input.add32(1752296534n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1752296542n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 1 (41842, 30009)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(41842n);
    input.add16(30009n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 54859n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 2 (30005, 30009)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(30005n);
    input.add16(30009n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 12n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 3 (30009, 30009)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(30009n);
    input.add16(30009n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 4 (30009, 30005)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(30009n);
    input.add16(30005n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 12n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 1 (940197925, 518965364)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(940197925n);
    input.add32(518965364n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 2 (518965360, 518965364)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(518965360n);
    input.add32(518965364n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 3 (518965364, 518965364)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(518965364n);
    input.add32(518965364n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 4 (518965364, 518965360)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(518965364n);
    input.add32(518965360n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });
});
