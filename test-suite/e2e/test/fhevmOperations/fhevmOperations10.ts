import { assert, expect } from 'chai';
import { ethers } from 'hardhat';

import type { FHEVMTestSuite1 } from '../../types/contracts/tests/FHEVMTestSuite1';
import type { FHEVMTestSuite2 } from '../../types/contracts/tests/FHEVMTestSuite2';
import type { FHEVMTestSuite3 } from '../../types/contracts/tests/FHEVMTestSuite3';
import type { FHEVMTestSuite4 } from '../../types/contracts/tests/FHEVMTestSuite4';
import type { FHEVMTestSuite5 } from '../../types/contracts/tests/FHEVMTestSuite5';
import type { FHEVMTestSuite6 } from '../../types/contracts/tests/FHEVMTestSuite6';
import type { FHEVMTestSuite7 } from '../../types/contracts/tests/FHEVMTestSuite7';
import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';

async function deployFHEVMTestFixture1(): Promise<FHEVMTestSuite1> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite1');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture2(): Promise<FHEVMTestSuite2> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite2');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture3(): Promise<FHEVMTestSuite3> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite3');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture4(): Promise<FHEVMTestSuite4> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite4');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture5(): Promise<FHEVMTestSuite5> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite5');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture6(): Promise<FHEVMTestSuite6> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite6');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture7(): Promise<FHEVMTestSuite7> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite7');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

describe('FHEVM operations 10', function () {
  before(async function () {
    await initSigners(1);
    this.signers = await getSigners();

    const contract1 = await deployFHEVMTestFixture1();
    this.contract1Address = await contract1.getAddress();
    this.contract1 = contract1;

    const contract2 = await deployFHEVMTestFixture2();
    this.contract2Address = await contract2.getAddress();
    this.contract2 = contract2;

    const contract3 = await deployFHEVMTestFixture3();
    this.contract3Address = await contract3.getAddress();
    this.contract3 = contract3;

    const contract4 = await deployFHEVMTestFixture4();
    this.contract4Address = await contract4.getAddress();
    this.contract4 = contract4;

    const contract5 = await deployFHEVMTestFixture5();
    this.contract5Address = await contract5.getAddress();
    this.contract5 = contract5;

    const contract6 = await deployFHEVMTestFixture6();
    this.contract6Address = await contract6.getAddress();
    this.contract6 = contract6;

    const contract7 = await deployFHEVMTestFixture7();
    this.contract7Address = await contract7.getAddress();
    this.contract7 = contract7;

    const instances = await createInstances(this.signers);
    this.instances = instances;
  });

  it('test operator "sub" overload (uint32, euint32) => euint32 test 1 (3210689907, 3210689907)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(3210689907n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_uint32_euint32(
      3210689907n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (uint32, euint32) => euint32 test 2 (3210689907, 3210689903)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(3210689903n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_uint32_euint32(
      3210689907n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 1 (26563, 54679)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(26563n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint32_uint32(encryptedAmount.handles[0], 54679n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1452438277n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 2 (53125, 53125)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(53125n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint32_uint32(encryptedAmount.handles[0], 53125n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2822265625n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 3 (53125, 53125)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(53125n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint32_uint32(encryptedAmount.handles[0], 53125n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2822265625n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 4 (53125, 53125)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(53125n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint32_uint32(encryptedAmount.handles[0], 53125n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2822265625n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 1 (63150, 54679)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(54679n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint32_euint32(63150n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3452978850n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 2 (53125, 53125)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(53125n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint32_euint32(53125n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2822265625n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 3 (53125, 53125)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(53125n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint32_euint32(53125n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2822265625n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 4 (53125, 53125)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(53125n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint32_euint32(53125n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2822265625n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 1 (3660910014, 2485848801)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3660910014n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint32_uint32(
      encryptedAmount.handles[0],
      2485848801n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 2 (1045328606, 1045328610)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1045328606n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint32_uint32(
      encryptedAmount.handles[0],
      1045328610n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 3 (1045328610, 1045328610)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1045328610n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint32_uint32(
      encryptedAmount.handles[0],
      1045328610n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 4 (1045328610, 1045328606)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1045328610n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint32_uint32(
      encryptedAmount.handles[0],
      1045328606n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 1 (2755601016, 993290999)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2755601016n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint32_uint32(
      encryptedAmount.handles[0],
      993290999n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 769019018n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 2 (2755601012, 2755601016)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2755601012n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint32_uint32(
      encryptedAmount.handles[0],
      2755601016n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2755601012n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 3 (2755601016, 2755601016)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2755601016n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint32_uint32(
      encryptedAmount.handles[0],
      2755601016n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 4 (2755601016, 2755601012)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2755601016n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint32_uint32(
      encryptedAmount.handles[0],
      2755601012n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint32, uint32) => euint32 test 1 (2290992677, 1504385400)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2290992677n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint32_uint32(
      encryptedAmount.handles[0],
      1504385400n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 143196192n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint32, uint32) => euint32 test 2 (1829524402, 1829524406)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1829524402n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint32_uint32(
      encryptedAmount.handles[0],
      1829524406n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1829524402n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint32, uint32) => euint32 test 3 (1829524406, 1829524406)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1829524406n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint32_uint32(
      encryptedAmount.handles[0],
      1829524406n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1829524406n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint32, uint32) => euint32 test 4 (1829524406, 1829524402)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1829524406n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint32_uint32(
      encryptedAmount.handles[0],
      1829524402n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1829524402n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint32, euint32) => euint32 test 1 (2909315891, 1504385400)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1504385400n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint32_euint32(
      2909315891n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 153616688n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint32, euint32) => euint32 test 2 (1829524402, 1829524406)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1829524406n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint32_euint32(
      1829524402n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1829524402n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint32, euint32) => euint32 test 3 (1829524406, 1829524406)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1829524406n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint32_euint32(
      1829524406n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1829524406n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint32, euint32) => euint32 test 4 (1829524406, 1829524402)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1829524402n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint32_euint32(
      1829524406n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1829524402n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, uint32) => euint32 test 1 (2298302465, 3564167147)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2298302465n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint32_uint32(
      encryptedAmount.handles[0],
      3564167147n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3707625451n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, uint32) => euint32 test 2 (2298302461, 2298302465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2298302461n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint32_uint32(
      encryptedAmount.handles[0],
      2298302465n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2298306557n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, uint32) => euint32 test 3 (2298302465, 2298302465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2298302465n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint32_uint32(
      encryptedAmount.handles[0],
      2298302465n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2298302465n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, uint32) => euint32 test 4 (2298302465, 2298302461)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2298302465n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint32_uint32(
      encryptedAmount.handles[0],
      2298302461n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2298306557n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint32, euint32) => euint32 test 1 (3069921687, 3564167147)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(3564167147n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint32_euint32(
      3069921687n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4143702015n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint32, euint32) => euint32 test 2 (2298302461, 2298302465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(2298302465n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint32_euint32(
      2298302461n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2298306557n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint32, euint32) => euint32 test 3 (2298302465, 2298302465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(2298302465n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint32_euint32(
      2298302465n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2298302465n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint32, euint32) => euint32 test 4 (2298302465, 2298302461)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(2298302461n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint32_euint32(
      2298302465n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2298306557n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, uint32) => euint32 test 1 (109044896, 1656011680)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(109044896n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint32_uint32(
      encryptedAmount.handles[0],
      1656011680n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1691049728n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, uint32) => euint32 test 2 (109044892, 109044896)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(109044892n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint32_uint32(
      encryptedAmount.handles[0],
      109044896n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 60n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, uint32) => euint32 test 3 (109044896, 109044896)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(109044896n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint32_uint32(
      encryptedAmount.handles[0],
      109044896n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, uint32) => euint32 test 4 (109044896, 109044892)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(109044896n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint32_uint32(
      encryptedAmount.handles[0],
      109044892n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 60n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (uint32, euint32) => euint32 test 1 (4246106217, 1656011680)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1656011680n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint32_euint32(
      4246106217n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2678246345n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (uint32, euint32) => euint32 test 2 (109044892, 109044896)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(109044896n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint32_euint32(
      109044892n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 60n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (uint32, euint32) => euint32 test 3 (109044896, 109044896)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(109044896n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint32_euint32(
      109044896n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (uint32, euint32) => euint32 test 4 (109044896, 109044892)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(109044892n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint32_euint32(
      109044896n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 60n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 1 (381835962, 3125155325)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(381835962n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint32_uint32(
      encryptedAmount.handles[0],
      3125155325n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 2 (381835958, 381835962)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(381835958n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint32_uint32(
      encryptedAmount.handles[0],
      381835962n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 3 (381835962, 381835962)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(381835962n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint32_uint32(
      encryptedAmount.handles[0],
      381835962n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 4 (381835962, 381835958)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(381835962n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint32_uint32(
      encryptedAmount.handles[0],
      381835958n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 1 (952161186, 3125155325)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(3125155325n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint32_euint32(
      952161186n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 2 (381835958, 381835962)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(381835962n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint32_euint32(
      381835958n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 3 (381835962, 381835962)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(381835962n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint32_euint32(
      381835962n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 4 (381835962, 381835958)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(381835958n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint32_euint32(
      381835962n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 1 (118700908, 159033857)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(118700908n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint32_uint32(
      encryptedAmount.handles[0],
      159033857n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 2 (118700904, 118700908)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(118700904n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint32_uint32(
      encryptedAmount.handles[0],
      118700908n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 3 (118700908, 118700908)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(118700908n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint32_uint32(
      encryptedAmount.handles[0],
      118700908n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 4 (118700908, 118700904)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(118700908n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint32_uint32(
      encryptedAmount.handles[0],
      118700904n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 1 (3407007129, 159033857)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(159033857n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint32_euint32(
      3407007129n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 2 (118700904, 118700908)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(118700908n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint32_euint32(
      118700904n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 3 (118700908, 118700908)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(118700908n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint32_euint32(
      118700908n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 4 (118700908, 118700904)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(118700904n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint32_euint32(
      118700908n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 1 (2889403208, 1420023929)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2889403208n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint32_uint32(
      encryptedAmount.handles[0],
      1420023929n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 2 (2889403204, 2889403208)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2889403204n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint32_uint32(
      encryptedAmount.handles[0],
      2889403208n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 3 (2889403208, 2889403208)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2889403208n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint32_uint32(
      encryptedAmount.handles[0],
      2889403208n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 4 (2889403208, 2889403204)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2889403208n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint32_uint32(
      encryptedAmount.handles[0],
      2889403204n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 1 (829032260, 1420023929)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1420023929n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint32_euint32(
      829032260n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 2 (2889403204, 2889403208)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(2889403208n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint32_euint32(
      2889403204n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 3 (2889403208, 2889403208)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(2889403208n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint32_euint32(
      2889403208n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 4 (2889403208, 2889403204)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(2889403204n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint32_euint32(
      2889403208n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 1 (3298838642, 310079030)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3298838642n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint32_uint32(
      encryptedAmount.handles[0],
      310079030n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 2 (3298838638, 3298838642)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3298838638n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint32_uint32(
      encryptedAmount.handles[0],
      3298838642n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 3 (3298838642, 3298838642)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3298838642n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint32_uint32(
      encryptedAmount.handles[0],
      3298838642n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 4 (3298838642, 3298838638)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3298838642n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint32_uint32(
      encryptedAmount.handles[0],
      3298838638n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 1 (880980950, 310079030)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(310079030n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint32_euint32(
      880980950n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 2 (3298838638, 3298838642)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(3298838642n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint32_euint32(
      3298838638n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 3 (3298838642, 3298838642)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(3298838642n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint32_euint32(
      3298838642n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 4 (3298838642, 3298838638)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(3298838638n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint32_euint32(
      3298838642n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 1 (2857230603, 2278654557)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2857230603n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint32_uint32(
      encryptedAmount.handles[0],
      2278654557n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 2 (2130145002, 2130145006)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2130145002n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint32_uint32(
      encryptedAmount.handles[0],
      2130145006n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 3 (2130145006, 2130145006)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2130145006n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint32_uint32(
      encryptedAmount.handles[0],
      2130145006n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 4 (2130145006, 2130145002)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2130145006n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint32_uint32(
      encryptedAmount.handles[0],
      2130145002n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 1 (405780353, 2278654557)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(2278654557n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint32_euint32(
      405780353n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 2 (2130145002, 2130145006)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(2130145006n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint32_euint32(
      2130145002n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 3 (2130145006, 2130145006)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(2130145006n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint32_euint32(
      2130145006n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 4 (2130145006, 2130145002)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(2130145002n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint32_euint32(
      2130145006n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 1 (3782775649, 3939976146)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3782775649n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint32_uint32(
      encryptedAmount.handles[0],
      3939976146n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 2 (3782775645, 3782775649)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3782775645n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint32_uint32(
      encryptedAmount.handles[0],
      3782775649n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 3 (3782775649, 3782775649)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3782775649n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint32_uint32(
      encryptedAmount.handles[0],
      3782775649n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 4 (3782775649, 3782775645)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3782775649n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint32_uint32(
      encryptedAmount.handles[0],
      3782775645n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 1 (3374607234, 3939976146)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(3939976146n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint32_euint32(
      3374607234n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 2 (3782775645, 3782775649)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(3782775649n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint32_euint32(
      3782775645n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 3 (3782775649, 3782775649)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(3782775649n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint32_euint32(
      3782775649n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 4 (3782775649, 3782775645)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(3782775645n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint32_euint32(
      3782775649n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 1 (2872867328, 2784393756)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2872867328n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint32_uint32(
      encryptedAmount.handles[0],
      2784393756n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2784393756n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 2 (51666484, 51666488)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(51666484n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint32_uint32(
      encryptedAmount.handles[0],
      51666488n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 51666484n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 3 (51666488, 51666488)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(51666488n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint32_uint32(
      encryptedAmount.handles[0],
      51666488n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 51666488n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 4 (51666488, 51666484)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(51666488n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint32_uint32(
      encryptedAmount.handles[0],
      51666484n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 51666484n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 1 (685309725, 2784393756)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(2784393756n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint32_euint32(
      685309725n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 685309725n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 2 (51666484, 51666488)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(51666488n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint32_euint32(
      51666484n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 51666484n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 3 (51666488, 51666488)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(51666488n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint32_euint32(
      51666488n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 51666488n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 4 (51666488, 51666484)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(51666484n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint32_euint32(
      51666488n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 51666484n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 1 (1782568505, 3077325518)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1782568505n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint32_uint32(
      encryptedAmount.handles[0],
      3077325518n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3077325518n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 2 (1782568501, 1782568505)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1782568501n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint32_uint32(
      encryptedAmount.handles[0],
      1782568505n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1782568505n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 3 (1782568505, 1782568505)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1782568505n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint32_uint32(
      encryptedAmount.handles[0],
      1782568505n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1782568505n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 4 (1782568505, 1782568501)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1782568505n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint32_uint32(
      encryptedAmount.handles[0],
      1782568501n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1782568505n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 1 (461790767, 3077325518)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(3077325518n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint32_euint32(
      461790767n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3077325518n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 2 (1782568501, 1782568505)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1782568505n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint32_euint32(
      1782568501n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1782568505n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 3 (1782568505, 1782568505)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1782568505n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint32_euint32(
      1782568505n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1782568505n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 4 (1782568505, 1782568501)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1782568501n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint32_euint32(
      1782568505n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1782568505n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 1 (9221609366113171721, 9221792306389218161)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(9221609366113171721n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint64_uint64(
      encryptedAmount.handles[0],
      9221792306389218161n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18443401672502389882n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 2 (9221330086412596226, 9221330086412596228)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(9221330086412596226n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint64_uint64(
      encryptedAmount.handles[0],
      9221330086412596228n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18442660172825192454n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 3 (9221330086412596228, 9221330086412596228)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(9221330086412596228n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint64_uint64(
      encryptedAmount.handles[0],
      9221330086412596228n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18442660172825192456n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 4 (9221330086412596228, 9221330086412596226)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(9221330086412596228n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint64_uint64(
      encryptedAmount.handles[0],
      9221330086412596226n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18442660172825192454n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 1 (9219988861089596110, 9221792306389218161)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(9221792306389218161n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint64_euint64(
      9219988861089596110n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18441781167478814271n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 2 (9221330086412596226, 9221330086412596228)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(9221330086412596228n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint64_euint64(
      9221330086412596226n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18442660172825192454n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 3 (9221330086412596228, 9221330086412596228)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(9221330086412596228n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint64_euint64(
      9221330086412596228n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18442660172825192456n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 4 (9221330086412596228, 9221330086412596226)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(9221330086412596226n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint64_euint64(
      9221330086412596228n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18442660172825192454n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint64, uint64) => euint64 test 1 (18439349086161565635, 18439349086161565635)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439349086161565635n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint64_uint64(
      encryptedAmount.handles[0],
      18439349086161565635n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint64, uint64) => euint64 test 2 (18439349086161565635, 18439349086161565631)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439349086161565635n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint64_uint64(
      encryptedAmount.handles[0],
      18439349086161565631n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (uint64, euint64) => euint64 test 1 (18439349086161565635, 18439349086161565635)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18439349086161565635n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_uint64_euint64(
      18439349086161565635n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (uint64, euint64) => euint64 test 2 (18439349086161565635, 18439349086161565631)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18439349086161565631n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_uint64_euint64(
      18439349086161565635n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 1 (4294477868, 4294644813)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(4294477868n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint64_uint64(
      encryptedAmount.handles[0],
      4294644813n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18443257100349498684n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 2 (4294477868, 4294477868)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(4294477868n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint64_uint64(
      encryptedAmount.handles[0],
      4294477868n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18442540158741825424n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 3 (4294477868, 4294477868)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(4294477868n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint64_uint64(
      encryptedAmount.handles[0],
      4294477868n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18442540158741825424n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 4 (4294477868, 4294477868)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(4294477868n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint64_uint64(
      encryptedAmount.handles[0],
      4294477868n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18442540158741825424n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 1 (4294268443, 4294644813)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(4294644813n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint64_euint64(
      4294268443n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18442357694359536159n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 2 (4294477868, 4294477868)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(4294477868n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint64_euint64(
      4294477868n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18442540158741825424n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 3 (4294477868, 4294477868)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(4294477868n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint64_euint64(
      4294477868n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18442540158741825424n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 4 (4294477868, 4294477868)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(4294477868n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint64_euint64(
      4294477868n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18442540158741825424n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 1 (18439435130812126625, 18444522074301627507)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439435130812126625n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint64_uint64(
      encryptedAmount.handles[0],
      18444522074301627507n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 2 (18439435130812126621, 18439435130812126625)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439435130812126621n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint64_uint64(
      encryptedAmount.handles[0],
      18439435130812126625n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 3 (18439435130812126625, 18439435130812126625)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439435130812126625n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint64_uint64(
      encryptedAmount.handles[0],
      18439435130812126625n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 4 (18439435130812126625, 18439435130812126621)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439435130812126625n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint64_uint64(
      encryptedAmount.handles[0],
      18439435130812126621n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 1 (18438476467970203075, 18440440645325194943)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438476467970203075n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint64_uint64(
      encryptedAmount.handles[0],
      18440440645325194943n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438476467970203075n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 2 (18438476467970203071, 18438476467970203075)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438476467970203071n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint64_uint64(
      encryptedAmount.handles[0],
      18438476467970203075n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438476467970203071n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 3 (18438476467970203075, 18438476467970203075)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438476467970203075n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint64_uint64(
      encryptedAmount.handles[0],
      18438476467970203075n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 4 (18438476467970203075, 18438476467970203071)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438476467970203075n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint64_uint64(
      encryptedAmount.handles[0],
      18438476467970203071n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint64, uint64) => euint64 test 1 (18445522164637762523, 18439461648163905749)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18445522164637762523n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint64_uint64(
      encryptedAmount.handles[0],
      18439461648163905749n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438335747717333201n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint64, uint64) => euint64 test 2 (18441000062831814437, 18441000062831814441)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441000062831814437n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint64_uint64(
      encryptedAmount.handles[0],
      18441000062831814441n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18441000062831814433n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint64, uint64) => euint64 test 3 (18441000062831814441, 18441000062831814441)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441000062831814441n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint64_uint64(
      encryptedAmount.handles[0],
      18441000062831814441n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18441000062831814441n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint64, uint64) => euint64 test 4 (18441000062831814441, 18441000062831814437)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441000062831814441n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint64_uint64(
      encryptedAmount.handles[0],
      18441000062831814437n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18441000062831814433n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint64, euint64) => euint64 test 1 (18443909887884936197, 18439461648163905749)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18439461648163905749n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint64_euint64(
      18443909887884936197n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438897959273889797n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint64, euint64) => euint64 test 2 (18441000062831814437, 18441000062831814441)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18441000062831814441n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint64_euint64(
      18441000062831814437n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18441000062831814433n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint64, euint64) => euint64 test 3 (18441000062831814441, 18441000062831814441)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18441000062831814441n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint64_euint64(
      18441000062831814441n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18441000062831814441n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint64, euint64) => euint64 test 4 (18441000062831814441, 18441000062831814437)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18441000062831814437n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint64_euint64(
      18441000062831814441n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18441000062831814433n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint64, uint64) => euint64 test 1 (18438749575894672661, 18441607271932800015)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438749575894672661n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint64_uint64(
      encryptedAmount.handles[0],
      18441607271932800015n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18442197713116638495n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint64, uint64) => euint64 test 2 (18438749575894672657, 18438749575894672661)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438749575894672657n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint64_uint64(
      encryptedAmount.handles[0],
      18438749575894672661n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438749575894672661n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint64, uint64) => euint64 test 3 (18438749575894672661, 18438749575894672661)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438749575894672661n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint64_uint64(
      encryptedAmount.handles[0],
      18438749575894672661n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438749575894672661n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint64, uint64) => euint64 test 4 (18438749575894672661, 18438749575894672657)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438749575894672661n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint64_uint64(
      encryptedAmount.handles[0],
      18438749575894672657n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438749575894672661n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint64, euint64) => euint64 test 1 (18438353648426897991, 18441607271932800015)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18441607271932800015n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint64_euint64(
      18438353648426897991n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18442223963755249231n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint64, euint64) => euint64 test 2 (18438749575894672657, 18438749575894672661)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18438749575894672661n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint64_euint64(
      18438749575894672657n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438749575894672661n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint64, euint64) => euint64 test 3 (18438749575894672661, 18438749575894672661)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18438749575894672661n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint64_euint64(
      18438749575894672661n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438749575894672661n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint64, euint64) => euint64 test 4 (18438749575894672661, 18438749575894672657)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18438749575894672657n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint64_euint64(
      18438749575894672661n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438749575894672661n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint64, uint64) => euint64 test 1 (18441324534209635901, 18438302185395209527)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441324534209635901n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint64_uint64(
      encryptedAmount.handles[0],
      18438302185395209527n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4148253705151242n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint64, uint64) => euint64 test 2 (18441324534209635897, 18441324534209635901)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441324534209635897n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint64_uint64(
      encryptedAmount.handles[0],
      18441324534209635901n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint64, uint64) => euint64 test 3 (18441324534209635901, 18441324534209635901)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441324534209635901n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint64_uint64(
      encryptedAmount.handles[0],
      18441324534209635901n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint64, uint64) => euint64 test 4 (18441324534209635901, 18441324534209635897)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441324534209635901n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint64_uint64(
      encryptedAmount.handles[0],
      18441324534209635897n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (uint64, euint64) => euint64 test 1 (18445244388602787701, 18438302185395209527)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18438302185395209527n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint64_euint64(
      18445244388602787701n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 6946915994694210n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (uint64, euint64) => euint64 test 2 (18441324534209635897, 18441324534209635901)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18441324534209635901n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint64_euint64(
      18441324534209635897n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (uint64, euint64) => euint64 test 3 (18441324534209635901, 18441324534209635901)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18441324534209635901n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint64_euint64(
      18441324534209635901n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (uint64, euint64) => euint64 test 4 (18441324534209635901, 18441324534209635897)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18441324534209635897n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint64_euint64(
      18441324534209635901n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 1 (18446487134777485899, 18440610975949482077)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18446487134777485899n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint64_uint64(
      encryptedAmount.handles[0],
      18440610975949482077n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 2 (18439551149204239205, 18439551149204239209)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439551149204239205n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint64_uint64(
      encryptedAmount.handles[0],
      18439551149204239209n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 3 (18439551149204239209, 18439551149204239209)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439551149204239209n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint64_uint64(
      encryptedAmount.handles[0],
      18439551149204239209n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 4 (18439551149204239209, 18439551149204239205)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439551149204239209n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint64_uint64(
      encryptedAmount.handles[0],
      18439551149204239205n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 1 (18444469238767404399, 18440610975949482077)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18440610975949482077n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint64_euint64(
      18444469238767404399n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 2 (18439551149204239205, 18439551149204239209)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18439551149204239209n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint64_euint64(
      18439551149204239205n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 3 (18439551149204239209, 18439551149204239209)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18439551149204239209n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint64_euint64(
      18439551149204239209n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 4 (18439551149204239209, 18439551149204239205)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18439551149204239205n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint64_euint64(
      18439551149204239209n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 1 (18439384522078171951, 18444868956589747821)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439384522078171951n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint64_uint64(
      encryptedAmount.handles[0],
      18444868956589747821n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 2 (18439384522078171947, 18439384522078171951)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439384522078171947n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint64_uint64(
      encryptedAmount.handles[0],
      18439384522078171951n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 3 (18439384522078171951, 18439384522078171951)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439384522078171951n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint64_uint64(
      encryptedAmount.handles[0],
      18439384522078171951n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 4 (18439384522078171951, 18439384522078171947)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439384522078171951n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint64_uint64(
      encryptedAmount.handles[0],
      18439384522078171947n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 1 (18441819322752799093, 18444868956589747821)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18444868956589747821n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint64_euint64(
      18441819322752799093n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 2 (18439384522078171947, 18439384522078171951)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18439384522078171951n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint64_euint64(
      18439384522078171947n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 3 (18439384522078171951, 18439384522078171951)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18439384522078171951n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint64_euint64(
      18439384522078171951n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 4 (18439384522078171951, 18439384522078171947)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18439384522078171947n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint64_euint64(
      18439384522078171951n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 1 (18442822924492984821, 18440395931538178735)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18442822924492984821n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint64_uint64(
      encryptedAmount.handles[0],
      18440395931538178735n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 2 (18439726932066773155, 18439726932066773159)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439726932066773155n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint64_uint64(
      encryptedAmount.handles[0],
      18439726932066773159n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 3 (18439726932066773159, 18439726932066773159)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439726932066773159n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint64_uint64(
      encryptedAmount.handles[0],
      18439726932066773159n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 4 (18439726932066773159, 18439726932066773155)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439726932066773159n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint64_uint64(
      encryptedAmount.handles[0],
      18439726932066773155n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 1 (18444419342997944247, 18440395931538178735)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18440395931538178735n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint64_euint64(
      18444419342997944247n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 2 (18439726932066773155, 18439726932066773159)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18439726932066773159n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint64_euint64(
      18439726932066773155n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 3 (18439726932066773159, 18439726932066773159)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18439726932066773159n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint64_euint64(
      18439726932066773159n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 4 (18439726932066773159, 18439726932066773155)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18439726932066773155n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint64_euint64(
      18439726932066773159n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 1 (18445740605270208159, 18445656963489448189)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18445740605270208159n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint64_uint64(
      encryptedAmount.handles[0],
      18445656963489448189n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 2 (18444197585734519025, 18444197585734519029)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18444197585734519025n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint64_uint64(
      encryptedAmount.handles[0],
      18444197585734519029n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 3 (18444197585734519029, 18444197585734519029)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18444197585734519029n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint64_uint64(
      encryptedAmount.handles[0],
      18444197585734519029n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 4 (18444197585734519029, 18444197585734519025)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18444197585734519029n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint64_uint64(
      encryptedAmount.handles[0],
      18444197585734519025n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 1 (18443391663789273939, 18445656963489448189)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18445656963489448189n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint64_euint64(
      18443391663789273939n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 2 (18444197585734519025, 18444197585734519029)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18444197585734519029n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint64_euint64(
      18444197585734519025n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 3 (18444197585734519029, 18444197585734519029)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18444197585734519029n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint64_euint64(
      18444197585734519029n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 4 (18444197585734519029, 18444197585734519025)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18444197585734519025n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint64_euint64(
      18444197585734519029n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 1 (18443144512468494533, 18443668639996943369)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18443144512468494533n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint64_uint64(
      encryptedAmount.handles[0],
      18443668639996943369n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 2 (18438624287992505971, 18438624287992505975)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438624287992505971n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint64_uint64(
      encryptedAmount.handles[0],
      18438624287992505975n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 3 (18438624287992505975, 18438624287992505975)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438624287992505975n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint64_uint64(
      encryptedAmount.handles[0],
      18438624287992505975n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 4 (18438624287992505975, 18438624287992505971)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438624287992505975n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint64_uint64(
      encryptedAmount.handles[0],
      18438624287992505971n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 1 (18445278640967881017, 18443668639996943369)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18443668639996943369n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint64_euint64(
      18445278640967881017n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 2 (18438624287992505971, 18438624287992505975)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18438624287992505975n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint64_euint64(
      18438624287992505971n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 3 (18438624287992505975, 18438624287992505975)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18438624287992505975n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint64_euint64(
      18438624287992505975n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 4 (18438624287992505975, 18438624287992505971)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18438624287992505971n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint64_euint64(
      18438624287992505975n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 1 (18440026078109391261, 18444384116530145523)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18440026078109391261n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint64_uint64(
      encryptedAmount.handles[0],
      18444384116530145523n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 2 (18440026078109391257, 18440026078109391261)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18440026078109391257n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint64_uint64(
      encryptedAmount.handles[0],
      18440026078109391261n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 3 (18440026078109391261, 18440026078109391261)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18440026078109391261n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint64_uint64(
      encryptedAmount.handles[0],
      18440026078109391261n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 4 (18440026078109391261, 18440026078109391257)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18440026078109391261n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint64_uint64(
      encryptedAmount.handles[0],
      18440026078109391257n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });
});
