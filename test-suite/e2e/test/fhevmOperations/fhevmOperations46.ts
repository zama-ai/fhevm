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

describe('FHEVM operations 46', function () {
  before(async function () {
    this.signer = await getSigner(46);

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

  it('test operator "and" overload (euint8, euint32) => euint32 test 1 (70, 3590917557)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add8(70n);
    input.add32(3590917557n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 2 (66, 70)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add8(66n);
    input.add32(70n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 66n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 3 (70, 70)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add8(70n);
    input.add32(70n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 70n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 4 (70, 66)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add8(70n);
    input.add32(66n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 66n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 1 (4181560352, 2552358620)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add32(4181560352n);
    input.add32(2552358620n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2552358620n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 2 (2552358616, 2552358620)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add32(2552358616n);
    input.add32(2552358620n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2552358616n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 3 (2552358620, 2552358620)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add32(2552358620n);
    input.add32(2552358620n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2552358620n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 4 (2552358620, 2552358616)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add32(2552358620n);
    input.add32(2552358616n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2552358616n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 1 (8, 18442166427191666299)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add8(8n);
    input.add64(18442166427191666299n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 2 (4, 8)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add8(4n);
    input.add64(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 3 (8, 8)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add8(8n);
    input.add64(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 4 (8, 4)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add8(8n);
    input.add64(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint16, euint256) => ebool test 1 (33020, 115792089237316195423570985008687907853269984665640564039457579969595862485817)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add16(33020n);
    input.add256(115792089237316195423570985008687907853269984665640564039457579969595862485817n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint16, euint256) => ebool test 2 (33016, 33020)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add16(33016n);
    input.add256(33020n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint16, euint256) => ebool test 3 (33020, 33020)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add16(33020n);
    input.add256(33020n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint16, euint256) => ebool test 4 (33020, 33016)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add16(33020n);
    input.add256(33016n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 1 (197, 10)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add8(197n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.shl_euint8_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract4.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 20n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 2 (6, 10)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add8(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.shl_euint8_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract4.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 24n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 3 (10, 10)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add8(10n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.shl_euint8_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract4.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 40n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 4 (10, 6)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add8(10n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.shl_euint8_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract4.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 128n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint16, euint32) => euint32 test 1 (40500, 40500)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add16(40500n);
    input.add32(40500n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint16, euint32) => euint32 test 2 (40500, 40496)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add16(40500n);
    input.add32(40496n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
