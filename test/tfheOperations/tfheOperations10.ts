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

describe('TFHE operations 10', function () {
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

  it('test operator "lt" overload (euint64, euint4) => ebool test 1 (18443056909764585511, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443056909764585511n);
    input.add4(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint4) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint4) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint4) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint4) => euint64 test 1 (18446705827641256747, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18446705827641256747n);
    input.add4(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(10n);
  });

  it('test operator "min" overload (euint64, euint4) => euint64 test 2 (6, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(6n);
    input.add4(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(6n);
  });

  it('test operator "min" overload (euint64, euint4) => euint64 test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(10n);
    input.add4(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(10n);
  });

  it('test operator "min" overload (euint64, euint4) => euint64 test 4 (10, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(10n);
    input.add4(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(6n);
  });

  it('test operator "max" overload (euint64, euint4) => euint64 test 1 (18445668950732298129, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445668950732298129n);
    input.add4(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(18445668950732298129n);
  });

  it('test operator "max" overload (euint64, euint4) => euint64 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint64, euint4) => euint64 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(8n);
  });

  it('test operator "max" overload (euint64, euint4) => euint64 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(8n);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 1 (129, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(129n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(131n);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 2 (89, 91)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(89n);
    input.add8(91n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(180n);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 3 (91, 91)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(91n);
    input.add8(91n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(182n);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 4 (91, 89)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(91n);
    input.add8(89n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(180n);
  });

  it('test operator "sub" overload (euint64, euint8) => euint64 test 1 (25, 25)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(25n);
    input.add8(25n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint8) => euint64 test 2 (25, 21)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(25n);
    input.add8(21n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 1 (65, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(65n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(130n);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 2 (9, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(9n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(90n);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(10n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(100n);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 4 (10, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(10n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(90n);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 1 (18441413446268134663, 40)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441413446268134663n);
    input.add8(40n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 2 (36, 40)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(36n);
    input.add8(40n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(32n);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 3 (40, 40)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(40n);
    input.add8(40n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(40n);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 4 (40, 36)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(40n);
    input.add8(36n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(32n);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 1 (18442062213086556497, 168)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442062213086556497n);
    input.add8(168n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(18442062213086556665n);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 2 (164, 168)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(164n);
    input.add8(168n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(172n);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 3 (168, 168)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(168n);
    input.add8(168n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(168n);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 4 (168, 164)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(168n);
    input.add8(164n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(172n);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 1 (18443626965217997733, 213)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443626965217997733n);
    input.add8(213n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(18443626965217997680n);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 2 (209, 213)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(209n);
    input.add8(213n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 3 (213, 213)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(213n);
    input.add8(213n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 4 (213, 209)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(213n);
    input.add8(209n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 1 (18446185668440461367, 98)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18446185668440461367n);
    input.add8(98n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 2 (94, 98)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(94n);
    input.add8(98n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 3 (98, 98)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(98n);
    input.add8(98n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 4 (98, 94)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(98n);
    input.add8(94n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 1 (18445474251392791649, 41)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445474251392791649n);
    input.add8(41n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 2 (37, 41)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(37n);
    input.add8(41n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 3 (41, 41)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(41n);
    input.add8(41n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 4 (41, 37)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(41n);
    input.add8(37n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 1 (18444294270645254277, 229)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18444294270645254277n);
    input.add8(229n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 2 (225, 229)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(225n);
    input.add8(229n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 3 (229, 229)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(229n);
    input.add8(229n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 4 (229, 225)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(229n);
    input.add8(225n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 1 (18446048537684194985, 113)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18446048537684194985n);
    input.add8(113n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 2 (109, 113)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(109n);
    input.add8(113n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 3 (113, 113)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(113n);
    input.add8(113n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 4 (113, 109)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(113n);
    input.add8(109n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 1 (18443934013808426023, 129)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18443934013808426023n);
    input.add8(129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 2 (125, 129)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(125n);
    input.add8(129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 3 (129, 129)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(129n);
    input.add8(129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 4 (129, 125)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(129n);
    input.add8(125n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 1 (18444118039600142421, 79)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18444118039600142421n);
    input.add8(79n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 2 (75, 79)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(75n);
    input.add8(79n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 3 (79, 79)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(79n);
    input.add8(79n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 4 (79, 75)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(79n);
    input.add8(75n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 1 (18440761228336909285, 170)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18440761228336909285n);
    input.add8(170n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(170n);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 2 (166, 170)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(166n);
    input.add8(170n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(166n);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 3 (170, 170)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(170n);
    input.add8(170n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(170n);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 4 (170, 166)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(170n);
    input.add8(166n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(166n);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 1 (18446007384031513667, 44)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18446007384031513667n);
    input.add8(44n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18446007384031513667n);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 2 (40, 44)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(40n);
    input.add8(44n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(44n);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 3 (44, 44)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(44n);
    input.add8(44n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(44n);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 4 (44, 40)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(44n);
    input.add8(40n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(44n);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 1 (65527, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(65527n);
    input.add16(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(65529n);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 2 (22102, 22104)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(22102n);
    input.add16(22104n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(44206n);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 3 (22104, 22104)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(22104n);
    input.add16(22104n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(44208n);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 4 (22104, 22102)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(22104n);
    input.add16(22102n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(44206n);
  });

  it('test operator "sub" overload (euint64, euint16) => euint64 test 1 (45502, 45502)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(45502n);
    input.add16(45502n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint16) => euint64 test 2 (45502, 45498)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(45502n);
    input.add16(45498n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 1 (32761, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(32761n);
    input.add16(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(65522n);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 2 (252, 252)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(252n);
    input.add16(252n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(63504n);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 3 (252, 252)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(252n);
    input.add16(252n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(63504n);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 4 (252, 252)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(252n);
    input.add16(252n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(63504n);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 1 (18446210605213941359, 52123)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18446210605213941359n);
    input.add16(52123n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(51723n);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 2 (52119, 52123)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(52119n);
    input.add16(52123n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(52115n);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 3 (52123, 52123)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(52123n);
    input.add16(52123n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(52123n);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 4 (52123, 52119)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(52123n);
    input.add16(52119n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(52115n);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 1 (18437769114436402597, 8954)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18437769114436402597n);
    input.add16(8954n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18437769114436403199n);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 2 (8950, 8954)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(8950n);
    input.add16(8954n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(8958n);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 3 (8954, 8954)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(8954n);
    input.add16(8954n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(8954n);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 4 (8954, 8950)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(8954n);
    input.add16(8950n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(8958n);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 1 (18442272025927693303, 37790)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442272025927693303n);
    input.add16(37790n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18442272025927721065n);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 2 (37786, 37790)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(37786n);
    input.add16(37790n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 3 (37790, 37790)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(37790n);
    input.add16(37790n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 4 (37790, 37786)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(37790n);
    input.add16(37786n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 1 (18439660654359160363, 41883)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18439660654359160363n);
    input.add16(41883n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 2 (41879, 41883)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(41879n);
    input.add16(41883n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 3 (41883, 41883)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(41883n);
    input.add16(41883n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 4 (41883, 41879)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(41883n);
    input.add16(41879n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 1 (18440863971063577997, 29590)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18440863971063577997n);
    input.add16(29590n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 2 (29586, 29590)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(29586n);
    input.add16(29590n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 3 (29590, 29590)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(29590n);
    input.add16(29590n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 4 (29590, 29586)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(29590n);
    input.add16(29586n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 1 (18437766862316060605, 31992)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18437766862316060605n);
    input.add16(31992n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 2 (31988, 31992)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(31988n);
    input.add16(31992n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 3 (31992, 31992)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(31992n);
    input.add16(31992n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 4 (31992, 31988)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(31992n);
    input.add16(31988n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 1 (18446546126614473389, 55175)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18446546126614473389n);
    input.add16(55175n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 2 (55171, 55175)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(55171n);
    input.add16(55175n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 3 (55175, 55175)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(55175n);
    input.add16(55175n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 4 (55175, 55171)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(55175n);
    input.add16(55171n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 1 (18445404390633548443, 34855)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18445404390633548443n);
    input.add16(34855n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 2 (34851, 34855)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(34851n);
    input.add16(34855n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 3 (34855, 34855)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(34855n);
    input.add16(34855n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 4 (34855, 34851)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(34855n);
    input.add16(34851n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 1 (18438335232029320935, 10564)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438335232029320935n);
    input.add16(10564n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 2 (10560, 10564)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(10560n);
    input.add16(10564n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 3 (10564, 10564)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(10564n);
    input.add16(10564n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 4 (10564, 10560)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(10564n);
    input.add16(10560n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 1 (18438622448280376415, 51692)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438622448280376415n);
    input.add16(51692n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(51692n);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 2 (51688, 51692)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(51688n);
    input.add16(51692n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(51688n);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 3 (51692, 51692)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(51692n);
    input.add16(51692n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(51692n);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 4 (51692, 51688)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(51692n);
    input.add16(51688n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(51688n);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 1 (18444284510736997839, 51628)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18444284510736997839n);
    input.add16(51628n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18444284510736997839n);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 2 (51624, 51628)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(51624n);
    input.add16(51628n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(51628n);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 3 (51628, 51628)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(51628n);
    input.add16(51628n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(51628n);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 4 (51628, 51624)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(51628n);
    input.add16(51624n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(51628n);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 1 (4293751578, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(4293751578n);
    input.add32(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(4293751580n);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 2 (1806019599, 1806019601)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(1806019599n);
    input.add32(1806019601n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(3612039200n);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 3 (1806019601, 1806019601)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(1806019601n);
    input.add32(1806019601n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(3612039202n);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 4 (1806019601, 1806019599)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(1806019601n);
    input.add32(1806019599n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(3612039200n);
  });

  it('test operator "sub" overload (euint64, euint32) => euint64 test 1 (3425308729, 3425308729)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(3425308729n);
    input.add32(3425308729n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint32) => euint64 test 2 (3425308729, 3425308725)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(3425308729n);
    input.add32(3425308725n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 1 (2147367784, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(2147367784n);
    input.add32(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(4294735568n);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 2 (50787, 50787)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(50787n);
    input.add32(50787n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(2579319369n);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 3 (50787, 50787)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(50787n);
    input.add32(50787n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(2579319369n);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 4 (50787, 50787)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(50787n);
    input.add32(50787n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(2579319369n);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 1 (18442857416785747441, 1547691532)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442857416785747441n);
    input.add32(1547691532n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(137446400n);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 2 (1547691528, 1547691532)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(1547691528n);
    input.add32(1547691532n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(1547691528n);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 3 (1547691532, 1547691532)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(1547691532n);
    input.add32(1547691532n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(1547691532n);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 4 (1547691532, 1547691528)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(1547691532n);
    input.add32(1547691528n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(1547691528n);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 1 (18438806940040470143, 729835566)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438806940040470143n);
    input.add32(729835566n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18438806940585754239n);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 2 (729835562, 729835566)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(729835562n);
    input.add32(729835566n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(729835566n);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 3 (729835566, 729835566)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(729835566n);
    input.add32(729835566n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(729835566n);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 4 (729835566, 729835562)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(729835566n);
    input.add32(729835562n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(729835566n);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 1 (18444516407156523745, 3421158537)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18444516407156523745n);
    input.add32(3421158537n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18444516404054425192n);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 2 (3421158533, 3421158537)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(3421158533n);
    input.add32(3421158537n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 3 (3421158537, 3421158537)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(3421158537n);
    input.add32(3421158537n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 4 (3421158537, 3421158533)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(3421158537n);
    input.add32(3421158533n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 1 (18445744067307966387, 926561519)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18445744067307966387n);
    input.add32(926561519n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 2 (926561515, 926561519)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(926561515n);
    input.add32(926561519n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 3 (926561519, 926561519)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(926561519n);
    input.add32(926561519n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 4 (926561519, 926561515)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(926561519n);
    input.add32(926561515n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 1 (18446476368209552349, 3024704127)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18446476368209552349n);
    input.add32(3024704127n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 2 (3024704123, 3024704127)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(3024704123n);
    input.add32(3024704127n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 3 (3024704127, 3024704127)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(3024704127n);
    input.add32(3024704127n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 4 (3024704127, 3024704123)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(3024704127n);
    input.add32(3024704123n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 1 (18440264286866387829, 85256317)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18440264286866387829n);
    input.add32(85256317n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 2 (85256313, 85256317)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(85256313n);
    input.add32(85256317n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 3 (85256317, 85256317)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(85256317n);
    input.add32(85256317n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 4 (85256317, 85256313)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(85256317n);
    input.add32(85256313n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 1 (18443613995542007651, 1679838761)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18443613995542007651n);
    input.add32(1679838761n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 2 (1679838757, 1679838761)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(1679838757n);
    input.add32(1679838761n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 3 (1679838761, 1679838761)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(1679838761n);
    input.add32(1679838761n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 4 (1679838761, 1679838757)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(1679838761n);
    input.add32(1679838757n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 1 (18443133688534935497, 622548585)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18443133688534935497n);
    input.add32(622548585n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 2 (622548581, 622548585)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(622548581n);
    input.add32(622548585n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 3 (622548585, 622548585)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(622548585n);
    input.add32(622548585n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 4 (622548585, 622548581)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(622548585n);
    input.add32(622548581n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 1 (18446696862115863979, 853267915)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18446696862115863979n);
    input.add32(853267915n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 2 (853267911, 853267915)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(853267911n);
    input.add32(853267915n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 3 (853267915, 853267915)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(853267915n);
    input.add32(853267915n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 4 (853267915, 853267911)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(853267915n);
    input.add32(853267911n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });
});
