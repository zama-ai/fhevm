import { expect } from 'chai';
import { ethers } from 'hardhat';

import type { FHEVMTestSuite1 } from '../../types/contracts/tests/FHEVMTestSuite1';
import type { FHEVMTestSuite2 } from '../../types/contracts/tests/FHEVMTestSuite2';
import type { FHEVMTestSuite3 } from '../../types/contracts/tests/FHEVMTestSuite3';
import type { FHEVMTestSuite4 } from '../../types/contracts/tests/FHEVMTestSuite4';
import type { FHEVMTestSuite5 } from '../../types/contracts/tests/FHEVMTestSuite5';
import type { FHEVMTestSuite6 } from '../../types/contracts/tests/FHEVMTestSuite6';
import type { FHEVMTestSuite7 } from '../../types/contracts/tests/FHEVMTestSuite7';
import {
  createInstances,
  decrypt8,
  decrypt16,
  decrypt32,
  decrypt64,
  decrypt128,
  decrypt256,
  decryptBool,
} from '../instance';
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

describe('FHEVM operations 9', function () {
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

  it('test operator "or" overload (uint8, euint8) => euint8 test 1 (17, 200)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(200n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint8_euint8(17n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(217n);
  });

  it('test operator "or" overload (uint8, euint8) => euint8 test 2 (67, 71)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(71n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint8_euint8(67n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(71n);
  });

  it('test operator "or" overload (uint8, euint8) => euint8 test 3 (71, 71)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(71n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint8_euint8(71n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(71n);
  });

  it('test operator "or" overload (uint8, euint8) => euint8 test 4 (71, 67)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(67n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint8_euint8(71n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(71n);
  });

  it('test operator "xor" overload (euint8, uint8) => euint8 test 1 (153, 57)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(153n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint8_uint8(encryptedAmount.handles[0], 57n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(160n);
  });

  it('test operator "xor" overload (euint8, uint8) => euint8 test 2 (122, 126)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(122n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint8_uint8(encryptedAmount.handles[0], 126n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint8, uint8) => euint8 test 3 (126, 126)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(126n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint8_uint8(encryptedAmount.handles[0], 126n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, uint8) => euint8 test 4 (126, 122)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(126n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint8_uint8(encryptedAmount.handles[0], 122n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (uint8, euint8) => euint8 test 1 (219, 57)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(57n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint8_euint8(219n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(226n);
  });

  it('test operator "xor" overload (uint8, euint8) => euint8 test 2 (122, 126)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(126n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint8_euint8(122n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (uint8, euint8) => euint8 test 3 (126, 126)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(126n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint8_euint8(126n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (uint8, euint8) => euint8 test 4 (126, 122)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(122n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint8_euint8(126n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 1 (142, 205)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(142n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint8_uint8(encryptedAmount.handles[0], 205n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 2 (55, 59)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(55n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint8_uint8(encryptedAmount.handles[0], 59n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 3 (59, 59)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(59n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint8_uint8(encryptedAmount.handles[0], 59n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 4 (59, 55)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(59n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint8_uint8(encryptedAmount.handles[0], 55n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint8) => ebool test 1 (125, 205)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(205n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_uint8_euint8(125n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint8) => ebool test 2 (55, 59)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(59n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_uint8_euint8(55n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint8) => ebool test 3 (59, 59)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(59n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_uint8_euint8(59n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint8, euint8) => ebool test 4 (59, 55)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(55n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_uint8_euint8(59n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 1 (230, 228)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(230n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint8_uint8(encryptedAmount.handles[0], 228n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 2 (120, 124)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(120n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint8_uint8(encryptedAmount.handles[0], 124n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 3 (124, 124)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(124n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint8_uint8(encryptedAmount.handles[0], 124n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 4 (124, 120)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(124n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint8_uint8(encryptedAmount.handles[0], 120n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 1 (231, 228)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(228n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_uint8_euint8(231n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 2 (120, 124)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(124n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_uint8_euint8(120n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 3 (124, 124)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(124n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_uint8_euint8(124n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 4 (124, 120)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(120n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_uint8_euint8(124n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 1 (215, 29)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(215n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint8_uint8(encryptedAmount.handles[0], 29n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 2 (175, 179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(175n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint8_uint8(encryptedAmount.handles[0], 179n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 3 (179, 179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(179n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint8_uint8(encryptedAmount.handles[0], 179n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 4 (179, 175)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(179n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint8_uint8(encryptedAmount.handles[0], 175n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 1 (55, 29)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(29n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_uint8_euint8(55n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 2 (175, 179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(179n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_uint8_euint8(175n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 3 (179, 179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(179n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_uint8_euint8(179n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 4 (179, 175)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(175n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_uint8_euint8(179n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 1 (191, 158)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(191n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint8_uint8(encryptedAmount.handles[0], 158n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 2 (187, 191)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(187n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint8_uint8(encryptedAmount.handles[0], 191n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 3 (191, 191)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(191n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint8_uint8(encryptedAmount.handles[0], 191n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 4 (191, 187)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(191n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint8_uint8(encryptedAmount.handles[0], 187n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 1 (172, 158)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(158n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_uint8_euint8(172n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 2 (187, 191)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(191n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_uint8_euint8(187n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 3 (191, 191)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(191n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_uint8_euint8(191n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 4 (191, 187)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(187n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_uint8_euint8(191n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 1 (221, 202)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(221n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint8_uint8(encryptedAmount.handles[0], 202n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 2 (217, 221)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(217n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint8_uint8(encryptedAmount.handles[0], 221n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 3 (221, 221)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(221n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint8_uint8(encryptedAmount.handles[0], 221n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 4 (221, 217)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(221n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint8_uint8(encryptedAmount.handles[0], 217n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 1 (140, 202)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(202n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_uint8_euint8(140n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 2 (217, 221)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(221n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_uint8_euint8(217n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 3 (221, 221)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(221n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_uint8_euint8(221n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 4 (221, 217)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(217n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_uint8_euint8(221n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 1 (210, 77)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(210n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint8_uint8(encryptedAmount.handles[0], 77n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 2 (183, 187)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(183n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint8_uint8(encryptedAmount.handles[0], 187n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 3 (187, 187)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(187n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint8_uint8(encryptedAmount.handles[0], 187n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 4 (187, 183)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(187n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint8_uint8(encryptedAmount.handles[0], 183n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 1 (234, 77)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(77n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_uint8_euint8(234n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 2 (183, 187)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(187n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_uint8_euint8(183n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 3 (187, 187)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(187n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_uint8_euint8(187n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 4 (187, 183)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(183n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_uint8_euint8(187n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 1 (103, 240)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(103n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint8_uint8(encryptedAmount.handles[0], 240n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(103n);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 2 (30, 34)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(30n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint8_uint8(encryptedAmount.handles[0], 34n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(30n);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 3 (34, 34)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(34n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint8_uint8(encryptedAmount.handles[0], 34n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(34n);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 4 (34, 30)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(34n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint8_uint8(encryptedAmount.handles[0], 30n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(30n);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 1 (57, 240)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(240n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_uint8_euint8(57n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(57n);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 2 (30, 34)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(34n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_uint8_euint8(30n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(30n);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 3 (34, 34)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(34n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_uint8_euint8(34n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(34n);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 4 (34, 30)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(30n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_uint8_euint8(34n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(30n);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 1 (5, 115)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint8_uint8(encryptedAmount.handles[0], 115n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(115n);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(1n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint8_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(5n);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint8_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(5n);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint8_uint8(encryptedAmount.handles[0], 1n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(5n);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 1 (175, 115)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(115n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_uint8_euint8(175n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(175n);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_uint8_euint8(1n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(5n);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_uint8_euint8(5n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(5n);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_uint8_euint8(5n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(5n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 1 (9954, 40099)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(9954n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint16_uint16(encryptedAmount.handles[0], 40099n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(50053n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 2 (269, 273)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(269n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint16_uint16(encryptedAmount.handles[0], 273n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(542n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 3 (273, 273)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(273n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint16_uint16(encryptedAmount.handles[0], 273n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(546n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 4 (273, 269)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(273n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint16_uint16(encryptedAmount.handles[0], 269n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(542n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 1 (23875, 20050)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(20050n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_uint16_euint16(23875n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(43925n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 2 (269, 273)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(273n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_uint16_euint16(269n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(542n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 3 (273, 273)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(273n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_uint16_euint16(273n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(546n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 4 (273, 269)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(269n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_uint16_euint16(273n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(542n);
  });

  it('test operator "sub" overload (euint16, uint16) => euint16 test 1 (16421, 16421)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(16421n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint16_uint16(encryptedAmount.handles[0], 16421n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, uint16) => euint16 test 2 (16421, 16417)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(16421n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint16_uint16(encryptedAmount.handles[0], 16417n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint16, euint16) => euint16 test 1 (16421, 16421)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(16421n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_uint16_euint16(16421n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint16, euint16) => euint16 test 2 (16421, 16417)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(16417n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_uint16_euint16(16421n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 1 (174, 123)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(174n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint16_uint16(encryptedAmount.handles[0], 123n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(21402n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 2 (174, 174)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(174n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint16_uint16(encryptedAmount.handles[0], 174n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(30276n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 3 (174, 174)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(174n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint16_uint16(encryptedAmount.handles[0], 174n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(30276n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 4 (174, 174)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(174n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint16_uint16(encryptedAmount.handles[0], 174n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(30276n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 1 (240, 123)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(123n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_uint16_euint16(240n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(29520n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 2 (174, 174)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(174n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_uint16_euint16(174n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(30276n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 3 (174, 174)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(174n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_uint16_euint16(174n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(30276n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 4 (174, 174)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(174n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_uint16_euint16(174n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(30276n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 1 (33604, 57113)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(33604n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.div_euint16_uint16(encryptedAmount.handles[0], 57113n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 2 (4640, 4644)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(4640n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.div_euint16_uint16(encryptedAmount.handles[0], 4644n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 3 (4644, 4644)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(4644n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.div_euint16_uint16(encryptedAmount.handles[0], 4644n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 4 (4644, 4640)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(4644n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.div_euint16_uint16(encryptedAmount.handles[0], 4640n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 1 (22935, 6555)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(22935n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rem_euint16_uint16(encryptedAmount.handles[0], 6555n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(3270n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 2 (6835, 6839)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(6835n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rem_euint16_uint16(encryptedAmount.handles[0], 6839n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(6835n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 3 (6839, 6839)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(6839n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rem_euint16_uint16(encryptedAmount.handles[0], 6839n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 4 (6839, 6835)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(6839n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rem_euint16_uint16(encryptedAmount.handles[0], 6835n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "and" overload (euint16, uint16) => euint16 test 1 (29651, 14267)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(29651n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint16_uint16(encryptedAmount.handles[0], 14267n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(13203n);
  });

  it('test operator "and" overload (euint16, uint16) => euint16 test 2 (29647, 29651)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(29647n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint16_uint16(encryptedAmount.handles[0], 29651n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(29635n);
  });

  it('test operator "and" overload (euint16, uint16) => euint16 test 3 (29651, 29651)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(29651n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint16_uint16(encryptedAmount.handles[0], 29651n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(29651n);
  });

  it('test operator "and" overload (euint16, uint16) => euint16 test 4 (29651, 29647)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(29651n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint16_uint16(encryptedAmount.handles[0], 29647n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(29635n);
  });

  it('test operator "and" overload (uint16, euint16) => euint16 test 1 (22302, 14267)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(14267n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint16_euint16(22302n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(5914n);
  });

  it('test operator "and" overload (uint16, euint16) => euint16 test 2 (29647, 29651)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(29651n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint16_euint16(29647n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(29635n);
  });

  it('test operator "and" overload (uint16, euint16) => euint16 test 3 (29651, 29651)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(29651n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint16_euint16(29651n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(29651n);
  });

  it('test operator "and" overload (uint16, euint16) => euint16 test 4 (29651, 29647)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(29647n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint16_euint16(29651n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(29635n);
  });

  it('test operator "or" overload (euint16, uint16) => euint16 test 1 (520, 4511)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(520n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint16_uint16(encryptedAmount.handles[0], 4511n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(5023n);
  });

  it('test operator "or" overload (euint16, uint16) => euint16 test 2 (516, 520)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(516n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint16_uint16(encryptedAmount.handles[0], 520n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(524n);
  });

  it('test operator "or" overload (euint16, uint16) => euint16 test 3 (520, 520)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(520n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint16_uint16(encryptedAmount.handles[0], 520n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(520n);
  });

  it('test operator "or" overload (euint16, uint16) => euint16 test 4 (520, 516)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(520n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint16_uint16(encryptedAmount.handles[0], 516n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(524n);
  });

  it('test operator "or" overload (uint16, euint16) => euint16 test 1 (59339, 4511)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(4511n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint16_euint16(59339n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(63455n);
  });

  it('test operator "or" overload (uint16, euint16) => euint16 test 2 (516, 520)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(520n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint16_euint16(516n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(524n);
  });

  it('test operator "or" overload (uint16, euint16) => euint16 test 3 (520, 520)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(520n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint16_euint16(520n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(520n);
  });

  it('test operator "or" overload (uint16, euint16) => euint16 test 4 (520, 516)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(516n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint16_euint16(520n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(524n);
  });

  it('test operator "xor" overload (euint16, uint16) => euint16 test 1 (21273, 20161)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(21273n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint16_uint16(encryptedAmount.handles[0], 20161n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(7640n);
  });

  it('test operator "xor" overload (euint16, uint16) => euint16 test 2 (20950, 20954)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(20950n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint16_uint16(encryptedAmount.handles[0], 20954n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint16, uint16) => euint16 test 3 (20954, 20954)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(20954n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint16_uint16(encryptedAmount.handles[0], 20954n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, uint16) => euint16 test 4 (20954, 20950)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(20954n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint16_uint16(encryptedAmount.handles[0], 20950n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (uint16, euint16) => euint16 test 1 (45566, 20161)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(20161n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint16_euint16(45566n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(65343n);
  });

  it('test operator "xor" overload (uint16, euint16) => euint16 test 2 (20950, 20954)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(20954n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint16_euint16(20950n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (uint16, euint16) => euint16 test 3 (20954, 20954)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(20954n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint16_euint16(20954n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (uint16, euint16) => euint16 test 4 (20954, 20950)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(20950n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint16_euint16(20954n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 1 (33201, 16946)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(33201n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint16_uint16(encryptedAmount.handles[0], 16946n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 2 (3545, 3549)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(3545n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint16_uint16(encryptedAmount.handles[0], 3549n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 3 (3549, 3549)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(3549n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint16_uint16(encryptedAmount.handles[0], 3549n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 4 (3549, 3545)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(3549n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint16_uint16(encryptedAmount.handles[0], 3545n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 1 (40600, 16946)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(16946n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint16_euint16(40600n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 2 (3545, 3549)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(3549n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint16_euint16(3545n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 3 (3549, 3549)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(3549n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint16_euint16(3549n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 4 (3549, 3545)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(3545n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint16_euint16(3549n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 1 (5555, 50685)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(5555n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint16_uint16(encryptedAmount.handles[0], 50685n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 2 (5551, 5555)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(5551n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint16_uint16(encryptedAmount.handles[0], 5555n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 3 (5555, 5555)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(5555n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint16_uint16(encryptedAmount.handles[0], 5555n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 4 (5555, 5551)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(5555n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint16_uint16(encryptedAmount.handles[0], 5551n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 1 (60110, 50685)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(50685n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint16_euint16(60110n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 2 (5551, 5555)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(5555n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint16_euint16(5551n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 3 (5555, 5555)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(5555n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint16_euint16(5555n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 4 (5555, 5551)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(5551n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint16_euint16(5555n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 1 (8765, 21855)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(8765n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint16_uint16(encryptedAmount.handles[0], 21855n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 2 (8761, 8765)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(8761n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint16_uint16(encryptedAmount.handles[0], 8765n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 3 (8765, 8765)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(8765n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint16_uint16(encryptedAmount.handles[0], 8765n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 4 (8765, 8761)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(8765n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint16_uint16(encryptedAmount.handles[0], 8761n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 1 (51512, 21855)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(21855n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint16_euint16(51512n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 2 (8761, 8765)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(8765n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint16_euint16(8761n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 3 (8765, 8765)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(8765n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint16_euint16(8765n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 4 (8765, 8761)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(8761n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint16_euint16(8765n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 1 (59476, 65255)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(59476n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint16_uint16(encryptedAmount.handles[0], 65255n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 2 (27573, 27577)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(27573n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint16_uint16(encryptedAmount.handles[0], 27577n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 3 (27577, 27577)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(27577n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint16_uint16(encryptedAmount.handles[0], 27577n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 4 (27577, 27573)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(27577n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint16_uint16(encryptedAmount.handles[0], 27573n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 1 (65445, 65255)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(65255n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint16_euint16(65445n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 2 (27573, 27577)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(27577n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint16_euint16(27573n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 3 (27577, 27577)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(27577n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint16_euint16(27577n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 4 (27577, 27573)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(27573n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint16_euint16(27577n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 1 (61759, 51526)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(61759n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint16_uint16(encryptedAmount.handles[0], 51526n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 2 (22463, 22467)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(22463n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint16_uint16(encryptedAmount.handles[0], 22467n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 3 (22467, 22467)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(22467n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint16_uint16(encryptedAmount.handles[0], 22467n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 4 (22467, 22463)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(22467n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint16_uint16(encryptedAmount.handles[0], 22463n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 1 (43501, 51526)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(51526n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint16_euint16(43501n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 2 (22463, 22467)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(22467n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint16_euint16(22463n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 3 (22467, 22467)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(22467n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint16_euint16(22467n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 4 (22467, 22463)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(22463n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint16_euint16(22467n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 1 (38296, 39)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(38296n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint16_uint16(encryptedAmount.handles[0], 39n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 2 (38292, 38296)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(38292n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint16_uint16(encryptedAmount.handles[0], 38296n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 3 (38296, 38296)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(38296n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint16_uint16(encryptedAmount.handles[0], 38296n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 4 (38296, 38292)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(38296n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint16_uint16(encryptedAmount.handles[0], 38292n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 1 (13964, 39)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(39n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint16_euint16(13964n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 2 (38292, 38296)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(38296n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint16_euint16(38292n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 3 (38296, 38296)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(38296n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint16_euint16(38296n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 4 (38296, 38292)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(38292n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint16_euint16(38296n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 1 (33760, 58427)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(33760n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint16_uint16(encryptedAmount.handles[0], 58427n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(33760n);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 2 (33756, 33760)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(33756n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint16_uint16(encryptedAmount.handles[0], 33760n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(33756n);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 3 (33760, 33760)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(33760n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint16_uint16(encryptedAmount.handles[0], 33760n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(33760n);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 4 (33760, 33756)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(33760n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint16_uint16(encryptedAmount.handles[0], 33756n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(33756n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 1 (35416, 58427)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(58427n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint16_euint16(35416n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(35416n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 2 (33756, 33760)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(33760n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint16_euint16(33756n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(33756n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 3 (33760, 33760)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(33760n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint16_euint16(33760n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(33760n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 4 (33760, 33756)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(33756n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint16_euint16(33760n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(33756n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 1 (21515, 51567)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(21515n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint16_uint16(encryptedAmount.handles[0], 51567n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(51567n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 2 (18541, 18545)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(18541n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint16_uint16(encryptedAmount.handles[0], 18545n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(18545n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 3 (18545, 18545)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(18545n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint16_uint16(encryptedAmount.handles[0], 18545n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(18545n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 4 (18545, 18541)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(18545n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint16_uint16(encryptedAmount.handles[0], 18541n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(18545n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 1 (16046, 51567)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(51567n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint16_euint16(16046n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(51567n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 2 (18541, 18545)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(18545n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint16_euint16(18541n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(18545n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 3 (18545, 18545)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(18545n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint16_euint16(18545n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(18545n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 4 (18545, 18541)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(18541n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint16_euint16(18545n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(18545n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 1 (1228989590, 1389164434)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1228989590n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint32_uint32(
      encryptedAmount.handles[0],
      1389164434n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(2618154024n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 2 (1228989588, 1228989590)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1228989588n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint32_uint32(
      encryptedAmount.handles[0],
      1228989590n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(2457979178n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 3 (1228989590, 1228989590)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1228989590n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint32_uint32(
      encryptedAmount.handles[0],
      1228989590n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(2457979180n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 4 (1228989590, 1228989588)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1228989590n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint32_uint32(
      encryptedAmount.handles[0],
      1228989588n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(2457979178n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 1 (651680331, 2778328867)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(2778328867n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint32_euint32(
      651680331n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(3430009198n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 2 (1228989588, 1228989590)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1228989590n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint32_euint32(
      1228989588n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(2457979178n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 3 (1228989590, 1228989590)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1228989590n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint32_euint32(
      1228989590n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(2457979180n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 4 (1228989590, 1228989588)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1228989588n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint32_euint32(
      1228989590n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(2457979178n);
  });

  it('test operator "sub" overload (euint32, uint32) => euint32 test 1 (2722427970, 2722427970)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2722427970n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint32_uint32(
      encryptedAmount.handles[0],
      2722427970n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, uint32) => euint32 test 2 (2722427970, 2722427966)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2722427970n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint32_uint32(
      encryptedAmount.handles[0],
      2722427966n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(4n);
  });
});
