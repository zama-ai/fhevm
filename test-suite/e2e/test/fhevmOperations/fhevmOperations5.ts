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

describe('FHEVM operations 5', function () {
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

  it('test operator "mul" overload (euint32, euint128) => euint128 test 1 (2, 1073741825)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2n);
    input.add128(1073741825n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2147483650n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, euint128) => euint128 test 2 (33932, 33932)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(33932n);
    input.add128(33932n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1151380624n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, euint128) => euint128 test 3 (33932, 33932)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(33932n);
    input.add128(33932n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1151380624n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, euint128) => euint128 test 4 (33932, 33932)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(33932n);
    input.add128(33932n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1151380624n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint32, euint128) => euint128 test 1 (3196832663, 340282366920938463463368786514520405095)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3196832663n);
    input.add128(340282366920938463463368786514520405095n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 369704967n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint32, euint128) => euint128 test 2 (3196832659, 3196832663)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3196832659n);
    input.add128(3196832663n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3196832659n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint32, euint128) => euint128 test 3 (3196832663, 3196832663)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3196832663n);
    input.add128(3196832663n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3196832663n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint32, euint128) => euint128 test 4 (3196832663, 3196832659)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3196832663n);
    input.add128(3196832659n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3196832659n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, euint128) => euint128 test 1 (1342286020, 340282366920938463463369766339974733293)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1342286020n);
    input.add128(340282366920938463463369766339974733293n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463369766339974774253n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, euint128) => euint128 test 2 (1342286016, 1342286020)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1342286016n);
    input.add128(1342286020n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1342286020n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, euint128) => euint128 test 3 (1342286020, 1342286020)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1342286020n);
    input.add128(1342286020n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1342286020n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, euint128) => euint128 test 4 (1342286020, 1342286016)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1342286020n);
    input.add128(1342286016n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1342286020n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint128) => euint128 test 1 (3573557144, 340282366920938463463366806511464387483)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3573557144n);
    input.add128(340282366920938463463366806511464387483n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463366806510071886851n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint128) => euint128 test 2 (3573557140, 3573557144)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3573557140n);
    input.add128(3573557144n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 12n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint128) => euint128 test 3 (3573557144, 3573557144)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3573557144n);
    input.add128(3573557144n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint128) => euint128 test 4 (3573557144, 3573557140)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3573557144n);
    input.add128(3573557140n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 12n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint32, euint128) => ebool test 1 (2540307962, 340282366920938463463366782625811984369)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2540307962n);
    input.add128(340282366920938463463366782625811984369n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint32, euint128) => ebool test 2 (2540307958, 2540307962)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2540307958n);
    input.add128(2540307962n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint32, euint128) => ebool test 3 (2540307962, 2540307962)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2540307962n);
    input.add128(2540307962n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint32, euint128) => ebool test 4 (2540307962, 2540307958)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2540307962n);
    input.add128(2540307958n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint32, euint128) => ebool test 1 (3776836771, 340282366920938463463366673840926858539)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3776836771n);
    input.add128(340282366920938463463366673840926858539n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint32, euint128) => ebool test 2 (3776836767, 3776836771)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3776836767n);
    input.add128(3776836771n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint32, euint128) => ebool test 3 (3776836771, 3776836771)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3776836771n);
    input.add128(3776836771n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint32, euint128) => ebool test 4 (3776836771, 3776836767)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3776836771n);
    input.add128(3776836767n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint32, euint128) => ebool test 1 (1353678396, 340282366920938463463370837128962063895)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1353678396n);
    input.add128(340282366920938463463370837128962063895n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint32, euint128) => ebool test 2 (1353678392, 1353678396)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1353678392n);
    input.add128(1353678396n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint32, euint128) => ebool test 3 (1353678396, 1353678396)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1353678396n);
    input.add128(1353678396n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint32, euint128) => ebool test 4 (1353678396, 1353678392)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1353678396n);
    input.add128(1353678392n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint32, euint128) => ebool test 1 (3023877819, 340282366920938463463373188274142516835)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3023877819n);
    input.add128(340282366920938463463373188274142516835n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint32, euint128) => ebool test 2 (3023877815, 3023877819)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3023877815n);
    input.add128(3023877819n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint32, euint128) => ebool test 3 (3023877819, 3023877819)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3023877819n);
    input.add128(3023877819n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint32, euint128) => ebool test 4 (3023877819, 3023877815)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3023877819n);
    input.add128(3023877815n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint32, euint128) => ebool test 1 (3782861213, 340282366920938463463371045960537607213)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3782861213n);
    input.add128(340282366920938463463371045960537607213n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint32, euint128) => ebool test 2 (3782861209, 3782861213)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3782861209n);
    input.add128(3782861213n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint32, euint128) => ebool test 3 (3782861213, 3782861213)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3782861213n);
    input.add128(3782861213n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint32, euint128) => ebool test 4 (3782861213, 3782861209)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3782861213n);
    input.add128(3782861209n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint32, euint128) => ebool test 1 (1271876114, 340282366920938463463369232425419313293)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1271876114n);
    input.add128(340282366920938463463369232425419313293n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint32, euint128) => ebool test 2 (1271876110, 1271876114)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1271876110n);
    input.add128(1271876114n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint32, euint128) => ebool test 3 (1271876114, 1271876114)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1271876114n);
    input.add128(1271876114n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint32, euint128) => ebool test 4 (1271876114, 1271876110)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1271876114n);
    input.add128(1271876110n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint32, euint128) => euint128 test 1 (4113403185, 340282366920938463463374024527003005721)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4113403185n);
    input.add128(340282366920938463463374024527003005721n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4113403185n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint32, euint128) => euint128 test 2 (4113403181, 4113403185)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4113403181n);
    input.add128(4113403185n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4113403181n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint32, euint128) => euint128 test 3 (4113403185, 4113403185)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4113403185n);
    input.add128(4113403185n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4113403185n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint32, euint128) => euint128 test 4 (4113403185, 4113403181)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4113403185n);
    input.add128(4113403181n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4113403181n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint32, euint128) => euint128 test 1 (3499209524, 340282366920938463463371223323602776895)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3499209524n);
    input.add128(340282366920938463463371223323602776895n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463371223323602776895n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint32, euint128) => euint128 test 2 (3499209520, 3499209524)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3499209520n);
    input.add128(3499209524n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3499209524n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint32, euint128) => euint128 test 3 (3499209524, 3499209524)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3499209524n);
    input.add128(3499209524n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3499209524n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint32, euint128) => euint128 test 4 (3499209524, 3499209520)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3499209524n);
    input.add128(3499209520n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3499209524n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint32, euint256) => euint256 test 1 (1829469970, 115792089237316195423570985008687907853269984665640564039457580388054221621959)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1829469970n);
    input.add256(115792089237316195423570985008687907853269984665640564039457580388054221621959n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1761741314n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint32, euint256) => euint256 test 2 (1829469966, 1829469970)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1829469966n);
    input.add256(1829469970n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1829469954n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint32, euint256) => euint256 test 3 (1829469970, 1829469970)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1829469970n);
    input.add256(1829469970n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1829469970n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint32, euint256) => euint256 test 4 (1829469970, 1829469966)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1829469970n);
    input.add256(1829469966n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1829469954n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, euint256) => euint256 test 1 (2978042672, 115792089237316195423570985008687907853269984665640564039457579090657366361759)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2978042672n);
    input.add256(115792089237316195423570985008687907853269984665640564039457579090657366361759n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457579090659799062463n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, euint256) => euint256 test 2 (2978042668, 2978042672)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2978042668n);
    input.add256(2978042672n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2978042684n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, euint256) => euint256 test 3 (2978042672, 2978042672)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2978042672n);
    input.add256(2978042672n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2978042672n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, euint256) => euint256 test 4 (2978042672, 2978042668)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2978042672n);
    input.add256(2978042668n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2978042684n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint256) => euint256 test 1 (2702851245, 115792089237316195423570985008687907853269984665640564039457581337126603158585)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2702851245n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581337126603158585n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457581337129270331540n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint256) => euint256 test 2 (2702851241, 2702851245)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2702851241n);
    input.add256(2702851245n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint256) => euint256 test 3 (2702851245, 2702851245)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2702851245n);
    input.add256(2702851245n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint256) => euint256 test 4 (2702851245, 2702851241)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2702851245n);
    input.add256(2702851241n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint256();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint32, euint256) => ebool test 1 (3729833681, 115792089237316195423570985008687907853269984665640564039457578988032187830595)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3729833681n);
    input.add256(115792089237316195423570985008687907853269984665640564039457578988032187830595n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint32, euint256) => ebool test 2 (3729833677, 3729833681)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3729833677n);
    input.add256(3729833681n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint32, euint256) => ebool test 3 (3729833681, 3729833681)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3729833681n);
    input.add256(3729833681n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint32, euint256) => ebool test 4 (3729833681, 3729833677)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3729833681n);
    input.add256(3729833677n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint32, euint256) => ebool test 1 (2010480401, 115792089237316195423570985008687907853269984665640564039457575019046312930229)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2010480401n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575019046312930229n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint32, euint256) => ebool test 2 (2010480397, 2010480401)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2010480397n);
    input.add256(2010480401n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint32, euint256) => ebool test 3 (2010480401, 2010480401)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2010480401n);
    input.add256(2010480401n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint32, euint256) => ebool test 4 (2010480401, 2010480397)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2010480401n);
    input.add256(2010480397n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 1 (129, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(129n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 131n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 2 (105, 107)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(105n);
    input.add8(107n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 212n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 3 (107, 107)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(107n);
    input.add8(107n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 214n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 4 (107, 105)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(107n);
    input.add8(105n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 212n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint64, euint8) => euint64 test 1 (172, 172)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(172n);
    input.add8(172n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint64, euint8) => euint64 test 2 (172, 168)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(172n);
    input.add8(168n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 1 (65, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
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
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 130n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 2 (6, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(6n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 60n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(10n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 100n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 4 (10, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(10n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 60n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 1 (18439373627278524157, 124)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18439373627278524157n);
    input.add8(124n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 124n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 2 (120, 124)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(120n);
    input.add8(124n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 120n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 3 (124, 124)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(124n);
    input.add8(124n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 124n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 4 (124, 120)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(124n);
    input.add8(120n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 120n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 1 (18442432448654667535, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18442432448654667535n);
    input.add8(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18442432448654667535n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 2 (8, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(8n);
    input.add8(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 12n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 3 (12, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(12n);
    input.add8(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 12n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 4 (12, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(12n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 12n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 1 (18443203758534649813, 138)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18443203758534649813n);
    input.add8(138n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18443203758534649695n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 2 (134, 138)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(134n);
    input.add8(138n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 12n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 3 (138, 138)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(138n);
    input.add8(138n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 4 (138, 134)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(138n);
    input.add8(134n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 12n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 1 (18442466888825635089, 209)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18442466888825635089n);
    input.add8(209n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 2 (205, 209)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(205n);
    input.add8(209n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 3 (209, 209)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(209n);
    input.add8(209n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 4 (209, 205)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(209n);
    input.add8(205n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 1 (18441415566690847005, 240)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18441415566690847005n);
    input.add8(240n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 2 (236, 240)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(236n);
    input.add8(240n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 3 (240, 240)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(240n);
    input.add8(240n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 4 (240, 236)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(240n);
    input.add8(236n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 1 (18446421399471592433, 96)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18446421399471592433n);
    input.add8(96n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 2 (92, 96)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(92n);
    input.add8(96n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 3 (96, 96)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(96n);
    input.add8(96n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 4 (96, 92)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(96n);
    input.add8(92n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 1 (18442979095678172065, 43)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18442979095678172065n);
    input.add8(43n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 2 (39, 43)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(39n);
    input.add8(43n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 3 (43, 43)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(43n);
    input.add8(43n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 4 (43, 39)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(43n);
    input.add8(39n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 1 (18446001382131942121, 239)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18446001382131942121n);
    input.add8(239n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 2 (235, 239)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(235n);
    input.add8(239n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 3 (239, 239)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(239n);
    input.add8(239n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 4 (239, 235)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(239n);
    input.add8(235n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 1 (18442383493980297819, 28)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18442383493980297819n);
    input.add8(28n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 2 (24, 28)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(24n);
    input.add8(28n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 3 (28, 28)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(28n);
    input.add8(28n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 4 (28, 24)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(28n);
    input.add8(24n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 1 (18443001839243245973, 133)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18443001839243245973n);
    input.add8(133n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 133n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 2 (129, 133)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(129n);
    input.add8(133n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 129n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 3 (133, 133)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(133n);
    input.add8(133n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 133n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 4 (133, 129)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(133n);
    input.add8(129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 129n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 1 (18439071506998467653, 212)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18439071506998467653n);
    input.add8(212n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18439071506998467653n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 2 (208, 212)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(208n);
    input.add8(212n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 212n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 3 (212, 212)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(212n);
    input.add8(212n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 212n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 4 (212, 208)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(212n);
    input.add8(208n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 212n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 1 (65533, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(65533n);
    input.add16(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 65535n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 2 (30855, 30857)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(30855n);
    input.add16(30857n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 61712n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 3 (30857, 30857)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(30857n);
    input.add16(30857n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 61714n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 4 (30857, 30855)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(30857n);
    input.add16(30855n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 61712n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint64, euint16) => euint64 test 1 (24454, 24454)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(24454n);
    input.add16(24454n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint64, euint16) => euint64 test 2 (24454, 24450)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(24454n);
    input.add16(24450n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 1 (32764, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(32764n);
    input.add16(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 65528n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 2 (177, 177)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(177n);
    input.add16(177n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 31329n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 3 (177, 177)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(177n);
    input.add16(177n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 31329n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 4 (177, 177)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(177n);
    input.add16(177n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 31329n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 1 (18444743792090021731, 45198)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18444743792090021731n);
    input.add16(45198n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 45058n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 2 (45194, 45198)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(45194n);
    input.add16(45198n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 45194n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 3 (45198, 45198)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(45198n);
    input.add16(45198n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 45198n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 4 (45198, 45194)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(45198n);
    input.add16(45194n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 45194n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 1 (18443717716439020299, 58875)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18443717716439020299n);
    input.add16(58875n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18443717716439070715n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 2 (58871, 58875)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(58871n);
    input.add16(58875n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 58879n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 3 (58875, 58875)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(58875n);
    input.add16(58875n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 58875n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 4 (58875, 58871)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(58875n);
    input.add16(58871n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 58879n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 1 (18446171774675541943, 13058)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18446171774675541943n);
    input.add16(13058n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18446171774675545269n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 2 (13054, 13058)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(13054n);
    input.add16(13058n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 508n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 3 (13058, 13058)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(13058n);
    input.add16(13058n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 4 (13058, 13054)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(13058n);
    input.add16(13054n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 508n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 1 (18441466749942160105, 17504)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18441466749942160105n);
    input.add16(17504n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 2 (17500, 17504)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(17500n);
    input.add16(17504n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 3 (17504, 17504)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(17504n);
    input.add16(17504n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 4 (17504, 17500)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(17504n);
    input.add16(17500n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 1 (18439760603592748181, 11758)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18439760603592748181n);
    input.add16(11758n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 2 (11754, 11758)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(11754n);
    input.add16(11758n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 3 (11758, 11758)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(11758n);
    input.add16(11758n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 4 (11758, 11754)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(11758n);
    input.add16(11754n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 1 (18437764494511136313, 27076)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18437764494511136313n);
    input.add16(27076n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 2 (27072, 27076)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(27072n);
    input.add16(27076n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 3 (27076, 27076)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(27076n);
    input.add16(27076n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 4 (27076, 27072)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(27076n);
    input.add16(27072n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 1 (18445319106007377599, 20986)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18445319106007377599n);
    input.add16(20986n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 2 (20982, 20986)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(20982n);
    input.add16(20986n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 3 (20986, 20986)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(20986n);
    input.add16(20986n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 4 (20986, 20982)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(20986n);
    input.add16(20982n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 1 (18443070930247635813, 57013)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18443070930247635813n);
    input.add16(57013n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 2 (57009, 57013)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(57009n);
    input.add16(57013n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 3 (57013, 57013)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(57013n);
    input.add16(57013n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 4 (57013, 57009)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(57013n);
    input.add16(57009n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 1 (18437939001633712775, 5081)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18437939001633712775n);
    input.add16(5081n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 2 (5077, 5081)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(5077n);
    input.add16(5081n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 3 (5081, 5081)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(5081n);
    input.add16(5081n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 4 (5081, 5077)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(5081n);
    input.add16(5077n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 1 (18438200734267984551, 61525)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18438200734267984551n);
    input.add16(61525n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 61525n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 2 (61521, 61525)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(61521n);
    input.add16(61525n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 61521n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 3 (61525, 61525)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(61525n);
    input.add16(61525n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 61525n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 4 (61525, 61521)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(61525n);
    input.add16(61521n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 61521n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 1 (18438423629937147339, 52033)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18438423629937147339n);
    input.add16(52033n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438423629937147339n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 2 (52029, 52033)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(52029n);
    input.add16(52033n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 52033n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 3 (52033, 52033)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(52033n);
    input.add16(52033n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 52033n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 4 (52033, 52029)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(52033n);
    input.add16(52029n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 52033n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 1 (4293973052, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(4293973052n);
    input.add32(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4293973054n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 2 (2095772557, 2095772559)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(2095772557n);
    input.add32(2095772559n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4191545116n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 3 (2095772559, 2095772559)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(2095772559n);
    input.add32(2095772559n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4191545118n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 4 (2095772559, 2095772557)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(2095772559n);
    input.add32(2095772557n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4191545116n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint64, euint32) => euint64 test 1 (2680546995, 2680546995)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(2680546995n);
    input.add32(2680546995n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint64, euint32) => euint64 test 2 (2680546995, 2680546991)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(2680546995n);
    input.add32(2680546991n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 1 (2147286010, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(2147286010n);
    input.add32(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4294572020n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 2 (45563, 45563)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(45563n);
    input.add32(45563n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2075986969n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 3 (45563, 45563)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(45563n);
    input.add32(45563n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2075986969n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 4 (45563, 45563)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(45563n);
    input.add32(45563n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2075986969n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 1 (18438994886836005911, 2178158629)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18438994886836005911n);
    input.add32(2178158629n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2165309445n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 2 (2178158625, 2178158629)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(2178158625n);
    input.add32(2178158629n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2178158625n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 3 (2178158629, 2178158629)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(2178158629n);
    input.add32(2178158629n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2178158629n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 4 (2178158629, 2178158625)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(2178158629n);
    input.add32(2178158625n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2178158625n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 1 (18439839299029545997, 2438051999)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18439839299029545997n);
    input.add32(2438051999n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18439839299029548191n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 2 (2438051995, 2438051999)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(2438051995n);
    input.add32(2438051999n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2438051999n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 3 (2438051999, 2438051999)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(2438051999n);
    input.add32(2438051999n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2438051999n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 4 (2438051999, 2438051995)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(2438051999n);
    input.add32(2438051995n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2438051999n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 1 (18445456627230610767, 999610792)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18445456627230610767n);
    input.add32(999610792n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18445456628177194215n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 2 (999610788, 999610792)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(999610788n);
    input.add32(999610792n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 12n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 3 (999610792, 999610792)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(999610792n);
    input.add32(999610792n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 4 (999610792, 999610788)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(999610792n);
    input.add32(999610788n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 12n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 1 (18444938388229639537, 3438920769)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18444938388229639537n);
    input.add32(3438920769n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 2 (3438920765, 3438920769)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(3438920765n);
    input.add32(3438920769n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 3 (3438920769, 3438920769)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(3438920769n);
    input.add32(3438920769n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 4 (3438920769, 3438920765)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(3438920769n);
    input.add32(3438920765n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });
});
