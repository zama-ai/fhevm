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

  it('test operator "sub" overload (uint32, euint32) => euint32 test 1 (132238280, 132238280)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(132238280n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_uint32_euint32(
      132238280n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint32, euint32) => euint32 test 2 (132238280, 132238276)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(132238276n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_uint32_euint32(
      132238280n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 1 (50552, 26195)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(50552n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint32_uint32(encryptedAmount.handles[0], 26195n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1324209640n);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 2 (35409, 35409)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(35409n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint32_uint32(encryptedAmount.handles[0], 35409n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1253797281n);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 3 (35409, 35409)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(35409n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint32_uint32(encryptedAmount.handles[0], 35409n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1253797281n);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 4 (35409, 35409)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(35409n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint32_uint32(encryptedAmount.handles[0], 35409n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1253797281n);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 1 (22502, 104775)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(104775n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint32_euint32(22502n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(2357647050n);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 2 (35409, 35409)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(35409n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint32_euint32(35409n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1253797281n);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 3 (35409, 35409)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(35409n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint32_euint32(35409n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1253797281n);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 4 (35409, 35409)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(35409n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint32_euint32(35409n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1253797281n);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 1 (1350052417, 3496561)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1350052417n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint32_uint32(
      encryptedAmount.handles[0],
      3496561n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(386n);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 2 (1350052413, 1350052417)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1350052413n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint32_uint32(
      encryptedAmount.handles[0],
      1350052417n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 3 (1350052417, 1350052417)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1350052417n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint32_uint32(
      encryptedAmount.handles[0],
      1350052417n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 4 (1350052417, 1350052413)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1350052417n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint32_uint32(
      encryptedAmount.handles[0],
      1350052413n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 1 (2208252093, 1984452074)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2208252093n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint32_uint32(
      encryptedAmount.handles[0],
      1984452074n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(223800019n);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 2 (13169219, 13169223)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(13169219n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint32_uint32(
      encryptedAmount.handles[0],
      13169223n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(13169219n);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 3 (13169223, 13169223)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(13169223n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint32_uint32(
      encryptedAmount.handles[0],
      13169223n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 4 (13169223, 13169219)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(13169223n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint32_uint32(
      encryptedAmount.handles[0],
      13169219n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(4n);
  });

  it('test operator "and" overload (euint32, uint32) => euint32 test 1 (387591518, 3903679933)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(387591518n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint32_uint32(
      encryptedAmount.handles[0],
      3903679933n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(535836n);
  });

  it('test operator "and" overload (euint32, uint32) => euint32 test 2 (387591514, 387591518)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(387591514n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint32_uint32(
      encryptedAmount.handles[0],
      387591518n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(387591514n);
  });

  it('test operator "and" overload (euint32, uint32) => euint32 test 3 (387591518, 387591518)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(387591518n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint32_uint32(
      encryptedAmount.handles[0],
      387591518n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(387591518n);
  });

  it('test operator "and" overload (euint32, uint32) => euint32 test 4 (387591518, 387591514)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(387591518n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint32_uint32(
      encryptedAmount.handles[0],
      387591514n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(387591514n);
  });

  it('test operator "and" overload (uint32, euint32) => euint32 test 1 (3241863698, 3903679933)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(3903679933n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint32_euint32(
      3241863698n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(3223873552n);
  });

  it('test operator "and" overload (uint32, euint32) => euint32 test 2 (387591514, 387591518)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(387591518n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint32_euint32(
      387591514n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(387591514n);
  });

  it('test operator "and" overload (uint32, euint32) => euint32 test 3 (387591518, 387591518)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(387591518n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint32_euint32(
      387591518n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(387591518n);
  });

  it('test operator "and" overload (uint32, euint32) => euint32 test 4 (387591518, 387591514)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(387591514n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint32_euint32(
      387591518n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(387591514n);
  });

  it('test operator "or" overload (euint32, uint32) => euint32 test 1 (2153872215, 1094302937)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2153872215n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint32_uint32(
      encryptedAmount.handles[0],
      1094302937n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(3245998047n);
  });

  it('test operator "or" overload (euint32, uint32) => euint32 test 2 (2153872211, 2153872215)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2153872211n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint32_uint32(
      encryptedAmount.handles[0],
      2153872215n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(2153872215n);
  });

  it('test operator "or" overload (euint32, uint32) => euint32 test 3 (2153872215, 2153872215)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2153872215n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint32_uint32(
      encryptedAmount.handles[0],
      2153872215n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(2153872215n);
  });

  it('test operator "or" overload (euint32, uint32) => euint32 test 4 (2153872215, 2153872211)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2153872215n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint32_uint32(
      encryptedAmount.handles[0],
      2153872211n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(2153872215n);
  });

  it('test operator "or" overload (uint32, euint32) => euint32 test 1 (2763474577, 1094302937)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1094302937n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint32_euint32(
      2763474577n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(3854565081n);
  });

  it('test operator "or" overload (uint32, euint32) => euint32 test 2 (2153872211, 2153872215)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(2153872215n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint32_euint32(
      2153872211n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(2153872215n);
  });

  it('test operator "or" overload (uint32, euint32) => euint32 test 3 (2153872215, 2153872215)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(2153872215n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint32_euint32(
      2153872215n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(2153872215n);
  });

  it('test operator "or" overload (uint32, euint32) => euint32 test 4 (2153872215, 2153872211)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(2153872211n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint32_euint32(
      2153872215n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(2153872215n);
  });

  it('test operator "xor" overload (euint32, uint32) => euint32 test 1 (3048069583, 3091176467)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3048069583n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint32_uint32(
      encryptedAmount.handles[0],
      3091176467n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(227689948n);
  });

  it('test operator "xor" overload (euint32, uint32) => euint32 test 2 (69043615, 69043619)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(69043615n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint32_uint32(
      encryptedAmount.handles[0],
      69043619n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(60n);
  });

  it('test operator "xor" overload (euint32, uint32) => euint32 test 3 (69043619, 69043619)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(69043619n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint32_uint32(
      encryptedAmount.handles[0],
      69043619n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, uint32) => euint32 test 4 (69043619, 69043615)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(69043619n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint32_uint32(
      encryptedAmount.handles[0],
      69043615n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(60n);
  });

  it('test operator "xor" overload (uint32, euint32) => euint32 test 1 (1734491575, 3091176467)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(3091176467n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint32_euint32(
      1734491575n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(3747455396n);
  });

  it('test operator "xor" overload (uint32, euint32) => euint32 test 2 (69043615, 69043619)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(69043619n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint32_euint32(
      69043615n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(60n);
  });

  it('test operator "xor" overload (uint32, euint32) => euint32 test 3 (69043619, 69043619)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(69043619n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint32_euint32(
      69043619n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (uint32, euint32) => euint32 test 4 (69043619, 69043615)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(69043615n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint32_euint32(
      69043619n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(60n);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 1 (797349228, 3408982793)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(797349228n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint32_uint32(
      encryptedAmount.handles[0],
      3408982793n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 2 (797349224, 797349228)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(797349224n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint32_uint32(
      encryptedAmount.handles[0],
      797349228n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 3 (797349228, 797349228)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(797349228n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint32_uint32(
      encryptedAmount.handles[0],
      797349228n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 4 (797349228, 797349224)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(797349228n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint32_uint32(
      encryptedAmount.handles[0],
      797349224n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 1 (1234970054, 3408982793)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(3408982793n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint32_euint32(
      1234970054n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 2 (797349224, 797349228)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(797349228n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint32_euint32(
      797349224n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 3 (797349228, 797349228)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(797349228n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint32_euint32(
      797349228n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 4 (797349228, 797349224)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(797349224n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint32_euint32(
      797349228n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 1 (774665816, 2308496870)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(774665816n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint32_uint32(
      encryptedAmount.handles[0],
      2308496870n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 2 (774665812, 774665816)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(774665812n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint32_uint32(
      encryptedAmount.handles[0],
      774665816n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 3 (774665816, 774665816)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(774665816n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint32_uint32(
      encryptedAmount.handles[0],
      774665816n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 4 (774665816, 774665812)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(774665816n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint32_uint32(
      encryptedAmount.handles[0],
      774665812n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 1 (2415716006, 2308496870)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(2308496870n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint32_euint32(
      2415716006n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 2 (774665812, 774665816)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(774665816n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint32_euint32(
      774665812n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 3 (774665816, 774665816)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(774665816n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint32_euint32(
      774665816n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 4 (774665816, 774665812)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(774665812n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint32_euint32(
      774665816n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 1 (2504440996, 2383920919)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2504440996n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint32_uint32(
      encryptedAmount.handles[0],
      2383920919n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 2 (2431958718, 2431958722)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2431958718n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint32_uint32(
      encryptedAmount.handles[0],
      2431958722n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 3 (2431958722, 2431958722)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2431958722n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint32_uint32(
      encryptedAmount.handles[0],
      2431958722n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 4 (2431958722, 2431958718)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2431958722n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint32_uint32(
      encryptedAmount.handles[0],
      2431958718n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 1 (1454492305, 2383920919)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(2383920919n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint32_euint32(
      1454492305n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 2 (2431958718, 2431958722)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(2431958722n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint32_euint32(
      2431958718n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 3 (2431958722, 2431958722)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(2431958722n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint32_euint32(
      2431958722n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 4 (2431958722, 2431958718)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(2431958718n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint32_euint32(
      2431958722n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 1 (3191222937, 4030766606)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3191222937n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint32_uint32(
      encryptedAmount.handles[0],
      4030766606n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 2 (1227859631, 1227859635)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1227859631n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint32_uint32(
      encryptedAmount.handles[0],
      1227859635n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 3 (1227859635, 1227859635)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1227859635n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint32_uint32(
      encryptedAmount.handles[0],
      1227859635n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 4 (1227859635, 1227859631)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1227859635n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint32_uint32(
      encryptedAmount.handles[0],
      1227859631n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 1 (980111110, 4030766606)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(4030766606n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint32_euint32(
      980111110n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 2 (1227859631, 1227859635)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1227859635n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint32_euint32(
      1227859631n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 3 (1227859635, 1227859635)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1227859635n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint32_euint32(
      1227859635n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 4 (1227859635, 1227859631)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1227859631n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint32_euint32(
      1227859635n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 1 (4222548999, 1984778059)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(4222548999n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint32_uint32(
      encryptedAmount.handles[0],
      1984778059n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 2 (2161439611, 2161439615)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2161439611n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint32_uint32(
      encryptedAmount.handles[0],
      2161439615n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 3 (2161439615, 2161439615)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2161439615n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint32_uint32(
      encryptedAmount.handles[0],
      2161439615n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 4 (2161439615, 2161439611)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2161439615n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint32_uint32(
      encryptedAmount.handles[0],
      2161439611n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 1 (978893112, 1984778059)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1984778059n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint32_euint32(
      978893112n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 2 (2161439611, 2161439615)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(2161439615n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint32_euint32(
      2161439611n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 3 (2161439615, 2161439615)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(2161439615n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint32_euint32(
      2161439615n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 4 (2161439615, 2161439611)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(2161439611n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint32_euint32(
      2161439615n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 1 (966626906, 2511796063)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(966626906n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint32_uint32(
      encryptedAmount.handles[0],
      2511796063n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 2 (55972212, 55972216)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(55972212n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint32_uint32(
      encryptedAmount.handles[0],
      55972216n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 3 (55972216, 55972216)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(55972216n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint32_uint32(
      encryptedAmount.handles[0],
      55972216n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 4 (55972216, 55972212)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(55972216n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint32_uint32(
      encryptedAmount.handles[0],
      55972212n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 1 (3348270243, 2511796063)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(2511796063n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint32_euint32(
      3348270243n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 2 (55972212, 55972216)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(55972216n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint32_euint32(
      55972212n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 3 (55972216, 55972216)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(55972216n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint32_euint32(
      55972216n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 4 (55972216, 55972212)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(55972212n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint32_euint32(
      55972216n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 1 (3105605558, 1386244495)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3105605558n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint32_uint32(
      encryptedAmount.handles[0],
      1386244495n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1386244495n);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 2 (752697276, 752697280)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(752697276n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint32_uint32(
      encryptedAmount.handles[0],
      752697280n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(752697276n);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 3 (752697280, 752697280)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(752697280n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint32_uint32(
      encryptedAmount.handles[0],
      752697280n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(752697280n);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 4 (752697280, 752697276)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(752697280n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint32_uint32(
      encryptedAmount.handles[0],
      752697276n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(752697276n);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 1 (2467835867, 1386244495)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1386244495n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint32_euint32(
      2467835867n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1386244495n);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 2 (752697276, 752697280)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(752697280n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint32_euint32(
      752697276n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(752697276n);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 3 (752697280, 752697280)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(752697280n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint32_euint32(
      752697280n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(752697280n);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 4 (752697280, 752697276)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(752697276n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint32_euint32(
      752697280n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(752697276n);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 1 (2148472030, 3691322481)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2148472030n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint32_uint32(
      encryptedAmount.handles[0],
      3691322481n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(3691322481n);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 2 (1689761847, 1689761851)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1689761847n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint32_uint32(
      encryptedAmount.handles[0],
      1689761851n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1689761851n);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 3 (1689761851, 1689761851)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1689761851n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint32_uint32(
      encryptedAmount.handles[0],
      1689761851n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1689761851n);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 4 (1689761851, 1689761847)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1689761851n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint32_uint32(
      encryptedAmount.handles[0],
      1689761847n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1689761851n);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 1 (568010035, 3691322481)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(3691322481n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint32_euint32(
      568010035n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(3691322481n);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 2 (1689761847, 1689761851)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1689761851n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint32_euint32(
      1689761847n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1689761851n);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 3 (1689761851, 1689761851)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1689761851n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint32_euint32(
      1689761851n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1689761851n);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 4 (1689761851, 1689761847)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1689761847n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint32_euint32(
      1689761851n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1689761851n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 1 (9222142026138250634, 9220540534979407489)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(9222142026138250634n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint64_uint64(
      encryptedAmount.handles[0],
      9220540534979407489n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18442682561117658123n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 2 (9219454887867520955, 9219454887867520957)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(9219454887867520955n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint64_uint64(
      encryptedAmount.handles[0],
      9219454887867520957n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18438909775735041912n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 3 (9219454887867520957, 9219454887867520957)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(9219454887867520957n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint64_uint64(
      encryptedAmount.handles[0],
      9219454887867520957n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18438909775735041914n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 4 (9219454887867520957, 9219454887867520955)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(9219454887867520957n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint64_uint64(
      encryptedAmount.handles[0],
      9219454887867520955n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18438909775735041912n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 1 (9220616982362565196, 9220540534979407489)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(9220540534979407489n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint64_euint64(
      9220616982362565196n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441157517341972685n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 2 (9219454887867520955, 9219454887867520957)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(9219454887867520957n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint64_euint64(
      9219454887867520955n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18438909775735041912n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 3 (9219454887867520957, 9219454887867520957)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(9219454887867520957n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint64_euint64(
      9219454887867520957n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18438909775735041914n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 4 (9219454887867520957, 9219454887867520955)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(9219454887867520955n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint64_euint64(
      9219454887867520957n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18438909775735041912n);
  });

  it('test operator "sub" overload (euint64, uint64) => euint64 test 1 (18444234584453967225, 18444234584453967225)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18444234584453967225n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint64_uint64(
      encryptedAmount.handles[0],
      18444234584453967225n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, uint64) => euint64 test 2 (18444234584453967225, 18444234584453967221)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18444234584453967225n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint64_uint64(
      encryptedAmount.handles[0],
      18444234584453967221n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint64, euint64) => euint64 test 1 (18444234584453967225, 18444234584453967225)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18444234584453967225n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_uint64_euint64(
      18444234584453967225n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint64, euint64) => euint64 test 2 (18444234584453967225, 18444234584453967221)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18444234584453967221n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_uint64_euint64(
      18444234584453967225n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 1 (4293106541, 4293222113)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(4293106541n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint64_uint64(
      encryptedAmount.handles[0],
      4293222113n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18431259935286141133n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 2 (4293106541, 4293106541)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(4293106541n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint64_uint64(
      encryptedAmount.handles[0],
      4293106541n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18430763772376984681n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 3 (4293106541, 4293106541)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(4293106541n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint64_uint64(
      encryptedAmount.handles[0],
      4293106541n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18430763772376984681n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 4 (4293106541, 4293106541)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(4293106541n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint64_uint64(
      encryptedAmount.handles[0],
      4293106541n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18430763772376984681n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 1 (4293310127, 4293222113)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(4293222113n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint64_euint64(
      4293310127n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18432133975203238351n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 2 (4293106541, 4293106541)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(4293106541n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint64_euint64(
      4293106541n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18430763772376984681n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 3 (4293106541, 4293106541)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(4293106541n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint64_euint64(
      4293106541n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18430763772376984681n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 4 (4293106541, 4293106541)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(4293106541n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint64_euint64(
      4293106541n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18430763772376984681n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 1 (18441468997612113713, 18441064813181044753)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441468997612113713n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint64_uint64(
      encryptedAmount.handles[0],
      18441064813181044753n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 2 (18440092716402549335, 18440092716402549339)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18440092716402549335n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint64_uint64(
      encryptedAmount.handles[0],
      18440092716402549339n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 3 (18440092716402549339, 18440092716402549339)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18440092716402549339n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint64_uint64(
      encryptedAmount.handles[0],
      18440092716402549339n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 4 (18440092716402549339, 18440092716402549335)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18440092716402549339n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint64_uint64(
      encryptedAmount.handles[0],
      18440092716402549335n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 1 (18440094283065391825, 18443191153139421935)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18440094283065391825n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint64_uint64(
      encryptedAmount.handles[0],
      18443191153139421935n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18440094283065391825n);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 2 (18437948955120201841, 18437948955120201845)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18437948955120201841n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint64_uint64(
      encryptedAmount.handles[0],
      18437948955120201845n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18437948955120201841n);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 3 (18437948955120201845, 18437948955120201845)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18437948955120201845n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint64_uint64(
      encryptedAmount.handles[0],
      18437948955120201845n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 4 (18437948955120201845, 18437948955120201841)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18437948955120201845n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint64_uint64(
      encryptedAmount.handles[0],
      18437948955120201841n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "and" overload (euint64, uint64) => euint64 test 1 (18437775404426187035, 18441300081626828863)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18437775404426187035n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint64_uint64(
      encryptedAmount.handles[0],
      18441300081626828863n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18437772070923391003n);
  });

  it('test operator "and" overload (euint64, uint64) => euint64 test 2 (18437775404426187031, 18437775404426187035)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18437775404426187031n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint64_uint64(
      encryptedAmount.handles[0],
      18437775404426187035n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18437775404426187027n);
  });

  it('test operator "and" overload (euint64, uint64) => euint64 test 3 (18437775404426187035, 18437775404426187035)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18437775404426187035n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint64_uint64(
      encryptedAmount.handles[0],
      18437775404426187035n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18437775404426187035n);
  });

  it('test operator "and" overload (euint64, uint64) => euint64 test 4 (18437775404426187035, 18437775404426187031)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18437775404426187035n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint64_uint64(
      encryptedAmount.handles[0],
      18437775404426187031n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18437775404426187027n);
  });

  it('test operator "and" overload (uint64, euint64) => euint64 test 1 (18444020759457142949, 18441300081626828863)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18441300081626828863n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint64_euint64(
      18444020759457142949n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18438862915309113381n);
  });

  it('test operator "and" overload (uint64, euint64) => euint64 test 2 (18437775404426187031, 18437775404426187035)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18437775404426187035n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint64_euint64(
      18437775404426187031n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18437775404426187027n);
  });

  it('test operator "and" overload (uint64, euint64) => euint64 test 3 (18437775404426187035, 18437775404426187035)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18437775404426187035n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint64_euint64(
      18437775404426187035n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18437775404426187035n);
  });

  it('test operator "and" overload (uint64, euint64) => euint64 test 4 (18437775404426187035, 18437775404426187031)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18437775404426187031n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint64_euint64(
      18437775404426187035n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18437775404426187027n);
  });

  it('test operator "or" overload (euint64, uint64) => euint64 test 1 (18444254542050234759, 18445089908119147775)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18444254542050234759n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint64_uint64(
      encryptedAmount.handles[0],
      18445089908119147775n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18446532743062853119n);
  });

  it('test operator "or" overload (euint64, uint64) => euint64 test 2 (18440818288594918423, 18440818288594918427)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18440818288594918423n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint64_uint64(
      encryptedAmount.handles[0],
      18440818288594918427n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18440818288594918431n);
  });

  it('test operator "or" overload (euint64, uint64) => euint64 test 3 (18440818288594918427, 18440818288594918427)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18440818288594918427n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint64_uint64(
      encryptedAmount.handles[0],
      18440818288594918427n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18440818288594918427n);
  });

  it('test operator "or" overload (euint64, uint64) => euint64 test 4 (18440818288594918427, 18440818288594918423)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18440818288594918427n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint64_uint64(
      encryptedAmount.handles[0],
      18440818288594918423n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18440818288594918431n);
  });

  it('test operator "or" overload (uint64, euint64) => euint64 test 1 (18443935919096592303, 18445089908119147775)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18445089908119147775n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint64_euint64(
      18443935919096592303n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18446216306246416383n);
  });

  it('test operator "or" overload (uint64, euint64) => euint64 test 2 (18440818288594918423, 18440818288594918427)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18440818288594918427n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint64_euint64(
      18440818288594918423n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18440818288594918431n);
  });

  it('test operator "or" overload (uint64, euint64) => euint64 test 3 (18440818288594918427, 18440818288594918427)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18440818288594918427n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint64_euint64(
      18440818288594918427n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18440818288594918427n);
  });

  it('test operator "or" overload (uint64, euint64) => euint64 test 4 (18440818288594918427, 18440818288594918423)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18440818288594918423n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint64_euint64(
      18440818288594918427n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18440818288594918431n);
  });

  it('test operator "xor" overload (euint64, uint64) => euint64 test 1 (18438246588938835891, 18439872707341614365)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438246588938835891n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint64_uint64(
      encryptedAmount.handles[0],
      18439872707341614365n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(1786785823042222n);
  });

  it('test operator "xor" overload (euint64, uint64) => euint64 test 2 (18438246588938835887, 18438246588938835891)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438246588938835887n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint64_uint64(
      encryptedAmount.handles[0],
      18438246588938835891n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(28n);
  });

  it('test operator "xor" overload (euint64, uint64) => euint64 test 3 (18438246588938835891, 18438246588938835891)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438246588938835891n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint64_uint64(
      encryptedAmount.handles[0],
      18438246588938835891n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, uint64) => euint64 test 4 (18438246588938835891, 18438246588938835887)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438246588938835891n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint64_uint64(
      encryptedAmount.handles[0],
      18438246588938835887n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(28n);
  });

  it('test operator "xor" overload (uint64, euint64) => euint64 test 1 (18442890745007670521, 18439872707341614365)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18439872707341614365n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint64_euint64(
      18442890745007670521n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(6150589243324900n);
  });

  it('test operator "xor" overload (uint64, euint64) => euint64 test 2 (18438246588938835887, 18438246588938835891)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18438246588938835891n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint64_euint64(
      18438246588938835887n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(28n);
  });

  it('test operator "xor" overload (uint64, euint64) => euint64 test 3 (18438246588938835891, 18438246588938835891)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18438246588938835891n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint64_euint64(
      18438246588938835891n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (uint64, euint64) => euint64 test 4 (18438246588938835891, 18438246588938835887)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18438246588938835887n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint64_euint64(
      18438246588938835891n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(28n);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 1 (18443270303576332009, 18439666281848160033)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18443270303576332009n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint64_uint64(
      encryptedAmount.handles[0],
      18439666281848160033n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 2 (18437830335110482759, 18437830335110482763)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18437830335110482759n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint64_uint64(
      encryptedAmount.handles[0],
      18437830335110482763n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 3 (18437830335110482763, 18437830335110482763)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18437830335110482763n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint64_uint64(
      encryptedAmount.handles[0],
      18437830335110482763n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 4 (18437830335110482763, 18437830335110482759)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18437830335110482763n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint64_uint64(
      encryptedAmount.handles[0],
      18437830335110482759n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 1 (18441410528946799485, 18439666281848160033)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18439666281848160033n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint64_euint64(
      18441410528946799485n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 2 (18437830335110482759, 18437830335110482763)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18437830335110482763n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint64_euint64(
      18437830335110482759n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 3 (18437830335110482763, 18437830335110482763)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18437830335110482763n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint64_euint64(
      18437830335110482763n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 4 (18437830335110482763, 18437830335110482759)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18437830335110482759n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint64_euint64(
      18437830335110482763n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 1 (18438128297233107899, 18445713881183968031)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438128297233107899n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint64_uint64(
      encryptedAmount.handles[0],
      18445713881183968031n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 2 (18438128297233107895, 18438128297233107899)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438128297233107895n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint64_uint64(
      encryptedAmount.handles[0],
      18438128297233107899n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 3 (18438128297233107899, 18438128297233107899)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438128297233107899n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint64_uint64(
      encryptedAmount.handles[0],
      18438128297233107899n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 4 (18438128297233107899, 18438128297233107895)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438128297233107899n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint64_uint64(
      encryptedAmount.handles[0],
      18438128297233107895n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 1 (18439799824826614883, 18445713881183968031)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18445713881183968031n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint64_euint64(
      18439799824826614883n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 2 (18438128297233107895, 18438128297233107899)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18438128297233107899n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint64_euint64(
      18438128297233107895n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 3 (18438128297233107899, 18438128297233107899)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18438128297233107899n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint64_euint64(
      18438128297233107899n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 4 (18438128297233107899, 18438128297233107895)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18438128297233107895n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint64_euint64(
      18438128297233107899n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 1 (18440736320910729425, 18443640871948895025)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18440736320910729425n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint64_uint64(
      encryptedAmount.handles[0],
      18443640871948895025n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 2 (18440736320910729421, 18440736320910729425)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18440736320910729421n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint64_uint64(
      encryptedAmount.handles[0],
      18440736320910729425n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 3 (18440736320910729425, 18440736320910729425)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18440736320910729425n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint64_uint64(
      encryptedAmount.handles[0],
      18440736320910729425n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 4 (18440736320910729425, 18440736320910729421)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18440736320910729425n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint64_uint64(
      encryptedAmount.handles[0],
      18440736320910729421n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 1 (18439142556413509831, 18443640871948895025)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18443640871948895025n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint64_euint64(
      18439142556413509831n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 2 (18440736320910729421, 18440736320910729425)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18440736320910729425n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint64_euint64(
      18440736320910729421n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 3 (18440736320910729425, 18440736320910729425)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18440736320910729425n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint64_euint64(
      18440736320910729425n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 4 (18440736320910729425, 18440736320910729421)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18440736320910729421n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint64_euint64(
      18440736320910729425n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 1 (18440082194335658565, 18439420160547120497)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18440082194335658565n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint64_uint64(
      encryptedAmount.handles[0],
      18439420160547120497n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 2 (18439818026261660815, 18439818026261660819)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439818026261660815n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint64_uint64(
      encryptedAmount.handles[0],
      18439818026261660819n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 3 (18439818026261660819, 18439818026261660819)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439818026261660819n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint64_uint64(
      encryptedAmount.handles[0],
      18439818026261660819n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 4 (18439818026261660819, 18439818026261660815)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439818026261660819n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint64_uint64(
      encryptedAmount.handles[0],
      18439818026261660815n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 1 (18440906050622768845, 18439420160547120497)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18439420160547120497n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint64_euint64(
      18440906050622768845n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 2 (18439818026261660815, 18439818026261660819)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18439818026261660819n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint64_euint64(
      18439818026261660815n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 3 (18439818026261660819, 18439818026261660819)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18439818026261660819n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint64_euint64(
      18439818026261660819n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 4 (18439818026261660819, 18439818026261660815)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18439818026261660815n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint64_euint64(
      18439818026261660819n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 1 (18446442434244374497, 18446403310716651743)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18446442434244374497n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint64_uint64(
      encryptedAmount.handles[0],
      18446403310716651743n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 2 (18444292413094251003, 18444292413094251007)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18444292413094251003n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint64_uint64(
      encryptedAmount.handles[0],
      18444292413094251007n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 3 (18444292413094251007, 18444292413094251007)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18444292413094251007n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint64_uint64(
      encryptedAmount.handles[0],
      18444292413094251007n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 4 (18444292413094251007, 18444292413094251003)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18444292413094251007n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint64_uint64(
      encryptedAmount.handles[0],
      18444292413094251003n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 1 (18439593596839592305, 18446403310716651743)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18446403310716651743n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint64_euint64(
      18439593596839592305n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 2 (18444292413094251003, 18444292413094251007)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18444292413094251007n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint64_euint64(
      18444292413094251003n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 3 (18444292413094251007, 18444292413094251007)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18444292413094251007n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint64_euint64(
      18444292413094251007n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 4 (18444292413094251007, 18444292413094251003)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18444292413094251003n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint64_euint64(
      18444292413094251007n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 1 (18445794030519917519, 18443676263070331705)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18445794030519917519n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint64_uint64(
      encryptedAmount.handles[0],
      18443676263070331705n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 2 (18445794030519917515, 18445794030519917519)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18445794030519917515n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint64_uint64(
      encryptedAmount.handles[0],
      18445794030519917519n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 3 (18445794030519917519, 18445794030519917519)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18445794030519917519n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint64_uint64(
      encryptedAmount.handles[0],
      18445794030519917519n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 4 (18445794030519917519, 18445794030519917515)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18445794030519917519n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint64_uint64(
      encryptedAmount.handles[0],
      18445794030519917515n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });
});
