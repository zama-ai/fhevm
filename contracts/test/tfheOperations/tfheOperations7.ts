import { expect } from 'chai';
import { ethers } from 'hardhat';

import type { TFHETestSuite1 } from '../../types/contracts/tests/TFHETestSuite1';
import type { TFHETestSuite2 } from '../../types/contracts/tests/TFHETestSuite2';
import type { TFHETestSuite3 } from '../../types/contracts/tests/TFHETestSuite3';
import type { TFHETestSuite4 } from '../../types/contracts/tests/TFHETestSuite4';
import type { TFHETestSuite5 } from '../../types/contracts/tests/TFHETestSuite5';
import type { TFHETestSuite6 } from '../../types/contracts/tests/TFHETestSuite6';
import type { TFHETestSuite7 } from '../../types/contracts/tests/TFHETestSuite7';
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

describe('TFHE operations 7', function () {
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

    const instances = await createInstances(this.signers);
    this.instances = instances;
  });

  it('test operator "min" overload (euint128, euint8) => euint128 test 1 (340282366920938463463370538850842096637, 221)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463370538850842096637n);
    input.add8(221n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(221n);
  });

  it('test operator "min" overload (euint128, euint8) => euint128 test 2 (217, 221)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(217n);
    input.add8(221n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(217n);
  });

  it('test operator "min" overload (euint128, euint8) => euint128 test 3 (221, 221)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(221n);
    input.add8(221n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(221n);
  });

  it('test operator "min" overload (euint128, euint8) => euint128 test 4 (221, 217)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(221n);
    input.add8(217n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(217n);
  });

  it('test operator "max" overload (euint128, euint8) => euint128 test 1 (340282366920938463463365715939325816819, 84)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463365715939325816819n);
    input.add8(84n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463365715939325816819n);
  });

  it('test operator "max" overload (euint128, euint8) => euint128 test 2 (80, 84)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(80n);
    input.add8(84n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(84n);
  });

  it('test operator "max" overload (euint128, euint8) => euint128 test 3 (84, 84)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(84n);
    input.add8(84n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(84n);
  });

  it('test operator "max" overload (euint128, euint8) => euint128 test 4 (84, 80)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(84n);
    input.add8(80n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(84n);
  });

  it('test operator "add" overload (euint128, euint16) => euint128 test 1 (32769, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(32769n);
    input.add16(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(32771n);
  });

  it('test operator "add" overload (euint128, euint16) => euint128 test 2 (17800, 17802)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(17800n);
    input.add16(17802n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(35602n);
  });

  it('test operator "add" overload (euint128, euint16) => euint128 test 3 (17802, 17802)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(17802n);
    input.add16(17802n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(35604n);
  });

  it('test operator "add" overload (euint128, euint16) => euint128 test 4 (17802, 17800)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(17802n);
    input.add16(17800n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(35602n);
  });

  it('test operator "sub" overload (euint128, euint16) => euint128 test 1 (24134, 24134)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(24134n);
    input.add16(24134n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint128, euint16) => euint128 test 2 (24134, 24130)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(24134n);
    input.add16(24130n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint128, euint16) => euint128 test 1 (16385, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(16385n);
    input.add16(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(32770n);
  });

  it('test operator "mul" overload (euint128, euint16) => euint128 test 2 (174, 174)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(174n);
    input.add16(174n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(30276n);
  });

  it('test operator "mul" overload (euint128, euint16) => euint128 test 3 (174, 174)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(174n);
    input.add16(174n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(30276n);
  });

  it('test operator "mul" overload (euint128, euint16) => euint128 test 4 (174, 174)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(174n);
    input.add16(174n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(30276n);
  });

  it('test operator "and" overload (euint128, euint16) => euint128 test 1 (340282366920938463463373441808723316901, 29305)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463373441808723316901n);
    input.add16(29305n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(4129n);
  });

  it('test operator "and" overload (euint128, euint16) => euint128 test 2 (29301, 29305)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(29301n);
    input.add16(29305n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(29297n);
  });

  it('test operator "and" overload (euint128, euint16) => euint128 test 3 (29305, 29305)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(29305n);
    input.add16(29305n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(29305n);
  });

  it('test operator "and" overload (euint128, euint16) => euint128 test 4 (29305, 29301)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(29305n);
    input.add16(29301n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(29297n);
  });

  it('test operator "or" overload (euint128, euint16) => euint128 test 1 (340282366920938463463370924350791536873, 9360)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463370924350791536873n);
    input.add16(9360n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463370924350791546105n);
  });

  it('test operator "or" overload (euint128, euint16) => euint128 test 2 (9356, 9360)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(9356n);
    input.add16(9360n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(9372n);
  });

  it('test operator "or" overload (euint128, euint16) => euint128 test 3 (9360, 9360)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(9360n);
    input.add16(9360n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(9360n);
  });

  it('test operator "or" overload (euint128, euint16) => euint128 test 4 (9360, 9356)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(9360n);
    input.add16(9356n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(9372n);
  });

  it('test operator "xor" overload (euint128, euint16) => euint128 test 1 (340282366920938463463368968192183577595, 1619)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463368968192183577595n);
    input.add16(1619n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463368968192183578024n);
  });

  it('test operator "xor" overload (euint128, euint16) => euint128 test 2 (1615, 1619)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1615n);
    input.add16(1619n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(28n);
  });

  it('test operator "xor" overload (euint128, euint16) => euint128 test 3 (1619, 1619)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1619n);
    input.add16(1619n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint128, euint16) => euint128 test 4 (1619, 1615)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1619n);
    input.add16(1615n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(28n);
  });

  it('test operator "eq" overload (euint128, euint16) => ebool test 1 (340282366920938463463373078640948433189, 11923)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463373078640948433189n);
    input.add16(11923n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint16) => ebool test 2 (11919, 11923)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(11919n);
    input.add16(11923n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint16) => ebool test 3 (11923, 11923)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(11923n);
    input.add16(11923n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint128, euint16) => ebool test 4 (11923, 11919)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(11923n);
    input.add16(11919n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint16) => ebool test 1 (340282366920938463463367555876216323275, 27216)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463367555876216323275n);
    input.add16(27216n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint16) => ebool test 2 (27212, 27216)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(27212n);
    input.add16(27216n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint16) => ebool test 3 (27216, 27216)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(27216n);
    input.add16(27216n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint16) => ebool test 4 (27216, 27212)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(27216n);
    input.add16(27212n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint16) => ebool test 1 (340282366920938463463372848513399747401, 39476)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463372848513399747401n);
    input.add16(39476n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint16) => ebool test 2 (39472, 39476)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(39472n);
    input.add16(39476n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint128, euint16) => ebool test 3 (39476, 39476)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(39476n);
    input.add16(39476n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint16) => ebool test 4 (39476, 39472)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(39476n);
    input.add16(39472n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint16) => ebool test 1 (340282366920938463463374566568308010601, 60745)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463374566568308010601n);
    input.add16(60745n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint16) => ebool test 2 (60741, 60745)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(60741n);
    input.add16(60745n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint16) => ebool test 3 (60745, 60745)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(60745n);
    input.add16(60745n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint16) => ebool test 4 (60745, 60741)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(60745n);
    input.add16(60741n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint16) => ebool test 1 (340282366920938463463370063031723138253, 1417)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463370063031723138253n);
    input.add16(1417n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint128, euint16) => ebool test 2 (1413, 1417)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1413n);
    input.add16(1417n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint16) => ebool test 3 (1417, 1417)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1417n);
    input.add16(1417n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint16) => ebool test 4 (1417, 1413)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1417n);
    input.add16(1413n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint16) => ebool test 1 (340282366920938463463367555010820974037, 21886)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463367555010820974037n);
    input.add16(21886n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint16) => ebool test 2 (21882, 21886)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(21882n);
    input.add16(21886n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint128, euint16) => ebool test 3 (21886, 21886)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(21886n);
    input.add16(21886n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint16) => ebool test 4 (21886, 21882)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(21886n);
    input.add16(21882n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint128, euint16) => euint128 test 1 (340282366920938463463373540523977679231, 6655)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463373540523977679231n);
    input.add16(6655n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(6655n);
  });

  it('test operator "min" overload (euint128, euint16) => euint128 test 2 (6651, 6655)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(6651n);
    input.add16(6655n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(6651n);
  });

  it('test operator "min" overload (euint128, euint16) => euint128 test 3 (6655, 6655)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(6655n);
    input.add16(6655n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(6655n);
  });

  it('test operator "min" overload (euint128, euint16) => euint128 test 4 (6655, 6651)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(6655n);
    input.add16(6651n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(6651n);
  });

  it('test operator "max" overload (euint128, euint16) => euint128 test 1 (340282366920938463463366289376410709253, 45099)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463366289376410709253n);
    input.add16(45099n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463366289376410709253n);
  });

  it('test operator "max" overload (euint128, euint16) => euint128 test 2 (45095, 45099)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(45095n);
    input.add16(45099n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(45099n);
  });

  it('test operator "max" overload (euint128, euint16) => euint128 test 3 (45099, 45099)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(45099n);
    input.add16(45099n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(45099n);
  });

  it('test operator "max" overload (euint128, euint16) => euint128 test 4 (45099, 45095)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(45099n);
    input.add16(45095n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(45099n);
  });

  it('test operator "add" overload (euint128, euint32) => euint128 test 1 (2147483649, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(2147483649n);
    input.add32(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(2147483651n);
  });

  it('test operator "add" overload (euint128, euint32) => euint128 test 2 (1458639136, 1458639138)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1458639136n);
    input.add32(1458639138n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(2917278274n);
  });

  it('test operator "add" overload (euint128, euint32) => euint128 test 3 (1458639138, 1458639138)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1458639138n);
    input.add32(1458639138n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(2917278276n);
  });

  it('test operator "add" overload (euint128, euint32) => euint128 test 4 (1458639138, 1458639136)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1458639138n);
    input.add32(1458639136n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(2917278274n);
  });

  it('test operator "sub" overload (euint128, euint32) => euint128 test 1 (568073438, 568073438)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(568073438n);
    input.add32(568073438n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint128, euint32) => euint128 test 2 (568073438, 568073434)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(568073438n);
    input.add32(568073434n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint128, euint32) => euint128 test 1 (1073741825, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1073741825n);
    input.add32(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(2147483650n);
  });

  it('test operator "mul" overload (euint128, euint32) => euint128 test 2 (46920, 46920)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(46920n);
    input.add32(46920n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(2201486400n);
  });

  it('test operator "mul" overload (euint128, euint32) => euint128 test 3 (46920, 46920)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(46920n);
    input.add32(46920n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(2201486400n);
  });

  it('test operator "mul" overload (euint128, euint32) => euint128 test 4 (46920, 46920)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(46920n);
    input.add32(46920n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(2201486400n);
  });

  it('test operator "and" overload (euint128, euint32) => euint128 test 1 (340282366920938463463374344092643821231, 1272345987)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463374344092643821231n);
    input.add32(1272345987n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(16778371n);
  });

  it('test operator "and" overload (euint128, euint32) => euint128 test 2 (1272345983, 1272345987)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1272345983n);
    input.add32(1272345987n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(1272345859n);
  });

  it('test operator "and" overload (euint128, euint32) => euint128 test 3 (1272345987, 1272345987)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1272345987n);
    input.add32(1272345987n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(1272345987n);
  });

  it('test operator "and" overload (euint128, euint32) => euint128 test 4 (1272345987, 1272345983)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1272345987n);
    input.add32(1272345983n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(1272345859n);
  });

  it('test operator "or" overload (euint128, euint32) => euint128 test 1 (340282366920938463463367678655808660909, 1825979623)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463367678655808660909n);
    input.add32(1825979623n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463367678657096580591n);
  });

  it('test operator "or" overload (euint128, euint32) => euint128 test 2 (1825979619, 1825979623)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1825979619n);
    input.add32(1825979623n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(1825979623n);
  });

  it('test operator "or" overload (euint128, euint32) => euint128 test 3 (1825979623, 1825979623)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1825979623n);
    input.add32(1825979623n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(1825979623n);
  });

  it('test operator "or" overload (euint128, euint32) => euint128 test 4 (1825979623, 1825979619)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1825979623n);
    input.add32(1825979619n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(1825979623n);
  });

  it('test operator "xor" overload (euint128, euint32) => euint128 test 1 (340282366920938463463370573080758812057, 3525451442)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463370573080758812057n);
    input.add32(3525451442n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463370573081595437867n);
  });

  it('test operator "xor" overload (euint128, euint32) => euint128 test 2 (3525451438, 3525451442)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(3525451438n);
    input.add32(3525451442n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(28n);
  });

  it('test operator "xor" overload (euint128, euint32) => euint128 test 3 (3525451442, 3525451442)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(3525451442n);
    input.add32(3525451442n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint128, euint32) => euint128 test 4 (3525451442, 3525451438)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(3525451442n);
    input.add32(3525451438n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(28n);
  });

  it('test operator "eq" overload (euint128, euint32) => ebool test 1 (340282366920938463463370240583337458465, 2005647798)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463370240583337458465n);
    input.add32(2005647798n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint32) => ebool test 2 (2005647794, 2005647798)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(2005647794n);
    input.add32(2005647798n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint32) => ebool test 3 (2005647798, 2005647798)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(2005647798n);
    input.add32(2005647798n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint128, euint32) => ebool test 4 (2005647798, 2005647794)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(2005647798n);
    input.add32(2005647794n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint32) => ebool test 1 (340282366920938463463369356395319575067, 3890076622)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463369356395319575067n);
    input.add32(3890076622n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint32) => ebool test 2 (3890076618, 3890076622)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(3890076618n);
    input.add32(3890076622n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint32) => ebool test 3 (3890076622, 3890076622)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(3890076622n);
    input.add32(3890076622n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint32) => ebool test 4 (3890076622, 3890076618)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(3890076622n);
    input.add32(3890076618n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint32) => ebool test 1 (340282366920938463463372177529252116265, 1035737659)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463372177529252116265n);
    input.add32(1035737659n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint32) => ebool test 2 (1035737655, 1035737659)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1035737655n);
    input.add32(1035737659n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint128, euint32) => ebool test 3 (1035737659, 1035737659)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1035737659n);
    input.add32(1035737659n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint32) => ebool test 4 (1035737659, 1035737655)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1035737659n);
    input.add32(1035737655n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint32) => ebool test 1 (340282366920938463463367775133279442315, 2630717593)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463367775133279442315n);
    input.add32(2630717593n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint32) => ebool test 2 (2630717589, 2630717593)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(2630717589n);
    input.add32(2630717593n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint32) => ebool test 3 (2630717593, 2630717593)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(2630717593n);
    input.add32(2630717593n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint32) => ebool test 4 (2630717593, 2630717589)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(2630717593n);
    input.add32(2630717589n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint32) => ebool test 1 (340282366920938463463365895215645660601, 332310589)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463365895215645660601n);
    input.add32(332310589n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint128, euint32) => ebool test 2 (332310585, 332310589)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(332310585n);
    input.add32(332310589n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint32) => ebool test 3 (332310589, 332310589)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(332310589n);
    input.add32(332310589n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint32) => ebool test 4 (332310589, 332310585)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(332310589n);
    input.add32(332310585n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint32) => ebool test 1 (340282366920938463463366520207498903961, 2191075228)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463366520207498903961n);
    input.add32(2191075228n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint32) => ebool test 2 (2191075224, 2191075228)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(2191075224n);
    input.add32(2191075228n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint128, euint32) => ebool test 3 (2191075228, 2191075228)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(2191075228n);
    input.add32(2191075228n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint32) => ebool test 4 (2191075228, 2191075224)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(2191075228n);
    input.add32(2191075224n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint128, euint32) => euint128 test 1 (340282366920938463463368077124844750611, 2219795384)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463368077124844750611n);
    input.add32(2219795384n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(2219795384n);
  });

  it('test operator "min" overload (euint128, euint32) => euint128 test 2 (2219795380, 2219795384)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(2219795380n);
    input.add32(2219795384n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(2219795380n);
  });

  it('test operator "min" overload (euint128, euint32) => euint128 test 3 (2219795384, 2219795384)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(2219795384n);
    input.add32(2219795384n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(2219795384n);
  });

  it('test operator "min" overload (euint128, euint32) => euint128 test 4 (2219795384, 2219795380)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(2219795384n);
    input.add32(2219795380n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(2219795380n);
  });

  it('test operator "max" overload (euint128, euint32) => euint128 test 1 (340282366920938463463373690962277796747, 3584920833)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463373690962277796747n);
    input.add32(3584920833n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463373690962277796747n);
  });

  it('test operator "max" overload (euint128, euint32) => euint128 test 2 (3584920829, 3584920833)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(3584920829n);
    input.add32(3584920833n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(3584920833n);
  });

  it('test operator "max" overload (euint128, euint32) => euint128 test 3 (3584920833, 3584920833)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(3584920833n);
    input.add32(3584920833n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(3584920833n);
  });

  it('test operator "max" overload (euint128, euint32) => euint128 test 4 (3584920833, 3584920829)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(3584920833n);
    input.add32(3584920829n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(3584920833n);
  });

  it('test operator "add" overload (euint128, euint64) => euint128 test 1 (9223372036854775809, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(9223372036854775809n);
    input.add64(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(9223372036854775811n);
  });

  it('test operator "add" overload (euint128, euint64) => euint128 test 2 (9219820420949206504, 9219820420949206506)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(9219820420949206504n);
    input.add64(9219820420949206506n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18439640841898413010n);
  });

  it('test operator "add" overload (euint128, euint64) => euint128 test 3 (9219820420949206506, 9219820420949206506)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(9219820420949206506n);
    input.add64(9219820420949206506n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18439640841898413012n);
  });

  it('test operator "add" overload (euint128, euint64) => euint128 test 4 (9219820420949206506, 9219820420949206504)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(9219820420949206506n);
    input.add64(9219820420949206504n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18439640841898413010n);
  });

  it('test operator "sub" overload (euint128, euint64) => euint128 test 1 (18439035038685183957, 18439035038685183957)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18439035038685183957n);
    input.add64(18439035038685183957n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint128, euint64) => euint128 test 2 (18439035038685183957, 18439035038685183953)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18439035038685183957n);
    input.add64(18439035038685183953n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint128, euint64) => euint128 test 1 (4611686018427387905, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(4611686018427387905n);
    input.add64(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(9223372036854775810n);
  });

  it('test operator "mul" overload (euint128, euint64) => euint128 test 2 (4294068838, 4294068838)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(4294068838n);
    input.add64(4294068838n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18439027185482670244n);
  });

  it('test operator "mul" overload (euint128, euint64) => euint128 test 3 (4294068838, 4294068838)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(4294068838n);
    input.add64(4294068838n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18439027185482670244n);
  });

  it('test operator "mul" overload (euint128, euint64) => euint128 test 4 (4294068838, 4294068838)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(4294068838n);
    input.add64(4294068838n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18439027185482670244n);
  });

  it('test operator "and" overload (euint128, euint64) => euint128 test 1 (340282366920938463463371986283049344683, 18446730491956748719)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463371986283049344683n);
    input.add64(18446730491956748719n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18444122856245706923n);
  });

  it('test operator "and" overload (euint128, euint64) => euint128 test 2 (18446730491956748715, 18446730491956748719)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18446730491956748715n);
    input.add64(18446730491956748719n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18446730491956748715n);
  });

  it('test operator "and" overload (euint128, euint64) => euint128 test 3 (18446730491956748719, 18446730491956748719)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18446730491956748719n);
    input.add64(18446730491956748719n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18446730491956748719n);
  });

  it('test operator "and" overload (euint128, euint64) => euint128 test 4 (18446730491956748719, 18446730491956748715)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18446730491956748719n);
    input.add64(18446730491956748715n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18446730491956748715n);
  });

  it('test operator "or" overload (euint128, euint64) => euint128 test 1 (340282366920938463463368775688723184525, 18441508155707784523)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463368775688723184525n);
    input.add64(18441508155707784523n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463369943302762627023n);
  });

  it('test operator "or" overload (euint128, euint64) => euint128 test 2 (18441508155707784519, 18441508155707784523)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18441508155707784519n);
    input.add64(18441508155707784523n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18441508155707784527n);
  });

  it('test operator "or" overload (euint128, euint64) => euint128 test 3 (18441508155707784523, 18441508155707784523)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18441508155707784523n);
    input.add64(18441508155707784523n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18441508155707784523n);
  });

  it('test operator "or" overload (euint128, euint64) => euint128 test 4 (18441508155707784523, 18441508155707784519)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18441508155707784523n);
    input.add64(18441508155707784519n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18441508155707784527n);
  });

  it('test operator "xor" overload (euint128, euint64) => euint128 test 1 (340282366920938463463370296928342523849, 18439150492607900437)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463370296928342523849n);
    input.add64(18439150492607900437n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463444933961469283718364n);
  });

  it('test operator "xor" overload (euint128, euint64) => euint128 test 2 (18439150492607900433, 18439150492607900437)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18439150492607900433n);
    input.add64(18439150492607900437n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint128, euint64) => euint128 test 3 (18439150492607900437, 18439150492607900437)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18439150492607900437n);
    input.add64(18439150492607900437n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint128, euint64) => euint128 test 4 (18439150492607900437, 18439150492607900433)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18439150492607900437n);
    input.add64(18439150492607900433n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint128, euint64) => ebool test 1 (340282366920938463463373358570340484291, 18446577284360055861)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463373358570340484291n);
    input.add64(18446577284360055861n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint64) => ebool test 2 (18446577284360055857, 18446577284360055861)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18446577284360055857n);
    input.add64(18446577284360055861n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint64) => ebool test 3 (18446577284360055861, 18446577284360055861)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18446577284360055861n);
    input.add64(18446577284360055861n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint128, euint64) => ebool test 4 (18446577284360055861, 18446577284360055857)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18446577284360055861n);
    input.add64(18446577284360055857n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint64) => ebool test 1 (340282366920938463463366975460280011425, 18442438718252544531)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463366975460280011425n);
    input.add64(18442438718252544531n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint64) => ebool test 2 (18442438718252544527, 18442438718252544531)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18442438718252544527n);
    input.add64(18442438718252544531n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint64) => ebool test 3 (18442438718252544531, 18442438718252544531)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18442438718252544531n);
    input.add64(18442438718252544531n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint64) => ebool test 4 (18442438718252544531, 18442438718252544527)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18442438718252544531n);
    input.add64(18442438718252544527n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint64) => ebool test 1 (340282366920938463463371165878875374611, 18438969046350530105)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463371165878875374611n);
    input.add64(18438969046350530105n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint64) => ebool test 2 (18438969046350530101, 18438969046350530105)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18438969046350530101n);
    input.add64(18438969046350530105n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint128, euint64) => ebool test 3 (18438969046350530105, 18438969046350530105)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18438969046350530105n);
    input.add64(18438969046350530105n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint64) => ebool test 4 (18438969046350530105, 18438969046350530101)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18438969046350530105n);
    input.add64(18438969046350530101n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint64) => ebool test 1 (340282366920938463463372252109401808541, 18437991477058457353)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463372252109401808541n);
    input.add64(18437991477058457353n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint64) => ebool test 2 (18437991477058457349, 18437991477058457353)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18437991477058457349n);
    input.add64(18437991477058457353n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint64) => ebool test 3 (18437991477058457353, 18437991477058457353)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18437991477058457353n);
    input.add64(18437991477058457353n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint64) => ebool test 4 (18437991477058457353, 18437991477058457349)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18437991477058457353n);
    input.add64(18437991477058457349n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint64) => ebool test 1 (340282366920938463463374549092044560961, 18440180459225386013)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463374549092044560961n);
    input.add64(18440180459225386013n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint128, euint64) => ebool test 2 (18440180459225386009, 18440180459225386013)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18440180459225386009n);
    input.add64(18440180459225386013n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint64) => ebool test 3 (18440180459225386013, 18440180459225386013)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18440180459225386013n);
    input.add64(18440180459225386013n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint64) => ebool test 4 (18440180459225386013, 18440180459225386009)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18440180459225386013n);
    input.add64(18440180459225386009n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint64) => ebool test 1 (340282366920938463463370795822285688657, 18438130360140647889)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463370795822285688657n);
    input.add64(18438130360140647889n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint64) => ebool test 2 (18438130360140647885, 18438130360140647889)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18438130360140647885n);
    input.add64(18438130360140647889n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint128, euint64) => ebool test 3 (18438130360140647889, 18438130360140647889)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18438130360140647889n);
    input.add64(18438130360140647889n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint64) => ebool test 4 (18438130360140647889, 18438130360140647885)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18438130360140647889n);
    input.add64(18438130360140647885n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint128, euint64) => euint128 test 1 (340282366920938463463370207059803993427, 18440283966485380923)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463370207059803993427n);
    input.add64(18440283966485380923n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18440283966485380923n);
  });

  it('test operator "min" overload (euint128, euint64) => euint128 test 2 (18440283966485380919, 18440283966485380923)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18440283966485380919n);
    input.add64(18440283966485380923n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18440283966485380919n);
  });

  it('test operator "min" overload (euint128, euint64) => euint128 test 3 (18440283966485380923, 18440283966485380923)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18440283966485380923n);
    input.add64(18440283966485380923n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18440283966485380923n);
  });

  it('test operator "min" overload (euint128, euint64) => euint128 test 4 (18440283966485380923, 18440283966485380919)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18440283966485380923n);
    input.add64(18440283966485380919n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18440283966485380919n);
  });

  it('test operator "max" overload (euint128, euint64) => euint128 test 1 (340282366920938463463368767663966130401, 18446373735300902713)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463368767663966130401n);
    input.add64(18446373735300902713n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463368767663966130401n);
  });

  it('test operator "max" overload (euint128, euint64) => euint128 test 2 (18446373735300902709, 18446373735300902713)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18446373735300902709n);
    input.add64(18446373735300902713n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18446373735300902713n);
  });

  it('test operator "max" overload (euint128, euint64) => euint128 test 3 (18446373735300902713, 18446373735300902713)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18446373735300902713n);
    input.add64(18446373735300902713n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18446373735300902713n);
  });

  it('test operator "max" overload (euint128, euint64) => euint128 test 4 (18446373735300902713, 18446373735300902709)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18446373735300902713n);
    input.add64(18446373735300902709n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18446373735300902713n);
  });

  it('test operator "add" overload (euint128, euint128) => euint128 test 1 (170141183460469231731685555527791261801, 170141183460469231731685861022425972391)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(170141183460469231731685555527791261801n);
    input.add128(170141183460469231731685861022425972391n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463371416550217234192n);
  });

  it('test operator "add" overload (euint128, euint128) => euint128 test 2 (170141183460469231731685555527791261799, 170141183460469231731685555527791261801)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(170141183460469231731685555527791261799n);
    input.add128(170141183460469231731685555527791261801n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463371111055582523600n);
  });

  it('test operator "add" overload (euint128, euint128) => euint128 test 3 (170141183460469231731685555527791261801, 170141183460469231731685555527791261801)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(170141183460469231731685555527791261801n);
    input.add128(170141183460469231731685555527791261801n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463371111055582523602n);
  });

  it('test operator "add" overload (euint128, euint128) => euint128 test 4 (170141183460469231731685555527791261801, 170141183460469231731685555527791261799)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(170141183460469231731685555527791261801n);
    input.add128(170141183460469231731685555527791261799n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463371111055582523600n);
  });

  it('test operator "sub" overload (euint128, euint128) => euint128 test 1 (340282366920938463463368591887439516571, 340282366920938463463368591887439516571)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463368591887439516571n);
    input.add128(340282366920938463463368591887439516571n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint128, euint128) => euint128 test 2 (340282366920938463463368591887439516571, 340282366920938463463368591887439516567)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463368591887439516571n);
    input.add128(340282366920938463463368591887439516567n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint128, euint128) => euint128 test 1 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(9223372036854775809n);
    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (euint128, euint128) => euint128 test 2 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(9223372036854775809n);
    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (euint128, euint128) => euint128 test 3 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(9223372036854775809n);
    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (euint128, euint128) => euint128 test 4 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(9223372036854775809n);
    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "and" overload (euint128, euint128) => euint128 test 1 (340282366920938463463367803233397397117, 340282366920938463463366127539405669875)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463367803233397397117n);
    input.add128(340282366920938463463366127539405669875n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463366114340386129009n);
  });

  it('test operator "and" overload (euint128, euint128) => euint128 test 2 (340282366920938463463366127539405669871, 340282366920938463463366127539405669875)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463366127539405669871n);
    input.add128(340282366920938463463366127539405669875n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463366127539405669859n);
  });

  it('test operator "and" overload (euint128, euint128) => euint128 test 3 (340282366920938463463366127539405669875, 340282366920938463463366127539405669875)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463366127539405669875n);
    input.add128(340282366920938463463366127539405669875n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463366127539405669875n);
  });

  it('test operator "and" overload (euint128, euint128) => euint128 test 4 (340282366920938463463366127539405669875, 340282366920938463463366127539405669871)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463366127539405669875n);
    input.add128(340282366920938463463366127539405669871n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463366127539405669859n);
  });

  it('test operator "or" overload (euint128, euint128) => euint128 test 1 (340282366920938463463366233126145935519, 340282366920938463463370541968157845523)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463366233126145935519n);
    input.add128(340282366920938463463370541968157845523n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(340282366920938463463371159361117022367n);
  });

  it('test operator "or" overload (euint128, euint128) => euint128 test 2 (340282366920938463463366233126145935515, 340282366920938463463366233126145935519)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463366233126145935515n);
    input.add128(340282366920938463463366233126145935519n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(340282366920938463463366233126145935519n);
  });

  it('test operator "or" overload (euint128, euint128) => euint128 test 3 (340282366920938463463366233126145935519, 340282366920938463463366233126145935519)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463366233126145935519n);
    input.add128(340282366920938463463366233126145935519n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(340282366920938463463366233126145935519n);
  });

  it('test operator "or" overload (euint128, euint128) => euint128 test 4 (340282366920938463463366233126145935519, 340282366920938463463366233126145935515)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463366233126145935519n);
    input.add128(340282366920938463463366233126145935515n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(340282366920938463463366233126145935519n);
  });

  it('test operator "xor" overload (euint128, euint128) => euint128 test 1 (340282366920938463463373425356319859407, 340282366920938463463366938450707128487)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463373425356319859407n);
    input.add128(340282366920938463463366938450707128487n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(8741060144333416n);
  });

  it('test operator "xor" overload (euint128, euint128) => euint128 test 2 (340282366920938463463366938450707128483, 340282366920938463463366938450707128487)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463366938450707128483n);
    input.add128(340282366920938463463366938450707128487n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint128, euint128) => euint128 test 3 (340282366920938463463366938450707128487, 340282366920938463463366938450707128487)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463366938450707128487n);
    input.add128(340282366920938463463366938450707128487n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint128, euint128) => euint128 test 4 (340282366920938463463366938450707128487, 340282366920938463463366938450707128483)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463366938450707128487n);
    input.add128(340282366920938463463366938450707128483n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint128, euint128) => ebool test 1 (340282366920938463463368967159887473275, 340282366920938463463373770817558207145)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463368967159887473275n);
    input.add128(340282366920938463463373770817558207145n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint128) => ebool test 2 (340282366920938463463368967159887473271, 340282366920938463463368967159887473275)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463368967159887473271n);
    input.add128(340282366920938463463368967159887473275n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint128) => ebool test 3 (340282366920938463463368967159887473275, 340282366920938463463368967159887473275)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463368967159887473275n);
    input.add128(340282366920938463463368967159887473275n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint128, euint128) => ebool test 4 (340282366920938463463368967159887473275, 340282366920938463463368967159887473271)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463368967159887473275n);
    input.add128(340282366920938463463368967159887473271n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint128) => ebool test 1 (340282366920938463463366182378497106561, 340282366920938463463369396906288511281)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463366182378497106561n);
    input.add128(340282366920938463463369396906288511281n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint128) => ebool test 2 (340282366920938463463366182378497106557, 340282366920938463463366182378497106561)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463366182378497106557n);
    input.add128(340282366920938463463366182378497106561n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint128) => ebool test 3 (340282366920938463463366182378497106561, 340282366920938463463366182378497106561)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463366182378497106561n);
    input.add128(340282366920938463463366182378497106561n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint128) => ebool test 4 (340282366920938463463366182378497106561, 340282366920938463463366182378497106557)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463366182378497106561n);
    input.add128(340282366920938463463366182378497106557n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });
});
