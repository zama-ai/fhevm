import { expect } from 'chai';
import { ethers } from 'hardhat';

import type { TFHETestSuite1 } from '../../types/contracts/tests/TFHETestSuite1';
import type { TFHETestSuite2 } from '../../types/contracts/tests/TFHETestSuite2';
import type { TFHETestSuite3 } from '../../types/contracts/tests/TFHETestSuite3';
import type { TFHETestSuite4 } from '../../types/contracts/tests/TFHETestSuite4';
import type { TFHETestSuite5 } from '../../types/contracts/tests/TFHETestSuite5';
import type { TFHETestSuite6 } from '../../types/contracts/tests/TFHETestSuite6';
import type { TFHETestSuite7 } from '../../types/contracts/tests/TFHETestSuite7';
import type { TFHETestSuite8 } from '../../types/contracts/tests/TFHETestSuite8';
import type { TFHETestSuite9 } from '../../types/contracts/tests/TFHETestSuite9';
import type { TFHETestSuite10 } from '../../types/contracts/tests/TFHETestSuite10';
import type { TFHETestSuite11 } from '../../types/contracts/tests/TFHETestSuite11';
import {
  createInstances,
  decrypt4,
  decrypt8,
  decrypt16,
  decrypt32,
  decrypt64,
  decrypt128,
  decrypt256,
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

describe('TFHE operations 9', function () {
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

  it('test operator "add" overload (euint128, euint4) => euint128 test 1 (9, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(9n);
    input.add4(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.add_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(11n);
  });

  it('test operator "add" overload (euint128, euint4) => euint128 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.add_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint128, euint4) => euint128 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(5n);
    input.add4(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.add_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint128, euint4) => euint128 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.add_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint128, euint4) => euint128 test 1 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.sub_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint128, euint4) => euint128 test 2 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.sub_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint128, euint4) => euint128 test 1 (5, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(5n);
    input.add4(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.mul_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(10n);
  });

  it('test operator "mul" overload (euint128, euint4) => euint128 test 2 (3, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(3n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.mul_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(12n);
  });

  it('test operator "mul" overload (euint128, euint4) => euint128 test 3 (3, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(3n);
    input.add4(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.mul_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint128, euint4) => euint128 test 4 (4, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(4n);
    input.add4(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.mul_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(12n);
  });

  it('test operator "and" overload (euint128, euint4) => euint128 test 1 (340282366920938463463372547368360839357, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463372547368360839357n);
    input.add4(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.and_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint128, euint4) => euint128 test 2 (6, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(6n);
    input.add4(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.and_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(2n);
  });

  it('test operator "and" overload (euint128, euint4) => euint128 test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(10n);
    input.add4(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.and_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(10n);
  });

  it('test operator "and" overload (euint128, euint4) => euint128 test 4 (10, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(10n);
    input.add4(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.and_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(2n);
  });

  it('test operator "or" overload (euint128, euint4) => euint128 test 1 (340282366920938463463366694641055128577, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463366694641055128577n);
    input.add4(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.or_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(340282366920938463463366694641055128589n);
  });

  it('test operator "or" overload (euint128, euint4) => euint128 test 2 (8, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(8n);
    input.add4(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.or_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint128, euint4) => euint128 test 3 (12, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(12n);
    input.add4(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.or_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint128, euint4) => euint128 test 4 (12, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(12n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.or_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint128, euint4) => euint128 test 1 (340282366920938463463370996799476031467, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463370996799476031467n);
    input.add4(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.xor_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(340282366920938463463370996799476031462n);
  });

  it('test operator "xor" overload (euint128, euint4) => euint128 test 2 (9, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(9n);
    input.add4(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.xor_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint128, euint4) => euint128 test 3 (13, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(13n);
    input.add4(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.xor_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint128, euint4) => euint128 test 4 (13, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(13n);
    input.add4(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.xor_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint128, euint4) => ebool test 1 (340282366920938463463369794780094019941, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463369794780094019941n);
    input.add4(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.eq_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint4) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.eq_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint4) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.eq_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint128, euint4) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.eq_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint4) => ebool test 1 (340282366920938463463371595006609590711, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463371595006609590711n);
    input.add4(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ne_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint4) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ne_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint4) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ne_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint4) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ne_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint4) => ebool test 1 (340282366920938463463366351520407667333, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463366351520407667333n);
    input.add4(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ge_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint4) => ebool test 2 (8, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(8n);
    input.add4(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ge_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint128, euint4) => ebool test 3 (12, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(12n);
    input.add4(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ge_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint4) => ebool test 4 (12, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(12n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ge_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint4) => ebool test 1 (340282366920938463463367426041495203879, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463367426041495203879n);
    input.add4(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint4) => ebool test 2 (9, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(9n);
    input.add4(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint4) => ebool test 3 (13, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(13n);
    input.add4(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint4) => ebool test 4 (13, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(13n);
    input.add4(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint4) => ebool test 1 (340282366920938463463373221658110991435, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463373221658110991435n);
    input.add4(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint128, euint4) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint4) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint4) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint4) => ebool test 1 (340282366920938463463368530671945258801, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463368530671945258801n);
    input.add4(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint4) => ebool test 2 (8, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(8n);
    input.add4(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint128, euint4) => ebool test 3 (12, 12)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(12n);
    input.add4(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint4) => ebool test 4 (12, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(12n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint128, euint4) => euint128 test 1 (340282366920938463463373617043976262819, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463373617043976262819n);
    input.add4(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(6n);
  });

  it('test operator "min" overload (euint128, euint4) => euint128 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "min" overload (euint128, euint4) => euint128 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint128, euint4) => euint128 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "max" overload (euint128, euint4) => euint128 test 1 (340282366920938463463368883145053125641, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463368883145053125641n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(340282366920938463463368883145053125641n);
  });

  it('test operator "max" overload (euint128, euint4) => euint128 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint128, euint4) => euint128 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint128, euint4) => euint128 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_euint128_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint128, euint8) => euint128 test 1 (129, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(129n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.add_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(131n);
  });

  it('test operator "add" overload (euint128, euint8) => euint128 test 2 (118, 120)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(118n);
    input.add8(120n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.add_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(238n);
  });

  it('test operator "add" overload (euint128, euint8) => euint128 test 3 (120, 120)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(120n);
    input.add8(120n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.add_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(240n);
  });

  it('test operator "add" overload (euint128, euint8) => euint128 test 4 (120, 118)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(120n);
    input.add8(118n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.add_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(238n);
  });

  it('test operator "sub" overload (euint128, euint8) => euint128 test 1 (232, 232)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(232n);
    input.add8(232n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.sub_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint128, euint8) => euint128 test 2 (232, 228)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(232n);
    input.add8(228n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.sub_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint128, euint8) => euint128 test 1 (65, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(65n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.mul_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(130n);
  });

  it('test operator "mul" overload (euint128, euint8) => euint128 test 2 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(9n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.mul_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(81n);
  });

  it('test operator "mul" overload (euint128, euint8) => euint128 test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(9n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.mul_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(81n);
  });

  it('test operator "mul" overload (euint128, euint8) => euint128 test 4 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(9n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.mul_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(81n);
  });

  it('test operator "and" overload (euint128, euint8) => euint128 test 1 (340282366920938463463366313789187731763, 114)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463366313789187731763n);
    input.add8(114n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.and_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(50n);
  });

  it('test operator "and" overload (euint128, euint8) => euint128 test 2 (110, 114)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(110n);
    input.add8(114n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.and_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(98n);
  });

  it('test operator "and" overload (euint128, euint8) => euint128 test 3 (114, 114)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(114n);
    input.add8(114n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.and_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(114n);
  });

  it('test operator "and" overload (euint128, euint8) => euint128 test 4 (114, 110)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(114n);
    input.add8(110n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.and_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(98n);
  });

  it('test operator "or" overload (euint128, euint8) => euint128 test 1 (340282366920938463463374227521343598445, 112)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463374227521343598445n);
    input.add8(112n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.or_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(340282366920938463463374227521343598461n);
  });

  it('test operator "or" overload (euint128, euint8) => euint128 test 2 (108, 112)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(108n);
    input.add8(112n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.or_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(124n);
  });

  it('test operator "or" overload (euint128, euint8) => euint128 test 3 (112, 112)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(112n);
    input.add8(112n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.or_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(112n);
  });

  it('test operator "or" overload (euint128, euint8) => euint128 test 4 (112, 108)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(112n);
    input.add8(108n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.or_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(124n);
  });

  it('test operator "xor" overload (euint128, euint8) => euint128 test 1 (340282366920938463463373857001835170765, 82)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463373857001835170765n);
    input.add8(82n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.xor_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(340282366920938463463373857001835170719n);
  });

  it('test operator "xor" overload (euint128, euint8) => euint128 test 2 (78, 82)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(78n);
    input.add8(82n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.xor_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(28n);
  });

  it('test operator "xor" overload (euint128, euint8) => euint128 test 3 (82, 82)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(82n);
    input.add8(82n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.xor_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint128, euint8) => euint128 test 4 (82, 78)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(82n);
    input.add8(78n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.xor_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(28n);
  });

  it('test operator "eq" overload (euint128, euint8) => ebool test 1 (340282366920938463463368012116669397375, 111)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463368012116669397375n);
    input.add8(111n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.eq_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint8) => ebool test 2 (107, 111)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(107n);
    input.add8(111n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.eq_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint8) => ebool test 3 (111, 111)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(111n);
    input.add8(111n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.eq_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint128, euint8) => ebool test 4 (111, 107)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(111n);
    input.add8(107n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.eq_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint8) => ebool test 1 (340282366920938463463374264796523764465, 244)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463374264796523764465n);
    input.add8(244n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ne_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint8) => ebool test 2 (240, 244)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(240n);
    input.add8(244n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ne_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint8) => ebool test 3 (244, 244)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(244n);
    input.add8(244n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ne_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint8) => ebool test 4 (244, 240)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(244n);
    input.add8(240n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ne_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint8) => ebool test 1 (340282366920938463463365913686345200467, 55)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463365913686345200467n);
    input.add8(55n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ge_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint8) => ebool test 2 (51, 55)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(51n);
    input.add8(55n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ge_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint128, euint8) => ebool test 3 (55, 55)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(55n);
    input.add8(55n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ge_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint8) => ebool test 4 (55, 51)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(55n);
    input.add8(51n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ge_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint8) => ebool test 1 (340282366920938463463373004399886925283, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463373004399886925283n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint8) => ebool test 2 (5, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(5n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint8) => ebool test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(9n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint8) => ebool test 4 (9, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(9n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint8) => ebool test 1 (340282366920938463463373663014854424779, 48)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463373663014854424779n);
    input.add8(48n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint128, euint8) => ebool test 2 (44, 48)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(44n);
    input.add8(48n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint8) => ebool test 3 (48, 48)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(48n);
    input.add8(48n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint8) => ebool test 4 (48, 44)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(48n);
    input.add8(44n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint8) => ebool test 1 (340282366920938463463371295793146087759, 46)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463371295793146087759n);
    input.add8(46n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint8) => ebool test 2 (42, 46)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(42n);
    input.add8(46n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint128, euint8) => ebool test 3 (46, 46)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(46n);
    input.add8(46n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint8) => ebool test 4 (46, 42)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(46n);
    input.add8(42n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint128, euint8) => euint128 test 1 (340282366920938463463366071622027178861, 33)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463366071622027178861n);
    input.add8(33n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(33n);
  });

  it('test operator "min" overload (euint128, euint8) => euint128 test 2 (29, 33)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(29n);
    input.add8(33n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(29n);
  });

  it('test operator "min" overload (euint128, euint8) => euint128 test 3 (33, 33)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(33n);
    input.add8(33n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(33n);
  });

  it('test operator "min" overload (euint128, euint8) => euint128 test 4 (33, 29)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(33n);
    input.add8(29n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(29n);
  });

  it('test operator "max" overload (euint128, euint8) => euint128 test 1 (340282366920938463463369691386082749829, 49)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463369691386082749829n);
    input.add8(49n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(340282366920938463463369691386082749829n);
  });

  it('test operator "max" overload (euint128, euint8) => euint128 test 2 (45, 49)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(45n);
    input.add8(49n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(49n);
  });

  it('test operator "max" overload (euint128, euint8) => euint128 test 3 (49, 49)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(49n);
    input.add8(49n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(49n);
  });

  it('test operator "max" overload (euint128, euint8) => euint128 test 4 (49, 45)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(49n);
    input.add8(45n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(49n);
  });

  it('test operator "add" overload (euint128, euint16) => euint128 test 1 (32769, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(32769n);
    input.add16(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.add_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(32771n);
  });

  it('test operator "add" overload (euint128, euint16) => euint128 test 2 (31253, 31255)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(31253n);
    input.add16(31255n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.add_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(62508n);
  });

  it('test operator "add" overload (euint128, euint16) => euint128 test 3 (31255, 31255)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(31255n);
    input.add16(31255n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.add_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(62510n);
  });

  it('test operator "add" overload (euint128, euint16) => euint128 test 4 (31255, 31253)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(31255n);
    input.add16(31253n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.add_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(62508n);
  });

  it('test operator "sub" overload (euint128, euint16) => euint128 test 1 (25392, 25392)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(25392n);
    input.add16(25392n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.sub_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint128, euint16) => euint128 test 2 (25392, 25388)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(25392n);
    input.add16(25388n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.sub_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint128, euint16) => euint128 test 1 (16385, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(16385n);
    input.add16(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.mul_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(32770n);
  });

  it('test operator "mul" overload (euint128, euint16) => euint128 test 2 (233, 233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(233n);
    input.add16(233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.mul_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(54289n);
  });

  it('test operator "mul" overload (euint128, euint16) => euint128 test 3 (233, 233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(233n);
    input.add16(233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.mul_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(54289n);
  });

  it('test operator "mul" overload (euint128, euint16) => euint128 test 4 (233, 233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(233n);
    input.add16(233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.mul_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(54289n);
  });

  it('test operator "and" overload (euint128, euint16) => euint128 test 1 (340282366920938463463371274545487982339, 11460)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463371274545487982339n);
    input.add16(11460n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.and_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(11264n);
  });

  it('test operator "and" overload (euint128, euint16) => euint128 test 2 (11456, 11460)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(11456n);
    input.add16(11460n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.and_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(11456n);
  });

  it('test operator "and" overload (euint128, euint16) => euint128 test 3 (11460, 11460)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(11460n);
    input.add16(11460n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.and_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(11460n);
  });

  it('test operator "and" overload (euint128, euint16) => euint128 test 4 (11460, 11456)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(11460n);
    input.add16(11456n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.and_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(11456n);
  });

  it('test operator "or" overload (euint128, euint16) => euint128 test 1 (340282366920938463463366623321205018087, 7136)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463366623321205018087n);
    input.add16(7136n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.or_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(340282366920938463463366623321205022695n);
  });

  it('test operator "or" overload (euint128, euint16) => euint128 test 2 (7132, 7136)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(7132n);
    input.add16(7136n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.or_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(7164n);
  });

  it('test operator "or" overload (euint128, euint16) => euint128 test 3 (7136, 7136)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(7136n);
    input.add16(7136n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.or_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(7136n);
  });

  it('test operator "or" overload (euint128, euint16) => euint128 test 4 (7136, 7132)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(7136n);
    input.add16(7132n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.or_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(7164n);
  });

  it('test operator "xor" overload (euint128, euint16) => euint128 test 1 (340282366920938463463371581047560381685, 64364)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463371581047560381685n);
    input.add16(64364n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.xor_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(340282366920938463463371581047560363929n);
  });

  it('test operator "xor" overload (euint128, euint16) => euint128 test 2 (64360, 64364)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(64360n);
    input.add16(64364n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.xor_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint128, euint16) => euint128 test 3 (64364, 64364)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(64364n);
    input.add16(64364n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.xor_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint128, euint16) => euint128 test 4 (64364, 64360)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(64364n);
    input.add16(64360n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.xor_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint128, euint16) => ebool test 1 (340282366920938463463365947218770561253, 63555)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463365947218770561253n);
    input.add16(63555n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.eq_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint16) => ebool test 2 (63551, 63555)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(63551n);
    input.add16(63555n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.eq_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint16) => ebool test 3 (63555, 63555)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(63555n);
    input.add16(63555n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.eq_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint128, euint16) => ebool test 4 (63555, 63551)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(63555n);
    input.add16(63551n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.eq_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint16) => ebool test 1 (340282366920938463463370265865713339199, 54670)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463370265865713339199n);
    input.add16(54670n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ne_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint16) => ebool test 2 (54666, 54670)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(54666n);
    input.add16(54670n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ne_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint16) => ebool test 3 (54670, 54670)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(54670n);
    input.add16(54670n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ne_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint16) => ebool test 4 (54670, 54666)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(54670n);
    input.add16(54666n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ne_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint16) => ebool test 1 (340282366920938463463373250379836355353, 44459)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463373250379836355353n);
    input.add16(44459n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ge_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint16) => ebool test 2 (44455, 44459)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(44455n);
    input.add16(44459n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ge_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint128, euint16) => ebool test 3 (44459, 44459)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(44459n);
    input.add16(44459n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ge_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint16) => ebool test 4 (44459, 44455)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(44459n);
    input.add16(44455n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ge_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint16) => ebool test 1 (340282366920938463463368543313259159187, 51225)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463368543313259159187n);
    input.add16(51225n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint16) => ebool test 2 (51221, 51225)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(51221n);
    input.add16(51225n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint16) => ebool test 3 (51225, 51225)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(51225n);
    input.add16(51225n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint16) => ebool test 4 (51225, 51221)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(51225n);
    input.add16(51221n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint16) => ebool test 1 (340282366920938463463371412966556965647, 54237)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463371412966556965647n);
    input.add16(54237n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint128, euint16) => ebool test 2 (54233, 54237)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(54233n);
    input.add16(54237n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint16) => ebool test 3 (54237, 54237)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(54237n);
    input.add16(54237n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint16) => ebool test 4 (54237, 54233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(54237n);
    input.add16(54233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint16) => ebool test 1 (340282366920938463463374223690909641239, 37855)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463374223690909641239n);
    input.add16(37855n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint16) => ebool test 2 (37851, 37855)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(37851n);
    input.add16(37855n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint128, euint16) => ebool test 3 (37855, 37855)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(37855n);
    input.add16(37855n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint16) => ebool test 4 (37855, 37851)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(37855n);
    input.add16(37851n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint128, euint16) => euint128 test 1 (340282366920938463463368542801038101753, 9714)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463368542801038101753n);
    input.add16(9714n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(9714n);
  });

  it('test operator "min" overload (euint128, euint16) => euint128 test 2 (9710, 9714)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(9710n);
    input.add16(9714n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(9710n);
  });

  it('test operator "min" overload (euint128, euint16) => euint128 test 3 (9714, 9714)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(9714n);
    input.add16(9714n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(9714n);
  });

  it('test operator "min" overload (euint128, euint16) => euint128 test 4 (9714, 9710)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(9714n);
    input.add16(9710n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(9710n);
  });

  it('test operator "max" overload (euint128, euint16) => euint128 test 1 (340282366920938463463370317782456018271, 38332)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463370317782456018271n);
    input.add16(38332n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(340282366920938463463370317782456018271n);
  });

  it('test operator "max" overload (euint128, euint16) => euint128 test 2 (38328, 38332)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(38328n);
    input.add16(38332n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(38332n);
  });

  it('test operator "max" overload (euint128, euint16) => euint128 test 3 (38332, 38332)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(38332n);
    input.add16(38332n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(38332n);
  });

  it('test operator "max" overload (euint128, euint16) => euint128 test 4 (38332, 38328)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(38332n);
    input.add16(38328n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(38332n);
  });

  it('test operator "add" overload (euint128, euint32) => euint128 test 1 (2147483649, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(2147483649n);
    input.add32(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.add_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(2147483651n);
  });

  it('test operator "add" overload (euint128, euint32) => euint128 test 2 (417384794, 417384798)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(417384794n);
    input.add32(417384798n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.add_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(834769592n);
  });

  it('test operator "add" overload (euint128, euint32) => euint128 test 3 (417384798, 417384798)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(417384798n);
    input.add32(417384798n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.add_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(834769596n);
  });

  it('test operator "add" overload (euint128, euint32) => euint128 test 4 (417384798, 417384794)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(417384798n);
    input.add32(417384794n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.add_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(834769592n);
  });

  it('test operator "sub" overload (euint128, euint32) => euint128 test 1 (373346389, 373346389)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(373346389n);
    input.add32(373346389n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.sub_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint128, euint32) => euint128 test 2 (373346389, 373346385)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(373346389n);
    input.add32(373346385n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.sub_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint128, euint32) => euint128 test 1 (1073741825, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(1073741825n);
    input.add32(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.mul_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(2147483650n);
  });

  it('test operator "mul" overload (euint128, euint32) => euint128 test 2 (46331, 46331)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(46331n);
    input.add32(46331n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.mul_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(2146561561n);
  });

  it('test operator "mul" overload (euint128, euint32) => euint128 test 3 (46331, 46331)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(46331n);
    input.add32(46331n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.mul_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(2146561561n);
  });

  it('test operator "mul" overload (euint128, euint32) => euint128 test 4 (46331, 46331)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(46331n);
    input.add32(46331n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.mul_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(2146561561n);
  });

  it('test operator "and" overload (euint128, euint32) => euint128 test 1 (340282366920938463463372796408614652419, 1038177724)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463372796408614652419n);
    input.add32(1038177724n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.and_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(272629760n);
  });

  it('test operator "and" overload (euint128, euint32) => euint128 test 2 (1038177720, 1038177724)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(1038177720n);
    input.add32(1038177724n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.and_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(1038177720n);
  });

  it('test operator "and" overload (euint128, euint32) => euint128 test 3 (1038177724, 1038177724)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(1038177724n);
    input.add32(1038177724n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.and_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(1038177724n);
  });

  it('test operator "and" overload (euint128, euint32) => euint128 test 4 (1038177724, 1038177720)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(1038177724n);
    input.add32(1038177720n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.and_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(1038177720n);
  });

  it('test operator "or" overload (euint128, euint32) => euint128 test 1 (340282366920938463463370714852003960045, 1823556556)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463370714852003960045n);
    input.add32(1823556556n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.or_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(340282366920938463463370714853213034477n);
  });

  it('test operator "or" overload (euint128, euint32) => euint128 test 2 (1823556552, 1823556556)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(1823556552n);
    input.add32(1823556556n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.or_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(1823556556n);
  });

  it('test operator "or" overload (euint128, euint32) => euint128 test 3 (1823556556, 1823556556)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(1823556556n);
    input.add32(1823556556n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.or_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(1823556556n);
  });

  it('test operator "or" overload (euint128, euint32) => euint128 test 4 (1823556556, 1823556552)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(1823556556n);
    input.add32(1823556552n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.or_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(1823556556n);
  });

  it('test operator "xor" overload (euint128, euint32) => euint128 test 1 (340282366920938463463372336872535218607, 2309231003)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463372336872535218607n);
    input.add32(2309231003n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.xor_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(340282366920938463463372336874538256436n);
  });

  it('test operator "xor" overload (euint128, euint32) => euint128 test 2 (2309230999, 2309231003)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(2309230999n);
    input.add32(2309231003n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.xor_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint128, euint32) => euint128 test 3 (2309231003, 2309231003)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(2309231003n);
    input.add32(2309231003n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.xor_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint128, euint32) => euint128 test 4 (2309231003, 2309230999)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(2309231003n);
    input.add32(2309230999n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.xor_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint128, euint32) => ebool test 1 (340282366920938463463365905779679562131, 1923235589)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463365905779679562131n);
    input.add32(1923235589n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.eq_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint32) => ebool test 2 (1923235585, 1923235589)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(1923235585n);
    input.add32(1923235589n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.eq_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint32) => ebool test 3 (1923235589, 1923235589)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(1923235589n);
    input.add32(1923235589n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.eq_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint128, euint32) => ebool test 4 (1923235589, 1923235585)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(1923235589n);
    input.add32(1923235585n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.eq_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint32) => ebool test 1 (340282366920938463463374155295872552931, 1145053382)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463374155295872552931n);
    input.add32(1145053382n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ne_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint32) => ebool test 2 (1145053378, 1145053382)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(1145053378n);
    input.add32(1145053382n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ne_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint32) => ebool test 3 (1145053382, 1145053382)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(1145053382n);
    input.add32(1145053382n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ne_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint32) => ebool test 4 (1145053382, 1145053378)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(1145053382n);
    input.add32(1145053378n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ne_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint32) => ebool test 1 (340282366920938463463370684518045415615, 2276077429)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463370684518045415615n);
    input.add32(2276077429n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ge_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint32) => ebool test 2 (2276077425, 2276077429)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(2276077425n);
    input.add32(2276077429n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ge_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint128, euint32) => ebool test 3 (2276077429, 2276077429)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(2276077429n);
    input.add32(2276077429n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ge_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint32) => ebool test 4 (2276077429, 2276077425)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(2276077429n);
    input.add32(2276077425n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ge_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint32) => ebool test 1 (340282366920938463463371084318415272059, 235874492)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463371084318415272059n);
    input.add32(235874492n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint32) => ebool test 2 (235874488, 235874492)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(235874488n);
    input.add32(235874492n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint32) => ebool test 3 (235874492, 235874492)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(235874492n);
    input.add32(235874492n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint32) => ebool test 4 (235874492, 235874488)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(235874492n);
    input.add32(235874488n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint32) => ebool test 1 (340282366920938463463372847665266392859, 623901814)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463372847665266392859n);
    input.add32(623901814n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint128, euint32) => ebool test 2 (623901810, 623901814)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(623901810n);
    input.add32(623901814n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint32) => ebool test 3 (623901814, 623901814)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(623901814n);
    input.add32(623901814n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint32) => ebool test 4 (623901814, 623901810)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(623901814n);
    input.add32(623901810n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint32) => ebool test 1 (340282366920938463463373212421821063067, 1708404829)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463373212421821063067n);
    input.add32(1708404829n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint32) => ebool test 2 (1708404825, 1708404829)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(1708404825n);
    input.add32(1708404829n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint128, euint32) => ebool test 3 (1708404829, 1708404829)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(1708404829n);
    input.add32(1708404829n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint32) => ebool test 4 (1708404829, 1708404825)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(1708404829n);
    input.add32(1708404825n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint128, euint32) => euint128 test 1 (340282366920938463463373948652081205153, 4076998775)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463373948652081205153n);
    input.add32(4076998775n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(4076998775n);
  });

  it('test operator "min" overload (euint128, euint32) => euint128 test 2 (4076998771, 4076998775)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(4076998771n);
    input.add32(4076998775n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(4076998771n);
  });

  it('test operator "min" overload (euint128, euint32) => euint128 test 3 (4076998775, 4076998775)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(4076998775n);
    input.add32(4076998775n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(4076998775n);
  });

  it('test operator "min" overload (euint128, euint32) => euint128 test 4 (4076998775, 4076998771)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(4076998775n);
    input.add32(4076998771n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(4076998771n);
  });

  it('test operator "max" overload (euint128, euint32) => euint128 test 1 (340282366920938463463374411933353920761, 2226263511)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463374411933353920761n);
    input.add32(2226263511n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(340282366920938463463374411933353920761n);
  });

  it('test operator "max" overload (euint128, euint32) => euint128 test 2 (2226263507, 2226263511)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(2226263507n);
    input.add32(2226263511n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(2226263511n);
  });

  it('test operator "max" overload (euint128, euint32) => euint128 test 3 (2226263511, 2226263511)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(2226263511n);
    input.add32(2226263511n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(2226263511n);
  });

  it('test operator "max" overload (euint128, euint32) => euint128 test 4 (2226263511, 2226263507)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(2226263511n);
    input.add32(2226263507n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(2226263511n);
  });

  it('test operator "add" overload (euint128, euint64) => euint128 test 1 (9223372036854775809, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(9223372036854775809n);
    input.add64(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.add_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(9223372036854775811n);
  });

  it('test operator "add" overload (euint128, euint64) => euint128 test 2 (9219789077213425563, 9219789077213425565)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(9219789077213425563n);
    input.add64(9219789077213425565n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.add_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(18439578154426851128n);
  });

  it('test operator "add" overload (euint128, euint64) => euint128 test 3 (9219789077213425565, 9219789077213425565)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(9219789077213425565n);
    input.add64(9219789077213425565n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.add_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(18439578154426851130n);
  });

  it('test operator "add" overload (euint128, euint64) => euint128 test 4 (9219789077213425565, 9219789077213425563)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(9219789077213425565n);
    input.add64(9219789077213425563n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.add_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(18439578154426851128n);
  });

  it('test operator "sub" overload (euint128, euint64) => euint128 test 1 (18441111603467904997, 18441111603467904997)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(18441111603467904997n);
    input.add64(18441111603467904997n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.sub_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint128, euint64) => euint128 test 2 (18441111603467904997, 18441111603467904993)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(18441111603467904997n);
    input.add64(18441111603467904993n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.sub_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint128, euint64) => euint128 test 1 (4611686018427387905, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(4611686018427387905n);
    input.add64(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.mul_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(9223372036854775810n);
  });

  it('test operator "mul" overload (euint128, euint64) => euint128 test 2 (4294304633, 4294304633)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(4294304633n);
    input.add64(4294304633n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.mul_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(18441052281005264689n);
  });

  it('test operator "mul" overload (euint128, euint64) => euint128 test 3 (4294304633, 4294304633)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(4294304633n);
    input.add64(4294304633n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.mul_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(18441052281005264689n);
  });

  it('test operator "mul" overload (euint128, euint64) => euint128 test 4 (4294304633, 4294304633)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(4294304633n);
    input.add64(4294304633n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.mul_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(18441052281005264689n);
  });

  it('test operator "and" overload (euint128, euint64) => euint128 test 1 (340282366920938463463368706571541146173, 18442558600081086197)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463368706571541146173n);
    input.add64(18442558600081086197n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.and_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(18438019519441568309n);
  });

  it('test operator "and" overload (euint128, euint64) => euint128 test 2 (18442558600081086193, 18442558600081086197)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(18442558600081086193n);
    input.add64(18442558600081086197n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.and_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(18442558600081086193n);
  });

  it('test operator "and" overload (euint128, euint64) => euint128 test 3 (18442558600081086197, 18442558600081086197)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(18442558600081086197n);
    input.add64(18442558600081086197n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.and_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(18442558600081086197n);
  });

  it('test operator "and" overload (euint128, euint64) => euint128 test 4 (18442558600081086197, 18442558600081086193)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(18442558600081086197n);
    input.add64(18442558600081086193n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.and_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(18442558600081086193n);
  });

  it('test operator "or" overload (euint128, euint64) => euint128 test 1 (340282366920938463463370949519145006273, 18439052795332501691)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463370949519145006273n);
    input.add64(18439052795332501691n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.or_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(340282366920938463463372265437599364347n);
  });

  it('test operator "or" overload (euint128, euint64) => euint128 test 2 (18439052795332501687, 18439052795332501691)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(18439052795332501687n);
    input.add64(18439052795332501691n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.or_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(18439052795332501695n);
  });

  it('test operator "or" overload (euint128, euint64) => euint128 test 3 (18439052795332501691, 18439052795332501691)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(18439052795332501691n);
    input.add64(18439052795332501691n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.or_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(18439052795332501691n);
  });

  it('test operator "or" overload (euint128, euint64) => euint128 test 4 (18439052795332501691, 18439052795332501687)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(18439052795332501691n);
    input.add64(18439052795332501687n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.or_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(18439052795332501695n);
  });

  it('test operator "xor" overload (euint128, euint64) => euint128 test 1 (340282366920938463463368977308831072123, 18442960701720483125)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463368977308831072123n);
    input.add64(18442960701720483125n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.xor_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(340282366920938463444935023814972271182n);
  });

  it('test operator "xor" overload (euint128, euint64) => euint128 test 2 (18442960701720483121, 18442960701720483125)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(18442960701720483121n);
    input.add64(18442960701720483125n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.xor_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint128, euint64) => euint128 test 3 (18442960701720483125, 18442960701720483125)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(18442960701720483125n);
    input.add64(18442960701720483125n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.xor_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint128, euint64) => euint128 test 4 (18442960701720483125, 18442960701720483121)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(18442960701720483125n);
    input.add64(18442960701720483121n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.xor_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint128, euint64) => ebool test 1 (340282366920938463463373700954879453501, 18443047059286955551)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463373700954879453501n);
    input.add64(18443047059286955551n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.eq_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint64) => ebool test 2 (18443047059286955547, 18443047059286955551)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(18443047059286955547n);
    input.add64(18443047059286955551n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.eq_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint64) => ebool test 3 (18443047059286955551, 18443047059286955551)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(18443047059286955551n);
    input.add64(18443047059286955551n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.eq_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint128, euint64) => ebool test 4 (18443047059286955551, 18443047059286955547)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(18443047059286955551n);
    input.add64(18443047059286955547n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.eq_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint64) => ebool test 1 (340282366920938463463367381041406724807, 18441880962783858343)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463367381041406724807n);
    input.add64(18441880962783858343n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ne_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint64) => ebool test 2 (18441880962783858339, 18441880962783858343)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(18441880962783858339n);
    input.add64(18441880962783858343n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ne_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint64) => ebool test 3 (18441880962783858343, 18441880962783858343)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(18441880962783858343n);
    input.add64(18441880962783858343n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ne_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint64) => ebool test 4 (18441880962783858343, 18441880962783858339)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(18441880962783858343n);
    input.add64(18441880962783858339n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ne_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint64) => ebool test 1 (340282366920938463463371033714627819917, 18439188692806479297)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463371033714627819917n);
    input.add64(18439188692806479297n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ge_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint64) => ebool test 2 (18439188692806479293, 18439188692806479297)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(18439188692806479293n);
    input.add64(18439188692806479297n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ge_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint128, euint64) => ebool test 3 (18439188692806479297, 18439188692806479297)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(18439188692806479297n);
    input.add64(18439188692806479297n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ge_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint64) => ebool test 4 (18439188692806479297, 18439188692806479293)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(18439188692806479297n);
    input.add64(18439188692806479293n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ge_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint64) => ebool test 1 (340282366920938463463366930376033772101, 18439752237470295949)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463366930376033772101n);
    input.add64(18439752237470295949n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint64) => ebool test 2 (18439752237470295945, 18439752237470295949)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(18439752237470295945n);
    input.add64(18439752237470295949n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint64) => ebool test 3 (18439752237470295949, 18439752237470295949)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(18439752237470295949n);
    input.add64(18439752237470295949n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint64) => ebool test 4 (18439752237470295949, 18439752237470295945)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(18439752237470295949n);
    input.add64(18439752237470295945n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint64) => ebool test 1 (340282366920938463463366067679463384359, 18440549964651513569)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463366067679463384359n);
    input.add64(18440549964651513569n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint128, euint64) => ebool test 2 (18440549964651513565, 18440549964651513569)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(18440549964651513565n);
    input.add64(18440549964651513569n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint64) => ebool test 3 (18440549964651513569, 18440549964651513569)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(18440549964651513569n);
    input.add64(18440549964651513569n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint64) => ebool test 4 (18440549964651513569, 18440549964651513565)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(18440549964651513569n);
    input.add64(18440549964651513565n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.le_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint64) => ebool test 1 (340282366920938463463369469329980440219, 18446437448475645327)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463369469329980440219n);
    input.add64(18446437448475645327n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint64) => ebool test 2 (18446437448475645323, 18446437448475645327)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(18446437448475645323n);
    input.add64(18446437448475645327n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint128, euint64) => ebool test 3 (18446437448475645327, 18446437448475645327)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(18446437448475645327n);
    input.add64(18446437448475645327n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint64) => ebool test 4 (18446437448475645327, 18446437448475645323)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(18446437448475645327n);
    input.add64(18446437448475645323n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.lt_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint128, euint64) => euint128 test 1 (340282366920938463463372700048433567093, 18437970528445398453)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463372700048433567093n);
    input.add64(18437970528445398453n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(18437970528445398453n);
  });

  it('test operator "min" overload (euint128, euint64) => euint128 test 2 (18437970528445398449, 18437970528445398453)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(18437970528445398449n);
    input.add64(18437970528445398453n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(18437970528445398449n);
  });

  it('test operator "min" overload (euint128, euint64) => euint128 test 3 (18437970528445398453, 18437970528445398453)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(18437970528445398453n);
    input.add64(18437970528445398453n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(18437970528445398453n);
  });

  it('test operator "min" overload (euint128, euint64) => euint128 test 4 (18437970528445398453, 18437970528445398449)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(18437970528445398453n);
    input.add64(18437970528445398449n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.min_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(18437970528445398449n);
  });

  it('test operator "max" overload (euint128, euint64) => euint128 test 1 (340282366920938463463373867307519267223, 18442144243272238465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463373867307519267223n);
    input.add64(18442144243272238465n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(340282366920938463463373867307519267223n);
  });

  it('test operator "max" overload (euint128, euint64) => euint128 test 2 (18442144243272238461, 18442144243272238465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(18442144243272238461n);
    input.add64(18442144243272238465n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(18442144243272238465n);
  });

  it('test operator "max" overload (euint128, euint64) => euint128 test 3 (18442144243272238465, 18442144243272238465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(18442144243272238465n);
    input.add64(18442144243272238465n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(18442144243272238465n);
  });

  it('test operator "max" overload (euint128, euint64) => euint128 test 4 (18442144243272238465, 18442144243272238461)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(18442144243272238465n);
    input.add64(18442144243272238461n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(18442144243272238465n);
  });

  it('test operator "add" overload (euint128, euint128) => euint128 test 1 (170141183460469231731685489259224150459, 170141183460469231731686328709125723252)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(170141183460469231731685489259224150459n);
    input.add128(170141183460469231731686328709125723252n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.add_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(340282366920938463463371817968349873711n);
  });

  it('test operator "add" overload (euint128, euint128) => euint128 test 2 (170141183460469231731685489259224150457, 170141183460469231731685489259224150459)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(170141183460469231731685489259224150457n);
    input.add128(170141183460469231731685489259224150459n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.add_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(340282366920938463463370978518448300916n);
  });

  it('test operator "add" overload (euint128, euint128) => euint128 test 3 (170141183460469231731685489259224150459, 170141183460469231731685489259224150459)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(170141183460469231731685489259224150459n);
    input.add128(170141183460469231731685489259224150459n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.add_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(340282366920938463463370978518448300918n);
  });

  it('test operator "add" overload (euint128, euint128) => euint128 test 4 (170141183460469231731685489259224150459, 170141183460469231731685489259224150457)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(170141183460469231731685489259224150459n);
    input.add128(170141183460469231731685489259224150457n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.add_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(340282366920938463463370978518448300916n);
  });

  it('test operator "sub" overload (euint128, euint128) => euint128 test 1 (340282366920938463463366688248127833841, 340282366920938463463366688248127833841)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463366688248127833841n);
    input.add128(340282366920938463463366688248127833841n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.sub_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint128, euint128) => euint128 test 2 (340282366920938463463366688248127833841, 340282366920938463463366688248127833837)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463366688248127833841n);
    input.add128(340282366920938463463366688248127833837n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.sub_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint128, euint128) => euint128 test 1 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(9223372036854775809n);
    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.mul_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (euint128, euint128) => euint128 test 2 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(9223372036854775809n);
    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.mul_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (euint128, euint128) => euint128 test 3 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(9223372036854775809n);
    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.mul_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (euint128, euint128) => euint128 test 4 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(9223372036854775809n);
    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.mul_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "and" overload (euint128, euint128) => euint128 test 1 (340282366920938463463365754576278416105, 340282366920938463463366665489173165177)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463365754576278416105n);
    input.add128(340282366920938463463366665489173165177n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.and_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(340282366920938463463365750145734418537n);
  });

  it('test operator "and" overload (euint128, euint128) => euint128 test 2 (340282366920938463463365754576278416101, 340282366920938463463365754576278416105)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463365754576278416101n);
    input.add128(340282366920938463463365754576278416105n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.and_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(340282366920938463463365754576278416097n);
  });

  it('test operator "and" overload (euint128, euint128) => euint128 test 3 (340282366920938463463365754576278416105, 340282366920938463463365754576278416105)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463365754576278416105n);
    input.add128(340282366920938463463365754576278416105n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.and_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(340282366920938463463365754576278416105n);
  });

  it('test operator "and" overload (euint128, euint128) => euint128 test 4 (340282366920938463463365754576278416105, 340282366920938463463365754576278416101)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463365754576278416105n);
    input.add128(340282366920938463463365754576278416101n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.and_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(340282366920938463463365754576278416097n);
  });

  it('test operator "or" overload (euint128, euint128) => euint128 test 1 (340282366920938463463367655063545859643, 340282366920938463463373469661379852713)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463367655063545859643n);
    input.add128(340282366920938463463373469661379852713n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.or_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(340282366920938463463374605231434923963n);
  });

  it('test operator "or" overload (euint128, euint128) => euint128 test 2 (340282366920938463463367655063545859639, 340282366920938463463367655063545859643)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463367655063545859639n);
    input.add128(340282366920938463463367655063545859643n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.or_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(340282366920938463463367655063545859647n);
  });

  it('test operator "or" overload (euint128, euint128) => euint128 test 3 (340282366920938463463367655063545859643, 340282366920938463463367655063545859643)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463367655063545859643n);
    input.add128(340282366920938463463367655063545859643n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.or_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(340282366920938463463367655063545859643n);
  });

  it('test operator "or" overload (euint128, euint128) => euint128 test 4 (340282366920938463463367655063545859643, 340282366920938463463367655063545859639)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463367655063545859643n);
    input.add128(340282366920938463463367655063545859639n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.or_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(340282366920938463463367655063545859647n);
  });

  it('test operator "xor" overload (euint128, euint128) => euint128 test 1 (340282366920938463463372080691181989981, 340282366920938463463368941059346865329)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463372080691181989981n);
    input.add128(340282366920938463463368941059346865329n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.xor_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(8122674366786796n);
  });

  it('test operator "xor" overload (euint128, euint128) => euint128 test 2 (340282366920938463463368941059346865325, 340282366920938463463368941059346865329)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463368941059346865325n);
    input.add128(340282366920938463463368941059346865329n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.xor_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(28n);
  });

  it('test operator "xor" overload (euint128, euint128) => euint128 test 3 (340282366920938463463368941059346865329, 340282366920938463463368941059346865329)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463368941059346865329n);
    input.add128(340282366920938463463368941059346865329n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.xor_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint128, euint128) => euint128 test 4 (340282366920938463463368941059346865329, 340282366920938463463368941059346865325)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463368941059346865329n);
    input.add128(340282366920938463463368941059346865325n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.xor_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(28n);
  });

  it('test operator "eq" overload (euint128, euint128) => ebool test 1 (340282366920938463463371368021216956093, 340282366920938463463370243874611927705)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463371368021216956093n);
    input.add128(340282366920938463463370243874611927705n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.eq_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint128) => ebool test 2 (340282366920938463463370243874611927701, 340282366920938463463370243874611927705)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463370243874611927701n);
    input.add128(340282366920938463463370243874611927705n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.eq_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint128) => ebool test 3 (340282366920938463463370243874611927705, 340282366920938463463370243874611927705)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463370243874611927705n);
    input.add128(340282366920938463463370243874611927705n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.eq_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint128, euint128) => ebool test 4 (340282366920938463463370243874611927705, 340282366920938463463370243874611927701)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463370243874611927705n);
    input.add128(340282366920938463463370243874611927701n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.eq_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint128) => ebool test 1 (340282366920938463463366690564024700303, 340282366920938463463366105643215363065)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463366690564024700303n);
    input.add128(340282366920938463463366105643215363065n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ne_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint128) => ebool test 2 (340282366920938463463366105643215363061, 340282366920938463463366105643215363065)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463366105643215363061n);
    input.add128(340282366920938463463366105643215363065n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ne_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint128) => ebool test 3 (340282366920938463463366105643215363065, 340282366920938463463366105643215363065)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463366105643215363065n);
    input.add128(340282366920938463463366105643215363065n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ne_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint128) => ebool test 4 (340282366920938463463366105643215363065, 340282366920938463463366105643215363061)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463366105643215363065n);
    input.add128(340282366920938463463366105643215363061n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ne_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint128) => ebool test 1 (340282366920938463463369991167430665173, 340282366920938463463367243418798341499)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463369991167430665173n);
    input.add128(340282366920938463463367243418798341499n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ge_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint128) => ebool test 2 (340282366920938463463367243418798341495, 340282366920938463463367243418798341499)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463367243418798341495n);
    input.add128(340282366920938463463367243418798341499n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ge_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint128, euint128) => ebool test 3 (340282366920938463463367243418798341499, 340282366920938463463367243418798341499)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463367243418798341499n);
    input.add128(340282366920938463463367243418798341499n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ge_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint128) => ebool test 4 (340282366920938463463367243418798341499, 340282366920938463463367243418798341495)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463367243418798341499n);
    input.add128(340282366920938463463367243418798341495n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.ge_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint128) => ebool test 1 (340282366920938463463367714696589499231, 340282366920938463463372280163710625125)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463367714696589499231n);
    input.add128(340282366920938463463372280163710625125n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint128) => ebool test 2 (340282366920938463463367714696589499227, 340282366920938463463367714696589499231)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463367714696589499227n);
    input.add128(340282366920938463463367714696589499231n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint128) => ebool test 3 (340282366920938463463367714696589499231, 340282366920938463463367714696589499231)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463367714696589499231n);
    input.add128(340282366920938463463367714696589499231n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint128) => ebool test 4 (340282366920938463463367714696589499231, 340282366920938463463367714696589499227)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463367714696589499231n);
    input.add128(340282366920938463463367714696589499227n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.gt_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract8.resb());
    expect(res).to.equal(true);
  });
});
