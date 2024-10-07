import { expect } from 'chai';
import { ethers } from 'hardhat';

import type { TFHETestSuite1 } from '../../types/contracts/tests/TFHETestSuite1';
import type { TFHETestSuite2 } from '../../types/contracts/tests/TFHETestSuite2';
import type { TFHETestSuite3 } from '../../types/contracts/tests/TFHETestSuite3';
import type { TFHETestSuite4 } from '../../types/contracts/tests/TFHETestSuite4';
import type { TFHETestSuite5 } from '../../types/contracts/tests/TFHETestSuite5';
import type { TFHETestSuite6 } from '../../types/contracts/tests/TFHETestSuite6';
import { createInstances, decrypt4, decrypt8, decrypt16, decrypt32, decrypt64, decryptBool } from '../instance';
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

describe('TFHE operations 11', function () {
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

    const instances = await createInstances(this.signers);
    this.instances = instances;
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 1 (18440186743754430625, 4137891061)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18440186743754430625n);
    input.add32(4137891061n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(4137891061n);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 2 (4137891057, 4137891061)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(4137891057n);
    input.add32(4137891061n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(4137891057n);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 3 (4137891061, 4137891061)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(4137891061n);
    input.add32(4137891061n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(4137891061n);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 4 (4137891061, 4137891057)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(4137891061n);
    input.add32(4137891057n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(4137891057n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 1 (18445662984886583599, 303914102)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18445662984886583599n);
    input.add32(303914102n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18445662984886583599n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 2 (303914098, 303914102)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(303914098n);
    input.add32(303914102n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(303914102n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 3 (303914102, 303914102)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(303914102n);
    input.add32(303914102n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(303914102n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 4 (303914102, 303914098)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(303914102n);
    input.add32(303914098n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(303914102n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 1 (9219336648067240893, 9223239842705786428)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(9219336648067240893n);
    input.add64(9223239842705786428n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18442576490773027321n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 2 (9219336648067240891, 9219336648067240893)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(9219336648067240891n);
    input.add64(9219336648067240893n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18438673296134481784n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 3 (9219336648067240893, 9219336648067240893)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(9219336648067240893n);
    input.add64(9219336648067240893n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18438673296134481786n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 4 (9219336648067240893, 9219336648067240891)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(9219336648067240893n);
    input.add64(9219336648067240891n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18438673296134481784n);
  });

  it('test operator "sub" overload (euint64, euint64) => euint64 test 1 (18442295292010752223, 18442295292010752223)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442295292010752223n);
    input.add64(18442295292010752223n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint64) => euint64 test 2 (18442295292010752223, 18442295292010752219)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442295292010752223n);
    input.add64(18442295292010752219n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 1 (4294744203, 4293841776)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(4294744203n);
    input.add64(4293841776n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18440952076075224528n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 2 (4293841776, 4293841776)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(4293841776n);
    input.add64(4293841776n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18437077197322834176n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 3 (4293841776, 4293841776)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(4293841776n);
    input.add64(4293841776n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18437077197322834176n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 4 (4293841776, 4293841776)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(4293841776n);
    input.add64(4293841776n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18437077197322834176n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 1 (18444924635377016941, 18446173308524975411)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18444924635377016941n);
    input.add64(18446173308524975411n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18444923419899985953n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 2 (18444924635377016937, 18444924635377016941)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18444924635377016937n);
    input.add64(18444924635377016941n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18444924635377016937n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 3 (18444924635377016941, 18444924635377016941)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18444924635377016941n);
    input.add64(18444924635377016941n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18444924635377016941n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 4 (18444924635377016941, 18444924635377016937)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18444924635377016941n);
    input.add64(18444924635377016937n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18444924635377016937n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 1 (18442105052999173891, 18440193626964063709)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442105052999173891n);
    input.add64(18440193626964063709n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18442168970778769375n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 2 (18440193626964063705, 18440193626964063709)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18440193626964063705n);
    input.add64(18440193626964063709n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18440193626964063709n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 3 (18440193626964063709, 18440193626964063709)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18440193626964063709n);
    input.add64(18440193626964063709n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18440193626964063709n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 4 (18440193626964063709, 18440193626964063705)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18440193626964063709n);
    input.add64(18440193626964063705n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18440193626964063709n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 1 (18440379911699358599, 18444259094994538125)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18440379911699358599n);
    input.add64(18444259094994538125n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(8523658389368074n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 2 (18440379911699358595, 18440379911699358599)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18440379911699358595n);
    input.add64(18440379911699358599n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 3 (18440379911699358599, 18440379911699358599)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18440379911699358599n);
    input.add64(18440379911699358599n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 4 (18440379911699358599, 18440379911699358595)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18440379911699358599n);
    input.add64(18440379911699358595n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 1 (18442470885463520691, 18445888562756211467)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442470885463520691n);
    input.add64(18445888562756211467n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 2 (18442470885463520687, 18442470885463520691)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442470885463520687n);
    input.add64(18442470885463520691n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 3 (18442470885463520691, 18442470885463520691)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442470885463520691n);
    input.add64(18442470885463520691n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 4 (18442470885463520691, 18442470885463520687)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442470885463520691n);
    input.add64(18442470885463520687n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 1 (18442343533763371027, 18438477816592523453)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442343533763371027n);
    input.add64(18438477816592523453n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 2 (18438477816592523449, 18438477816592523453)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438477816592523449n);
    input.add64(18438477816592523453n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 3 (18438477816592523453, 18438477816592523453)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438477816592523453n);
    input.add64(18438477816592523453n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 4 (18438477816592523453, 18438477816592523449)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438477816592523453n);
    input.add64(18438477816592523449n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 1 (18441951286640352465, 18443086804815428517)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441951286640352465n);
    input.add64(18443086804815428517n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 2 (18441951286640352461, 18441951286640352465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441951286640352461n);
    input.add64(18441951286640352465n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 3 (18441951286640352465, 18441951286640352465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441951286640352465n);
    input.add64(18441951286640352465n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 4 (18441951286640352465, 18441951286640352461)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441951286640352465n);
    input.add64(18441951286640352461n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 1 (18443021258691337483, 18443593997946075985)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18443021258691337483n);
    input.add64(18443593997946075985n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 2 (18443021258691337479, 18443021258691337483)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18443021258691337479n);
    input.add64(18443021258691337483n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 3 (18443021258691337483, 18443021258691337483)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18443021258691337483n);
    input.add64(18443021258691337483n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 4 (18443021258691337483, 18443021258691337479)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18443021258691337483n);
    input.add64(18443021258691337479n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 1 (18443782238591650169, 18441978790273961913)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18443782238591650169n);
    input.add64(18441978790273961913n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 2 (18441978790273961909, 18441978790273961913)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441978790273961909n);
    input.add64(18441978790273961913n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 3 (18441978790273961913, 18441978790273961913)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441978790273961913n);
    input.add64(18441978790273961913n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 4 (18441978790273961913, 18441978790273961909)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441978790273961913n);
    input.add64(18441978790273961909n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 1 (18445972196307174659, 18438760003066140571)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18445972196307174659n);
    input.add64(18438760003066140571n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 2 (18438760003066140567, 18438760003066140571)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438760003066140567n);
    input.add64(18438760003066140571n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 3 (18438760003066140571, 18438760003066140571)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438760003066140571n);
    input.add64(18438760003066140571n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 4 (18438760003066140571, 18438760003066140567)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438760003066140571n);
    input.add64(18438760003066140567n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 1 (18446277918636147641, 18439099819121701507)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18446277918636147641n);
    input.add64(18439099819121701507n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18439099819121701507n);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 2 (18439099819121701503, 18439099819121701507)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18439099819121701503n);
    input.add64(18439099819121701507n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18439099819121701503n);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 3 (18439099819121701507, 18439099819121701507)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18439099819121701507n);
    input.add64(18439099819121701507n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18439099819121701507n);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 4 (18439099819121701507, 18439099819121701503)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18439099819121701507n);
    input.add64(18439099819121701503n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18439099819121701503n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 1 (18438631683554897479, 18438807769877774597)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438631683554897479n);
    input.add64(18438807769877774597n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18438807769877774597n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 2 (18438631683554897475, 18438631683554897479)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438631683554897475n);
    input.add64(18438631683554897479n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18438631683554897479n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 3 (18438631683554897479, 18438631683554897479)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438631683554897479n);
    input.add64(18438631683554897479n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18438631683554897479n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 4 (18438631683554897479, 18438631683554897475)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438631683554897479n);
    input.add64(18438631683554897475n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18438631683554897479n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 1 (9219336648067240893, 9219865027937887870)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(9219336648067240893n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint64_uint64(
      encryptedAmount.handles[0],
      9219865027937887870n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18439201676005128763n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 2 (9219336648067240891, 9219336648067240893)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(9219336648067240891n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint64_uint64(
      encryptedAmount.handles[0],
      9219336648067240893n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18438673296134481784n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 3 (9219336648067240893, 9219336648067240893)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(9219336648067240893n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint64_uint64(
      encryptedAmount.handles[0],
      9219336648067240893n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18438673296134481786n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 4 (9219336648067240893, 9219336648067240891)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(9219336648067240893n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint64_uint64(
      encryptedAmount.handles[0],
      9219336648067240891n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18438673296134481784n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 1 (9221821645145019755, 9219865027937887870)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(9219865027937887870n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_uint64_euint64(
      9221821645145019755n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18441686673082907625n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 2 (9219336648067240891, 9219336648067240893)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(9219336648067240893n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_uint64_euint64(
      9219336648067240891n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18438673296134481784n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 3 (9219336648067240893, 9219336648067240893)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(9219336648067240893n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_uint64_euint64(
      9219336648067240893n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18438673296134481786n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 4 (9219336648067240893, 9219336648067240891)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(9219336648067240891n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_uint64_euint64(
      9219336648067240893n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18438673296134481784n);
  });

  it('test operator "sub" overload (euint64, uint64) => euint64 test 1 (18442295292010752223, 18442295292010752223)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442295292010752223n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint64_uint64(
      encryptedAmount.handles[0],
      18442295292010752223n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, uint64) => euint64 test 2 (18442295292010752223, 18442295292010752219)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442295292010752223n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint64_uint64(
      encryptedAmount.handles[0],
      18442295292010752219n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint64, euint64) => euint64 test 1 (18442295292010752223, 18442295292010752223)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18442295292010752223n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_uint64_euint64(
      18442295292010752223n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint64, euint64) => euint64 test 2 (18442295292010752223, 18442295292010752219)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18442295292010752219n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_uint64_euint64(
      18442295292010752223n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 1 (4294744203, 4294589707)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(4294744203n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint64_uint64(
      encryptedAmount.handles[0],
      4294589707n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18444164248401718521n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 2 (4293841776, 4293841776)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(4293841776n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint64_uint64(
      encryptedAmount.handles[0],
      4293841776n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18437077197322834176n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 3 (4293841776, 4293841776)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(4293841776n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint64_uint64(
      encryptedAmount.handles[0],
      4293841776n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18437077197322834176n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 4 (4293841776, 4293841776)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(4293841776n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint64_uint64(
      encryptedAmount.handles[0],
      4293841776n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18437077197322834176n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 1 (4293263967, 4294589707)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(4294589707n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_uint64_euint64(
      4293263967n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18437807242112187669n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 2 (4293841776, 4293841776)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(4293841776n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_uint64_euint64(
      4293841776n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18437077197322834176n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 3 (4293841776, 4293841776)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(4293841776n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_uint64_euint64(
      4293841776n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18437077197322834176n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 4 (4293841776, 4293841776)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(4293841776n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_uint64_euint64(
      4293841776n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18437077197322834176n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 1 (18442885434890559161, 18443612733078410209)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442885434890559161n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.div_euint64_uint64(
      encryptedAmount.handles[0],
      18443612733078410209n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 2 (18442885434890559157, 18442885434890559161)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442885434890559157n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.div_euint64_uint64(
      encryptedAmount.handles[0],
      18442885434890559161n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 3 (18442885434890559161, 18442885434890559161)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442885434890559161n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.div_euint64_uint64(
      encryptedAmount.handles[0],
      18442885434890559161n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 4 (18442885434890559161, 18442885434890559157)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442885434890559161n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.div_euint64_uint64(
      encryptedAmount.handles[0],
      18442885434890559157n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 1 (18445299104872939013, 18441298343819074757)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18445299104872939013n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rem_euint64_uint64(
      encryptedAmount.handles[0],
      18441298343819074757n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(4000761053864256n);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 2 (18441202573467232859, 18441202573467232863)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441202573467232859n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rem_euint64_uint64(
      encryptedAmount.handles[0],
      18441202573467232863n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18441202573467232859n);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 3 (18441202573467232863, 18441202573467232863)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441202573467232863n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rem_euint64_uint64(
      encryptedAmount.handles[0],
      18441202573467232863n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 4 (18441202573467232863, 18441202573467232859)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441202573467232863n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rem_euint64_uint64(
      encryptedAmount.handles[0],
      18441202573467232859n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 1 (18442470885463520691, 18444839379902899701)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442470885463520691n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint64_uint64(
      encryptedAmount.handles[0],
      18444839379902899701n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 2 (18442470885463520687, 18442470885463520691)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442470885463520687n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint64_uint64(
      encryptedAmount.handles[0],
      18442470885463520691n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 3 (18442470885463520691, 18442470885463520691)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442470885463520691n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint64_uint64(
      encryptedAmount.handles[0],
      18442470885463520691n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 4 (18442470885463520691, 18442470885463520687)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442470885463520691n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint64_uint64(
      encryptedAmount.handles[0],
      18442470885463520687n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 1 (18446106064654681685, 18444839379902899701)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18444839379902899701n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_uint64_euint64(
      18446106064654681685n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 2 (18442470885463520687, 18442470885463520691)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18442470885463520691n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_uint64_euint64(
      18442470885463520687n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 3 (18442470885463520691, 18442470885463520691)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18442470885463520691n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_uint64_euint64(
      18442470885463520691n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 4 (18442470885463520691, 18442470885463520687)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18442470885463520687n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_uint64_euint64(
      18442470885463520691n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 1 (18442343533763371027, 18445982647402256701)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442343533763371027n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_uint64(
      encryptedAmount.handles[0],
      18445982647402256701n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 2 (18438477816592523449, 18438477816592523453)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438477816592523449n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_uint64(
      encryptedAmount.handles[0],
      18438477816592523453n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 3 (18438477816592523453, 18438477816592523453)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438477816592523453n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_uint64(
      encryptedAmount.handles[0],
      18438477816592523453n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 4 (18438477816592523453, 18438477816592523449)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438477816592523453n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_uint64(
      encryptedAmount.handles[0],
      18438477816592523449n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 1 (18440875682349733785, 18445982647402256701)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18445982647402256701n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_uint64_euint64(
      18440875682349733785n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 2 (18438477816592523449, 18438477816592523453)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18438477816592523453n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_uint64_euint64(
      18438477816592523449n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 3 (18438477816592523453, 18438477816592523453)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18438477816592523453n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_uint64_euint64(
      18438477816592523453n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 4 (18438477816592523453, 18438477816592523449)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18438477816592523449n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_uint64_euint64(
      18438477816592523453n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 1 (18441951286640352465, 18443569233121144247)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441951286640352465n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint64_uint64(
      encryptedAmount.handles[0],
      18443569233121144247n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 2 (18441951286640352461, 18441951286640352465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441951286640352461n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint64_uint64(
      encryptedAmount.handles[0],
      18441951286640352465n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 3 (18441951286640352465, 18441951286640352465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441951286640352465n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint64_uint64(
      encryptedAmount.handles[0],
      18441951286640352465n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 4 (18441951286640352465, 18441951286640352461)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441951286640352465n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint64_uint64(
      encryptedAmount.handles[0],
      18441951286640352461n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 1 (18439035083186430691, 18443569233121144247)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18443569233121144247n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_uint64_euint64(
      18439035083186430691n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 2 (18441951286640352461, 18441951286640352465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18441951286640352465n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_uint64_euint64(
      18441951286640352461n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 3 (18441951286640352465, 18441951286640352465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18441951286640352465n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_uint64_euint64(
      18441951286640352465n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 4 (18441951286640352465, 18441951286640352461)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18441951286640352461n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_uint64_euint64(
      18441951286640352465n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 1 (18443021258691337483, 18440565481263531071)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18443021258691337483n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint64_uint64(
      encryptedAmount.handles[0],
      18440565481263531071n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 2 (18443021258691337479, 18443021258691337483)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18443021258691337479n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint64_uint64(
      encryptedAmount.handles[0],
      18443021258691337483n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 3 (18443021258691337483, 18443021258691337483)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18443021258691337483n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint64_uint64(
      encryptedAmount.handles[0],
      18443021258691337483n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 4 (18443021258691337483, 18443021258691337479)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18443021258691337483n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint64_uint64(
      encryptedAmount.handles[0],
      18443021258691337479n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 1 (18439903764692512057, 18440565481263531071)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18440565481263531071n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_uint64_euint64(
      18439903764692512057n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 2 (18443021258691337479, 18443021258691337483)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18443021258691337483n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_uint64_euint64(
      18443021258691337479n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 3 (18443021258691337483, 18443021258691337483)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18443021258691337483n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_uint64_euint64(
      18443021258691337483n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 4 (18443021258691337483, 18443021258691337479)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18443021258691337479n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_uint64_euint64(
      18443021258691337483n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 1 (18443782238591650169, 18439298139086159343)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18443782238591650169n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_uint64(
      encryptedAmount.handles[0],
      18439298139086159343n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 2 (18441978790273961909, 18441978790273961913)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441978790273961909n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_uint64(
      encryptedAmount.handles[0],
      18441978790273961913n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 3 (18441978790273961913, 18441978790273961913)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441978790273961913n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_uint64(
      encryptedAmount.handles[0],
      18441978790273961913n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 4 (18441978790273961913, 18441978790273961909)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441978790273961913n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_uint64(
      encryptedAmount.handles[0],
      18441978790273961909n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 1 (18438386930936343347, 18439298139086159343)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18439298139086159343n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_uint64_euint64(
      18438386930936343347n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 2 (18441978790273961909, 18441978790273961913)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18441978790273961913n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_uint64_euint64(
      18441978790273961909n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 3 (18441978790273961913, 18441978790273961913)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18441978790273961913n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_uint64_euint64(
      18441978790273961913n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 4 (18441978790273961913, 18441978790273961909)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18441978790273961909n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_uint64_euint64(
      18441978790273961913n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 1 (18445972196307174659, 18441359463085216855)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18445972196307174659n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_uint64(
      encryptedAmount.handles[0],
      18441359463085216855n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 2 (18438760003066140567, 18438760003066140571)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438760003066140567n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_uint64(
      encryptedAmount.handles[0],
      18438760003066140571n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 3 (18438760003066140571, 18438760003066140571)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438760003066140571n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_uint64(
      encryptedAmount.handles[0],
      18438760003066140571n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 4 (18438760003066140571, 18438760003066140567)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438760003066140571n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_uint64(
      encryptedAmount.handles[0],
      18438760003066140567n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 1 (18440565799944043085, 18441359463085216855)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18441359463085216855n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_uint64_euint64(
      18440565799944043085n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 2 (18438760003066140567, 18438760003066140571)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18438760003066140571n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_uint64_euint64(
      18438760003066140567n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 3 (18438760003066140571, 18438760003066140571)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18438760003066140571n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_uint64_euint64(
      18438760003066140571n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 4 (18438760003066140571, 18438760003066140567)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18438760003066140567n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_uint64_euint64(
      18438760003066140571n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 1 (18446277918636147641, 18444361908116912137)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18446277918636147641n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_uint64(
      encryptedAmount.handles[0],
      18444361908116912137n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18444361908116912137n);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 2 (18439099819121701503, 18439099819121701507)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18439099819121701503n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_uint64(
      encryptedAmount.handles[0],
      18439099819121701507n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18439099819121701503n);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 3 (18439099819121701507, 18439099819121701507)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18439099819121701507n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_uint64(
      encryptedAmount.handles[0],
      18439099819121701507n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18439099819121701507n);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 4 (18439099819121701507, 18439099819121701503)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18439099819121701507n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_uint64(
      encryptedAmount.handles[0],
      18439099819121701503n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18439099819121701503n);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 1 (18445356693251014369, 18444361908116912137)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18444361908116912137n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_uint64_euint64(
      18445356693251014369n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18444361908116912137n);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 2 (18439099819121701503, 18439099819121701507)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18439099819121701507n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_uint64_euint64(
      18439099819121701503n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18439099819121701503n);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 3 (18439099819121701507, 18439099819121701507)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18439099819121701507n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_uint64_euint64(
      18439099819121701507n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18439099819121701507n);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 4 (18439099819121701507, 18439099819121701503)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18439099819121701503n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_uint64_euint64(
      18439099819121701507n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18439099819121701503n);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 1 (18438631683554897479, 18445384533883180417)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438631683554897479n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_uint64(
      encryptedAmount.handles[0],
      18445384533883180417n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18445384533883180417n);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 2 (18438631683554897475, 18438631683554897479)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438631683554897475n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_uint64(
      encryptedAmount.handles[0],
      18438631683554897479n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18438631683554897479n);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 3 (18438631683554897479, 18438631683554897479)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438631683554897479n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_uint64(
      encryptedAmount.handles[0],
      18438631683554897479n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18438631683554897479n);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 4 (18438631683554897479, 18438631683554897475)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438631683554897479n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_uint64(
      encryptedAmount.handles[0],
      18438631683554897475n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18438631683554897479n);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 1 (18440833161069655121, 18445384533883180417)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18445384533883180417n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_uint64_euint64(
      18440833161069655121n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18445384533883180417n);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 2 (18438631683554897475, 18438631683554897479)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18438631683554897479n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_uint64_euint64(
      18438631683554897475n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18438631683554897479n);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 3 (18438631683554897479, 18438631683554897479)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18438631683554897479n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_uint64_euint64(
      18438631683554897479n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18438631683554897479n);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 4 (18438631683554897479, 18438631683554897475)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18438631683554897475n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_uint64_euint64(
      18438631683554897479n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18438631683554897479n);
  });

  it('test operator "shl" overload (euint4, uint8) => euint4 test 1 (4, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add4(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shl_euint4_uint8(encryptedAmount.handles[0], 1n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract5.res4());
    expect(res).to.equal(8n);
  });

  it('test operator "shl" overload (euint4, uint8) => euint4 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add4(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shl_euint4_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract5.res4());
    expect(res).to.equal(4n);
  });

  it('test operator "shl" overload (euint4, uint8) => euint4 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add4(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shl_euint4_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract5.res4());
    expect(res).to.equal(8n);
  });

  it('test operator "shl" overload (euint4, uint8) => euint4 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add4(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shl_euint4_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract5.res4());
    expect(res).to.equal(8n);
  });

  it('test operator "shr" overload (euint4, uint8) => euint4 test 1 (7, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add4(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shr_euint4_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract5.res4());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint4, uint8) => euint4 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add4(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shr_euint4_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract5.res4());
    expect(res).to.equal(4n);
  });

  it('test operator "shr" overload (euint4, uint8) => euint4 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add4(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shr_euint4_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract5.res4());
    expect(res).to.equal(8n);
  });

  it('test operator "shr" overload (euint4, uint8) => euint4 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add4(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.shr_euint4_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract5.res4());
    expect(res).to.equal(8n);
  });

  it('test operator "rotl" overload (euint4, uint8) => euint4 test 1 (14, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add4(14n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotl_euint4_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract5.res4());
    expect(res).to.equal(7n);
  });

  it('test operator "rotl" overload (euint4, uint8) => euint4 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add4(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotl_euint4_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract5.res4());
    expect(res).to.equal(4n);
  });

  it('test operator "rotl" overload (euint4, uint8) => euint4 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add4(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotl_euint4_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract5.res4());
    expect(res).to.equal(8n);
  });

  it('test operator "rotl" overload (euint4, uint8) => euint4 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add4(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotl_euint4_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract5.res4());
    expect(res).to.equal(8n);
  });
});
