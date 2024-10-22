import { expect } from 'chai';
import { ethers } from 'hardhat';

import type {
  TFHETestSuite1,
  TFHETestSuite10,
  TFHETestSuite11,
  TFHETestSuite2,
  TFHETestSuite3,
  TFHETestSuite4,
  TFHETestSuite5,
  TFHETestSuite6,
  TFHETestSuite7,
  TFHETestSuite8,
  TFHETestSuite9,
} from '../../types';
import {
  createInstances,
  decrypt128,
  decrypt16,
  decrypt32,
  decrypt4,
  decrypt64,
  decrypt8,
  decryptBool,
} from '../instance';
import { getSigners, initSigners } from '../signers';

async function deployTfheTestFixture1(): Promise<TFHETestSuite1> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite1');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture2(): Promise<TFHETestSuite2> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite2');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture3(): Promise<TFHETestSuite3> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite3');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture4(): Promise<TFHETestSuite4> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite4');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture5(): Promise<TFHETestSuite5> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite5');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture6(): Promise<TFHETestSuite6> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite6');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture7(): Promise<TFHETestSuite7> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite7');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture8(): Promise<TFHETestSuite8> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite8');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture9(): Promise<TFHETestSuite9> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite9');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture10(): Promise<TFHETestSuite10> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite10');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture11(): Promise<TFHETestSuite11> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite11');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

describe('TFHE operations 1', function () {
  before(async function () {
    await initSigners(1);
    this.signers = await getSigners();

    const contract1 = await deployTfheTestFixture1();
    this.contract1Address = await contract1.getAddress();
    this.contract1 = contract1;

    const contract2 = await deployTfheTestFixture2();
    this.contract2Address = await contract2.getAddress();
    this.contract2 = contract2;

    const contract3 = await deployTfheTestFixture3();
    this.contract3Address = await contract3.getAddress();
    this.contract3 = contract3;

    const contract4 = await deployTfheTestFixture4();
    this.contract4Address = await contract4.getAddress();
    this.contract4 = contract4;

    const contract5 = await deployTfheTestFixture5();
    this.contract5Address = await contract5.getAddress();
    this.contract5 = contract5;

    const contract6 = await deployTfheTestFixture6();
    this.contract6Address = await contract6.getAddress();
    this.contract6 = contract6;

    const contract7 = await deployTfheTestFixture7();
    this.contract7Address = await contract7.getAddress();
    this.contract7 = contract7;

    const contract8 = await deployTfheTestFixture8();
    this.contract8Address = await contract8.getAddress();
    this.contract8 = contract8;

    const contract9 = await deployTfheTestFixture9();
    this.contract9Address = await contract9.getAddress();
    this.contract9 = contract9;

    const contract10 = await deployTfheTestFixture10();
    this.contract10Address = await contract10.getAddress();
    this.contract10 = contract10;

    const contract11 = await deployTfheTestFixture11();
    this.contract11Address = await contract11.getAddress();
    this.contract11 = contract11;

    const instances = await createInstances(this.signers);
    this.instances = instances;
  });

  it('test operator "add" overload (euint4, euint4) => euint4 test 1 (2, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(2n);
    input.add4(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(9n);
  });

  it('test operator "add" overload (euint4, euint4) => euint4 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint4, euint4) => euint4 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(5n);
    input.add4(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint4, euint4) => euint4 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint4, euint4) => euint4 test 1 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint4, euint4) => euint4 test 2 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint4, euint4) => euint4 test 1 (1, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(1n);
    input.add4(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(7n);
  });

  it('test operator "mul" overload (euint4, euint4) => euint4 test 2 (3, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(3n);
    input.add4(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(15n);
  });

  it('test operator "mul" overload (euint4, euint4) => euint4 test 3 (3, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(3n);
    input.add4(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, euint4) => euint4 test 4 (5, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(5n);
    input.add4(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(15n);
  });

  it('test operator "and" overload (euint4, euint4) => euint4 test 1 (14, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(14n);
    input.add4(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(6n);
  });

  it('test operator "and" overload (euint4, euint4) => euint4 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint4, euint4) => euint4 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint4, euint4) => euint4 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint4, euint4) => euint4 test 1 (10, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add4(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(14n);
  });

  it('test operator "or" overload (euint4, euint4) => euint4 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint4, euint4) => euint4 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint4, euint4) => euint4 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint4, euint4) => euint4 test 1 (9, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(13n);
  });

  it('test operator "xor" overload (euint4, euint4) => euint4 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint4, euint4) => euint4 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint4, euint4) => euint4 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint4, euint4) => ebool test 1 (2, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(2n);
    input.add4(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, euint4) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint4) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, euint4) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint4) => ebool test 1 (1, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(1n);
    input.add4(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint4) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint4) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint4) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint4) => ebool test 1 (6, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(6n);
    input.add4(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint4) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint4) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint4) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint4, euint4) => ebool test 1 (12, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(12n);
    input.add4(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint4) => ebool test 2 (8, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add4(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint4) => ebool test 3 (12, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(12n);
    input.add4(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint4) => ebool test 4 (12, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(12n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint4) => ebool test 1 (6, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(6n);
    input.add4(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint4) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint4) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint4) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint4) => ebool test 1 (2, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(2n);
    input.add4(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint4) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint4) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint4) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint4, euint4) => euint4 test 1 (14, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(14n);
    input.add4(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(13n);
  });

  it('test operator "min" overload (euint4, euint4) => euint4 test 2 (9, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);
    input.add4(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(9n);
  });

  it('test operator "min" overload (euint4, euint4) => euint4 test 3 (13, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);
    input.add4(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(13n);
  });

  it('test operator "min" overload (euint4, euint4) => euint4 test 4 (13, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);
    input.add4(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(9n);
  });

  it('test operator "max" overload (euint4, euint4) => euint4 test 1 (9, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);
    input.add4(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(13n);
  });

  it('test operator "max" overload (euint4, euint4) => euint4 test 2 (5, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(5n);
    input.add4(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(9n);
  });

  it('test operator "max" overload (euint4, euint4) => euint4 test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);
    input.add4(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(9n);
  });

  it('test operator "max" overload (euint4, euint4) => euint4 test 4 (9, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);
    input.add4(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt4(await this.contract1.res4());
    expect(res).to.equal(9n);
  });

  it('test operator "add" overload (euint4, euint8) => euint8 test 1 (2, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(2n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint4, euint8) => euint8 test 2 (5, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(5n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(14n);
  });

  it('test operator "add" overload (euint4, euint8) => euint8 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(5n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint4, euint8) => euint8 test 4 (9, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(14n);
  });

  it('test operator "sub" overload (euint4, euint8) => euint8 test 1 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint4, euint8) => euint8 test 2 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint4, euint8) => euint8 test 1 (2, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(2n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(10n);
  });

  it('test operator "mul" overload (euint4, euint8) => euint8 test 2 (3, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(3n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(15n);
  });

  it('test operator "mul" overload (euint4, euint8) => euint8 test 3 (3, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(3n);
    input.add8(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, euint8) => euint8 test 4 (5, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(5n);
    input.add8(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(15n);
  });

  it('test operator "and" overload (euint4, euint8) => euint8 test 1 (10, 116)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add8(116n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint4, euint8) => euint8 test 2 (6, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(6n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(2n);
  });

  it('test operator "and" overload (euint4, euint8) => euint8 test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(10n);
  });

  it('test operator "and" overload (euint4, euint8) => euint8 test 4 (10, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(2n);
  });

  it('test operator "or" overload (euint4, euint8) => euint8 test 1 (8, 185)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add8(185n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(185n);
  });

  it('test operator "or" overload (euint4, euint8) => euint8 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint4, euint8) => euint8 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint4, euint8) => euint8 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint4, euint8) => euint8 test 1 (10, 103)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add8(103n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(109n);
  });

  it('test operator "xor" overload (euint4, euint8) => euint8 test 2 (6, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(6n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint4, euint8) => euint8 test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint4, euint8) => euint8 test 4 (10, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint4, euint8) => ebool test 1 (14, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(14n);
    input.add8(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint8) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint8) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, euint8) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint8) => ebool test 1 (8, 128)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add8(128n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint8) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint8) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint8) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint8) => ebool test 1 (3, 79)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(3n);
    input.add8(79n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint8) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint8) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint8) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint4, euint8) => ebool test 1 (1, 59)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(1n);
    input.add8(59n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint8) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint8) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint8) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint8) => ebool test 1 (1, 98)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(1n);
    input.add8(98n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint8) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint8) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint8) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint8) => ebool test 1 (12, 31)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(12n);
    input.add8(31n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint8) => ebool test 2 (8, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add8(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint8) => ebool test 3 (12, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(12n);
    input.add8(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint8) => ebool test 4 (12, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(12n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint4, euint8) => euint8 test 1 (5, 254)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(5n);
    input.add8(254n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(5n);
  });

  it('test operator "min" overload (euint4, euint8) => euint8 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint4, euint8) => euint8 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint4, euint8) => euint8 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint4, euint8) => euint8 test 1 (14, 226)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(14n);
    input.add8(226n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(226n);
  });

  it('test operator "max" overload (euint4, euint8) => euint8 test 2 (10, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add8(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(14n);
  });

  it('test operator "max" overload (euint4, euint8) => euint8 test 3 (14, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(14n);
    input.add8(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(14n);
  });

  it('test operator "max" overload (euint4, euint8) => euint8 test 4 (14, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(14n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract1.res8());
    expect(res).to.equal(14n);
  });

  it('test operator "add" overload (euint4, euint16) => euint16 test 1 (2, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(2n);
    input.add16(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(11n);
  });

  it('test operator "add" overload (euint4, euint16) => euint16 test 2 (6, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(6n);
    input.add16(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(14n);
  });

  it('test operator "add" overload (euint4, euint16) => euint16 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(5n);
    input.add16(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint4, euint16) => euint16 test 4 (8, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add16(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(14n);
  });

  it('test operator "sub" overload (euint4, euint16) => euint16 test 1 (13, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);
    input.add16(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint4, euint16) => euint16 test 2 (13, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);
    input.add16(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint4, euint16) => euint16 test 1 (2, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(2n);
    input.add16(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(12n);
  });

  it('test operator "mul" overload (euint4, euint16) => euint16 test 2 (3, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(3n);
    input.add16(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(15n);
  });

  it('test operator "mul" overload (euint4, euint16) => euint16 test 3 (3, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(3n);
    input.add16(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, euint16) => euint16 test 4 (5, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(5n);
    input.add16(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(15n);
  });

  it('test operator "and" overload (euint4, euint16) => euint16 test 1 (3, 65312)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(3n);
    input.add16(65312n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint4, euint16) => euint16 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add16(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint4, euint16) => euint16 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add16(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint4, euint16) => euint16 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add16(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint4, euint16) => euint16 test 1 (9, 28733)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);
    input.add16(28733n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(28733n);
  });

  it('test operator "or" overload (euint4, euint16) => euint16 test 2 (5, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(5n);
    input.add16(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(13n);
  });

  it('test operator "or" overload (euint4, euint16) => euint16 test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);
    input.add16(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(9n);
  });

  it('test operator "or" overload (euint4, euint16) => euint16 test 4 (9, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);
    input.add16(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(13n);
  });

  it('test operator "xor" overload (euint4, euint16) => euint16 test 1 (14, 11463)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(14n);
    input.add16(11463n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(11465n);
  });

  it('test operator "xor" overload (euint4, euint16) => euint16 test 2 (10, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add16(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint4, euint16) => euint16 test 3 (14, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(14n);
    input.add16(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint4, euint16) => euint16 test 4 (14, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(14n);
    input.add16(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint4, euint16) => ebool test 1 (11, 40901)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(11n);
    input.add16(40901n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint16) => ebool test 2 (7, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(7n);
    input.add16(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint16) => ebool test 3 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(11n);
    input.add16(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, euint16) => ebool test 4 (11, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(11n);
    input.add16(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint16) => ebool test 1 (14, 35568)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(14n);
    input.add16(35568n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint16) => ebool test 2 (10, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add16(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint16) => ebool test 3 (14, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(14n);
    input.add16(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint16) => ebool test 4 (14, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(14n);
    input.add16(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint16) => ebool test 1 (1, 53247)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(1n);
    input.add16(53247n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint16) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add16(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint16) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add16(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint16) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add16(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint4, euint16) => ebool test 1 (13, 43765)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);
    input.add16(43765n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint16) => ebool test 2 (9, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);
    input.add16(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint16) => ebool test 3 (13, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);
    input.add16(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint16) => ebool test 4 (13, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);
    input.add16(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint16) => ebool test 1 (14, 62468)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(14n);
    input.add16(62468n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint16) => ebool test 2 (10, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add16(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint16) => ebool test 3 (14, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(14n);
    input.add16(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint16) => ebool test 4 (14, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(14n);
    input.add16(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint16) => ebool test 1 (10, 24653)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add16(24653n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint16) => ebool test 2 (6, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(6n);
    input.add16(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint16) => ebool test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add16(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint16) => ebool test 4 (10, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add16(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint4, euint16) => euint16 test 1 (1, 50108)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(1n);
    input.add16(50108n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(1n);
  });

  it('test operator "min" overload (euint4, euint16) => euint16 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add16(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint4, euint16) => euint16 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add16(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint4, euint16) => euint16 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add16(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint4, euint16) => euint16 test 1 (14, 33411)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(14n);
    input.add16(33411n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(33411n);
  });

  it('test operator "max" overload (euint4, euint16) => euint16 test 2 (10, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add16(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(14n);
  });

  it('test operator "max" overload (euint4, euint16) => euint16 test 3 (14, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(14n);
    input.add16(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(14n);
  });

  it('test operator "max" overload (euint4, euint16) => euint16 test 4 (14, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(14n);
    input.add16(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract1.res16());
    expect(res).to.equal(14n);
  });

  it('test operator "add" overload (euint4, euint32) => euint32 test 1 (2, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(2n);
    input.add32(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(11n);
  });

  it('test operator "add" overload (euint4, euint32) => euint32 test 2 (4, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add32(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint4, euint32) => euint32 test 3 (6, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(6n);
    input.add32(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint4, euint32) => euint32 test 4 (6, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(6n);
    input.add32(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(10n);
  });

  it('test operator "sub" overload (euint4, euint32) => euint32 test 1 (13, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);
    input.add32(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint4, euint32) => euint32 test 2 (13, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);
    input.add32(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint4, euint32) => euint32 test 1 (2, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(2n);
    input.add32(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(10n);
  });

  it('test operator "mul" overload (euint4, euint32) => euint32 test 2 (3, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(3n);
    input.add32(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(12n);
  });

  it('test operator "mul" overload (euint4, euint32) => euint32 test 3 (3, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(3n);
    input.add32(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, euint32) => euint32 test 4 (4, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add32(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(12n);
  });

  it('test operator "and" overload (euint4, euint32) => euint32 test 1 (6, 3666388192)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(6n);
    input.add32(3666388192n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint4, euint32) => euint32 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add32(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint4, euint32) => euint32 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add32(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint4, euint32) => euint32 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add32(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint4, euint32) => euint32 test 1 (13, 371092847)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);
    input.add32(371092847n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(371092847n);
  });

  it('test operator "or" overload (euint4, euint32) => euint32 test 2 (9, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);
    input.add32(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(13n);
  });

  it('test operator "or" overload (euint4, euint32) => euint32 test 3 (13, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);
    input.add32(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(13n);
  });

  it('test operator "or" overload (euint4, euint32) => euint32 test 4 (13, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);
    input.add32(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(13n);
  });

  it('test operator "xor" overload (euint4, euint32) => euint32 test 1 (1, 1939413163)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(1n);
    input.add32(1939413163n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(1939413162n);
  });

  it('test operator "xor" overload (euint4, euint32) => euint32 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add32(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint4, euint32) => euint32 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add32(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint4, euint32) => euint32 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add32(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint4, euint32) => ebool test 1 (9, 1089894566)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);
    input.add32(1089894566n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint32) => ebool test 2 (5, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(5n);
    input.add32(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint32) => ebool test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);
    input.add32(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, euint32) => ebool test 4 (9, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);
    input.add32(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint32) => ebool test 1 (5, 2818677977)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(5n);
    input.add32(2818677977n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint32) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add32(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint32) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add32(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint32) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add32(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint32) => ebool test 1 (1, 2262617776)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(1n);
    input.add32(2262617776n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint32) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add32(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint32) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add32(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint32) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add32(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint4, euint32) => ebool test 1 (8, 1791761269)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add32(1791761269n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint32) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add32(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint32) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add32(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint32) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add32(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint32) => ebool test 1 (4, 1782114748)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add32(1782114748n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint32) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add32(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint32) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add32(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint32) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add32(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint32) => ebool test 1 (1, 3154883490)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(1n);
    input.add32(3154883490n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint32) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add32(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint32) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add32(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint32) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add32(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint4, euint32) => euint32 test 1 (9, 2831156674)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);
    input.add32(2831156674n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(9n);
  });

  it('test operator "min" overload (euint4, euint32) => euint32 test 2 (5, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(5n);
    input.add32(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(5n);
  });

  it('test operator "min" overload (euint4, euint32) => euint32 test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);
    input.add32(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(9n);
  });

  it('test operator "min" overload (euint4, euint32) => euint32 test 4 (9, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);
    input.add32(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(5n);
  });

  it('test operator "max" overload (euint4, euint32) => euint32 test 1 (12, 623453237)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(12n);
    input.add32(623453237n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(623453237n);
  });

  it('test operator "max" overload (euint4, euint32) => euint32 test 2 (8, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add32(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(12n);
  });

  it('test operator "max" overload (euint4, euint32) => euint32 test 3 (12, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(12n);
    input.add32(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(12n);
  });

  it('test operator "max" overload (euint4, euint32) => euint32 test 4 (12, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(12n);
    input.add32(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract1.res32());
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint4, euint64) => euint64 test 1 (1, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(1n);
    input.add64(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint4, euint64) => euint64 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add64(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint4, euint64) => euint64 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(5n);
    input.add64(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint4, euint64) => euint64 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add64(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint4, euint64) => euint64 test 1 (12, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(12n);
    input.add64(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint4, euint64) => euint64 test 2 (12, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(12n);
    input.add64(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint4, euint64) => euint64 test 1 (2, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(2n);
    input.add64(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(10n);
  });

  it('test operator "mul" overload (euint4, euint64) => euint64 test 2 (3, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(3n);
    input.add64(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(15n);
  });

  it('test operator "mul" overload (euint4, euint64) => euint64 test 3 (3, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(3n);
    input.add64(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, euint64) => euint64 test 4 (5, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(5n);
    input.add64(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(15n);
  });

  it('test operator "and" overload (euint4, euint64) => euint64 test 1 (6, 18438764365497303935)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(6n);
    input.add64(18438764365497303935n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(6n);
  });

  it('test operator "and" overload (euint4, euint64) => euint64 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add64(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint4, euint64) => euint64 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add64(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint4, euint64) => euint64 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add64(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint4, euint64) => euint64 test 1 (14, 18440698678220010551)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(14n);
    input.add64(18440698678220010551n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(18440698678220010559n);
  });

  it('test operator "or" overload (euint4, euint64) => euint64 test 2 (10, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add64(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(14n);
  });

  it('test operator "or" overload (euint4, euint64) => euint64 test 3 (14, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(14n);
    input.add64(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(14n);
  });

  it('test operator "or" overload (euint4, euint64) => euint64 test 4 (14, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(14n);
    input.add64(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(14n);
  });

  it('test operator "xor" overload (euint4, euint64) => euint64 test 1 (2, 18438078325844207475)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(2n);
    input.add64(18438078325844207475n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(18438078325844207473n);
  });

  it('test operator "xor" overload (euint4, euint64) => euint64 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add64(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint4, euint64) => euint64 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add64(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint4, euint64) => euint64 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add64(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint4, euint64) => ebool test 1 (3, 18443050121569259433)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(3n);
    input.add64(18443050121569259433n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint64) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add64(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint64) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add64(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, euint64) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add64(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint64) => ebool test 1 (7, 18446555094221865045)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(7n);
    input.add64(18446555094221865045n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint64) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add64(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint64) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add64(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint64) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add64(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint64) => ebool test 1 (12, 18446558548033148537)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(12n);
    input.add64(18446558548033148537n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint64) => ebool test 2 (8, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add64(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint64) => ebool test 3 (12, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(12n);
    input.add64(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint64) => ebool test 4 (12, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(12n);
    input.add64(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint4, euint64) => ebool test 1 (11, 18445624350177245281)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(11n);
    input.add64(18445624350177245281n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint64) => ebool test 2 (7, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(7n);
    input.add64(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint64) => ebool test 3 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(11n);
    input.add64(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint64) => ebool test 4 (11, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(11n);
    input.add64(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint64) => ebool test 1 (14, 18446414069240547725)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(14n);
    input.add64(18446414069240547725n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint64) => ebool test 2 (10, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add64(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint64) => ebool test 3 (14, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(14n);
    input.add64(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint4, euint64) => ebool test 4 (14, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(14n);
    input.add64(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint64) => ebool test 1 (13, 18441885120383916401)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);
    input.add64(18441885120383916401n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint64) => ebool test 2 (9, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);
    input.add64(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint4, euint64) => ebool test 3 (13, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);
    input.add64(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint4, euint64) => ebool test 4 (13, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);
    input.add64(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint4, euint64) => euint64 test 1 (12, 18440035039683442233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(12n);
    input.add64(18440035039683442233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(12n);
  });

  it('test operator "min" overload (euint4, euint64) => euint64 test 2 (8, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add64(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint4, euint64) => euint64 test 3 (12, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(12n);
    input.add64(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(12n);
  });

  it('test operator "min" overload (euint4, euint64) => euint64 test 4 (12, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(12n);
    input.add64(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint4, euint64) => euint64 test 1 (9, 18439762941056895609)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);
    input.add64(18439762941056895609n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(18439762941056895609n);
  });

  it('test operator "max" overload (euint4, euint64) => euint64 test 2 (5, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(5n);
    input.add64(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(9n);
  });

  it('test operator "max" overload (euint4, euint64) => euint64 test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);
    input.add64(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(9n);
  });

  it('test operator "max" overload (euint4, euint64) => euint64 test 4 (9, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);
    input.add64(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint4_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract1.res64());
    expect(res).to.equal(9n);
  });

  it('test operator "add" overload (euint4, euint128) => euint128 test 1 (2, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(2n);
    input.add128(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.res128());
    expect(res).to.equal(11n);
  });

  it('test operator "add" overload (euint4, euint128) => euint128 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add128(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.res128());
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint4, euint128) => euint128 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(5n);
    input.add128(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.res128());
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint4, euint128) => euint128 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add128(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint4_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.res128());
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint4, euint128) => euint128 test 1 (13, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);
    input.add128(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint4_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint4, euint128) => euint128 test 2 (13, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);
    input.add128(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint4_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint4, euint128) => euint128 test 1 (2, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(2n);
    input.add128(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.res128());
    expect(res).to.equal(10n);
  });

  it('test operator "mul" overload (euint4, euint128) => euint128 test 2 (3, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(3n);
    input.add128(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.res128());
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, euint128) => euint128 test 3 (3, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(3n);
    input.add128(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.res128());
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint4, euint128) => euint128 test 4 (3, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(3n);
    input.add128(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint4_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.res128());
    expect(res).to.equal(9n);
  });

  it('test operator "and" overload (euint4, euint128) => euint128 test 1 (8, 340282366920938463463367802109078157213)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add128(340282366920938463463367802109078157213n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.res128());
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint4, euint128) => euint128 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add128(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint4, euint128) => euint128 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add128(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.res128());
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint4, euint128) => euint128 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add128(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint4_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint4, euint128) => euint128 test 1 (4, 340282366920938463463366343089484611447)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add128(340282366920938463463366343089484611447n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.res128());
    expect(res).to.equal(340282366920938463463366343089484611447n);
  });

  it('test operator "or" overload (euint4, euint128) => euint128 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add128(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.res128());
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint4, euint128) => euint128 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add128(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.res128());
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint4, euint128) => euint128 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add128(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint4_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.res128());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint4, euint128) => euint128 test 1 (13, 340282366920938463463366123999290070707)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);
    input.add128(340282366920938463463366123999290070707n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.res128());
    expect(res).to.equal(340282366920938463463366123999290070718n);
  });

  it('test operator "xor" overload (euint4, euint128) => euint128 test 2 (9, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);
    input.add128(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint4, euint128) => euint128 test 3 (13, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);
    input.add128(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint4, euint128) => euint128 test 4 (13, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);
    input.add128(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint4_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract1.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint4, euint128) => ebool test 1 (10, 340282366920938463463370073243865312497)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add128(340282366920938463463370073243865312497n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint128) => ebool test 2 (6, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(6n);
    input.add128(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint4, euint128) => ebool test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add128(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint4, euint128) => ebool test 4 (10, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add128(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint4_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint128) => ebool test 1 (13, 340282366920938463463368725666766226609)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);
    input.add128(340282366920938463463368725666766226609n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint128) => ebool test 2 (9, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(9n);
    input.add128(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint4, euint128) => ebool test 3 (13, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);
    input.add128(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint4, euint128) => ebool test 4 (13, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(13n);
    input.add128(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint4_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint128) => ebool test 1 (3, 340282366920938463463374467311851431135)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(3n);
    input.add128(340282366920938463463374467311851431135n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint128) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(4n);
    input.add128(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint4, euint128) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add128(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint4, euint128) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(8n);
    input.add128(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint4_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint4, euint128) => ebool test 1 (14, 340282366920938463463369565306924367885)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(14n);
    input.add128(340282366920938463463369565306924367885n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint128) => ebool test 2 (10, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(10n);
    input.add128(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint128) => ebool test 3 (14, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(14n);
    input.add128(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint4, euint128) => ebool test 4 (14, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add4(14n);
    input.add128(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint4_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract1.resb());
    expect(res).to.equal(true);
  });
});
