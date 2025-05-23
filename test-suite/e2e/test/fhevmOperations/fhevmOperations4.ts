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

describe('FHEVM operations 4', function () {
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

  it('test operator "eq" overload (euint32, euint8) => ebool test 1 (1423272652, 157)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(1423272652n);
    input.add8(157n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 2 (153, 157)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(153n);
    input.add8(157n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 3 (157, 157)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(157n);
    input.add8(157n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 4 (157, 153)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(157n);
    input.add8(153n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 1 (313227401, 149)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(313227401n);
    input.add8(149n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 2 (145, 149)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(145n);
    input.add8(149n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 3 (149, 149)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(149n);
    input.add8(149n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 4 (149, 145)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(149n);
    input.add8(145n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 1 (1666296724, 145)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(1666296724n);
    input.add8(145n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 2 (141, 145)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(141n);
    input.add8(145n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 3 (145, 145)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(145n);
    input.add8(145n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 4 (145, 141)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(145n);
    input.add8(141n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 1 (2221473344, 173)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(2221473344n);
    input.add8(173n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 2 (169, 173)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(169n);
    input.add8(173n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 3 (173, 173)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(173n);
    input.add8(173n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 4 (173, 169)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(173n);
    input.add8(169n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 1 (4068253180, 136)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(4068253180n);
    input.add8(136n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 2 (132, 136)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(132n);
    input.add8(136n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 3 (136, 136)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(136n);
    input.add8(136n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 4 (136, 132)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(136n);
    input.add8(132n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 1 (3341408025, 147)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(3341408025n);
    input.add8(147n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 2 (143, 147)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(143n);
    input.add8(147n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 3 (147, 147)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(147n);
    input.add8(147n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 4 (147, 143)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(147n);
    input.add8(143n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 1 (4232992361, 25)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(4232992361n);
    input.add8(25n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 25n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 2 (21, 25)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(21n);
    input.add8(25n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 21n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 3 (25, 25)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(25n);
    input.add8(25n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 25n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 4 (25, 21)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(25n);
    input.add8(21n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 21n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 1 (2770769928, 118)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(2770769928n);
    input.add8(118n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2770769928n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 2 (114, 118)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(114n);
    input.add8(118n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 118n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 3 (118, 118)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(118n);
    input.add8(118n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 118n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 4 (118, 114)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(118n);
    input.add8(114n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 118n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 1 (50040, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(50040n);
    input.add16(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 50043n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 2 (31105, 31109)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(31105n);
    input.add16(31109n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 62214n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 3 (31109, 31109)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(31109n);
    input.add16(31109n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 62218n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 4 (31109, 31105)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(31109n);
    input.add16(31105n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 62214n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint32, euint16) => euint32 test 1 (54948, 54948)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(54948n);
    input.add16(54948n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint32, euint16) => euint32 test 2 (54948, 54944)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(54948n);
    input.add16(54944n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 1 (28782, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(28782n);
    input.add16(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 57564n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 2 (182, 182)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(182n);
    input.add16(182n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 33124n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 3 (182, 182)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(182n);
    input.add16(182n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 33124n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 4 (182, 182)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(182n);
    input.add16(182n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 33124n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 1 (2777246769, 16219)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(2777246769n);
    input.add16(16219n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 11281n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 2 (16215, 16219)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(16215n);
    input.add16(16219n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 16211n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 3 (16219, 16219)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(16219n);
    input.add16(16219n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 16219n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 4 (16219, 16215)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(16219n);
    input.add16(16215n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 16211n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 1 (791143750, 42936)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(791143750n);
    input.add16(42936n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 791144446n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 2 (42932, 42936)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(42932n);
    input.add16(42936n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 42940n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 3 (42936, 42936)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(42936n);
    input.add16(42936n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 42936n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 4 (42936, 42932)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(42936n);
    input.add16(42932n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 42940n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 1 (299345368, 22653)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(299345368n);
    input.add16(22653n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 299367845n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 2 (22649, 22653)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(22649n);
    input.add16(22653n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 3 (22653, 22653)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(22653n);
    input.add16(22653n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 4 (22653, 22649)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(22653n);
    input.add16(22649n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 1 (2992604919, 63555)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(2992604919n);
    input.add16(63555n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 2 (63551, 63555)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(63551n);
    input.add16(63555n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 3 (63555, 63555)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(63555n);
    input.add16(63555n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 4 (63555, 63551)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(63555n);
    input.add16(63551n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 1 (1529736122, 59897)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(1529736122n);
    input.add16(59897n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 2 (59893, 59897)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(59893n);
    input.add16(59897n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 3 (59897, 59897)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(59897n);
    input.add16(59897n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 4 (59897, 59893)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(59897n);
    input.add16(59893n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 1 (1233160565, 16831)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(1233160565n);
    input.add16(16831n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 2 (16827, 16831)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(16827n);
    input.add16(16831n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 3 (16831, 16831)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(16831n);
    input.add16(16831n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 4 (16831, 16827)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(16831n);
    input.add16(16827n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 1 (293700211, 60466)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(293700211n);
    input.add16(60466n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 2 (60462, 60466)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(60462n);
    input.add16(60466n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 3 (60466, 60466)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(60466n);
    input.add16(60466n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 4 (60466, 60462)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(60466n);
    input.add16(60462n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 1 (2370482161, 41554)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(2370482161n);
    input.add16(41554n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 2 (41550, 41554)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(41550n);
    input.add16(41554n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 3 (41554, 41554)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(41554n);
    input.add16(41554n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 4 (41554, 41550)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(41554n);
    input.add16(41550n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 1 (3508741897, 33021)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(3508741897n);
    input.add16(33021n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 2 (33017, 33021)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(33017n);
    input.add16(33021n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 3 (33021, 33021)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(33021n);
    input.add16(33021n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 4 (33021, 33017)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(33021n);
    input.add16(33017n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 1 (3882746587, 46012)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(3882746587n);
    input.add16(46012n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 46012n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 2 (46008, 46012)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(46008n);
    input.add16(46012n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 46008n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 3 (46012, 46012)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(46012n);
    input.add16(46012n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 46012n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 4 (46012, 46008)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(46012n);
    input.add16(46008n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 46008n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 1 (4161833553, 36486)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(4161833553n);
    input.add16(36486n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4161833553n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 2 (36482, 36486)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(36482n);
    input.add16(36486n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 36486n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 3 (36486, 36486)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(36486n);
    input.add16(36486n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 36486n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 4 (36486, 36482)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(36486n);
    input.add16(36482n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 36486n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 1 (752212683, 3115656154)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(752212683n);
    input.add32(3115656154n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3867868837n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 2 (752212679, 752212683)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(752212679n);
    input.add32(752212683n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1504425362n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 3 (752212683, 752212683)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(752212683n);
    input.add32(752212683n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1504425366n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 4 (752212683, 752212679)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(752212683n);
    input.add32(752212679n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1504425362n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint32, euint32) => euint32 test 1 (3210689907, 3210689907)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(3210689907n);
    input.add32(3210689907n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint32, euint32) => euint32 test 2 (3210689907, 3210689903)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(3210689907n);
    input.add32(3210689903n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 1 (53125, 54001)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(53125n);
    input.add32(54001n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2868803125n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 2 (53125, 53125)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(53125n);
    input.add32(53125n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2822265625n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 3 (53125, 53125)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(53125n);
    input.add32(53125n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2822265625n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 4 (53125, 53125)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(53125n);
    input.add32(53125n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2822265625n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 1 (2290992677, 1829524406)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2290992677n);
    input.add32(1829524406n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 135021092n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 2 (1829524402, 1829524406)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1829524402n);
    input.add32(1829524406n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1829524402n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 3 (1829524406, 1829524406)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1829524406n);
    input.add32(1829524406n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1829524406n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 4 (1829524406, 1829524402)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1829524406n);
    input.add32(1829524402n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1829524402n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 1 (2298302465, 2837882125)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2298302465n);
    input.add32(2837882125n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2852123917n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 2 (2298302461, 2298302465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2298302461n);
    input.add32(2298302465n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2298306557n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 3 (2298302465, 2298302465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2298302465n);
    input.add32(2298302465n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2298302465n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 4 (2298302465, 2298302461)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2298302465n);
    input.add32(2298302461n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2298306557n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 1 (109044896, 2869857770)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(109044896n);
    input.add32(2869857770n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2909890890n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 2 (109044892, 109044896)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(109044892n);
    input.add32(109044896n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 60n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 3 (109044896, 109044896)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(109044896n);
    input.add32(109044896n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 4 (109044896, 109044892)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(109044896n);
    input.add32(109044892n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 60n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 1 (381835962, 1378546554)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(381835962n);
    input.add32(1378546554n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint32(
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

  it('test operator "eq" overload (euint32, euint32) => ebool test 2 (381835958, 381835962)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(381835958n);
    input.add32(381835962n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint32(
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

  it('test operator "eq" overload (euint32, euint32) => ebool test 3 (381835962, 381835962)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(381835962n);
    input.add32(381835962n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint32(
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

  it('test operator "eq" overload (euint32, euint32) => ebool test 4 (381835962, 381835958)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(381835962n);
    input.add32(381835958n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint32(
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

  it('test operator "ne" overload (euint32, euint32) => ebool test 1 (118700908, 236627906)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(118700908n);
    input.add32(236627906n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint32(
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

  it('test operator "ne" overload (euint32, euint32) => ebool test 2 (118700904, 118700908)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(118700904n);
    input.add32(118700908n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint32(
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

  it('test operator "ne" overload (euint32, euint32) => ebool test 3 (118700908, 118700908)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(118700908n);
    input.add32(118700908n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint32(
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

  it('test operator "ne" overload (euint32, euint32) => ebool test 4 (118700908, 118700904)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(118700908n);
    input.add32(118700904n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint32(
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

  it('test operator "ge" overload (euint32, euint32) => ebool test 1 (2889403208, 3292448851)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2889403208n);
    input.add32(3292448851n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint32(
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

  it('test operator "ge" overload (euint32, euint32) => ebool test 2 (2889403204, 2889403208)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2889403204n);
    input.add32(2889403208n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint32(
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

  it('test operator "ge" overload (euint32, euint32) => ebool test 3 (2889403208, 2889403208)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2889403208n);
    input.add32(2889403208n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint32(
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

  it('test operator "ge" overload (euint32, euint32) => ebool test 4 (2889403208, 2889403204)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2889403208n);
    input.add32(2889403204n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint32(
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

  it('test operator "gt" overload (euint32, euint32) => ebool test 1 (3298838642, 3428306889)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3298838642n);
    input.add32(3428306889n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint32(
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

  it('test operator "gt" overload (euint32, euint32) => ebool test 2 (3298838638, 3298838642)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3298838638n);
    input.add32(3298838642n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint32(
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

  it('test operator "gt" overload (euint32, euint32) => ebool test 3 (3298838642, 3298838642)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3298838642n);
    input.add32(3298838642n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint32(
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

  it('test operator "gt" overload (euint32, euint32) => ebool test 4 (3298838642, 3298838638)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3298838642n);
    input.add32(3298838638n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint32(
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

  it('test operator "le" overload (euint32, euint32) => ebool test 1 (2857230603, 2130145006)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2857230603n);
    input.add32(2130145006n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint32(
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

  it('test operator "le" overload (euint32, euint32) => ebool test 2 (2130145002, 2130145006)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2130145002n);
    input.add32(2130145006n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint32(
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

  it('test operator "le" overload (euint32, euint32) => ebool test 3 (2130145006, 2130145006)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2130145006n);
    input.add32(2130145006n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint32(
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

  it('test operator "le" overload (euint32, euint32) => ebool test 4 (2130145006, 2130145002)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2130145006n);
    input.add32(2130145002n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint32(
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

  it('test operator "lt" overload (euint32, euint32) => ebool test 1 (3782775649, 3807788295)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3782775649n);
    input.add32(3807788295n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint32(
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

  it('test operator "lt" overload (euint32, euint32) => ebool test 2 (3782775645, 3782775649)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3782775645n);
    input.add32(3782775649n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint32(
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

  it('test operator "lt" overload (euint32, euint32) => ebool test 3 (3782775649, 3782775649)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3782775649n);
    input.add32(3782775649n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint32(
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

  it('test operator "lt" overload (euint32, euint32) => ebool test 4 (3782775649, 3782775645)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3782775649n);
    input.add32(3782775645n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint32(
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

  it('test operator "min" overload (euint32, euint32) => euint32 test 1 (2872867328, 51666488)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2872867328n);
    input.add32(51666488n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 51666488n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 2 (51666484, 51666488)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(51666484n);
    input.add32(51666488n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 51666484n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 3 (51666488, 51666488)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(51666488n);
    input.add32(51666488n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 51666488n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 4 (51666488, 51666484)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(51666488n);
    input.add32(51666484n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 51666484n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 1 (1782568505, 3148826302)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1782568505n);
    input.add32(3148826302n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3148826302n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 2 (1782568501, 1782568505)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1782568501n);
    input.add32(1782568505n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1782568505n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 3 (1782568505, 1782568505)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1782568505n);
    input.add32(1782568505n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1782568505n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 4 (1782568505, 1782568501)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1782568505n);
    input.add32(1782568501n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1782568505n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 1 (2, 4294699026)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2n);
    input.add64(4294699026n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4294699028n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 2 (1703089650, 1703089652)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1703089650n);
    input.add64(1703089652n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3406179302n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 3 (1703089652, 1703089652)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1703089652n);
    input.add64(1703089652n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3406179304n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 4 (1703089652, 1703089650)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1703089652n);
    input.add64(1703089650n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3406179302n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint32, euint64) => euint64 test 1 (3559878237, 3559878237)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3559878237n);
    input.add64(3559878237n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint32_euint64(
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

  it('test operator "sub" overload (euint32, euint64) => euint64 test 2 (3559878237, 3559878233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3559878237n);
    input.add64(3559878233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint32_euint64(
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

  it('test operator "mul" overload (euint32, euint64) => euint64 test 1 (2, 2147439152)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2n);
    input.add64(2147439152n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4294878304n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 2 (48661, 48661)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(48661n);
    input.add64(48661n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2367892921n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 3 (48661, 48661)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(48661n);
    input.add64(48661n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2367892921n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 4 (48661, 48661)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(48661n);
    input.add64(48661n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2367892921n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 1 (3455047282, 18444235173765416753)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3455047282n);
    input.add64(18444235173765416753n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 67995184n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 2 (3455047278, 3455047282)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3455047278n);
    input.add64(3455047282n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3455047266n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 3 (3455047282, 3455047282)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3455047282n);
    input.add64(3455047282n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3455047282n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 4 (3455047282, 3455047278)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3455047282n);
    input.add64(3455047278n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3455047266n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 1 (4209423152, 18444884520941259115)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4209423152n);
    input.add64(18444884520941259115n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18444884524600834939n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 2 (4209423148, 4209423152)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4209423148n);
    input.add64(4209423152n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4209423164n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 3 (4209423152, 4209423152)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4209423152n);
    input.add64(4209423152n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4209423152n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 4 (4209423152, 4209423148)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4209423152n);
    input.add64(4209423148n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4209423164n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 1 (1062331714, 18439080858726780031)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1062331714n);
    input.add64(18439080858726780031n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18439080857874245949n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 2 (1062331710, 1062331714)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1062331710n);
    input.add64(1062331714n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint64(
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

  it('test operator "xor" overload (euint32, euint64) => euint64 test 3 (1062331714, 1062331714)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1062331714n);
    input.add64(1062331714n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint64(
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

  it('test operator "xor" overload (euint32, euint64) => euint64 test 4 (1062331714, 1062331710)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1062331714n);
    input.add64(1062331710n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint64(
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

  it('test operator "eq" overload (euint32, euint64) => ebool test 1 (1318209944, 18439033598442082921)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1318209944n);
    input.add64(18439033598442082921n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint64(
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

  it('test operator "eq" overload (euint32, euint64) => ebool test 2 (1318209940, 1318209944)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1318209940n);
    input.add64(1318209944n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint64(
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

  it('test operator "eq" overload (euint32, euint64) => ebool test 3 (1318209944, 1318209944)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1318209944n);
    input.add64(1318209944n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint64(
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

  it('test operator "eq" overload (euint32, euint64) => ebool test 4 (1318209944, 1318209940)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1318209944n);
    input.add64(1318209940n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint64(
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

  it('test operator "ne" overload (euint32, euint64) => ebool test 1 (2402813595, 18439986531599294593)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2402813595n);
    input.add64(18439986531599294593n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint64(
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

  it('test operator "ne" overload (euint32, euint64) => ebool test 2 (2402813591, 2402813595)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2402813591n);
    input.add64(2402813595n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint64(
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

  it('test operator "ne" overload (euint32, euint64) => ebool test 3 (2402813595, 2402813595)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2402813595n);
    input.add64(2402813595n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint64(
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

  it('test operator "ne" overload (euint32, euint64) => ebool test 4 (2402813595, 2402813591)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2402813595n);
    input.add64(2402813591n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint64(
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

  it('test operator "ge" overload (euint32, euint64) => ebool test 1 (1760110106, 18439085538745382867)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1760110106n);
    input.add64(18439085538745382867n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint64(
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

  it('test operator "ge" overload (euint32, euint64) => ebool test 2 (1760110102, 1760110106)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1760110102n);
    input.add64(1760110106n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint64(
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

  it('test operator "ge" overload (euint32, euint64) => ebool test 3 (1760110106, 1760110106)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1760110106n);
    input.add64(1760110106n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint64(
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

  it('test operator "ge" overload (euint32, euint64) => ebool test 4 (1760110106, 1760110102)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1760110106n);
    input.add64(1760110102n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint64(
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

  it('test operator "gt" overload (euint32, euint64) => ebool test 1 (4254933942, 18438468459713348567)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4254933942n);
    input.add64(18438468459713348567n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint64(
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

  it('test operator "gt" overload (euint32, euint64) => ebool test 2 (4254933938, 4254933942)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4254933938n);
    input.add64(4254933942n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint64(
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

  it('test operator "gt" overload (euint32, euint64) => ebool test 3 (4254933942, 4254933942)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4254933942n);
    input.add64(4254933942n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint64(
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

  it('test operator "gt" overload (euint32, euint64) => ebool test 4 (4254933942, 4254933938)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4254933942n);
    input.add64(4254933938n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint64(
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

  it('test operator "le" overload (euint32, euint64) => ebool test 1 (635959669, 18438090817574913429)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(635959669n);
    input.add64(18438090817574913429n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint64(
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

  it('test operator "le" overload (euint32, euint64) => ebool test 2 (635959665, 635959669)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(635959665n);
    input.add64(635959669n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint64(
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

  it('test operator "le" overload (euint32, euint64) => ebool test 3 (635959669, 635959669)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(635959669n);
    input.add64(635959669n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint64(
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

  it('test operator "le" overload (euint32, euint64) => ebool test 4 (635959669, 635959665)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(635959669n);
    input.add64(635959665n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint64(
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

  it('test operator "lt" overload (euint32, euint64) => ebool test 1 (1085333703, 18441204693238996767)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1085333703n);
    input.add64(18441204693238996767n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint64(
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

  it('test operator "lt" overload (euint32, euint64) => ebool test 2 (1085333699, 1085333703)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1085333699n);
    input.add64(1085333703n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint64(
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

  it('test operator "lt" overload (euint32, euint64) => ebool test 3 (1085333703, 1085333703)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1085333703n);
    input.add64(1085333703n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint64(
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

  it('test operator "lt" overload (euint32, euint64) => ebool test 4 (1085333703, 1085333699)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1085333703n);
    input.add64(1085333699n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint64(
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

  it('test operator "min" overload (euint32, euint64) => euint64 test 1 (3589633201, 18444224495860553585)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3589633201n);
    input.add64(18444224495860553585n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3589633201n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 2 (3589633197, 3589633201)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3589633197n);
    input.add64(3589633201n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3589633197n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 3 (3589633201, 3589633201)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3589633201n);
    input.add64(3589633201n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3589633201n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 4 (3589633201, 3589633197)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3589633201n);
    input.add64(3589633197n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3589633197n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 1 (2922548230, 18445791240924949651)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2922548230n);
    input.add64(18445791240924949651n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18445791240924949651n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 2 (2922548226, 2922548230)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2922548226n);
    input.add64(2922548230n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2922548230n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 3 (2922548230, 2922548230)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2922548230n);
    input.add64(2922548230n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2922548230n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 4 (2922548230, 2922548226)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2922548230n);
    input.add64(2922548226n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2922548230n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint32, euint128) => euint128 test 1 (2, 2147483649)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2n);
    input.add128(2147483649n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2147483651n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint32, euint128) => euint128 test 2 (1779283705, 1779283709)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1779283705n);
    input.add128(1779283709n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3558567414n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint32, euint128) => euint128 test 3 (1779283709, 1779283709)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1779283709n);
    input.add128(1779283709n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3558567418n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint32, euint128) => euint128 test 4 (1779283709, 1779283705)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1779283709n);
    input.add128(1779283705n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3558567414n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint32, euint128) => euint128 test 1 (1580188494, 1580188494)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1580188494n);
    input.add128(1580188494n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint32_euint128(
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

  it('test operator "sub" overload (euint32, euint128) => euint128 test 2 (1580188494, 1580188490)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1580188494n);
    input.add128(1580188490n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instances.alice.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
