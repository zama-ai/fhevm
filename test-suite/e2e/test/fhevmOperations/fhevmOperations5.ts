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

describe('FHEVM operations 5', function () {
  before(async function () {
    this.signer = await getSigner(5);

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

  it('test operator "min" overload (euint8, uint8) => euint8 test 1 (82, 210)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(82n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_uint8(encryptedAmount.handles[0], 210n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 82n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 2 (78, 82)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(78n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_uint8(encryptedAmount.handles[0], 82n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 78n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 3 (82, 82)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(82n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_uint8(encryptedAmount.handles[0], 82n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 82n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 4 (82, 78)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(82n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_uint8(encryptedAmount.handles[0], 78n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 78n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 1 (1450084384, 51351)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(1450084384n);
    input.add16(51351n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint32_euint16(
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 2 (51347, 51351)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(51347n);
    input.add16(51351n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint32_euint16(
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 3 (51351, 51351)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(51351n);
    input.add16(51351n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint32_euint16(
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 4 (51351, 51347)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(51351n);
    input.add16(51347n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint32_euint16(
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 1 (58105, 2380872833)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(58105n);
    input.add32(2380872833n);
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 2 (58101, 58105)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(58101n);
    input.add32(58105n);
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 3 (58105, 58105)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(58105n);
    input.add32(58105n);
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 4 (58105, 58101)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(58105n);
    input.add32(58101n);
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 1 (1691714054, 2857364901)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(1691714054n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.div_euint32_uint32(
      encryptedAmount.handles[0],
      2857364901n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 2 (486270740, 486270744)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(486270740n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.div_euint32_uint32(
      encryptedAmount.handles[0],
      486270744n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 3 (486270744, 486270744)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(486270744n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.div_euint32_uint32(
      encryptedAmount.handles[0],
      486270744n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 4 (486270744, 486270740)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(486270744n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.div_euint32_uint32(
      encryptedAmount.handles[0],
      486270740n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 1 (53, 18445287305732048029)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(53n);
    input.add64(18445287305732048029n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 21n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 2 (49, 53)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(49n);
    input.add64(53n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 49n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 3 (53, 53)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(53n);
    input.add64(53n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 53n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 4 (53, 49)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(53n);
    input.add64(49n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 49n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "ge" overload (euint128, euint64) => ebool test 1 (340282366920938463463366987195931318507, 18439531046471582951)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add128(340282366920938463463366987195931318507n);
    input.add64(18439531046471582951n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint128_euint64(
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "ge" overload (euint128, euint64) => ebool test 2 (18439531046471582947, 18439531046471582951)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add128(18439531046471582947n);
    input.add64(18439531046471582951n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint128_euint64(
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "ge" overload (euint128, euint64) => ebool test 3 (18439531046471582951, 18439531046471582951)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add128(18439531046471582951n);
    input.add64(18439531046471582951n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint128_euint64(
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "ge" overload (euint128, euint64) => ebool test 4 (18439531046471582951, 18439531046471582947)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add128(18439531046471582951n);
    input.add64(18439531046471582947n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint128_euint64(
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
    assert.deepEqual(res.clearValues, expectedRes);
  });
});
