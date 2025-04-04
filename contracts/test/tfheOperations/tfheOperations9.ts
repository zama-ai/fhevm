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

    const instances = await createInstances(this.signers);
    this.instances = instances;
  });

  it('test operator "min" overload (euint128, euint32) => euint128 test 1 (340282366920938463463365949657313009859, 11760744)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463365949657313009859n);
    input.add32(11760744n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(11760744n);
  });

  it('test operator "min" overload (euint128, euint32) => euint128 test 2 (11760740, 11760744)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(11760740n);
    input.add32(11760744n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(11760740n);
  });

  it('test operator "min" overload (euint128, euint32) => euint128 test 3 (11760744, 11760744)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(11760744n);
    input.add32(11760744n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(11760744n);
  });

  it('test operator "min" overload (euint128, euint32) => euint128 test 4 (11760744, 11760740)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(11760744n);
    input.add32(11760740n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(11760740n);
  });

  it('test operator "max" overload (euint128, euint32) => euint128 test 1 (340282366920938463463366096897937462703, 1372823313)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463366096897937462703n);
    input.add32(1372823313n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(340282366920938463463366096897937462703n);
  });

  it('test operator "max" overload (euint128, euint32) => euint128 test 2 (1372823309, 1372823313)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(1372823309n);
    input.add32(1372823313n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(1372823313n);
  });

  it('test operator "max" overload (euint128, euint32) => euint128 test 3 (1372823313, 1372823313)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(1372823313n);
    input.add32(1372823313n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(1372823313n);
  });

  it('test operator "max" overload (euint128, euint32) => euint128 test 4 (1372823313, 1372823309)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(1372823313n);
    input.add32(1372823309n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(1372823313n);
  });

  it('test operator "add" overload (euint128, euint64) => euint128 test 1 (9223372036854775809, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(9223372036854775809n);
    input.add64(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(9223372036854775811n);
  });

  it('test operator "add" overload (euint128, euint64) => euint128 test 2 (9222199232386585896, 9222199232386585898)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(9222199232386585896n);
    input.add64(9222199232386585898n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(18444398464773171794n);
  });

  it('test operator "add" overload (euint128, euint64) => euint128 test 3 (9222199232386585898, 9222199232386585898)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(9222199232386585898n);
    input.add64(9222199232386585898n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(18444398464773171796n);
  });

  it('test operator "add" overload (euint128, euint64) => euint128 test 4 (9222199232386585898, 9222199232386585896)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(9222199232386585898n);
    input.add64(9222199232386585896n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(18444398464773171794n);
  });

  it('test operator "sub" overload (euint128, euint64) => euint128 test 1 (18442240831828327187, 18442240831828327187)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(18442240831828327187n);
    input.add64(18442240831828327187n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint128, euint64) => euint128 test 2 (18442240831828327187, 18442240831828327183)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(18442240831828327187n);
    input.add64(18442240831828327183n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint128, euint64) => euint128 test 1 (4611686018427387905, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(4611686018427387905n);
    input.add64(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(9223372036854775810n);
  });

  it('test operator "mul" overload (euint128, euint64) => euint128 test 2 (4293318218, 4293318218)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(4293318218n);
    input.add64(4293318218n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(18432581321010695524n);
  });

  it('test operator "mul" overload (euint128, euint64) => euint128 test 3 (4293318218, 4293318218)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(4293318218n);
    input.add64(4293318218n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(18432581321010695524n);
  });

  it('test operator "mul" overload (euint128, euint64) => euint128 test 4 (4293318218, 4293318218)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(4293318218n);
    input.add64(4293318218n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(18432581321010695524n);
  });

  it('test operator "and" overload (euint128, euint64) => euint128 test 1 (340282366920938463463369459151314777937, 18439629108536680321)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463369459151314777937n);
    input.add64(18439629108536680321n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(18439057018624415489n);
  });

  it('test operator "and" overload (euint128, euint64) => euint128 test 2 (18439629108536680317, 18439629108536680321)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(18439629108536680317n);
    input.add64(18439629108536680321n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(18439629108536680193n);
  });

  it('test operator "and" overload (euint128, euint64) => euint128 test 3 (18439629108536680321, 18439629108536680321)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(18439629108536680321n);
    input.add64(18439629108536680321n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(18439629108536680321n);
  });

  it('test operator "and" overload (euint128, euint64) => euint128 test 4 (18439629108536680321, 18439629108536680317)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(18439629108536680321n);
    input.add64(18439629108536680317n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(18439629108536680193n);
  });

  it('test operator "or" overload (euint128, euint64) => euint128 test 1 (340282366920938463463366008745326109131, 18439688249765123927)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463366008745326109131n);
    input.add64(18439688249765123927n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(340282366920938463463367851808942061535n);
  });

  it('test operator "or" overload (euint128, euint64) => euint128 test 2 (18439688249765123923, 18439688249765123927)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(18439688249765123923n);
    input.add64(18439688249765123927n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(18439688249765123927n);
  });

  it('test operator "or" overload (euint128, euint64) => euint128 test 3 (18439688249765123927, 18439688249765123927)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(18439688249765123927n);
    input.add64(18439688249765123927n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(18439688249765123927n);
  });

  it('test operator "or" overload (euint128, euint64) => euint128 test 4 (18439688249765123927, 18439688249765123923)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(18439688249765123927n);
    input.add64(18439688249765123923n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(18439688249765123927n);
  });

  it('test operator "xor" overload (euint128, euint64) => euint128 test 1 (340282366920938463463369381682612791979, 18438977594737970619)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463369381682612791979n);
    input.add64(18438977594737970619n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(340282366920938463444930404710715336464n);
  });

  it('test operator "xor" overload (euint128, euint64) => euint128 test 2 (18438977594737970615, 18438977594737970619)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(18438977594737970615n);
    input.add64(18438977594737970619n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint128, euint64) => euint128 test 3 (18438977594737970619, 18438977594737970619)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(18438977594737970619n);
    input.add64(18438977594737970619n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint128, euint64) => euint128 test 4 (18438977594737970619, 18438977594737970615)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(18438977594737970619n);
    input.add64(18438977594737970615n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint128, euint64) => ebool test 1 (340282366920938463463372303004574561063, 18443076163949049659)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463372303004574561063n);
    input.add64(18443076163949049659n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint64) => ebool test 2 (18443076163949049655, 18443076163949049659)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(18443076163949049655n);
    input.add64(18443076163949049659n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint64) => ebool test 3 (18443076163949049659, 18443076163949049659)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(18443076163949049659n);
    input.add64(18443076163949049659n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint128, euint64) => ebool test 4 (18443076163949049659, 18443076163949049655)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(18443076163949049659n);
    input.add64(18443076163949049655n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint64) => ebool test 1 (340282366920938463463374474621740026571, 18441262625269537579)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463374474621740026571n);
    input.add64(18441262625269537579n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint64) => ebool test 2 (18441262625269537575, 18441262625269537579)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(18441262625269537575n);
    input.add64(18441262625269537579n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint64) => ebool test 3 (18441262625269537579, 18441262625269537579)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(18441262625269537579n);
    input.add64(18441262625269537579n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint64) => ebool test 4 (18441262625269537579, 18441262625269537575)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(18441262625269537579n);
    input.add64(18441262625269537575n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint64) => ebool test 1 (340282366920938463463368895570182747957, 18438455664337628119)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368895570182747957n);
    input.add64(18438455664337628119n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint64) => ebool test 2 (18438455664337628115, 18438455664337628119)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(18438455664337628115n);
    input.add64(18438455664337628119n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint128, euint64) => ebool test 3 (18438455664337628119, 18438455664337628119)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(18438455664337628119n);
    input.add64(18438455664337628119n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint64) => ebool test 4 (18438455664337628119, 18438455664337628115)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(18438455664337628119n);
    input.add64(18438455664337628115n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint64) => ebool test 1 (340282366920938463463368689796627405019, 18442565153215400819)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368689796627405019n);
    input.add64(18442565153215400819n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint64) => ebool test 2 (18442565153215400815, 18442565153215400819)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(18442565153215400815n);
    input.add64(18442565153215400819n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint64) => ebool test 3 (18442565153215400819, 18442565153215400819)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(18442565153215400819n);
    input.add64(18442565153215400819n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint64) => ebool test 4 (18442565153215400819, 18442565153215400815)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(18442565153215400819n);
    input.add64(18442565153215400815n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint64) => ebool test 1 (340282366920938463463372354564945762101, 18439042372297403413)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463372354564945762101n);
    input.add64(18439042372297403413n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint128, euint64) => ebool test 2 (18439042372297403409, 18439042372297403413)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(18439042372297403409n);
    input.add64(18439042372297403413n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint64) => ebool test 3 (18439042372297403413, 18439042372297403413)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(18439042372297403413n);
    input.add64(18439042372297403413n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint64) => ebool test 4 (18439042372297403413, 18439042372297403409)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(18439042372297403413n);
    input.add64(18439042372297403409n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint64) => ebool test 1 (340282366920938463463369014973101426783, 18438118495628390561)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463369014973101426783n);
    input.add64(18438118495628390561n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint64) => ebool test 2 (18438118495628390557, 18438118495628390561)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(18438118495628390557n);
    input.add64(18438118495628390561n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint128, euint64) => ebool test 3 (18438118495628390561, 18438118495628390561)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(18438118495628390561n);
    input.add64(18438118495628390561n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint64) => ebool test 4 (18438118495628390561, 18438118495628390557)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(18438118495628390561n);
    input.add64(18438118495628390557n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint128, euint64) => euint128 test 1 (340282366920938463463367379900781721405, 18438188219947096021)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463367379900781721405n);
    input.add64(18438188219947096021n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(18438188219947096021n);
  });

  it('test operator "min" overload (euint128, euint64) => euint128 test 2 (18438188219947096017, 18438188219947096021)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(18438188219947096017n);
    input.add64(18438188219947096021n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(18438188219947096017n);
  });

  it('test operator "min" overload (euint128, euint64) => euint128 test 3 (18438188219947096021, 18438188219947096021)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(18438188219947096021n);
    input.add64(18438188219947096021n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(18438188219947096021n);
  });

  it('test operator "min" overload (euint128, euint64) => euint128 test 4 (18438188219947096021, 18438188219947096017)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(18438188219947096021n);
    input.add64(18438188219947096017n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(18438188219947096017n);
  });

  it('test operator "max" overload (euint128, euint64) => euint128 test 1 (340282366920938463463367341409498345311, 18438967523523513549)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463367341409498345311n);
    input.add64(18438967523523513549n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(340282366920938463463367341409498345311n);
  });

  it('test operator "max" overload (euint128, euint64) => euint128 test 2 (18438967523523513545, 18438967523523513549)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(18438967523523513545n);
    input.add64(18438967523523513549n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(18438967523523513549n);
  });

  it('test operator "max" overload (euint128, euint64) => euint128 test 3 (18438967523523513549, 18438967523523513549)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(18438967523523513549n);
    input.add64(18438967523523513549n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(18438967523523513549n);
  });

  it('test operator "max" overload (euint128, euint64) => euint128 test 4 (18438967523523513549, 18438967523523513545)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(18438967523523513549n);
    input.add64(18438967523523513545n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(18438967523523513549n);
  });

  it('test operator "add" overload (euint128, euint128) => euint128 test 1 (170141183460469231731685516098205407077, 170141183460469231731684538760324994647)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(170141183460469231731685516098205407077n);
    input.add128(170141183460469231731684538760324994647n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(340282366920938463463370054858530401724n);
  });

  it('test operator "add" overload (euint128, euint128) => euint128 test 2 (170141183460469231731684538760324994645, 170141183460469231731684538760324994647)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(170141183460469231731684538760324994645n);
    input.add128(170141183460469231731684538760324994647n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(340282366920938463463369077520649989292n);
  });

  it('test operator "add" overload (euint128, euint128) => euint128 test 3 (170141183460469231731684538760324994647, 170141183460469231731684538760324994647)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(170141183460469231731684538760324994647n);
    input.add128(170141183460469231731684538760324994647n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(340282366920938463463369077520649989294n);
  });

  it('test operator "add" overload (euint128, euint128) => euint128 test 4 (170141183460469231731684538760324994647, 170141183460469231731684538760324994645)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(170141183460469231731684538760324994647n);
    input.add128(170141183460469231731684538760324994645n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(340282366920938463463369077520649989292n);
  });

  it('test operator "sub" overload (euint128, euint128) => euint128 test 1 (340282366920938463463370046637544281703, 340282366920938463463370046637544281703)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463370046637544281703n);
    input.add128(340282366920938463463370046637544281703n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint128, euint128) => euint128 test 2 (340282366920938463463370046637544281703, 340282366920938463463370046637544281699)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463370046637544281703n);
    input.add128(340282366920938463463370046637544281699n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint128, euint128) => euint128 test 1 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(9223372036854775809n);
    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (euint128, euint128) => euint128 test 2 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(9223372036854775809n);
    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (euint128, euint128) => euint128 test 3 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(9223372036854775809n);
    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (euint128, euint128) => euint128 test 4 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(9223372036854775809n);
    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "and" overload (euint128, euint128) => euint128 test 1 (340282366920938463463372324400996915909, 340282366920938463463373519755374973619)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463372324400996915909n);
    input.add128(340282366920938463463373519755374973619n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(340282366920938463463371267665615196801n);
  });

  it('test operator "and" overload (euint128, euint128) => euint128 test 2 (340282366920938463463372324400996915905, 340282366920938463463372324400996915909)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463372324400996915905n);
    input.add128(340282366920938463463372324400996915909n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(340282366920938463463372324400996915905n);
  });

  it('test operator "and" overload (euint128, euint128) => euint128 test 3 (340282366920938463463372324400996915909, 340282366920938463463372324400996915909)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463372324400996915909n);
    input.add128(340282366920938463463372324400996915909n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(340282366920938463463372324400996915909n);
  });

  it('test operator "and" overload (euint128, euint128) => euint128 test 4 (340282366920938463463372324400996915909, 340282366920938463463372324400996915905)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463372324400996915909n);
    input.add128(340282366920938463463372324400996915905n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(340282366920938463463372324400996915905n);
  });

  it('test operator "or" overload (euint128, euint128) => euint128 test 1 (340282366920938463463372770999278311555, 340282366920938463463371606275412654519)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463372770999278311555n);
    input.add128(340282366920938463463371606275412654519n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(340282366920938463463373903705634765239n);
  });

  it('test operator "or" overload (euint128, euint128) => euint128 test 2 (340282366920938463463371606275412654515, 340282366920938463463371606275412654519)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463371606275412654515n);
    input.add128(340282366920938463463371606275412654519n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(340282366920938463463371606275412654519n);
  });

  it('test operator "or" overload (euint128, euint128) => euint128 test 3 (340282366920938463463371606275412654519, 340282366920938463463371606275412654519)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463371606275412654519n);
    input.add128(340282366920938463463371606275412654519n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(340282366920938463463371606275412654519n);
  });

  it('test operator "or" overload (euint128, euint128) => euint128 test 4 (340282366920938463463371606275412654519, 340282366920938463463371606275412654515)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463371606275412654519n);
    input.add128(340282366920938463463371606275412654515n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(340282366920938463463371606275412654519n);
  });

  it('test operator "xor" overload (euint128, euint128) => euint128 test 1 (340282366920938463463371675801867378167, 340282366920938463463369836943911233433)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463371675801867378167n);
    input.add128(340282366920938463463369836943911233433n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(7486533951355502n);
  });

  it('test operator "xor" overload (euint128, euint128) => euint128 test 2 (340282366920938463463369836943911233429, 340282366920938463463369836943911233433)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463369836943911233429n);
    input.add128(340282366920938463463369836943911233433n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint128, euint128) => euint128 test 3 (340282366920938463463369836943911233433, 340282366920938463463369836943911233433)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463369836943911233433n);
    input.add128(340282366920938463463369836943911233433n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint128, euint128) => euint128 test 4 (340282366920938463463369836943911233433, 340282366920938463463369836943911233429)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463369836943911233433n);
    input.add128(340282366920938463463369836943911233429n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint128, euint128) => ebool test 1 (340282366920938463463366888394195434295, 340282366920938463463371793830918830605)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463366888394195434295n);
    input.add128(340282366920938463463371793830918830605n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint128) => ebool test 2 (340282366920938463463366888394195434291, 340282366920938463463366888394195434295)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463366888394195434291n);
    input.add128(340282366920938463463366888394195434295n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint128) => ebool test 3 (340282366920938463463366888394195434295, 340282366920938463463366888394195434295)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463366888394195434295n);
    input.add128(340282366920938463463366888394195434295n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint128, euint128) => ebool test 4 (340282366920938463463366888394195434295, 340282366920938463463366888394195434291)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463366888394195434295n);
    input.add128(340282366920938463463366888394195434291n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint128) => ebool test 1 (340282366920938463463374254363369205911, 340282366920938463463366845706132855553)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463374254363369205911n);
    input.add128(340282366920938463463366845706132855553n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint128) => ebool test 2 (340282366920938463463366845706132855549, 340282366920938463463366845706132855553)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463366845706132855549n);
    input.add128(340282366920938463463366845706132855553n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint128) => ebool test 3 (340282366920938463463366845706132855553, 340282366920938463463366845706132855553)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463366845706132855553n);
    input.add128(340282366920938463463366845706132855553n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint128) => ebool test 4 (340282366920938463463366845706132855553, 340282366920938463463366845706132855549)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463366845706132855553n);
    input.add128(340282366920938463463366845706132855549n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint128) => ebool test 1 (340282366920938463463374062692812447739, 340282366920938463463368733357817288299)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463374062692812447739n);
    input.add128(340282366920938463463368733357817288299n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint128) => ebool test 2 (340282366920938463463368733357817288295, 340282366920938463463368733357817288299)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368733357817288295n);
    input.add128(340282366920938463463368733357817288299n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint128, euint128) => ebool test 3 (340282366920938463463368733357817288299, 340282366920938463463368733357817288299)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368733357817288299n);
    input.add128(340282366920938463463368733357817288299n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint128) => ebool test 4 (340282366920938463463368733357817288299, 340282366920938463463368733357817288295)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368733357817288299n);
    input.add128(340282366920938463463368733357817288295n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint128) => ebool test 1 (340282366920938463463368915607430090497, 340282366920938463463372287339193953057)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368915607430090497n);
    input.add128(340282366920938463463372287339193953057n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint128) => ebool test 2 (340282366920938463463368915607430090493, 340282366920938463463368915607430090497)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368915607430090493n);
    input.add128(340282366920938463463368915607430090497n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint128) => ebool test 3 (340282366920938463463368915607430090497, 340282366920938463463368915607430090497)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368915607430090497n);
    input.add128(340282366920938463463368915607430090497n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint128) => ebool test 4 (340282366920938463463368915607430090497, 340282366920938463463368915607430090493)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368915607430090497n);
    input.add128(340282366920938463463368915607430090493n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint128) => ebool test 1 (340282366920938463463373001702715130267, 340282366920938463463369072062439799757)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463373001702715130267n);
    input.add128(340282366920938463463369072062439799757n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint128, euint128) => ebool test 2 (340282366920938463463369072062439799753, 340282366920938463463369072062439799757)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463369072062439799753n);
    input.add128(340282366920938463463369072062439799757n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint128) => ebool test 3 (340282366920938463463369072062439799757, 340282366920938463463369072062439799757)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463369072062439799757n);
    input.add128(340282366920938463463369072062439799757n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint128) => ebool test 4 (340282366920938463463369072062439799757, 340282366920938463463369072062439799753)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463369072062439799757n);
    input.add128(340282366920938463463369072062439799753n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint128) => ebool test 1 (340282366920938463463366980508623565339, 340282366920938463463370386420209982707)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463366980508623565339n);
    input.add128(340282366920938463463370386420209982707n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint128, euint128) => ebool test 2 (340282366920938463463366980508623565335, 340282366920938463463366980508623565339)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463366980508623565335n);
    input.add128(340282366920938463463366980508623565339n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint128, euint128) => ebool test 3 (340282366920938463463366980508623565339, 340282366920938463463366980508623565339)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463366980508623565339n);
    input.add128(340282366920938463463366980508623565339n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint128) => ebool test 4 (340282366920938463463366980508623565339, 340282366920938463463366980508623565335)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463366980508623565339n);
    input.add128(340282366920938463463366980508623565335n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint128, euint128) => euint128 test 1 (340282366920938463463368061225670626939, 340282366920938463463373172380778486341)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368061225670626939n);
    input.add128(340282366920938463463373172380778486341n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(340282366920938463463368061225670626939n);
  });

  it('test operator "min" overload (euint128, euint128) => euint128 test 2 (340282366920938463463368061225670626935, 340282366920938463463368061225670626939)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368061225670626935n);
    input.add128(340282366920938463463368061225670626939n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(340282366920938463463368061225670626935n);
  });

  it('test operator "min" overload (euint128, euint128) => euint128 test 3 (340282366920938463463368061225670626939, 340282366920938463463368061225670626939)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368061225670626939n);
    input.add128(340282366920938463463368061225670626939n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(340282366920938463463368061225670626939n);
  });

  it('test operator "min" overload (euint128, euint128) => euint128 test 4 (340282366920938463463368061225670626939, 340282366920938463463368061225670626935)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368061225670626939n);
    input.add128(340282366920938463463368061225670626935n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(340282366920938463463368061225670626935n);
  });

  it('test operator "max" overload (euint128, euint128) => euint128 test 1 (340282366920938463463374443859252212225, 340282366920938463463366360738217278179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463374443859252212225n);
    input.add128(340282366920938463463366360738217278179n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(340282366920938463463374443859252212225n);
  });

  it('test operator "max" overload (euint128, euint128) => euint128 test 2 (340282366920938463463366360738217278175, 340282366920938463463366360738217278179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463366360738217278175n);
    input.add128(340282366920938463463366360738217278179n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(340282366920938463463366360738217278179n);
  });

  it('test operator "max" overload (euint128, euint128) => euint128 test 3 (340282366920938463463366360738217278179, 340282366920938463463366360738217278179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463366360738217278179n);
    input.add128(340282366920938463463366360738217278179n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(340282366920938463463366360738217278179n);
  });

  it('test operator "max" overload (euint128, euint128) => euint128 test 4 (340282366920938463463366360738217278179, 340282366920938463463366360738217278175)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463366360738217278179n);
    input.add128(340282366920938463463366360738217278175n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(340282366920938463463366360738217278179n);
  });

  it('test operator "add" overload (euint128, euint256) => euint256 test 1 (2, 170141183460469231731687303715884105729)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(2n);
    input.add256(170141183460469231731687303715884105729n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(170141183460469231731687303715884105731n);
  });

  it('test operator "add" overload (euint128, euint256) => euint256 test 2 (170141183460469231731684029132572422383, 170141183460469231731684029132572422385)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(170141183460469231731684029132572422383n);
    input.add256(170141183460469231731684029132572422385n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(340282366920938463463368058265144844768n);
  });

  it('test operator "add" overload (euint128, euint256) => euint256 test 3 (170141183460469231731684029132572422385, 170141183460469231731684029132572422385)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(170141183460469231731684029132572422385n);
    input.add256(170141183460469231731684029132572422385n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(340282366920938463463368058265144844770n);
  });

  it('test operator "add" overload (euint128, euint256) => euint256 test 4 (170141183460469231731684029132572422385, 170141183460469231731684029132572422383)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(170141183460469231731684029132572422385n);
    input.add256(170141183460469231731684029132572422383n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(340282366920938463463368058265144844768n);
  });

  it('test operator "sub" overload (euint128, euint256) => euint256 test 1 (340282366920938463463371172651106482265, 340282366920938463463371172651106482265)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463371172651106482265n);
    input.add256(340282366920938463463371172651106482265n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint128, euint256) => euint256 test 2 (340282366920938463463371172651106482265, 340282366920938463463371172651106482261)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463371172651106482265n);
    input.add256(340282366920938463463371172651106482261n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint128, euint256) => euint256 test 1 (2, 85070591730234615865843651857942052865)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(2n);
    input.add256(85070591730234615865843651857942052865n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(170141183460469231731687303715884105730n);
  });

  it('test operator "mul" overload (euint128, euint256) => euint256 test 2 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(9223372036854775809n);
    input.add256(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (euint128, euint256) => euint256 test 3 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(9223372036854775809n);
    input.add256(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (euint128, euint256) => euint256 test 4 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(9223372036854775809n);
    input.add256(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "and" overload (euint128, euint256) => euint256 test 1 (340282366920938463463366188165745480519, 115792089237316195423570985008687907853269984665640564039457580046446135158141)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463366188165745480519n);
    input.add256(115792089237316195423570985008687907853269984665640564039457580046446135158141n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(340282366920938463463365604699665334597n);
  });

  it('test operator "and" overload (euint128, euint256) => euint256 test 2 (340282366920938463463366188165745480515, 340282366920938463463366188165745480519)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463366188165745480515n);
    input.add256(340282366920938463463366188165745480519n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(340282366920938463463366188165745480515n);
  });

  it('test operator "and" overload (euint128, euint256) => euint256 test 3 (340282366920938463463366188165745480519, 340282366920938463463366188165745480519)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463366188165745480519n);
    input.add256(340282366920938463463366188165745480519n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(340282366920938463463366188165745480519n);
  });

  it('test operator "and" overload (euint128, euint256) => euint256 test 4 (340282366920938463463366188165745480519, 340282366920938463463366188165745480515)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463366188165745480519n);
    input.add256(340282366920938463463366188165745480515n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(340282366920938463463366188165745480515n);
  });

  it('test operator "or" overload (euint128, euint256) => euint256 test 1 (340282366920938463463372234967848689689, 115792089237316195423570985008687907853269984665640564039457579193673619836659)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463372234967848689689n);
    input.add256(115792089237316195423570985008687907853269984665640564039457579193673619836659n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457583998540285718267n);
  });

  it('test operator "or" overload (euint128, euint256) => euint256 test 2 (340282366920938463463372234967848689685, 340282366920938463463372234967848689689)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463372234967848689685n);
    input.add256(340282366920938463463372234967848689689n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(340282366920938463463372234967848689693n);
  });

  it('test operator "or" overload (euint128, euint256) => euint256 test 3 (340282366920938463463372234967848689689, 340282366920938463463372234967848689689)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463372234967848689689n);
    input.add256(340282366920938463463372234967848689689n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(340282366920938463463372234967848689689n);
  });

  it('test operator "or" overload (euint128, euint256) => euint256 test 4 (340282366920938463463372234967848689689, 340282366920938463463372234967848689685)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463372234967848689689n);
    input.add256(340282366920938463463372234967848689685n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(340282366920938463463372234967848689693n);
  });

  it('test operator "xor" overload (euint128, euint256) => euint256 test 1 (340282366920938463463370542110790227675, 115792089237316195423570985008687907853269984665640564039457576826414573143275)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463370542110790227675n);
    input.add256(115792089237316195423570985008687907853269984665640564039457576826414573143275n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(115792089237316195423570985008687907852929702298719625575994216141499685425712n);
  });

  it('test operator "xor" overload (euint128, euint256) => euint256 test 2 (340282366920938463463370542110790227671, 340282366920938463463370542110790227675)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463370542110790227671n);
    input.add256(340282366920938463463370542110790227675n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint128, euint256) => euint256 test 3 (340282366920938463463370542110790227675, 340282366920938463463370542110790227675)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463370542110790227675n);
    input.add256(340282366920938463463370542110790227675n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint128, euint256) => euint256 test 4 (340282366920938463463370542110790227675, 340282366920938463463370542110790227671)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463370542110790227675n);
    input.add256(340282366920938463463370542110790227671n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint128, euint256) => ebool test 1 (340282366920938463463368356405023004313, 115792089237316195423570985008687907853269984665640564039457581568247724896245)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368356405023004313n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581568247724896245n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint256) => ebool test 2 (340282366920938463463368356405023004309, 340282366920938463463368356405023004313)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368356405023004309n);
    input.add256(340282366920938463463368356405023004313n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint256) => ebool test 3 (340282366920938463463368356405023004313, 340282366920938463463368356405023004313)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368356405023004313n);
    input.add256(340282366920938463463368356405023004313n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint128, euint256) => ebool test 4 (340282366920938463463368356405023004313, 340282366920938463463368356405023004309)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368356405023004313n);
    input.add256(340282366920938463463368356405023004309n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint256) => ebool test 1 (340282366920938463463372635609281732377, 115792089237316195423570985008687907853269984665640564039457581782367165969971)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463372635609281732377n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581782367165969971n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint256) => ebool test 2 (340282366920938463463372635609281732373, 340282366920938463463372635609281732377)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463372635609281732373n);
    input.add256(340282366920938463463372635609281732377n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint256) => ebool test 3 (340282366920938463463372635609281732377, 340282366920938463463372635609281732377)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463372635609281732377n);
    input.add256(340282366920938463463372635609281732377n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint256) => ebool test 4 (340282366920938463463372635609281732377, 340282366920938463463372635609281732373)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463372635609281732377n);
    input.add256(340282366920938463463372635609281732373n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint256) => ebool test 1 (340282366920938463463373729063132626807, 115792089237316195423570985008687907853269984665640564039457583531225186004299)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463373729063132626807n);
    input.add256(115792089237316195423570985008687907853269984665640564039457583531225186004299n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint128, euint256) => ebool test 2 (340282366920938463463373729063132626803, 340282366920938463463373729063132626807)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463373729063132626803n);
    input.add256(340282366920938463463373729063132626807n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint128, euint256) => ebool test 3 (340282366920938463463373729063132626807, 340282366920938463463373729063132626807)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463373729063132626807n);
    input.add256(340282366920938463463373729063132626807n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint256) => ebool test 4 (340282366920938463463373729063132626807, 340282366920938463463373729063132626803)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463373729063132626807n);
    input.add256(340282366920938463463373729063132626803n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint256) => ebool test 1 (340282366920938463463372226475580558287, 115792089237316195423570985008687907853269984665640564039457576771818477678927)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463372226475580558287n);
    input.add256(115792089237316195423570985008687907853269984665640564039457576771818477678927n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint256) => ebool test 2 (340282366920938463463372226475580558283, 340282366920938463463372226475580558287)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463372226475580558283n);
    input.add256(340282366920938463463372226475580558287n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint256) => ebool test 3 (340282366920938463463372226475580558287, 340282366920938463463372226475580558287)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463372226475580558287n);
    input.add256(340282366920938463463372226475580558287n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint256) => ebool test 4 (340282366920938463463372226475580558287, 340282366920938463463372226475580558283)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463372226475580558287n);
    input.add256(340282366920938463463372226475580558283n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint256) => ebool test 1 (340282366920938463463371023221170759329, 115792089237316195423570985008687907853269984665640564039457576505431540037399)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463371023221170759329n);
    input.add256(115792089237316195423570985008687907853269984665640564039457576505431540037399n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint256) => ebool test 2 (340282366920938463463371023221170759325, 340282366920938463463371023221170759329)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463371023221170759325n);
    input.add256(340282366920938463463371023221170759329n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint256) => ebool test 3 (340282366920938463463371023221170759329, 340282366920938463463371023221170759329)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463371023221170759329n);
    input.add256(340282366920938463463371023221170759329n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint256) => ebool test 4 (340282366920938463463371023221170759329, 340282366920938463463371023221170759325)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463371023221170759329n);
    input.add256(340282366920938463463371023221170759325n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint256) => ebool test 1 (340282366920938463463365668556338397707, 115792089237316195423570985008687907853269984665640564039457579082361845675021)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463365668556338397707n);
    input.add256(115792089237316195423570985008687907853269984665640564039457579082361845675021n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint128, euint256) => ebool test 2 (340282366920938463463365668556338397703, 340282366920938463463365668556338397707)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463365668556338397703n);
    input.add256(340282366920938463463365668556338397707n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint128, euint256) => ebool test 3 (340282366920938463463365668556338397707, 340282366920938463463365668556338397707)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463365668556338397707n);
    input.add256(340282366920938463463365668556338397707n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint256) => ebool test 4 (340282366920938463463365668556338397707, 340282366920938463463365668556338397703)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463365668556338397707n);
    input.add256(340282366920938463463365668556338397703n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint128, euint256) => euint256 test 1 (340282366920938463463369524944509465765, 115792089237316195423570985008687907853269984665640564039457582438369905979805)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463369524944509465765n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582438369905979805n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(340282366920938463463369524944509465765n);
  });

  it('test operator "min" overload (euint128, euint256) => euint256 test 2 (340282366920938463463369524944509465761, 340282366920938463463369524944509465765)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463369524944509465761n);
    input.add256(340282366920938463463369524944509465765n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(340282366920938463463369524944509465761n);
  });

  it('test operator "min" overload (euint128, euint256) => euint256 test 3 (340282366920938463463369524944509465765, 340282366920938463463369524944509465765)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463369524944509465765n);
    input.add256(340282366920938463463369524944509465765n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(340282366920938463463369524944509465765n);
  });

  it('test operator "min" overload (euint128, euint256) => euint256 test 4 (340282366920938463463369524944509465765, 340282366920938463463369524944509465761)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463369524944509465765n);
    input.add256(340282366920938463463369524944509465761n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(340282366920938463463369524944509465761n);
  });

  it('test operator "max" overload (euint128, euint256) => euint256 test 1 (340282366920938463463365885120776956637, 115792089237316195423570985008687907853269984665640564039457579143137147893451)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463365885120776956637n);
    input.add256(115792089237316195423570985008687907853269984665640564039457579143137147893451n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579143137147893451n);
  });

  it('test operator "max" overload (euint128, euint256) => euint256 test 2 (340282366920938463463365885120776956633, 340282366920938463463365885120776956637)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463365885120776956633n);
    input.add256(340282366920938463463365885120776956637n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(340282366920938463463365885120776956637n);
  });

  it('test operator "max" overload (euint128, euint256) => euint256 test 3 (340282366920938463463365885120776956637, 340282366920938463463365885120776956637)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463365885120776956637n);
    input.add256(340282366920938463463365885120776956637n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(340282366920938463463365885120776956637n);
  });

  it('test operator "max" overload (euint128, euint256) => euint256 test 4 (340282366920938463463365885120776956637, 340282366920938463463365885120776956633)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463365885120776956637n);
    input.add256(340282366920938463463365885120776956633n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(340282366920938463463365885120776956637n);
  });

  it('test operator "add" overload (euint128, uint128) => euint128 test 1 (170141183460469231731685516098205407077, 170141183460469231731686260513550935580)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(170141183460469231731685516098205407077n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint128_uint128(
      encryptedAmount.handles[0],
      170141183460469231731686260513550935580n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463371776611756342657n);
  });

  it('test operator "add" overload (euint128, uint128) => euint128 test 2 (170141183460469231731684538760324994645, 170141183460469231731684538760324994647)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(170141183460469231731684538760324994645n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint128_uint128(
      encryptedAmount.handles[0],
      170141183460469231731684538760324994647n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463369077520649989292n);
  });

  it('test operator "add" overload (euint128, uint128) => euint128 test 3 (170141183460469231731684538760324994647, 170141183460469231731684538760324994647)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(170141183460469231731684538760324994647n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint128_uint128(
      encryptedAmount.handles[0],
      170141183460469231731684538760324994647n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463369077520649989294n);
  });

  it('test operator "add" overload (euint128, uint128) => euint128 test 4 (170141183460469231731684538760324994647, 170141183460469231731684538760324994645)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(170141183460469231731684538760324994647n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint128_uint128(
      encryptedAmount.handles[0],
      170141183460469231731684538760324994645n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463369077520649989292n);
  });

  it('test operator "add" overload (uint128, euint128) => euint128 test 1 (170141183460469231731686286969551878350, 170141183460469231731686260513550935580)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(170141183460469231731686260513550935580n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_uint128_euint128(
      170141183460469231731686286969551878350n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463372547483102813930n);
  });

  it('test operator "add" overload (uint128, euint128) => euint128 test 2 (170141183460469231731684538760324994645, 170141183460469231731684538760324994647)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(170141183460469231731684538760324994647n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_uint128_euint128(
      170141183460469231731684538760324994645n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463369077520649989292n);
  });

  it('test operator "add" overload (uint128, euint128) => euint128 test 3 (170141183460469231731684538760324994647, 170141183460469231731684538760324994647)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(170141183460469231731684538760324994647n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_uint128_euint128(
      170141183460469231731684538760324994647n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463369077520649989294n);
  });

  it('test operator "add" overload (uint128, euint128) => euint128 test 4 (170141183460469231731684538760324994647, 170141183460469231731684538760324994645)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(170141183460469231731684538760324994645n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_uint128_euint128(
      170141183460469231731684538760324994647n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463369077520649989292n);
  });

  it('test operator "sub" overload (euint128, uint128) => euint128 test 1 (340282366920938463463370046637544281703, 340282366920938463463370046637544281703)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463370046637544281703n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.sub_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463370046637544281703n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint128, uint128) => euint128 test 2 (340282366920938463463370046637544281703, 340282366920938463463370046637544281699)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463370046637544281703n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.sub_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463370046637544281699n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint128, euint128) => euint128 test 1 (340282366920938463463370046637544281703, 340282366920938463463370046637544281703)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463370046637544281703n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.sub_uint128_euint128(
      340282366920938463463370046637544281703n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint128, euint128) => euint128 test 2 (340282366920938463463370046637544281703, 340282366920938463463370046637544281699)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463370046637544281699n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.sub_uint128_euint128(
      340282366920938463463370046637544281703n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint128, uint128) => euint128 test 1 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(9223372036854775809n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint128_uint128(
      encryptedAmount.handles[0],
      9223372036854775809n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (euint128, uint128) => euint128 test 2 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(9223372036854775809n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint128_uint128(
      encryptedAmount.handles[0],
      9223372036854775809n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (euint128, uint128) => euint128 test 3 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(9223372036854775809n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint128_uint128(
      encryptedAmount.handles[0],
      9223372036854775809n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (euint128, uint128) => euint128 test 4 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(9223372036854775809n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint128_uint128(
      encryptedAmount.handles[0],
      9223372036854775809n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (uint128, euint128) => euint128 test 1 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_uint128_euint128(
      9223372036854775809n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (uint128, euint128) => euint128 test 2 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_uint128_euint128(
      9223372036854775809n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (uint128, euint128) => euint128 test 3 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_uint128_euint128(
      9223372036854775809n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (uint128, euint128) => euint128 test 4 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_uint128_euint128(
      9223372036854775809n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "div" overload (euint128, uint128) => euint128 test 1 (340282366920938463463365926114980153755, 340282366920938463463373846051125897477)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463365926114980153755n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.div_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463373846051125897477n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint128, uint128) => euint128 test 2 (340282366920938463463365926114980153751, 340282366920938463463365926114980153755)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463365926114980153751n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.div_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463365926114980153755n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint128, uint128) => euint128 test 3 (340282366920938463463365926114980153755, 340282366920938463463365926114980153755)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463365926114980153755n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.div_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463365926114980153755n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint128, uint128) => euint128 test 4 (340282366920938463463365926114980153755, 340282366920938463463365926114980153751)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463365926114980153755n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.div_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463365926114980153751n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint128, uint128) => euint128 test 1 (340282366920938463463366728616835626655, 340282366920938463463370152721765813661)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366728616835626655n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rem_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463370152721765813661n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463366728616835626655n);
  });

  it('test operator "rem" overload (euint128, uint128) => euint128 test 2 (340282366920938463463366728616835626651, 340282366920938463463366728616835626655)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366728616835626651n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rem_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366728616835626655n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463366728616835626651n);
  });

  it('test operator "rem" overload (euint128, uint128) => euint128 test 3 (340282366920938463463366728616835626655, 340282366920938463463366728616835626655)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366728616835626655n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rem_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366728616835626655n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint128, uint128) => euint128 test 4 (340282366920938463463366728616835626655, 340282366920938463463366728616835626651)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366728616835626655n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rem_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366728616835626651n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "and" overload (euint128, uint128) => euint128 test 1 (340282366920938463463372324400996915909, 340282366920938463463366154363094255921)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463372324400996915909n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366154363094255921n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463366131949738199041n);
  });

  it('test operator "and" overload (euint128, uint128) => euint128 test 2 (340282366920938463463372324400996915905, 340282366920938463463372324400996915909)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463372324400996915905n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463372324400996915909n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463372324400996915905n);
  });

  it('test operator "and" overload (euint128, uint128) => euint128 test 3 (340282366920938463463372324400996915909, 340282366920938463463372324400996915909)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463372324400996915909n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463372324400996915909n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463372324400996915909n);
  });

  it('test operator "and" overload (euint128, uint128) => euint128 test 4 (340282366920938463463372324400996915909, 340282366920938463463372324400996915905)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463372324400996915909n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463372324400996915905n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463372324400996915905n);
  });

  it('test operator "and" overload (uint128, euint128) => euint128 test 1 (340282366920938463463370748994735039739, 340282366920938463463366154363094255921)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463366154363094255921n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_uint128_euint128(
      340282366920938463463370748994735039739n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463365673627400618033n);
  });

  it('test operator "and" overload (uint128, euint128) => euint128 test 2 (340282366920938463463372324400996915905, 340282366920938463463372324400996915909)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463372324400996915909n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_uint128_euint128(
      340282366920938463463372324400996915905n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463372324400996915905n);
  });

  it('test operator "and" overload (uint128, euint128) => euint128 test 3 (340282366920938463463372324400996915909, 340282366920938463463372324400996915909)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463372324400996915909n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_uint128_euint128(
      340282366920938463463372324400996915909n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463372324400996915909n);
  });

  it('test operator "and" overload (uint128, euint128) => euint128 test 4 (340282366920938463463372324400996915909, 340282366920938463463372324400996915905)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463372324400996915905n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_uint128_euint128(
      340282366920938463463372324400996915909n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463372324400996915905n);
  });

  it('test operator "or" overload (euint128, uint128) => euint128 test 1 (340282366920938463463372770999278311555, 340282366920938463463371683211663356447)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463372770999278311555n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463371683211663356447n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463374042214070991519n);
  });

  it('test operator "or" overload (euint128, uint128) => euint128 test 2 (340282366920938463463371606275412654515, 340282366920938463463371606275412654519)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463371606275412654515n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463371606275412654519n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463371606275412654519n);
  });

  it('test operator "or" overload (euint128, uint128) => euint128 test 3 (340282366920938463463371606275412654519, 340282366920938463463371606275412654519)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463371606275412654519n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463371606275412654519n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463371606275412654519n);
  });

  it('test operator "or" overload (euint128, uint128) => euint128 test 4 (340282366920938463463371606275412654519, 340282366920938463463371606275412654515)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463371606275412654519n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463371606275412654515n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463371606275412654519n);
  });

  it('test operator "or" overload (uint128, euint128) => euint128 test 1 (340282366920938463463373311078423183599, 340282366920938463463371683211663356447)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463371683211663356447n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_uint128_euint128(
      340282366920938463463373311078423183599n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463374604133090718463n);
  });

  it('test operator "or" overload (uint128, euint128) => euint128 test 2 (340282366920938463463371606275412654515, 340282366920938463463371606275412654519)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463371606275412654519n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_uint128_euint128(
      340282366920938463463371606275412654515n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463371606275412654519n);
  });

  it('test operator "or" overload (uint128, euint128) => euint128 test 3 (340282366920938463463371606275412654519, 340282366920938463463371606275412654519)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463371606275412654519n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_uint128_euint128(
      340282366920938463463371606275412654519n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463371606275412654519n);
  });

  it('test operator "or" overload (uint128, euint128) => euint128 test 4 (340282366920938463463371606275412654519, 340282366920938463463371606275412654515)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463371606275412654515n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_uint128_euint128(
      340282366920938463463371606275412654519n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463371606275412654519n);
  });

  it('test operator "xor" overload (euint128, uint128) => euint128 test 1 (340282366920938463463371675801867378167, 340282366920938463463372122936455023921)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463371675801867378167n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463372122936455023921n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(767389898412230n);
  });

  it('test operator "xor" overload (euint128, uint128) => euint128 test 2 (340282366920938463463369836943911233429, 340282366920938463463369836943911233433)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463369836943911233429n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369836943911233433n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint128, uint128) => euint128 test 3 (340282366920938463463369836943911233433, 340282366920938463463369836943911233433)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463369836943911233433n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369836943911233433n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint128, uint128) => euint128 test 4 (340282366920938463463369836943911233433, 340282366920938463463369836943911233429)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463369836943911233433n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369836943911233429n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (uint128, euint128) => euint128 test 1 (340282366920938463463374387591957854199, 340282366920938463463372122936455023921)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463372122936455023921n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_uint128_euint128(
      340282366920938463463374387591957854199n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(2274150108068550n);
  });

  it('test operator "xor" overload (uint128, euint128) => euint128 test 2 (340282366920938463463369836943911233429, 340282366920938463463369836943911233433)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463369836943911233433n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_uint128_euint128(
      340282366920938463463369836943911233429n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (uint128, euint128) => euint128 test 3 (340282366920938463463369836943911233433, 340282366920938463463369836943911233433)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463369836943911233433n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_uint128_euint128(
      340282366920938463463369836943911233433n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (uint128, euint128) => euint128 test 4 (340282366920938463463369836943911233433, 340282366920938463463369836943911233429)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463369836943911233429n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_uint128_euint128(
      340282366920938463463369836943911233433n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint128, uint128) => ebool test 1 (340282366920938463463366888394195434295, 340282366920938463463371208508516878231)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366888394195434295n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463371208508516878231n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, uint128) => ebool test 2 (340282366920938463463366888394195434291, 340282366920938463463366888394195434295)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366888394195434291n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366888394195434295n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, uint128) => ebool test 3 (340282366920938463463366888394195434295, 340282366920938463463366888394195434295)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366888394195434295n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366888394195434295n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint128, uint128) => ebool test 4 (340282366920938463463366888394195434295, 340282366920938463463366888394195434291)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366888394195434295n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366888394195434291n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint128, euint128) => ebool test 1 (340282366920938463463369973465199782601, 340282366920938463463371208508516878231)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463371208508516878231n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_uint128_euint128(
      340282366920938463463369973465199782601n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint128, euint128) => ebool test 2 (340282366920938463463366888394195434291, 340282366920938463463366888394195434295)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463366888394195434295n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_uint128_euint128(
      340282366920938463463366888394195434291n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint128, euint128) => ebool test 3 (340282366920938463463366888394195434295, 340282366920938463463366888394195434295)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463366888394195434295n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_uint128_euint128(
      340282366920938463463366888394195434295n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint128, euint128) => ebool test 4 (340282366920938463463366888394195434295, 340282366920938463463366888394195434291)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463366888394195434291n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_uint128_euint128(
      340282366920938463463366888394195434295n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, uint128) => ebool test 1 (340282366920938463463374254363369205911, 340282366920938463463369625325157598821)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463374254363369205911n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369625325157598821n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, uint128) => ebool test 2 (340282366920938463463366845706132855549, 340282366920938463463366845706132855553)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366845706132855549n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366845706132855553n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, uint128) => ebool test 3 (340282366920938463463366845706132855553, 340282366920938463463366845706132855553)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366845706132855553n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366845706132855553n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, uint128) => ebool test 4 (340282366920938463463366845706132855553, 340282366920938463463366845706132855549)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366845706132855553n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366845706132855549n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint128, euint128) => ebool test 1 (340282366920938463463371505512933078875, 340282366920938463463369625325157598821)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463369625325157598821n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_uint128_euint128(
      340282366920938463463371505512933078875n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint128, euint128) => ebool test 2 (340282366920938463463366845706132855549, 340282366920938463463366845706132855553)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463366845706132855553n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_uint128_euint128(
      340282366920938463463366845706132855549n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint128, euint128) => ebool test 3 (340282366920938463463366845706132855553, 340282366920938463463366845706132855553)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463366845706132855553n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_uint128_euint128(
      340282366920938463463366845706132855553n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint128, euint128) => ebool test 4 (340282366920938463463366845706132855553, 340282366920938463463366845706132855549)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463366845706132855549n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_uint128_euint128(
      340282366920938463463366845706132855553n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });
});
