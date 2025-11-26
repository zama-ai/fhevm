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

describe('FHEVM operations 44', function () {
  before(async function () {
    this.signer = await getSigner(44);

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

  it('test operator "sub" overload (euint32, euint16) => euint32 test 1 (19540, 19540)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add32(19540n);
    input.add16(19540n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "sub" overload (euint32, euint16) => euint32 test 2 (19540, 19536)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add32(19540n);
    input.add16(19536n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 1 (4269728288, 49414)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add32(4269728288n);
    input.add16(49414n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 49152n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 2 (49410, 49414)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add32(49410n);
    input.add16(49414n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 49410n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 3 (49414, 49414)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add32(49414n);
    input.add16(49414n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 49414n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 4 (49414, 49410)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add32(49414n);
    input.add16(49410n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 49410n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 1 (1348456077, 742459888)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add32(1348456077n);
    input.add32(742459888n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint32(
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 2 (742459884, 742459888)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add32(742459884n);
    input.add32(742459888n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint32(
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 3 (742459888, 742459888)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add32(742459888n);
    input.add32(742459888n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint32(
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 4 (742459888, 742459884)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add32(742459888n);
    input.add32(742459884n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint32(
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 1 (3892520086, 2401584102)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);

    input.add32(2401584102n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_uint32_euint32(
      3892520086n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 2 (742459884, 742459888)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);

    input.add32(742459888n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_uint32_euint32(
      742459884n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 3 (742459888, 742459888)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);

    input.add32(742459888n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_uint32_euint32(
      742459888n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 4 (742459888, 742459884)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);

    input.add32(742459884n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_uint32_euint32(
      742459888n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 1 (23121, 18443118401380936881)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add16(23121n);
    input.add64(18443118401380936881n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 23121n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 2 (23117, 23121)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add16(23117n);
    input.add64(23121n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 23117n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 3 (23121, 23121)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add16(23121n);
    input.add64(23121n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 23121n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 4 (23121, 23117)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add16(23121n);
    input.add64(23117n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 23117n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 1 (65, 2)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(65n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 130n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 2 (14, 16)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(14n);
    input.add8(16n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 224n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 3 (9, 9)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(9n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 81n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 4 (16, 14)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(16n);
    input.add8(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 224n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });
});
