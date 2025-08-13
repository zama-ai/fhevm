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

describe('FHEVM operations 56', function () {
  before(async function () {
    this.signer = await getSigner(56);

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

  it('test operator "rotl" overload (euint32, uint8) => euint32 test 1 (780367346, 7)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add32(780367346n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.rotl_euint32_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract4.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1102772503n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint32, uint8) => euint32 test 2 (3, 7)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add32(3n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.rotl_euint32_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract4.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 384n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint32, uint8) => euint32 test 3 (7, 7)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add32(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.rotl_euint32_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract4.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 896n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint32, uint8) => euint32 test 4 (7, 3)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add32(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.rotl_euint32_uint8(encryptedAmount.handles[0], 3n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract4.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 56n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint128, uint128) => ebool test 1 (340282366920938463463369847520313727655, 340282366920938463463372503569758885355)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(340282366920938463463369847520313727655n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463372503569758885355n,
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

  it('test operator "eq" overload (euint128, uint128) => ebool test 2 (340282366920938463463369457203716781685, 340282366920938463463369457203716781689)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(340282366920938463463369457203716781685n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369457203716781689n,
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

  it('test operator "eq" overload (euint128, uint128) => ebool test 3 (340282366920938463463369457203716781689, 340282366920938463463369457203716781689)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(340282366920938463463369457203716781689n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369457203716781689n,
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

  it('test operator "eq" overload (euint128, uint128) => ebool test 4 (340282366920938463463369457203716781689, 340282366920938463463369457203716781685)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(340282366920938463463369457203716781689n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369457203716781685n,
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

  it('test operator "lt" overload (uint64, euint64) => ebool test 1 (18437947885450600111, 18444376458197882751)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);

    input.add64(18444376458197882751n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_uint64_euint64(
      18437947885450600111n,
      encryptedAmount.handles[0],
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

  it('test operator "lt" overload (uint64, euint64) => ebool test 2 (18441745152358063283, 18441745152358063287)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);

    input.add64(18441745152358063287n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_uint64_euint64(
      18441745152358063283n,
      encryptedAmount.handles[0],
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

  it('test operator "lt" overload (uint64, euint64) => ebool test 3 (18441745152358063287, 18441745152358063287)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);

    input.add64(18441745152358063287n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_uint64_euint64(
      18441745152358063287n,
      encryptedAmount.handles[0],
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

  it('test operator "lt" overload (uint64, euint64) => ebool test 4 (18441745152358063287, 18441745152358063283)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);

    input.add64(18441745152358063283n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_uint64_euint64(
      18441745152358063287n,
      encryptedAmount.handles[0],
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

  it('test operator "sub" overload (euint64, uint64) => euint64 test 1 (18438100183900742705, 18438100183900742705)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add64(18438100183900742705n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint64_uint64(
      encryptedAmount.handles[0],
      18438100183900742705n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint64, uint64) => euint64 test 2 (18438100183900742705, 18438100183900742701)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add64(18438100183900742705n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint64_uint64(
      encryptedAmount.handles[0],
      18438100183900742701n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 1 (18445501639359798905, 8)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add64(18445501639359798905n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.shl_euint64_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract4.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18128680880172857600n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 2 (4, 8)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add64(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.shl_euint64_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract4.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1024n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 3 (8, 8)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add64(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.shl_euint64_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract4.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2048n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 4 (8, 4)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add64(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.shl_euint64_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract4.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 128n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 1 (18442385177406502893, 18437816847096639195)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);

    input.add64(18437816847096639195n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_uint64_euint64(
      18442385177406502893n,
      encryptedAmount.handles[0],
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

  it('test operator "le" overload (uint64, euint64) => ebool test 2 (18443903610248017293, 18443903610248017297)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);

    input.add64(18443903610248017297n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_uint64_euint64(
      18443903610248017293n,
      encryptedAmount.handles[0],
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

  it('test operator "le" overload (uint64, euint64) => ebool test 3 (18443903610248017297, 18443903610248017297)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);

    input.add64(18443903610248017297n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_uint64_euint64(
      18443903610248017297n,
      encryptedAmount.handles[0],
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

  it('test operator "le" overload (uint64, euint64) => ebool test 4 (18443903610248017297, 18443903610248017293)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);

    input.add64(18443903610248017293n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_uint64_euint64(
      18443903610248017297n,
      encryptedAmount.handles[0],
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
});
