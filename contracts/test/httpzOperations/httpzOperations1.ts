import { expect } from 'chai';
import { ethers } from 'hardhat';

import type { HTTPZTestSuite1 } from '../../types/contracts/tests/HTTPZTestSuite1';
import type { HTTPZTestSuite2 } from '../../types/contracts/tests/HTTPZTestSuite2';
import type { HTTPZTestSuite3 } from '../../types/contracts/tests/HTTPZTestSuite3';
import type { HTTPZTestSuite4 } from '../../types/contracts/tests/HTTPZTestSuite4';
import type { HTTPZTestSuite5 } from '../../types/contracts/tests/HTTPZTestSuite5';
import type { HTTPZTestSuite6 } from '../../types/contracts/tests/HTTPZTestSuite6';
import type { HTTPZTestSuite7 } from '../../types/contracts/tests/HTTPZTestSuite7';
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

async function deployHTTPZTestFixture1(): Promise<HTTPZTestSuite1> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('HTTPZTestSuite1');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployHTTPZTestFixture2(): Promise<HTTPZTestSuite2> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('HTTPZTestSuite2');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployHTTPZTestFixture3(): Promise<HTTPZTestSuite3> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('HTTPZTestSuite3');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployHTTPZTestFixture4(): Promise<HTTPZTestSuite4> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('HTTPZTestSuite4');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployHTTPZTestFixture5(): Promise<HTTPZTestSuite5> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('HTTPZTestSuite5');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployHTTPZTestFixture6(): Promise<HTTPZTestSuite6> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('HTTPZTestSuite6');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployHTTPZTestFixture7(): Promise<HTTPZTestSuite7> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('HTTPZTestSuite7');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

describe('HTTPZ operations 1', function () {
  before(async function () {
    await initSigners(1);
    this.signers = await getSigners();

    const contract1 = await deployHTTPZTestFixture1();
    this.contract1Address = await contract1.getAddress();
    this.contract1 = contract1;

    const contract2 = await deployHTTPZTestFixture2();
    this.contract2Address = await contract2.getAddress();
    this.contract2 = contract2;

    const contract3 = await deployHTTPZTestFixture3();
    this.contract3Address = await contract3.getAddress();
    this.contract3 = contract3;

    const contract4 = await deployHTTPZTestFixture4();
    this.contract4Address = await contract4.getAddress();
    this.contract4 = contract4;

    const contract5 = await deployHTTPZTestFixture5();
    this.contract5Address = await contract5.getAddress();
    this.contract5 = contract5;

    const contract6 = await deployHTTPZTestFixture6();
    this.contract6Address = await contract6.getAddress();
    this.contract6 = contract6;

    const contract7 = await deployHTTPZTestFixture7();
    this.contract7Address = await contract7.getAddress();
    this.contract7 = contract7;

    const instances = await createInstances(this.signers);
    this.instances = instances;
  });

  it('test operator "add" overload (euint8, euint8) => euint8 test 1 (80, 61)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(80n);
    input.add8(61n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.resEuint8());
    expect(res).to.equal(141n);
  });

  it('test operator "add" overload (euint8, euint8) => euint8 test 2 (117, 121)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(117n);
    input.add8(121n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.resEuint8());
    expect(res).to.equal(238n);
  });

  it('test operator "add" overload (euint8, euint8) => euint8 test 3 (121, 121)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(121n);
    input.add8(121n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.resEuint8());
    expect(res).to.equal(242n);
  });

  it('test operator "add" overload (euint8, euint8) => euint8 test 4 (121, 117)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(121n);
    input.add8(117n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.resEuint8());
    expect(res).to.equal(238n);
  });

  it('test operator "sub" overload (euint8, euint8) => euint8 test 1 (58, 58)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(58n);
    input.add8(58n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.resEuint8());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, euint8) => euint8 test 2 (58, 54)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(58n);
    input.add8(54n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.resEuint8());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 1 (11, 22)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(11n);
    input.add8(22n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.resEuint8());
    expect(res).to.equal(242n);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 2 (10, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(10n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.resEuint8());
    expect(res).to.equal(110n);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 3 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(11n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.resEuint8());
    expect(res).to.equal(121n);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 4 (11, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(11n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.resEuint8());
    expect(res).to.equal(110n);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 1 (112, 244)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(112n);
    input.add8(244n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.resEuint8());
    expect(res).to.equal(112n);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 2 (108, 112)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(108n);
    input.add8(112n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.resEuint8());
    expect(res).to.equal(96n);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 3 (112, 112)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(112n);
    input.add8(112n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.resEuint8());
    expect(res).to.equal(112n);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 4 (112, 108)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(112n);
    input.add8(108n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.resEuint8());
    expect(res).to.equal(96n);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 1 (244, 114)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(244n);
    input.add8(114n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.resEuint8());
    expect(res).to.equal(246n);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 2 (110, 114)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(110n);
    input.add8(114n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.resEuint8());
    expect(res).to.equal(126n);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 3 (114, 114)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(114n);
    input.add8(114n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.resEuint8());
    expect(res).to.equal(114n);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 4 (114, 110)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(114n);
    input.add8(110n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.resEuint8());
    expect(res).to.equal(126n);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 1 (63, 161)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(63n);
    input.add8(161n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.resEuint8());
    expect(res).to.equal(158n);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 2 (59, 63)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(59n);
    input.add8(63n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.resEuint8());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 3 (63, 63)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(63n);
    input.add8(63n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.resEuint8());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 4 (63, 59)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(63n);
    input.add8(59n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.resEuint8());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint8, euint8) => ebool test 1 (143, 90)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(143n);
    input.add8(90n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint8) => ebool test 2 (86, 90)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(86n);
    input.add8(90n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint8) => ebool test 3 (90, 90)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(90n);
    input.add8(90n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint8) => ebool test 4 (90, 86)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(90n);
    input.add8(86n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint8) => ebool test 1 (140, 239)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(140n);
    input.add8(239n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint8) => ebool test 2 (136, 140)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(136n);
    input.add8(140n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint8) => ebool test 3 (140, 140)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(140n);
    input.add8(140n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint8) => ebool test 4 (140, 136)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(140n);
    input.add8(136n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint8) => ebool test 1 (6, 250)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(6n);
    input.add8(250n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint8) => ebool test 2 (2, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(2n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint8) => ebool test 3 (6, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(6n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint8) => ebool test 4 (6, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(6n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint8) => ebool test 1 (41, 144)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(41n);
    input.add8(144n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint8) => ebool test 2 (37, 41)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(37n);
    input.add8(41n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint8) => ebool test 3 (41, 41)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(41n);
    input.add8(41n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint8) => ebool test 4 (41, 37)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(41n);
    input.add8(37n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint8) => ebool test 1 (136, 179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(136n);
    input.add8(179n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint8) => ebool test 2 (132, 136)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(132n);
    input.add8(136n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint8) => ebool test 3 (136, 136)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(136n);
    input.add8(136n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint8) => ebool test 4 (136, 132)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(136n);
    input.add8(132n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint8) => ebool test 1 (177, 168)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(177n);
    input.add8(168n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint8) => ebool test 2 (164, 168)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(164n);
    input.add8(168n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint8) => ebool test 3 (168, 168)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(168n);
    input.add8(168n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint8) => ebool test 4 (168, 164)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(168n);
    input.add8(164n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 1 (5, 73)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(5n);
    input.add8(73n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.resEuint8());
    expect(res).to.equal(5n);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(1n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.resEuint8());
    expect(res).to.equal(1n);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(5n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.resEuint8());
    expect(res).to.equal(5n);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(5n);
    input.add8(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.resEuint8());
    expect(res).to.equal(1n);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 1 (249, 182)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(249n);
    input.add8(182n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.resEuint8());
    expect(res).to.equal(249n);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 2 (178, 182)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(178n);
    input.add8(182n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.resEuint8());
    expect(res).to.equal(182n);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 3 (182, 182)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(182n);
    input.add8(182n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.resEuint8());
    expect(res).to.equal(182n);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 4 (182, 178)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(182n);
    input.add8(178n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.resEuint8());
    expect(res).to.equal(182n);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 1 (2, 143)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(2n);
    input.add16(143n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(145n);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 2 (40, 44)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(40n);
    input.add16(44n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(84n);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 3 (44, 44)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(44n);
    input.add16(44n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(88n);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 4 (44, 40)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(44n);
    input.add16(40n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(84n);
  });

  it('test operator "sub" overload (euint8, euint16) => euint16 test 1 (107, 107)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(107n);
    input.add16(107n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, euint16) => euint16 test 2 (107, 103)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(107n);
    input.add16(103n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 1 (3, 58)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(3n);
    input.add16(58n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(174n);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 2 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(9n);
    input.add16(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(81n);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(9n);
    input.add16(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(81n);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 4 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(9n);
    input.add16(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(81n);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 1 (58, 17624)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(58n);
    input.add16(17624n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(24n);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 2 (54, 58)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(54n);
    input.add16(58n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(50n);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 3 (58, 58)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(58n);
    input.add16(58n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(58n);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 4 (58, 54)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(58n);
    input.add16(54n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(50n);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 1 (21, 62721)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(21n);
    input.add16(62721n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(62741n);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 2 (17, 21)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(17n);
    input.add16(21n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(21n);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 3 (21, 21)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(21n);
    input.add16(21n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(21n);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 4 (21, 17)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(21n);
    input.add16(17n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(21n);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 1 (242, 45052)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(242n);
    input.add16(45052n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(44814n);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 2 (238, 242)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(238n);
    input.add16(242n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(28n);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 3 (242, 242)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(242n);
    input.add16(242n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 4 (242, 238)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(242n);
    input.add16(238n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(28n);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 1 (123, 58030)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(123n);
    input.add16(58030n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 2 (119, 123)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(119n);
    input.add16(123n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 3 (123, 123)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(123n);
    input.add16(123n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 4 (123, 119)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(123n);
    input.add16(119n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 1 (20, 13572)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(20n);
    input.add16(13572n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 2 (16, 20)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(16n);
    input.add16(20n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 3 (20, 20)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(20n);
    input.add16(20n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 4 (20, 16)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(20n);
    input.add16(16n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 1 (212, 4744)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(212n);
    input.add16(4744n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 2 (208, 212)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(208n);
    input.add16(212n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 3 (212, 212)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(212n);
    input.add16(212n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 4 (212, 208)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(212n);
    input.add16(208n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 1 (204, 60999)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(204n);
    input.add16(60999n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 2 (200, 204)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(200n);
    input.add16(204n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 3 (204, 204)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(204n);
    input.add16(204n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 4 (204, 200)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(204n);
    input.add16(200n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 1 (68, 28953)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(68n);
    input.add16(28953n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 2 (64, 68)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(64n);
    input.add16(68n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 3 (68, 68)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(68n);
    input.add16(68n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 4 (68, 64)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(68n);
    input.add16(64n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 1 (123, 3825)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(123n);
    input.add16(3825n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 2 (119, 123)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(119n);
    input.add16(123n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 3 (123, 123)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(123n);
    input.add16(123n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 4 (123, 119)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(123n);
    input.add16(119n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 1 (250, 45165)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(250n);
    input.add16(45165n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(250n);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 2 (246, 250)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(246n);
    input.add16(250n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(246n);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 3 (250, 250)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(250n);
    input.add16(250n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(250n);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 4 (250, 246)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(250n);
    input.add16(246n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(246n);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 1 (247, 10102)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(247n);
    input.add16(10102n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(10102n);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 2 (243, 247)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(243n);
    input.add16(247n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(247n);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 3 (247, 247)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(247n);
    input.add16(247n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(247n);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 4 (247, 243)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(247n);
    input.add16(243n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.resEuint16());
    expect(res).to.equal(247n);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 1 (2, 233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(2n);
    input.add32(233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.resEuint32());
    expect(res).to.equal(235n);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 2 (121, 123)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(121n);
    input.add32(123n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.resEuint32());
    expect(res).to.equal(244n);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 3 (123, 123)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(123n);
    input.add32(123n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.resEuint32());
    expect(res).to.equal(246n);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 4 (123, 121)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(123n);
    input.add32(121n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.resEuint32());
    expect(res).to.equal(244n);
  });

  it('test operator "sub" overload (euint8, euint32) => euint32 test 1 (224, 224)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(224n);
    input.add32(224n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, euint32) => euint32 test 2 (224, 220)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(224n);
    input.add32(220n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.resEuint32());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 1 (2, 67)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(2n);
    input.add32(67n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.resEuint32());
    expect(res).to.equal(134n);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 2 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(9n);
    input.add32(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.resEuint32());
    expect(res).to.equal(81n);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(9n);
    input.add32(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.resEuint32());
    expect(res).to.equal(81n);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 4 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(9n);
    input.add32(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.resEuint32());
    expect(res).to.equal(81n);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 1 (223, 1939384274)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(223n);
    input.add32(1939384274n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.resEuint32());
    expect(res).to.equal(210n);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 2 (219, 223)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(219n);
    input.add32(223n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.resEuint32());
    expect(res).to.equal(219n);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 3 (223, 223)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(223n);
    input.add32(223n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.resEuint32());
    expect(res).to.equal(223n);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 4 (223, 219)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(223n);
    input.add32(219n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.resEuint32());
    expect(res).to.equal(219n);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 1 (116, 1474869154)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(116n);
    input.add32(1474869154n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.resEuint32());
    expect(res).to.equal(1474869238n);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 2 (112, 116)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(112n);
    input.add32(116n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.resEuint32());
    expect(res).to.equal(116n);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 3 (116, 116)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(116n);
    input.add32(116n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.resEuint32());
    expect(res).to.equal(116n);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 4 (116, 112)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(116n);
    input.add32(112n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.resEuint32());
    expect(res).to.equal(116n);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 1 (136, 2874647321)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(136n);
    input.add32(2874647321n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.resEuint32());
    expect(res).to.equal(2874647441n);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 2 (132, 136)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(132n);
    input.add32(136n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.resEuint32());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 3 (136, 136)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(136n);
    input.add32(136n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 4 (136, 132)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(136n);
    input.add32(132n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.resEuint32());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 1 (10, 3748083816)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(10n);
    input.add32(3748083816n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 2 (6, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(6n);
    input.add32(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(10n);
    input.add32(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 4 (10, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(10n);
    input.add32(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 1 (47, 227190832)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(47n);
    input.add32(227190832n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 2 (43, 47)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(43n);
    input.add32(47n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 3 (47, 47)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(47n);
    input.add32(47n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 4 (47, 43)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(47n);
    input.add32(43n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 1 (163, 4218482325)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(163n);
    input.add32(4218482325n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 2 (159, 163)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(159n);
    input.add32(163n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 3 (163, 163)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(163n);
    input.add32(163n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 4 (163, 159)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(163n);
    input.add32(159n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 1 (27, 3492747848)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(27n);
    input.add32(3492747848n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 2 (23, 27)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(23n);
    input.add32(27n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 3 (27, 27)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(27n);
    input.add32(27n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 4 (27, 23)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(27n);
    input.add32(23n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 1 (177, 1867522397)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(177n);
    input.add32(1867522397n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 2 (173, 177)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(173n);
    input.add32(177n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 3 (177, 177)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(177n);
    input.add32(177n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 4 (177, 173)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(177n);
    input.add32(173n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 1 (157, 2698749031)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(157n);
    input.add32(2698749031n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 2 (153, 157)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(153n);
    input.add32(157n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 3 (157, 157)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(157n);
    input.add32(157n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 4 (157, 153)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(157n);
    input.add32(153n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 1 (42, 674467172)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(42n);
    input.add32(674467172n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.resEuint32());
    expect(res).to.equal(42n);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 2 (38, 42)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(38n);
    input.add32(42n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.resEuint32());
    expect(res).to.equal(38n);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 3 (42, 42)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(42n);
    input.add32(42n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.resEuint32());
    expect(res).to.equal(42n);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 4 (42, 38)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(42n);
    input.add32(38n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.resEuint32());
    expect(res).to.equal(38n);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 1 (228, 3112442613)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(228n);
    input.add32(3112442613n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.resEuint32());
    expect(res).to.equal(3112442613n);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 2 (224, 228)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(224n);
    input.add32(228n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.resEuint32());
    expect(res).to.equal(228n);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 3 (228, 228)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(228n);
    input.add32(228n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.resEuint32());
    expect(res).to.equal(228n);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 4 (228, 224)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(228n);
    input.add32(224n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.resEuint32());
    expect(res).to.equal(228n);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 1 (2, 129)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(2n);
    input.add64(129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.resEuint64());
    expect(res).to.equal(131n);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 2 (67, 69)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(67n);
    input.add64(69n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.resEuint64());
    expect(res).to.equal(136n);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 3 (69, 69)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(69n);
    input.add64(69n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.resEuint64());
    expect(res).to.equal(138n);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 4 (69, 67)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(69n);
    input.add64(67n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.resEuint64());
    expect(res).to.equal(136n);
  });

  it('test operator "sub" overload (euint8, euint64) => euint64 test 1 (33, 33)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(33n);
    input.add64(33n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, euint64) => euint64 test 2 (33, 29)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(33n);
    input.add64(29n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, euint64) => euint64 test 1 (2, 65)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(2n);
    input.add64(65n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.resEuint64());
    expect(res).to.equal(130n);
  });

  it('test operator "mul" overload (euint8, euint64) => euint64 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(1n);
    input.add64(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.resEuint64());
    expect(res).to.equal(5n);
  });

  it('test operator "mul" overload (euint8, euint64) => euint64 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(5n);
    input.add64(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.resEuint64());
    expect(res).to.equal(25n);
  });

  it('test operator "mul" overload (euint8, euint64) => euint64 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(5n);
    input.add64(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.resEuint64());
    expect(res).to.equal(5n);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 1 (75, 18443905121357369457)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(75n);
    input.add64(18443905121357369457n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.resEuint64());
    expect(res).to.equal(65n);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 2 (71, 75)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(71n);
    input.add64(75n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.resEuint64());
    expect(res).to.equal(67n);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 3 (75, 75)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(75n);
    input.add64(75n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.resEuint64());
    expect(res).to.equal(75n);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 4 (75, 71)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(75n);
    input.add64(71n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.resEuint64());
    expect(res).to.equal(67n);
  });

  it('test operator "or" overload (euint8, euint64) => euint64 test 1 (220, 18439424040301617993)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(220n);
    input.add64(18439424040301617993n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.resEuint64());
    expect(res).to.equal(18439424040301618141n);
  });

  it('test operator "or" overload (euint8, euint64) => euint64 test 2 (216, 220)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(216n);
    input.add64(220n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.resEuint64());
    expect(res).to.equal(220n);
  });

  it('test operator "or" overload (euint8, euint64) => euint64 test 3 (220, 220)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(220n);
    input.add64(220n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.resEuint64());
    expect(res).to.equal(220n);
  });

  it('test operator "or" overload (euint8, euint64) => euint64 test 4 (220, 216)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(220n);
    input.add64(216n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.resEuint64());
    expect(res).to.equal(220n);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 1 (239, 18443965601336513263)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(239n);
    input.add64(18443965601336513263n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.resEuint64());
    expect(res).to.equal(18443965601336513024n);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 2 (235, 239)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(235n);
    input.add64(239n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 3 (239, 239)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(239n);
    input.add64(239n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 4 (239, 235)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(239n);
    input.add64(235n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 1 (112, 18441797542442137655)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(112n);
    input.add64(18441797542442137655n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 2 (108, 112)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(108n);
    input.add64(112n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 3 (112, 112)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(112n);
    input.add64(112n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 4 (112, 108)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(112n);
    input.add64(108n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint64) => ebool test 1 (5, 18445907386035380693)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(5n);
    input.add64(18445907386035380693n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint64) => ebool test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(1n);
    input.add64(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint64) => ebool test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(5n);
    input.add64(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint64) => ebool test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(5n);
    input.add64(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint64) => ebool test 1 (124, 18446273172900886433)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(124n);
    input.add64(18446273172900886433n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint64) => ebool test 2 (120, 124)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(120n);
    input.add64(124n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint64) => ebool test 3 (124, 124)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(124n);
    input.add64(124n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint64) => ebool test 4 (124, 120)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(124n);
    input.add64(120n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint64) => ebool test 1 (128, 18439174394133754227)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(128n);
    input.add64(18439174394133754227n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint64) => ebool test 2 (124, 128)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(124n);
    input.add64(128n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint64) => ebool test 3 (128, 128)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(128n);
    input.add64(128n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint64) => ebool test 4 (128, 124)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(128n);
    input.add64(124n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });
});
