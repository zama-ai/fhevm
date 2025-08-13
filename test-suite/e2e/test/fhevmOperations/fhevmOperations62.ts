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

describe('FHEVM operations 62', function () {
  before(async function () {
    this.signer = await getSigner(62);

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

  it('test operator "add" overload (euint16, euint64) => euint64 test 1 (2, 65521)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(2n);
    input.add64(65521n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 65523n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 2 (23183, 23185)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(23183n);
    input.add64(23185n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 46368n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 3 (23185, 23185)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(23185n);
    input.add64(23185n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 46370n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 4 (23185, 23183)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(23185n);
    input.add64(23183n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 46368n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 1 (18438544864566218419, 18445606066151884665)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add64(18438544864566218419n);
    input.add64(18445606066151884665n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438533869314639409n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 2 (18438544864566218415, 18438544864566218419)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add64(18438544864566218415n);
    input.add64(18438544864566218419n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438544864566218403n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 3 (18438544864566218419, 18438544864566218419)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add64(18438544864566218419n);
    input.add64(18438544864566218419n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438544864566218419n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 4 (18438544864566218419, 18438544864566218415)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add64(18438544864566218419n);
    input.add64(18438544864566218415n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438544864566218403n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint16, euint128) => euint128 test 1 (2, 32769)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(2n);
    input.add128(32769n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 32771n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint16, euint128) => euint128 test 2 (8128, 8132)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(8128n);
    input.add128(8132n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 16260n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint16, euint128) => euint128 test 3 (8132, 8132)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(8132n);
    input.add128(8132n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 16264n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint16, euint128) => euint128 test 4 (8132, 8128)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(8132n);
    input.add128(8128n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 16260n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (uint16, euint16) => euint16 test 1 (54814, 54814)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);

    input.add16(54814n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_uint16_euint16(54814n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (uint16, euint16) => euint16 test 2 (54814, 54810)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);

    input.add16(54810n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_uint16_euint16(54814n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 1 (37, 224)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add8(37n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint8_uint8(encryptedAmount.handles[0], 224n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 2 (33, 37)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add8(33n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint8_uint8(encryptedAmount.handles[0], 37n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 3 (37, 37)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add8(37n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint8_uint8(encryptedAmount.handles[0], 37n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 4 (37, 33)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add8(37n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint8_uint8(encryptedAmount.handles[0], 33n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "not" overload (euint8) => euint8 test 1 (184)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add8(184n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.not_euint8(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 71n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
