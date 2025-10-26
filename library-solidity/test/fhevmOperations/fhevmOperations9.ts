import { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/signers';
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

  it('test operator "or" overload (uint8, euint8) => euint8 test 1 (209, 130)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(130n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint8_euint8(209n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(211n);
  });

  it('test operator "or" overload (uint8, euint8) => euint8 test 2 (129, 133)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(133n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint8_euint8(129n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(133n);
  });

  it('test operator "or" overload (uint8, euint8) => euint8 test 3 (133, 133)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(133n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint8_euint8(133n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(133n);
  });

  it('test operator "or" overload (uint8, euint8) => euint8 test 4 (133, 129)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint8_euint8(133n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(133n);
  });

  it('test operator "xor" overload (euint8, uint8) => euint8 test 1 (111, 94)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(111n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint8_uint8(encryptedAmount.handles[0], 94n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(49n);
  });

  it('test operator "xor" overload (euint8, uint8) => euint8 test 2 (14, 18)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(14n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint8_uint8(encryptedAmount.handles[0], 18n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(28n);
  });

  it('test operator "xor" overload (euint8, uint8) => euint8 test 3 (18, 18)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(18n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint8_uint8(encryptedAmount.handles[0], 18n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, uint8) => euint8 test 4 (18, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(18n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint8_uint8(encryptedAmount.handles[0], 14n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(28n);
  });

  it('test operator "xor" overload (uint8, euint8) => euint8 test 1 (38, 94)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(94n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint8_euint8(38n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(120n);
  });

  it('test operator "xor" overload (uint8, euint8) => euint8 test 2 (14, 18)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(18n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint8_euint8(14n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(28n);
  });

  it('test operator "xor" overload (uint8, euint8) => euint8 test 3 (18, 18)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(18n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint8_euint8(18n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (uint8, euint8) => euint8 test 4 (18, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint8_euint8(18n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(28n);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 1 (208, 211)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(208n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint8_uint8(encryptedAmount.handles[0], 211n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 2 (183, 187)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(183n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint8_uint8(encryptedAmount.handles[0], 187n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 3 (187, 187)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(187n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint8_uint8(encryptedAmount.handles[0], 187n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 4 (187, 183)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(187n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint8_uint8(encryptedAmount.handles[0], 183n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint8) => ebool test 1 (245, 211)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(211n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_uint8_euint8(245n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint8) => ebool test 2 (183, 187)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(187n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_uint8_euint8(183n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint8) => ebool test 3 (187, 187)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(187n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_uint8_euint8(187n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint8, euint8) => ebool test 4 (187, 183)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(183n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_uint8_euint8(187n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 1 (103, 136)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(103n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint8_uint8(encryptedAmount.handles[0], 136n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 2 (99, 103)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(99n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint8_uint8(encryptedAmount.handles[0], 103n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 3 (103, 103)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(103n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint8_uint8(encryptedAmount.handles[0], 103n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 4 (103, 99)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(103n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint8_uint8(encryptedAmount.handles[0], 99n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 1 (198, 136)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(136n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_uint8_euint8(198n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 2 (99, 103)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(103n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_uint8_euint8(99n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 3 (103, 103)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(103n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_uint8_euint8(103n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 4 (103, 99)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(99n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_uint8_euint8(103n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 1 (80, 144)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(80n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint8_uint8(encryptedAmount.handles[0], 144n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 2 (34, 38)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(34n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint8_uint8(encryptedAmount.handles[0], 38n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 3 (38, 38)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(38n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint8_uint8(encryptedAmount.handles[0], 38n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 4 (38, 34)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(38n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint8_uint8(encryptedAmount.handles[0], 34n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 1 (19, 144)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(144n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_uint8_euint8(19n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 2 (34, 38)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(38n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_uint8_euint8(34n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 3 (38, 38)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(38n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_uint8_euint8(38n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 4 (38, 34)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(34n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_uint8_euint8(38n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 1 (185, 188)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(185n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint8_uint8(encryptedAmount.handles[0], 188n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 2 (163, 167)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(163n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint8_uint8(encryptedAmount.handles[0], 167n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 3 (167, 167)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(167n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint8_uint8(encryptedAmount.handles[0], 167n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 4 (167, 163)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(167n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint8_uint8(encryptedAmount.handles[0], 163n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 1 (153, 188)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(188n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_uint8_euint8(153n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 2 (163, 167)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(167n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_uint8_euint8(163n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 3 (167, 167)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(167n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_uint8_euint8(167n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 4 (167, 163)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(163n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_uint8_euint8(167n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 1 (42, 227)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(42n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint8_uint8(encryptedAmount.handles[0], 227n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 2 (38, 42)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(38n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint8_uint8(encryptedAmount.handles[0], 42n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 3 (42, 42)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(42n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint8_uint8(encryptedAmount.handles[0], 42n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 4 (42, 38)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(42n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint8_uint8(encryptedAmount.handles[0], 38n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 1 (86, 227)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(227n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_uint8_euint8(86n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 2 (38, 42)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(42n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_uint8_euint8(38n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 3 (42, 42)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(42n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_uint8_euint8(42n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 4 (42, 38)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(38n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_uint8_euint8(42n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 1 (53, 190)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(53n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint8_uint8(encryptedAmount.handles[0], 190n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 2 (49, 53)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(49n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint8_uint8(encryptedAmount.handles[0], 53n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 3 (53, 53)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(53n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint8_uint8(encryptedAmount.handles[0], 53n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 4 (53, 49)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(53n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint8_uint8(encryptedAmount.handles[0], 49n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 1 (73, 190)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(190n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_uint8_euint8(73n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 2 (49, 53)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(53n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_uint8_euint8(49n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 3 (53, 53)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(53n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_uint8_euint8(53n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 4 (53, 49)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(49n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_uint8_euint8(53n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 1 (6, 144)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint8_uint8(encryptedAmount.handles[0], 144n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(6n);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 2 (2, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(2n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint8_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(2n);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 3 (6, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint8_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(6n);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 4 (6, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint8_uint8(encryptedAmount.handles[0], 2n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(2n);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 1 (71, 144)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(144n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_uint8_euint8(71n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(71n);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 2 (2, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_uint8_euint8(2n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(2n);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 3 (6, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_uint8_euint8(6n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(6n);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 4 (6, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_uint8_euint8(6n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(2n);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 1 (206, 223)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(206n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint8_uint8(encryptedAmount.handles[0], 223n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(223n);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 2 (186, 190)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(186n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint8_uint8(encryptedAmount.handles[0], 190n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(190n);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 3 (190, 190)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(190n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint8_uint8(encryptedAmount.handles[0], 190n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(190n);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 4 (190, 186)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(190n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint8_uint8(encryptedAmount.handles[0], 186n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(190n);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 1 (107, 223)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(223n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_uint8_euint8(107n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(223n);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 2 (186, 190)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(190n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_uint8_euint8(186n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(190n);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 3 (190, 190)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(190n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_uint8_euint8(190n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(190n);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 4 (190, 186)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(186n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_uint8_euint8(190n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(190n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 1 (21898, 14420)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(21898n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint16_uint16(encryptedAmount.handles[0], 14420n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(36318n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 2 (16860, 16862)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(16860n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint16_uint16(encryptedAmount.handles[0], 16862n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(33722n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 3 (16862, 16862)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(16862n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint16_uint16(encryptedAmount.handles[0], 16862n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(33724n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 4 (16862, 16860)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(16862n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint16_uint16(encryptedAmount.handles[0], 16860n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(33722n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 1 (15773, 28839)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(28839n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_uint16_euint16(15773n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(44612n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 2 (16860, 16862)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(16862n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_uint16_euint16(16860n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(33722n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 3 (16862, 16862)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(16862n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_uint16_euint16(16862n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(33724n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 4 (16862, 16860)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(16860n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_uint16_euint16(16862n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(33722n);
  });

  it('test operator "sub" overload (euint16, uint16) => euint16 test 1 (6587, 6587)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(6587n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint16_uint16(encryptedAmount.handles[0], 6587n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, uint16) => euint16 test 2 (6587, 6583)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(6587n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint16_uint16(encryptedAmount.handles[0], 6583n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint16, euint16) => euint16 test 1 (6587, 6587)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(6587n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_uint16_euint16(6587n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint16, euint16) => euint16 test 2 (6587, 6583)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(6583n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_uint16_euint16(6587n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 1 (221, 123)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(221n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint16_uint16(encryptedAmount.handles[0], 123n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(27183n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 2 (221, 221)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(221n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint16_uint16(encryptedAmount.handles[0], 221n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(48841n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 3 (221, 221)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(221n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint16_uint16(encryptedAmount.handles[0], 221n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(48841n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 4 (221, 221)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(221n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint16_uint16(encryptedAmount.handles[0], 221n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(48841n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 1 (171, 123)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(123n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_uint16_euint16(171n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(21033n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 2 (221, 221)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(221n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_uint16_euint16(221n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(48841n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 3 (221, 221)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(221n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_uint16_euint16(221n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(48841n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 4 (221, 221)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(221n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_uint16_euint16(221n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(48841n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 1 (45300, 54300)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(45300n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.div_euint16_uint16(encryptedAmount.handles[0], 54300n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 2 (36995, 36999)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(36995n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.div_euint16_uint16(encryptedAmount.handles[0], 36999n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 3 (36999, 36999)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(36999n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.div_euint16_uint16(encryptedAmount.handles[0], 36999n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 4 (36999, 36995)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(36999n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.div_euint16_uint16(encryptedAmount.handles[0], 36995n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 1 (14561, 38954)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(14561n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rem_euint16_uint16(encryptedAmount.handles[0], 38954n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(14561n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 2 (14294, 14298)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(14294n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rem_euint16_uint16(encryptedAmount.handles[0], 14298n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(14294n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 3 (14298, 14298)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(14298n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rem_euint16_uint16(encryptedAmount.handles[0], 14298n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 4 (14298, 14294)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(14298n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rem_euint16_uint16(encryptedAmount.handles[0], 14294n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "and" overload (euint16, uint16) => euint16 test 1 (36796, 19333)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(36796n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint16_uint16(encryptedAmount.handles[0], 19333n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(2948n);
  });

  it('test operator "and" overload (euint16, uint16) => euint16 test 2 (21461, 21465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(21461n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint16_uint16(encryptedAmount.handles[0], 21465n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(21457n);
  });

  it('test operator "and" overload (euint16, uint16) => euint16 test 3 (21465, 21465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(21465n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint16_uint16(encryptedAmount.handles[0], 21465n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(21465n);
  });

  it('test operator "and" overload (euint16, uint16) => euint16 test 4 (21465, 21461)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(21465n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint16_uint16(encryptedAmount.handles[0], 21461n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(21457n);
  });

  it('test operator "and" overload (uint16, euint16) => euint16 test 1 (22481, 19333)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(19333n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint16_euint16(22481n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(17281n);
  });

  it('test operator "and" overload (uint16, euint16) => euint16 test 2 (21461, 21465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(21465n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint16_euint16(21461n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(21457n);
  });

  it('test operator "and" overload (uint16, euint16) => euint16 test 3 (21465, 21465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(21465n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint16_euint16(21465n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(21465n);
  });

  it('test operator "and" overload (uint16, euint16) => euint16 test 4 (21465, 21461)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(21461n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint16_euint16(21465n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(21457n);
  });

  it('test operator "or" overload (euint16, uint16) => euint16 test 1 (13841, 28921)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(13841n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint16_uint16(encryptedAmount.handles[0], 28921n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(30457n);
  });

  it('test operator "or" overload (euint16, uint16) => euint16 test 2 (12868, 12872)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(12868n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint16_uint16(encryptedAmount.handles[0], 12872n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(12876n);
  });

  it('test operator "or" overload (euint16, uint16) => euint16 test 3 (12872, 12872)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(12872n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint16_uint16(encryptedAmount.handles[0], 12872n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(12872n);
  });

  it('test operator "or" overload (euint16, uint16) => euint16 test 4 (12872, 12868)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(12872n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint16_uint16(encryptedAmount.handles[0], 12868n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(12876n);
  });

  it('test operator "or" overload (uint16, euint16) => euint16 test 1 (52479, 28921)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(28921n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint16_euint16(52479n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(64767n);
  });

  it('test operator "or" overload (uint16, euint16) => euint16 test 2 (12868, 12872)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(12872n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint16_euint16(12868n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(12876n);
  });

  it('test operator "or" overload (uint16, euint16) => euint16 test 3 (12872, 12872)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(12872n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint16_euint16(12872n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(12872n);
  });

  it('test operator "or" overload (uint16, euint16) => euint16 test 4 (12872, 12868)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(12868n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint16_euint16(12872n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(12876n);
  });

  it('test operator "xor" overload (euint16, uint16) => euint16 test 1 (5643, 26664)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(5643n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint16_uint16(encryptedAmount.handles[0], 26664n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(32291n);
  });

  it('test operator "xor" overload (euint16, uint16) => euint16 test 2 (5639, 5643)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(5639n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint16_uint16(encryptedAmount.handles[0], 5643n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint16, uint16) => euint16 test 3 (5643, 5643)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(5643n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint16_uint16(encryptedAmount.handles[0], 5643n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, uint16) => euint16 test 4 (5643, 5639)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(5643n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint16_uint16(encryptedAmount.handles[0], 5639n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (uint16, euint16) => euint16 test 1 (19953, 26664)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(26664n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint16_euint16(19953n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(9689n);
  });

  it('test operator "xor" overload (uint16, euint16) => euint16 test 2 (5639, 5643)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(5643n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint16_euint16(5639n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (uint16, euint16) => euint16 test 3 (5643, 5643)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(5643n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint16_euint16(5643n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (uint16, euint16) => euint16 test 4 (5643, 5639)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(5639n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint16_euint16(5643n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract5.resEuint16());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 1 (22634, 4381)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(22634n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint16_uint16(encryptedAmount.handles[0], 4381n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 2 (22630, 22634)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(22630n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint16_uint16(encryptedAmount.handles[0], 22634n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 3 (22634, 22634)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(22634n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint16_uint16(encryptedAmount.handles[0], 22634n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 4 (22634, 22630)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(22634n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint16_uint16(encryptedAmount.handles[0], 22630n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 1 (2316, 4381)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(4381n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint16_euint16(2316n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 2 (22630, 22634)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(22634n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint16_euint16(22630n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 3 (22634, 22634)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(22634n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint16_euint16(22634n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 4 (22634, 22630)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(22630n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint16_euint16(22634n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 1 (50588, 23196)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(50588n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint16_uint16(encryptedAmount.handles[0], 23196n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 2 (37588, 37592)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(37588n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint16_uint16(encryptedAmount.handles[0], 37592n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 3 (37592, 37592)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(37592n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint16_uint16(encryptedAmount.handles[0], 37592n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 4 (37592, 37588)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(37592n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint16_uint16(encryptedAmount.handles[0], 37588n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 1 (35011, 23196)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(23196n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint16_euint16(35011n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 2 (37588, 37592)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(37592n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint16_euint16(37588n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 3 (37592, 37592)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(37592n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint16_euint16(37592n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 4 (37592, 37588)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(37588n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint16_euint16(37592n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 1 (64742, 26823)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(64742n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint16_uint16(encryptedAmount.handles[0], 26823n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 2 (23124, 23128)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(23124n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint16_uint16(encryptedAmount.handles[0], 23128n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 3 (23128, 23128)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(23128n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint16_uint16(encryptedAmount.handles[0], 23128n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 4 (23128, 23124)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(23128n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint16_uint16(encryptedAmount.handles[0], 23124n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 1 (59518, 26823)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(26823n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint16_euint16(59518n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 2 (23124, 23128)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(23128n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint16_euint16(23124n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 3 (23128, 23128)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(23128n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint16_euint16(23128n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 4 (23128, 23124)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(23124n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint16_euint16(23128n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 1 (16063, 6099)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(16063n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint16_uint16(encryptedAmount.handles[0], 6099n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 2 (16059, 16063)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(16059n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint16_uint16(encryptedAmount.handles[0], 16063n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 3 (16063, 16063)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(16063n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint16_uint16(encryptedAmount.handles[0], 16063n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 4 (16063, 16059)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(16063n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint16_uint16(encryptedAmount.handles[0], 16059n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 1 (25243, 6099)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(6099n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint16_euint16(25243n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 2 (16059, 16063)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(16063n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint16_euint16(16059n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 3 (16063, 16063)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(16063n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint16_euint16(16063n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 4 (16063, 16059)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(16059n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint16_euint16(16063n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 1 (39480, 46429)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(39480n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint16_uint16(encryptedAmount.handles[0], 46429n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 2 (18260, 18264)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(18260n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint16_uint16(encryptedAmount.handles[0], 18264n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 3 (18264, 18264)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(18264n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint16_uint16(encryptedAmount.handles[0], 18264n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 4 (18264, 18260)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(18264n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint16_uint16(encryptedAmount.handles[0], 18260n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 1 (35570, 46429)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(46429n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint16_euint16(35570n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 2 (18260, 18264)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(18264n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint16_euint16(18260n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 3 (18264, 18264)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(18264n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint16_euint16(18264n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 4 (18264, 18260)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(18260n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint16_euint16(18264n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 1 (40322, 33879)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(40322n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint16_uint16(encryptedAmount.handles[0], 33879n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 2 (23104, 23108)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(23104n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint16_uint16(encryptedAmount.handles[0], 23108n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 3 (23108, 23108)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(23108n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint16_uint16(encryptedAmount.handles[0], 23108n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 4 (23108, 23104)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(23108n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint16_uint16(encryptedAmount.handles[0], 23104n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 1 (58949, 33879)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(33879n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint16_euint16(58949n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 2 (23104, 23108)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(23108n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint16_euint16(23104n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 3 (23108, 23108)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(23108n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint16_euint16(23108n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 4 (23108, 23104)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(23104n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint16_euint16(23108n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 1 (4164, 3878)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(4164n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint16_uint16(encryptedAmount.handles[0], 3878n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(3878n);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 2 (4160, 4164)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(4160n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint16_uint16(encryptedAmount.handles[0], 4164n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(4160n);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 3 (4164, 4164)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(4164n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint16_uint16(encryptedAmount.handles[0], 4164n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(4164n);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 4 (4164, 4160)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(4164n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint16_uint16(encryptedAmount.handles[0], 4160n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(4160n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 1 (50876, 3878)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(3878n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint16_euint16(50876n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(3878n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 2 (4160, 4164)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(4164n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint16_euint16(4160n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(4160n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 3 (4164, 4164)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(4164n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint16_euint16(4164n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(4164n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 4 (4164, 4160)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(4160n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint16_euint16(4164n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(4160n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 1 (56758, 31723)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(56758n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint16_uint16(encryptedAmount.handles[0], 31723n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(56758n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 2 (51759, 51763)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(51759n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint16_uint16(encryptedAmount.handles[0], 51763n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(51763n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 3 (51763, 51763)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(51763n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint16_uint16(encryptedAmount.handles[0], 51763n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(51763n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 4 (51763, 51759)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(51763n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint16_uint16(encryptedAmount.handles[0], 51759n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(51763n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 1 (44453, 31723)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(31723n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint16_euint16(44453n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(44453n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 2 (51759, 51763)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(51763n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint16_euint16(51759n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(51763n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 3 (51763, 51763)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(51763n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint16_euint16(51763n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(51763n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 4 (51763, 51759)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(51759n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint16_euint16(51763n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract6.resEuint16());
    expect(res).to.equal(51763n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 1 (980875616, 1189789850)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(980875616n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint32_uint32(
      encryptedAmount.handles[0],
      1189789850n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(2170665466n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 2 (1614966264, 1614966268)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1614966264n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint32_uint32(
      encryptedAmount.handles[0],
      1614966268n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(3229932532n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 3 (1614966268, 1614966268)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1614966268n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint32_uint32(
      encryptedAmount.handles[0],
      1614966268n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(3229932536n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 4 (1614966268, 1614966264)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1614966268n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint32_uint32(
      encryptedAmount.handles[0],
      1614966264n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(3229932532n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 1 (968757152, 1189789850)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1189789850n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint32_euint32(
      968757152n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(2158547002n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 2 (1614966264, 1614966268)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1614966268n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint32_euint32(
      1614966264n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(3229932532n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 3 (1614966268, 1614966268)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1614966268n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint32_euint32(
      1614966268n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(3229932536n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 4 (1614966268, 1614966264)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1614966264n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint32_euint32(
      1614966268n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(3229932532n);
  });

  it('test operator "sub" overload (euint32, uint32) => euint32 test 1 (132238280, 132238280)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(132238280n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint32_uint32(
      encryptedAmount.handles[0],
      132238280n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, uint32) => euint32 test 2 (132238280, 132238276)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(132238280n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint32_uint32(
      encryptedAmount.handles[0],
      132238276n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(4n);
  });
});
