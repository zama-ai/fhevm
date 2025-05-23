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

describe('FHEVM operations 11', function () {
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

  it('test operator "lt" overload (uint64, euint64) => ebool test 1 (18437800176746093485, 18444384116530145523)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18444384116530145523n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint64_euint64(
      18437800176746093485n,
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

  it('test operator "lt" overload (uint64, euint64) => ebool test 2 (18440026078109391257, 18440026078109391261)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18440026078109391261n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint64_euint64(
      18440026078109391257n,
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

  it('test operator "lt" overload (uint64, euint64) => ebool test 3 (18440026078109391261, 18440026078109391261)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18440026078109391261n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint64_euint64(
      18440026078109391261n,
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

  it('test operator "lt" overload (uint64, euint64) => ebool test 4 (18440026078109391261, 18440026078109391257)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18440026078109391257n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint64_euint64(
      18440026078109391261n,
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

  it('test operator "min" overload (euint64, uint64) => euint64 test 1 (18442053845935641397, 18439044702482932767)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18442053845935641397n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint64_uint64(
      encryptedAmount.handles[0],
      18439044702482932767n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18439044702482932767n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 2 (18442053845935641393, 18442053845935641397)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18442053845935641393n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint64_uint64(
      encryptedAmount.handles[0],
      18442053845935641397n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18442053845935641393n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 3 (18442053845935641397, 18442053845935641397)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18442053845935641397n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint64_uint64(
      encryptedAmount.handles[0],
      18442053845935641397n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18442053845935641397n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 4 (18442053845935641397, 18442053845935641393)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18442053845935641397n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint64_uint64(
      encryptedAmount.handles[0],
      18442053845935641393n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18442053845935641393n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 1 (18442390566256160137, 18439044702482932767)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18439044702482932767n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint64_euint64(
      18442390566256160137n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18439044702482932767n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 2 (18442053845935641393, 18442053845935641397)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18442053845935641397n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint64_euint64(
      18442053845935641393n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18442053845935641393n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 3 (18442053845935641397, 18442053845935641397)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18442053845935641397n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint64_euint64(
      18442053845935641397n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18442053845935641397n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 4 (18442053845935641397, 18442053845935641393)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18442053845935641393n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint64_euint64(
      18442053845935641397n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18442053845935641393n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 1 (18440771157638493757, 18441026510516421163)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18440771157638493757n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint64_uint64(
      encryptedAmount.handles[0],
      18441026510516421163n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18441026510516421163n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 2 (18440771157638493753, 18440771157638493757)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18440771157638493753n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint64_uint64(
      encryptedAmount.handles[0],
      18440771157638493757n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18440771157638493757n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 3 (18440771157638493757, 18440771157638493757)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18440771157638493757n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint64_uint64(
      encryptedAmount.handles[0],
      18440771157638493757n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18440771157638493757n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 4 (18440771157638493757, 18440771157638493753)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18440771157638493757n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint64_uint64(
      encryptedAmount.handles[0],
      18440771157638493753n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18440771157638493757n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 1 (18444854611093799187, 18441026510516421163)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18441026510516421163n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint64_euint64(
      18444854611093799187n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18444854611093799187n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 2 (18440771157638493753, 18440771157638493757)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18440771157638493757n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint64_euint64(
      18440771157638493753n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18440771157638493757n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 3 (18440771157638493757, 18440771157638493757)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18440771157638493757n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint64_euint64(
      18440771157638493757n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18440771157638493757n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 4 (18440771157638493757, 18440771157638493753)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18440771157638493753n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint64_euint64(
      18440771157638493757n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18440771157638493757n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint128, uint128) => euint128 test 1 (170141183460469231731684290473764587013, 170141183460469231731686340898121964533)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(170141183460469231731684290473764587013n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_uint128(
      encryptedAmount.handles[0],
      170141183460469231731686340898121964533n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463370631371886551546n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint128, uint128) => euint128 test 2 (170141183460469231731684290473764587011, 170141183460469231731684290473764587013)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(170141183460469231731684290473764587011n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_uint128(
      encryptedAmount.handles[0],
      170141183460469231731684290473764587013n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463368580947529174024n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint128, uint128) => euint128 test 3 (170141183460469231731684290473764587013, 170141183460469231731684290473764587013)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(170141183460469231731684290473764587013n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_uint128(
      encryptedAmount.handles[0],
      170141183460469231731684290473764587013n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463368580947529174026n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint128, uint128) => euint128 test 4 (170141183460469231731684290473764587013, 170141183460469231731684290473764587011)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(170141183460469231731684290473764587013n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_uint128(
      encryptedAmount.handles[0],
      170141183460469231731684290473764587011n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463368580947529174024n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (uint128, euint128) => euint128 test 1 (170141183460469231731683186727962549760, 170141183460469231731686340898121964533)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(170141183460469231731686340898121964533n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint128_euint128(
      170141183460469231731683186727962549760n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463369527626084514293n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (uint128, euint128) => euint128 test 2 (170141183460469231731684290473764587011, 170141183460469231731684290473764587013)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(170141183460469231731684290473764587013n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint128_euint128(
      170141183460469231731684290473764587011n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463368580947529174024n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (uint128, euint128) => euint128 test 3 (170141183460469231731684290473764587013, 170141183460469231731684290473764587013)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(170141183460469231731684290473764587013n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint128_euint128(
      170141183460469231731684290473764587013n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463368580947529174026n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (uint128, euint128) => euint128 test 4 (170141183460469231731684290473764587013, 170141183460469231731684290473764587011)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(170141183460469231731684290473764587011n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint128_euint128(
      170141183460469231731684290473764587013n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463368580947529174024n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint128, uint128) => euint128 test 1 (340282366920938463463366318931405103991, 340282366920938463463366318931405103991)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463366318931405103991n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366318931405103991n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint128, uint128) => euint128 test 2 (340282366920938463463366318931405103991, 340282366920938463463366318931405103987)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463366318931405103991n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366318931405103987n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (uint128, euint128) => euint128 test 1 (340282366920938463463366318931405103991, 340282366920938463463366318931405103991)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463366318931405103991n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_uint128_euint128(
      340282366920938463463366318931405103991n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (uint128, euint128) => euint128 test 2 (340282366920938463463366318931405103991, 340282366920938463463366318931405103987)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463366318931405103987n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_uint128_euint128(
      340282366920938463463366318931405103991n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint128, uint128) => euint128 test 1 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(9223372036854775809n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint128_uint128(
      encryptedAmount.handles[0],
      9223372036854775809n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 85070591730234615884290395931651604481n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint128, uint128) => euint128 test 2 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(9223372036854775809n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint128_uint128(
      encryptedAmount.handles[0],
      9223372036854775809n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 85070591730234615884290395931651604481n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint128, uint128) => euint128 test 3 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(9223372036854775809n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint128_uint128(
      encryptedAmount.handles[0],
      9223372036854775809n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 85070591730234615884290395931651604481n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint128, uint128) => euint128 test 4 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(9223372036854775809n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint128_uint128(
      encryptedAmount.handles[0],
      9223372036854775809n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 85070591730234615884290395931651604481n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (uint128, euint128) => euint128 test 1 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint128_euint128(
      9223372036854775809n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 85070591730234615884290395931651604481n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (uint128, euint128) => euint128 test 2 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint128_euint128(
      9223372036854775809n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 85070591730234615884290395931651604481n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (uint128, euint128) => euint128 test 3 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint128_euint128(
      9223372036854775809n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 85070591730234615884290395931651604481n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (uint128, euint128) => euint128 test 4 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint128_euint128(
      9223372036854775809n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 85070591730234615884290395931651604481n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "div" overload (euint128, uint128) => euint128 test 1 (340282366920938463463370356609457744025, 340282366920938463463372549484335395241)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463370356609457744025n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463372549484335395241n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "div" overload (euint128, uint128) => euint128 test 2 (340282366920938463463368823852789581029, 340282366920938463463368823852789581033)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368823852789581029n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368823852789581033n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "div" overload (euint128, uint128) => euint128 test 3 (340282366920938463463368823852789581033, 340282366920938463463368823852789581033)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368823852789581033n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368823852789581033n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "div" overload (euint128, uint128) => euint128 test 4 (340282366920938463463368823852789581033, 340282366920938463463368823852789581029)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368823852789581033n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368823852789581029n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rem" overload (euint128, uint128) => euint128 test 1 (340282366920938463463372668207170118131, 340282366920938463463367305362708892969)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463372668207170118131n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367305362708892969n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 5362844461225162n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rem" overload (euint128, uint128) => euint128 test 2 (340282366920938463463372600281520067225, 340282366920938463463372600281520067229)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463372600281520067225n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463372600281520067229n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463372600281520067225n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rem" overload (euint128, uint128) => euint128 test 3 (340282366920938463463372600281520067229, 340282366920938463463372600281520067229)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463372600281520067229n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463372600281520067229n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rem" overload (euint128, uint128) => euint128 test 4 (340282366920938463463372600281520067229, 340282366920938463463372600281520067225)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463372600281520067229n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463372600281520067225n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint128, uint128) => euint128 test 1 (340282366920938463463366311865381154455, 340282366920938463463372321128222100551)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463366311865381154455n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463372321128222100551n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463366304024918297607n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint128, uint128) => euint128 test 2 (340282366920938463463366311865381154451, 340282366920938463463366311865381154455)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463366311865381154451n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366311865381154455n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463366311865381154451n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint128, uint128) => euint128 test 3 (340282366920938463463366311865381154455, 340282366920938463463366311865381154455)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463366311865381154455n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366311865381154455n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463366311865381154455n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint128, uint128) => euint128 test 4 (340282366920938463463366311865381154455, 340282366920938463463366311865381154451)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463366311865381154455n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366311865381154451n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463366311865381154451n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint128, euint128) => euint128 test 1 (340282366920938463463367006757894823593, 340282366920938463463372321128222100551)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463372321128222100551n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint128_euint128(
      340282366920938463463367006757894823593n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463366972535516761089n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint128, euint128) => euint128 test 2 (340282366920938463463366311865381154451, 340282366920938463463366311865381154455)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463366311865381154455n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint128_euint128(
      340282366920938463463366311865381154451n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463366311865381154451n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint128, euint128) => euint128 test 3 (340282366920938463463366311865381154455, 340282366920938463463366311865381154455)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463366311865381154455n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint128_euint128(
      340282366920938463463366311865381154455n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463366311865381154455n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint128, euint128) => euint128 test 4 (340282366920938463463366311865381154455, 340282366920938463463366311865381154451)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463366311865381154451n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint128_euint128(
      340282366920938463463366311865381154455n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463366311865381154451n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint128, uint128) => euint128 test 1 (340282366920938463463366170718150515657, 340282366920938463463374564130149489471)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463366170718150515657n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463374564130149489471n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463374571004271516671n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint128, uint128) => euint128 test 2 (340282366920938463463366170718150515653, 340282366920938463463366170718150515657)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463366170718150515653n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366170718150515657n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463366170718150515661n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint128, uint128) => euint128 test 3 (340282366920938463463366170718150515657, 340282366920938463463366170718150515657)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463366170718150515657n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366170718150515657n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463366170718150515657n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint128, uint128) => euint128 test 4 (340282366920938463463366170718150515657, 340282366920938463463366170718150515653)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463366170718150515657n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366170718150515653n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463366170718150515661n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint128, euint128) => euint128 test 1 (340282366920938463463367926322828366281, 340282366920938463463374564130149489471)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463374564130149489471n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint128_euint128(
      340282366920938463463367926322828366281n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463374567433256819711n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint128, euint128) => euint128 test 2 (340282366920938463463366170718150515653, 340282366920938463463366170718150515657)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463366170718150515657n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint128_euint128(
      340282366920938463463366170718150515653n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463366170718150515661n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint128, euint128) => euint128 test 3 (340282366920938463463366170718150515657, 340282366920938463463366170718150515657)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463366170718150515657n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint128_euint128(
      340282366920938463463366170718150515657n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463366170718150515657n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint128, euint128) => euint128 test 4 (340282366920938463463366170718150515657, 340282366920938463463366170718150515653)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463366170718150515653n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint128_euint128(
      340282366920938463463366170718150515657n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463366170718150515661n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint128, uint128) => euint128 test 1 (340282366920938463463368650249676349963, 340282366920938463463370195653458313813)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368650249676349963n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463370195653458313813n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 7466214994452574n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint128, uint128) => euint128 test 2 (340282366920938463463368650249676349959, 340282366920938463463368650249676349963)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368650249676349959n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368650249676349963n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 12n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint128, uint128) => euint128 test 3 (340282366920938463463368650249676349963, 340282366920938463463368650249676349963)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368650249676349963n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368650249676349963n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint128, uint128) => euint128 test 4 (340282366920938463463368650249676349963, 340282366920938463463368650249676349959)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368650249676349963n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368650249676349959n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 12n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (uint128, euint128) => euint128 test 1 (340282366920938463463368123711914181783, 340282366920938463463370195653458313813)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463370195653458313813n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint128_euint128(
      340282366920938463463368123711914181783n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 6936361517303490n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (uint128, euint128) => euint128 test 2 (340282366920938463463368650249676349959, 340282366920938463463368650249676349963)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463368650249676349963n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint128_euint128(
      340282366920938463463368650249676349959n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 12n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (uint128, euint128) => euint128 test 3 (340282366920938463463368650249676349963, 340282366920938463463368650249676349963)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463368650249676349963n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint128_euint128(
      340282366920938463463368650249676349963n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (uint128, euint128) => euint128 test 4 (340282366920938463463368650249676349963, 340282366920938463463368650249676349959)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add128(340282366920938463463368650249676349959n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint128_euint128(
      340282366920938463463368650249676349963n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 12n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint128, uint128) => ebool test 1 (340282366920938463463371755576443048217, 340282366920938463463367333157757347221)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463371755576443048217n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367333157757347221n,
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

  it('test operator "eq" overload (euint128, uint128) => ebool test 2 (340282366920938463463371755576443048213, 340282366920938463463371755576443048217)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463371755576443048213n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463371755576443048217n,
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

  it('test operator "eq" overload (euint128, uint128) => ebool test 3 (340282366920938463463371755576443048217, 340282366920938463463371755576443048217)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463371755576443048217n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463371755576443048217n,
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

  it('test operator "eq" overload (euint128, uint128) => ebool test 4 (340282366920938463463371755576443048217, 340282366920938463463371755576443048213)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463371755576443048217n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463371755576443048213n,
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

  it('test operator "eq" overload (uint128, euint128) => ebool test 1 (340282366920938463463373876534784259739, 340282366920938463463367333157757347221)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463367333157757347221n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_uint128_euint128(
      340282366920938463463373876534784259739n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (uint128, euint128) => ebool test 2 (340282366920938463463371755576443048213, 340282366920938463463371755576443048217)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463371755576443048217n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_uint128_euint128(
      340282366920938463463371755576443048213n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (uint128, euint128) => ebool test 3 (340282366920938463463371755576443048217, 340282366920938463463371755576443048217)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463371755576443048217n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_uint128_euint128(
      340282366920938463463371755576443048217n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (uint128, euint128) => ebool test 4 (340282366920938463463371755576443048217, 340282366920938463463371755576443048213)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463371755576443048213n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_uint128_euint128(
      340282366920938463463371755576443048217n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint128, uint128) => ebool test 1 (340282366920938463463372998114203923211, 340282366920938463463371420279198068639)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463372998114203923211n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463371420279198068639n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint128, uint128) => ebool test 2 (340282366920938463463371649783846953553, 340282366920938463463371649783846953557)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463371649783846953553n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463371649783846953557n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint128, uint128) => ebool test 3 (340282366920938463463371649783846953557, 340282366920938463463371649783846953557)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463371649783846953557n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463371649783846953557n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint128, uint128) => ebool test 4 (340282366920938463463371649783846953557, 340282366920938463463371649783846953553)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463371649783846953557n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463371649783846953553n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (uint128, euint128) => ebool test 1 (340282366920938463463372644902056273801, 340282366920938463463371420279198068639)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463371420279198068639n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_uint128_euint128(
      340282366920938463463372644902056273801n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (uint128, euint128) => ebool test 2 (340282366920938463463371649783846953553, 340282366920938463463371649783846953557)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463371649783846953557n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_uint128_euint128(
      340282366920938463463371649783846953553n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (uint128, euint128) => ebool test 3 (340282366920938463463371649783846953557, 340282366920938463463371649783846953557)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463371649783846953557n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_uint128_euint128(
      340282366920938463463371649783846953557n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (uint128, euint128) => ebool test 4 (340282366920938463463371649783846953557, 340282366920938463463371649783846953553)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463371649783846953553n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_uint128_euint128(
      340282366920938463463371649783846953557n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint128, uint128) => ebool test 1 (340282366920938463463366102821272814471, 340282366920938463463369505999917308855)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366102821272814471n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369505999917308855n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint128, uint128) => ebool test 2 (340282366920938463463366102821272814467, 340282366920938463463366102821272814471)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366102821272814467n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366102821272814471n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint128, uint128) => ebool test 3 (340282366920938463463366102821272814471, 340282366920938463463366102821272814471)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366102821272814471n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366102821272814471n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint128, uint128) => ebool test 4 (340282366920938463463366102821272814471, 340282366920938463463366102821272814467)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366102821272814471n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366102821272814467n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (uint128, euint128) => ebool test 1 (340282366920938463463372188964606490321, 340282366920938463463369505999917308855)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463369505999917308855n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_uint128_euint128(
      340282366920938463463372188964606490321n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (uint128, euint128) => ebool test 2 (340282366920938463463366102821272814467, 340282366920938463463366102821272814471)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463366102821272814471n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_uint128_euint128(
      340282366920938463463366102821272814467n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (uint128, euint128) => ebool test 3 (340282366920938463463366102821272814471, 340282366920938463463366102821272814471)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463366102821272814471n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_uint128_euint128(
      340282366920938463463366102821272814471n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (uint128, euint128) => ebool test 4 (340282366920938463463366102821272814471, 340282366920938463463366102821272814467)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463366102821272814467n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_uint128_euint128(
      340282366920938463463366102821272814471n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint128, uint128) => ebool test 1 (340282366920938463463369344739707431379, 340282366920938463463370989122178417933)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463369344739707431379n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463370989122178417933n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint128, uint128) => ebool test 2 (340282366920938463463369344739707431375, 340282366920938463463369344739707431379)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463369344739707431375n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369344739707431379n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint128, uint128) => ebool test 3 (340282366920938463463369344739707431379, 340282366920938463463369344739707431379)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463369344739707431379n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369344739707431379n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint128, uint128) => ebool test 4 (340282366920938463463369344739707431379, 340282366920938463463369344739707431375)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463369344739707431379n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369344739707431375n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (uint128, euint128) => ebool test 1 (340282366920938463463365953290605925917, 340282366920938463463370989122178417933)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463370989122178417933n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_uint128_euint128(
      340282366920938463463365953290605925917n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (uint128, euint128) => ebool test 2 (340282366920938463463369344739707431375, 340282366920938463463369344739707431379)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463369344739707431379n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_uint128_euint128(
      340282366920938463463369344739707431375n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (uint128, euint128) => ebool test 3 (340282366920938463463369344739707431379, 340282366920938463463369344739707431379)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463369344739707431379n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_uint128_euint128(
      340282366920938463463369344739707431379n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (uint128, euint128) => ebool test 4 (340282366920938463463369344739707431379, 340282366920938463463369344739707431375)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463369344739707431375n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_uint128_euint128(
      340282366920938463463369344739707431379n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint128, uint128) => ebool test 1 (340282366920938463463373327259895010209, 340282366920938463463368574633120726095)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463373327259895010209n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368574633120726095n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint128, uint128) => ebool test 2 (340282366920938463463373327259895010205, 340282366920938463463373327259895010209)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463373327259895010205n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463373327259895010209n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint128, uint128) => ebool test 3 (340282366920938463463373327259895010209, 340282366920938463463373327259895010209)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463373327259895010209n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463373327259895010209n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint128, uint128) => ebool test 4 (340282366920938463463373327259895010209, 340282366920938463463373327259895010205)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463373327259895010209n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463373327259895010205n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (uint128, euint128) => ebool test 1 (340282366920938463463366902248162088555, 340282366920938463463368574633120726095)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463368574633120726095n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_uint128_euint128(
      340282366920938463463366902248162088555n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (uint128, euint128) => ebool test 2 (340282366920938463463373327259895010205, 340282366920938463463373327259895010209)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463373327259895010209n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_uint128_euint128(
      340282366920938463463373327259895010205n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (uint128, euint128) => ebool test 3 (340282366920938463463373327259895010209, 340282366920938463463373327259895010209)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463373327259895010209n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_uint128_euint128(
      340282366920938463463373327259895010209n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (uint128, euint128) => ebool test 4 (340282366920938463463373327259895010209, 340282366920938463463373327259895010205)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463373327259895010205n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_uint128_euint128(
      340282366920938463463373327259895010209n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint128, uint128) => ebool test 1 (340282366920938463463365894072006949183, 340282366920938463463368660543232477801)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463365894072006949183n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368660543232477801n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint128, uint128) => ebool test 2 (340282366920938463463365894072006949179, 340282366920938463463365894072006949183)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463365894072006949179n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463365894072006949183n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint128, uint128) => ebool test 3 (340282366920938463463365894072006949183, 340282366920938463463365894072006949183)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463365894072006949183n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463365894072006949183n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint128, uint128) => ebool test 4 (340282366920938463463365894072006949183, 340282366920938463463365894072006949179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463365894072006949183n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463365894072006949179n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (uint128, euint128) => ebool test 1 (340282366920938463463373802267756861013, 340282366920938463463368660543232477801)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463368660543232477801n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_uint128_euint128(
      340282366920938463463373802267756861013n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (uint128, euint128) => ebool test 2 (340282366920938463463365894072006949179, 340282366920938463463365894072006949183)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463365894072006949183n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_uint128_euint128(
      340282366920938463463365894072006949179n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (uint128, euint128) => ebool test 3 (340282366920938463463365894072006949183, 340282366920938463463365894072006949183)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463365894072006949183n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_uint128_euint128(
      340282366920938463463365894072006949183n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (uint128, euint128) => ebool test 4 (340282366920938463463365894072006949183, 340282366920938463463365894072006949179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463365894072006949179n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_uint128_euint128(
      340282366920938463463365894072006949183n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint128, uint128) => euint128 test 1 (340282366920938463463370809911101463147, 340282366920938463463371563668813565353)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463370809911101463147n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463371563668813565353n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463370809911101463147n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint128, uint128) => euint128 test 2 (340282366920938463463370809911101463143, 340282366920938463463370809911101463147)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463370809911101463143n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463370809911101463147n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463370809911101463143n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint128, uint128) => euint128 test 3 (340282366920938463463370809911101463147, 340282366920938463463370809911101463147)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463370809911101463147n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463370809911101463147n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463370809911101463147n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint128, uint128) => euint128 test 4 (340282366920938463463370809911101463147, 340282366920938463463370809911101463143)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463370809911101463147n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463370809911101463143n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463370809911101463143n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (uint128, euint128) => euint128 test 1 (340282366920938463463366055952442089615, 340282366920938463463371563668813565353)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463371563668813565353n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_uint128_euint128(
      340282366920938463463366055952442089615n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463366055952442089615n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (uint128, euint128) => euint128 test 2 (340282366920938463463370809911101463143, 340282366920938463463370809911101463147)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463370809911101463147n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_uint128_euint128(
      340282366920938463463370809911101463143n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463370809911101463143n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (uint128, euint128) => euint128 test 3 (340282366920938463463370809911101463147, 340282366920938463463370809911101463147)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463370809911101463147n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_uint128_euint128(
      340282366920938463463370809911101463147n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463370809911101463147n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (uint128, euint128) => euint128 test 4 (340282366920938463463370809911101463147, 340282366920938463463370809911101463143)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463370809911101463143n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_uint128_euint128(
      340282366920938463463370809911101463147n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463370809911101463143n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint128, uint128) => euint128 test 1 (340282366920938463463372335026694200621, 340282366920938463463366336988031139621)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463372335026694200621n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366336988031139621n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463372335026694200621n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint128, uint128) => euint128 test 2 (340282366920938463463371236394929219651, 340282366920938463463371236394929219655)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463371236394929219651n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463371236394929219655n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463371236394929219655n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint128, uint128) => euint128 test 3 (340282366920938463463371236394929219655, 340282366920938463463371236394929219655)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463371236394929219655n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463371236394929219655n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463371236394929219655n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint128, uint128) => euint128 test 4 (340282366920938463463371236394929219655, 340282366920938463463371236394929219651)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463371236394929219655n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463371236394929219651n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463371236394929219655n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint128, euint128) => euint128 test 1 (340282366920938463463371302060466355775, 340282366920938463463366336988031139621)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463366336988031139621n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_uint128_euint128(
      340282366920938463463371302060466355775n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463371302060466355775n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint128, euint128) => euint128 test 2 (340282366920938463463371236394929219651, 340282366920938463463371236394929219655)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463371236394929219655n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_uint128_euint128(
      340282366920938463463371236394929219651n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463371236394929219655n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint128, euint128) => euint128 test 3 (340282366920938463463371236394929219655, 340282366920938463463371236394929219655)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463371236394929219655n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_uint128_euint128(
      340282366920938463463371236394929219655n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463371236394929219655n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint128, euint128) => euint128 test 4 (340282366920938463463371236394929219655, 340282366920938463463371236394929219651)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463371236394929219651n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_uint128_euint128(
      340282366920938463463371236394929219655n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463371236394929219655n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint256, uint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457580982834624348641, 115792089237316195423570985008687907853269984665640564039457580474200029765923)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580982834624348641n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457580474200029765923n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457579856230308905249n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint256, uint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457575223313874547579, 115792089237316195423570985008687907853269984665640564039457575223313874547583)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575223313874547579n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575223313874547583n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457575223313874547579n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint256, uint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457575223313874547583, 115792089237316195423570985008687907853269984665640564039457575223313874547583)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575223313874547583n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575223313874547583n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457575223313874547583n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint256, uint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457575223313874547583, 115792089237316195423570985008687907853269984665640564039457575223313874547579)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575223313874547583n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575223313874547579n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457575223313874547579n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457580941023235117327, 115792089237316195423570985008687907853269984665640564039457580474200029765923)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457580474200029765923n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457580941023235117327n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457579805622860349699n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457575223313874547579, 115792089237316195423570985008687907853269984665640564039457575223313874547583)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575223313874547583n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575223313874547579n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457575223313874547579n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457575223313874547583, 115792089237316195423570985008687907853269984665640564039457575223313874547583)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575223313874547583n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575223313874547583n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457575223313874547583n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457575223313874547583, 115792089237316195423570985008687907853269984665640564039457575223313874547579)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575223313874547579n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575223313874547583n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457575223313874547579n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint256, uint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457582392894412137563, 115792089237316195423570985008687907853269984665640564039457575331542339863133)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582392894412137563n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575331542339863133n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457582723576831403615n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint256, uint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457582392894412137559, 115792089237316195423570985008687907853269984665640564039457582392894412137563)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582392894412137559n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457582392894412137563n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457582392894412137567n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint256, uint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457582392894412137563, 115792089237316195423570985008687907853269984665640564039457582392894412137563)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582392894412137563n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457582392894412137563n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457582392894412137563n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint256, uint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457582392894412137563, 115792089237316195423570985008687907853269984665640564039457582392894412137559)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582392894412137563n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457582392894412137559n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457582392894412137567n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457575858219995323923, 115792089237316195423570985008687907853269984665640564039457575331542339863133)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575331542339863133n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575858219995323923n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457575897811003891295n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457582392894412137559, 115792089237316195423570985008687907853269984665640564039457582392894412137563)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457582392894412137563n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457582392894412137559n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457582392894412137567n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457582392894412137563, 115792089237316195423570985008687907853269984665640564039457582392894412137563)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457582392894412137563n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457582392894412137563n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457582392894412137563n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457582392894412137563, 115792089237316195423570985008687907853269984665640564039457582392894412137559)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457582392894412137559n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457582392894412137563n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457582392894412137567n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint256, uint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457583939772404403733, 115792089237316195423570985008687907853269984665640564039457582629332176899917)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457583939772404403733n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457582629332176899917n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1363630042087768n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint256, uint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457578055689492796285, 115792089237316195423570985008687907853269984665640564039457578055689492796289)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578055689492796285n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457578055689492796289n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 252n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint256, uint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457578055689492796289, 115792089237316195423570985008687907853269984665640564039457578055689492796289)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578055689492796289n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457578055689492796289n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint256, uint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457578055689492796289, 115792089237316195423570985008687907853269984665640564039457578055689492796285)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578055689492796289n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457578055689492796285n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 252n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (uint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457577322712580175615, 115792089237316195423570985008687907853269984665640564039457582629332176899917)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457582629332176899917n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457577322712580175615n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 5389704798910898n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (uint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457578055689492796285, 115792089237316195423570985008687907853269984665640564039457578055689492796289)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457578055689492796289n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457578055689492796285n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 252n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (uint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457578055689492796289, 115792089237316195423570985008687907853269984665640564039457578055689492796289)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457578055689492796289n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457578055689492796289n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (uint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457578055689492796289, 115792089237316195423570985008687907853269984665640564039457578055689492796285)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457578055689492796285n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457578055689492796289n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 252n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint256, uint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457579359290905296655, 115792089237316195423570985008687907853269984665640564039457579346065685102227)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579359290905296655n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457579346065685102227n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint256, uint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457577117353060079329, 115792089237316195423570985008687907853269984665640564039457577117353060079333)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577117353060079329n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457577117353060079333n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint256, uint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457577117353060079333, 115792089237316195423570985008687907853269984665640564039457577117353060079333)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577117353060079333n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457577117353060079333n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint256, uint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457577117353060079333, 115792089237316195423570985008687907853269984665640564039457577117353060079329)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577117353060079333n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457577117353060079329n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (uint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457577642473150177035, 115792089237316195423570985008687907853269984665640564039457579346065685102227)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457579346065685102227n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457577642473150177035n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (uint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457577117353060079329, 115792089237316195423570985008687907853269984665640564039457577117353060079333)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457577117353060079333n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457577117353060079329n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (uint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457577117353060079333, 115792089237316195423570985008687907853269984665640564039457577117353060079333)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457577117353060079333n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457577117353060079333n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (uint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457577117353060079333, 115792089237316195423570985008687907853269984665640564039457577117353060079329)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457577117353060079329n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457577117353060079333n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint256, uint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457578399758854865067, 115792089237316195423570985008687907853269984665640564039457582892742172339549)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578399758854865067n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457582892742172339549n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint256, uint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457578399758854865063, 115792089237316195423570985008687907853269984665640564039457578399758854865067)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578399758854865063n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457578399758854865067n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint256, uint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457578399758854865067, 115792089237316195423570985008687907853269984665640564039457578399758854865067)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578399758854865067n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457578399758854865067n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint256, uint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457578399758854865067, 115792089237316195423570985008687907853269984665640564039457578399758854865063)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578399758854865067n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457578399758854865063n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (uint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457581580098872605199, 115792089237316195423570985008687907853269984665640564039457582892742172339549)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457582892742172339549n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457581580098872605199n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (uint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457578399758854865063, 115792089237316195423570985008687907853269984665640564039457578399758854865067)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457578399758854865067n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457578399758854865063n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (uint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457578399758854865067, 115792089237316195423570985008687907853269984665640564039457578399758854865067)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457578399758854865067n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457578399758854865067n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (uint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457578399758854865067, 115792089237316195423570985008687907853269984665640564039457578399758854865063)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457578399758854865063n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457578399758854865067n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 1 (22, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(22n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 192n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(1n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 32n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(5n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 160n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(5n);
    input.add8(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 10n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 1 (22, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(22n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint8_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 192n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(1n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint8_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 32n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint8_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 160n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint8_uint8(encryptedAmount.handles[0], 1n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 10n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 1 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(11n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 2 (7, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(7n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 3 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(11n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 4 (11, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(11n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 1 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(11n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint8_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 2 (7, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint8_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 3 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(11n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint8_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 4 (11, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(11n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint8_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint8, euint8) => euint8 test 1 (5, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(5n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 40n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint8, euint8) => euint8 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(1n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 32n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint8, euint8) => euint8 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(5n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 160n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint8, euint8) => euint8 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(5n);
    input.add8(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 10n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint8, uint8) => euint8 test 1 (5, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint8_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 40n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint8, uint8) => euint8 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(1n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint8_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 32n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint8, uint8) => euint8 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint8_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 160n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint8, uint8) => euint8 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint8_uint8(encryptedAmount.handles[0], 1n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 10n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint8, euint8) => euint8 test 1 (118, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(118n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 118n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 8n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add8(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 128n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
