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

describe('TFHE operations 8', function () {
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

  it('test operator "and" overload (euint64, uint64) => euint64 test 1 (18438288845569948835, 18444816578527641653)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438288845569948835n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint64_uint64(
      encryptedAmount.handles[0],
      18444816578527641653n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18438060131043262497n);
  });

  it('test operator "and" overload (euint64, uint64) => euint64 test 2 (18438288845569948831, 18438288845569948835)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438288845569948831n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint64_uint64(
      encryptedAmount.handles[0],
      18438288845569948835n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18438288845569948803n);
  });

  it('test operator "and" overload (euint64, uint64) => euint64 test 3 (18438288845569948835, 18438288845569948835)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438288845569948835n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint64_uint64(
      encryptedAmount.handles[0],
      18438288845569948835n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18438288845569948835n);
  });

  it('test operator "and" overload (euint64, uint64) => euint64 test 4 (18438288845569948835, 18438288845569948831)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438288845569948835n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint64_uint64(
      encryptedAmount.handles[0],
      18438288845569948831n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18438288845569948803n);
  });

  it('test operator "and" overload (uint64, euint64) => euint64 test 1 (18441148552025575777, 18444816578527641653)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18444816578527641653n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint64_euint64(
      18441148552025575777n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18439996250660012065n);
  });

  it('test operator "and" overload (uint64, euint64) => euint64 test 2 (18438288845569948831, 18438288845569948835)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18438288845569948835n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint64_euint64(
      18438288845569948831n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18438288845569948803n);
  });

  it('test operator "and" overload (uint64, euint64) => euint64 test 3 (18438288845569948835, 18438288845569948835)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18438288845569948835n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint64_euint64(
      18438288845569948835n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18438288845569948835n);
  });

  it('test operator "and" overload (uint64, euint64) => euint64 test 4 (18438288845569948835, 18438288845569948831)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18438288845569948831n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint64_euint64(
      18438288845569948835n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18438288845569948803n);
  });

  it('test operator "or" overload (euint64, uint64) => euint64 test 1 (18441718910132730219, 18438685796806051089)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441718910132730219n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint64_uint64(
      encryptedAmount.handles[0],
      18438685796806051089n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18442099375812050299n);
  });

  it('test operator "or" overload (euint64, uint64) => euint64 test 2 (18441718910132730215, 18441718910132730219)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441718910132730215n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint64_uint64(
      encryptedAmount.handles[0],
      18441718910132730219n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18441718910132730223n);
  });

  it('test operator "or" overload (euint64, uint64) => euint64 test 3 (18441718910132730219, 18441718910132730219)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441718910132730219n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint64_uint64(
      encryptedAmount.handles[0],
      18441718910132730219n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18441718910132730219n);
  });

  it('test operator "or" overload (euint64, uint64) => euint64 test 4 (18441718910132730219, 18441718910132730215)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441718910132730219n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint64_uint64(
      encryptedAmount.handles[0],
      18441718910132730215n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18441718910132730223n);
  });

  it('test operator "or" overload (uint64, euint64) => euint64 test 1 (18439408010330556653, 18438685796806051089)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18438685796806051089n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint64_euint64(
      18439408010330556653n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18439988587156917757n);
  });

  it('test operator "or" overload (uint64, euint64) => euint64 test 2 (18441718910132730215, 18441718910132730219)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18441718910132730219n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint64_euint64(
      18441718910132730215n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18441718910132730223n);
  });

  it('test operator "or" overload (uint64, euint64) => euint64 test 3 (18441718910132730219, 18441718910132730219)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18441718910132730219n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint64_euint64(
      18441718910132730219n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18441718910132730219n);
  });

  it('test operator "or" overload (uint64, euint64) => euint64 test 4 (18441718910132730219, 18441718910132730215)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18441718910132730215n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint64_euint64(
      18441718910132730219n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18441718910132730223n);
  });

  it('test operator "xor" overload (euint64, uint64) => euint64 test 1 (18439273159029534023, 18443969378468224271)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18439273159029534023n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint64_uint64(
      encryptedAmount.handles[0],
      18443969378468224271n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(5437444897796168n);
  });

  it('test operator "xor" overload (euint64, uint64) => euint64 test 2 (18439273159029534019, 18439273159029534023)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18439273159029534019n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint64_uint64(
      encryptedAmount.handles[0],
      18439273159029534023n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint64, uint64) => euint64 test 3 (18439273159029534023, 18439273159029534023)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18439273159029534023n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint64_uint64(
      encryptedAmount.handles[0],
      18439273159029534023n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, uint64) => euint64 test 4 (18439273159029534023, 18439273159029534019)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18439273159029534023n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint64_uint64(
      encryptedAmount.handles[0],
      18439273159029534019n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (uint64, euint64) => euint64 test 1 (18442844639706172065, 18443969378468224271)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18443969378468224271n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint64_euint64(
      18442844639706172065n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(1127075362674606n);
  });

  it('test operator "xor" overload (uint64, euint64) => euint64 test 2 (18439273159029534019, 18439273159029534023)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18439273159029534023n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint64_euint64(
      18439273159029534019n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (uint64, euint64) => euint64 test 3 (18439273159029534023, 18439273159029534023)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18439273159029534023n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint64_euint64(
      18439273159029534023n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (uint64, euint64) => euint64 test 4 (18439273159029534023, 18439273159029534019)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18439273159029534019n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint64_euint64(
      18439273159029534023n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 1 (18441380505660187447, 18443565496928144751)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441380505660187447n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint64_uint64(
      encryptedAmount.handles[0],
      18443565496928144751n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 2 (18441380505660187443, 18441380505660187447)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441380505660187443n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint64_uint64(
      encryptedAmount.handles[0],
      18441380505660187447n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 3 (18441380505660187447, 18441380505660187447)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441380505660187447n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint64_uint64(
      encryptedAmount.handles[0],
      18441380505660187447n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 4 (18441380505660187447, 18441380505660187443)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441380505660187447n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint64_uint64(
      encryptedAmount.handles[0],
      18441380505660187443n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 1 (18445774206960795205, 18443565496928144751)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18443565496928144751n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_uint64_euint64(
      18445774206960795205n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 2 (18441380505660187443, 18441380505660187447)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18441380505660187447n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_uint64_euint64(
      18441380505660187443n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 3 (18441380505660187447, 18441380505660187447)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18441380505660187447n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_uint64_euint64(
      18441380505660187447n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 4 (18441380505660187447, 18441380505660187443)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18441380505660187443n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_uint64_euint64(
      18441380505660187447n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 1 (18442006653940621261, 18445076347981598257)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442006653940621261n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_uint64(
      encryptedAmount.handles[0],
      18445076347981598257n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 2 (18442006653940621257, 18442006653940621261)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442006653940621257n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_uint64(
      encryptedAmount.handles[0],
      18442006653940621261n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 3 (18442006653940621261, 18442006653940621261)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442006653940621261n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_uint64(
      encryptedAmount.handles[0],
      18442006653940621261n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 4 (18442006653940621261, 18442006653940621257)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442006653940621261n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_uint64(
      encryptedAmount.handles[0],
      18442006653940621257n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 1 (18441865368204221349, 18445076347981598257)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18445076347981598257n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_uint64_euint64(
      18441865368204221349n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 2 (18442006653940621257, 18442006653940621261)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18442006653940621261n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_uint64_euint64(
      18442006653940621257n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 3 (18442006653940621261, 18442006653940621261)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18442006653940621261n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_uint64_euint64(
      18442006653940621261n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 4 (18442006653940621261, 18442006653940621257)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18442006653940621257n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_uint64_euint64(
      18442006653940621261n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 1 (18443459697052755653, 18440809623611186563)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18443459697052755653n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint64_uint64(
      encryptedAmount.handles[0],
      18440809623611186563n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 2 (18440630546892693039, 18440630546892693043)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18440630546892693039n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint64_uint64(
      encryptedAmount.handles[0],
      18440630546892693043n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 3 (18440630546892693043, 18440630546892693043)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18440630546892693043n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint64_uint64(
      encryptedAmount.handles[0],
      18440630546892693043n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 4 (18440630546892693043, 18440630546892693039)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18440630546892693043n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint64_uint64(
      encryptedAmount.handles[0],
      18440630546892693039n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 1 (18439264083414039003, 18440809623611186563)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18440809623611186563n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_uint64_euint64(
      18439264083414039003n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 2 (18440630546892693039, 18440630546892693043)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18440630546892693043n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_uint64_euint64(
      18440630546892693039n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 3 (18440630546892693043, 18440630546892693043)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18440630546892693043n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_uint64_euint64(
      18440630546892693043n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 4 (18440630546892693043, 18440630546892693039)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18440630546892693039n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_uint64_euint64(
      18440630546892693043n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 1 (18440239741167735311, 18439299750028762115)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18440239741167735311n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint64_uint64(
      encryptedAmount.handles[0],
      18439299750028762115n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 2 (18439106717100863705, 18439106717100863709)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18439106717100863705n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint64_uint64(
      encryptedAmount.handles[0],
      18439106717100863709n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 3 (18439106717100863709, 18439106717100863709)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18439106717100863709n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint64_uint64(
      encryptedAmount.handles[0],
      18439106717100863709n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 4 (18439106717100863709, 18439106717100863705)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18439106717100863709n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint64_uint64(
      encryptedAmount.handles[0],
      18439106717100863705n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 1 (18442107957659276659, 18439299750028762115)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18439299750028762115n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_uint64_euint64(
      18442107957659276659n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 2 (18439106717100863705, 18439106717100863709)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18439106717100863709n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_uint64_euint64(
      18439106717100863705n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 3 (18439106717100863709, 18439106717100863709)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18439106717100863709n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_uint64_euint64(
      18439106717100863709n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 4 (18439106717100863709, 18439106717100863705)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18439106717100863705n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_uint64_euint64(
      18439106717100863709n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 1 (18443095947977784597, 18442198296159931725)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18443095947977784597n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_uint64(
      encryptedAmount.handles[0],
      18442198296159931725n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 2 (18440873805049812051, 18440873805049812055)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18440873805049812051n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_uint64(
      encryptedAmount.handles[0],
      18440873805049812055n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 3 (18440873805049812055, 18440873805049812055)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18440873805049812055n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_uint64(
      encryptedAmount.handles[0],
      18440873805049812055n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 4 (18440873805049812055, 18440873805049812051)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18440873805049812055n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_uint64(
      encryptedAmount.handles[0],
      18440873805049812051n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 1 (18443597914071393227, 18442198296159931725)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18442198296159931725n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_uint64_euint64(
      18443597914071393227n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 2 (18440873805049812051, 18440873805049812055)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18440873805049812055n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_uint64_euint64(
      18440873805049812051n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 3 (18440873805049812055, 18440873805049812055)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18440873805049812055n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_uint64_euint64(
      18440873805049812055n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 4 (18440873805049812055, 18440873805049812051)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add64(18440873805049812051n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_uint64_euint64(
      18440873805049812055n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 1 (18438938996100953011, 18437912790819001121)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438938996100953011n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint64_uint64(
      encryptedAmount.handles[0],
      18437912790819001121n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 2 (18438938996100953007, 18438938996100953011)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438938996100953007n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint64_uint64(
      encryptedAmount.handles[0],
      18438938996100953011n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 3 (18438938996100953011, 18438938996100953011)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438938996100953011n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint64_uint64(
      encryptedAmount.handles[0],
      18438938996100953011n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 4 (18438938996100953011, 18438938996100953007)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438938996100953011n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint64_uint64(
      encryptedAmount.handles[0],
      18438938996100953007n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 1 (18439091760430839739, 18437912790819001121)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18437912790819001121n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint64_euint64(
      18439091760430839739n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 2 (18438938996100953007, 18438938996100953011)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18438938996100953011n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint64_euint64(
      18438938996100953007n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 3 (18438938996100953011, 18438938996100953011)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18438938996100953011n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint64_euint64(
      18438938996100953011n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint64, euint64) => ebool test 4 (18438938996100953011, 18438938996100953007)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18438938996100953007n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint64_euint64(
      18438938996100953011n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 1 (18443917910351942171, 18439074624335339465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18443917910351942171n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint64_uint64(
      encryptedAmount.handles[0],
      18439074624335339465n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(18439074624335339465n);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 2 (18438866711307263079, 18438866711307263083)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438866711307263079n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint64_uint64(
      encryptedAmount.handles[0],
      18438866711307263083n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(18438866711307263079n);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 3 (18438866711307263083, 18438866711307263083)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438866711307263083n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint64_uint64(
      encryptedAmount.handles[0],
      18438866711307263083n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(18438866711307263083n);
  });

  it('test operator "min" overload (euint64, uint64) => euint64 test 4 (18438866711307263083, 18438866711307263079)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438866711307263083n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint64_uint64(
      encryptedAmount.handles[0],
      18438866711307263079n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(18438866711307263079n);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 1 (18441411991460023399, 18439074624335339465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18439074624335339465n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint64_euint64(
      18441411991460023399n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(18439074624335339465n);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 2 (18438866711307263079, 18438866711307263083)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18438866711307263083n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint64_euint64(
      18438866711307263079n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(18438866711307263079n);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 3 (18438866711307263083, 18438866711307263083)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18438866711307263083n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint64_euint64(
      18438866711307263083n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(18438866711307263083n);
  });

  it('test operator "min" overload (uint64, euint64) => euint64 test 4 (18438866711307263083, 18438866711307263079)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18438866711307263079n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint64_euint64(
      18438866711307263083n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(18438866711307263079n);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 1 (18444366681014095179, 18438373001644295523)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18444366681014095179n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint64_uint64(
      encryptedAmount.handles[0],
      18438373001644295523n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(18444366681014095179n);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 2 (18439526748827954071, 18439526748827954075)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439526748827954071n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint64_uint64(
      encryptedAmount.handles[0],
      18439526748827954075n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(18439526748827954075n);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 3 (18439526748827954075, 18439526748827954075)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439526748827954075n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint64_uint64(
      encryptedAmount.handles[0],
      18439526748827954075n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(18439526748827954075n);
  });

  it('test operator "max" overload (euint64, uint64) => euint64 test 4 (18439526748827954075, 18439526748827954071)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439526748827954075n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint64_uint64(
      encryptedAmount.handles[0],
      18439526748827954071n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(18439526748827954075n);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 1 (18439918292761913061, 18438373001644295523)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18438373001644295523n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint64_euint64(
      18439918292761913061n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(18439918292761913061n);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 2 (18439526748827954071, 18439526748827954075)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18439526748827954075n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint64_euint64(
      18439526748827954071n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(18439526748827954075n);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 3 (18439526748827954075, 18439526748827954075)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18439526748827954075n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint64_euint64(
      18439526748827954075n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(18439526748827954075n);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 4 (18439526748827954075, 18439526748827954071)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18439526748827954071n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint64_euint64(
      18439526748827954075n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(18439526748827954075n);
  });

  it('test operator "add" overload (euint128, euint8) => euint128 test 1 (129, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(129n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(131n);
  });

  it('test operator "add" overload (euint128, euint8) => euint128 test 2 (95, 97)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(95n);
    input.add8(97n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(192n);
  });

  it('test operator "add" overload (euint128, euint8) => euint128 test 3 (97, 97)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(97n);
    input.add8(97n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(194n);
  });

  it('test operator "add" overload (euint128, euint8) => euint128 test 4 (97, 95)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(97n);
    input.add8(95n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(192n);
  });

  it('test operator "sub" overload (euint128, euint8) => euint128 test 1 (251, 251)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(251n);
    input.add8(251n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint128, euint8) => euint128 test 2 (251, 247)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(251n);
    input.add8(247n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint128, euint8) => euint128 test 1 (65, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(65n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(130n);
  });

  it('test operator "mul" overload (euint128, euint8) => euint128 test 2 (5, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(5n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(45n);
  });

  it('test operator "mul" overload (euint128, euint8) => euint128 test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(9n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(81n);
  });

  it('test operator "mul" overload (euint128, euint8) => euint128 test 4 (9, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(9n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(45n);
  });

  it('test operator "and" overload (euint128, euint8) => euint128 test 1 (340282366920938463463374440379069679429, 55)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463374440379069679429n);
    input.add8(55n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(5n);
  });

  it('test operator "and" overload (euint128, euint8) => euint128 test 2 (51, 55)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(51n);
    input.add8(55n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(51n);
  });

  it('test operator "and" overload (euint128, euint8) => euint128 test 3 (55, 55)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(55n);
    input.add8(55n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(55n);
  });

  it('test operator "and" overload (euint128, euint8) => euint128 test 4 (55, 51)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(55n);
    input.add8(51n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(51n);
  });

  it('test operator "or" overload (euint128, euint8) => euint128 test 1 (340282366920938463463370184450644016425, 219)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463370184450644016425n);
    input.add8(219n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(340282366920938463463370184450644016635n);
  });

  it('test operator "or" overload (euint128, euint8) => euint128 test 2 (215, 219)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(215n);
    input.add8(219n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(223n);
  });

  it('test operator "or" overload (euint128, euint8) => euint128 test 3 (219, 219)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(219n);
    input.add8(219n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(219n);
  });

  it('test operator "or" overload (euint128, euint8) => euint128 test 4 (219, 215)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(219n);
    input.add8(215n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(223n);
  });

  it('test operator "xor" overload (euint128, euint8) => euint128 test 1 (340282366920938463463369357205050683457, 250)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463369357205050683457n);
    input.add8(250n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(340282366920938463463369357205050683579n);
  });

  it('test operator "xor" overload (euint128, euint8) => euint128 test 2 (246, 250)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(246n);
    input.add8(250n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint128, euint8) => euint128 test 3 (250, 250)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(250n);
    input.add8(250n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint128, euint8) => euint128 test 4 (250, 246)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(250n);
    input.add8(246n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint128, euint8) => ebool test 1 (340282366920938463463371173783702089729, 151)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463371173783702089729n);
    input.add8(151n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint8) => ebool test 2 (147, 151)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(147n);
    input.add8(151n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint8) => ebool test 3 (151, 151)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(151n);
    input.add8(151n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint128, euint8) => ebool test 4 (151, 147)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(151n);
    input.add8(147n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint8) => ebool test 1 (340282366920938463463368243460974306219, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368243460974306219n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint8) => ebool test 2 (6, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(6n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint8) => ebool test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(10n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint8) => ebool test 4 (10, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(10n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint8) => ebool test 1 (340282366920938463463367320388075568227, 160)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463367320388075568227n);
    input.add8(160n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint8) => ebool test 2 (156, 160)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(156n);
    input.add8(160n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint128, euint8) => ebool test 3 (160, 160)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(160n);
    input.add8(160n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint8) => ebool test 4 (160, 156)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(160n);
    input.add8(156n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint8) => ebool test 1 (340282366920938463463371452286754799841, 129)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463371452286754799841n);
    input.add8(129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint8) => ebool test 2 (125, 129)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(125n);
    input.add8(129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint8) => ebool test 3 (129, 129)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(129n);
    input.add8(129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint8) => ebool test 4 (129, 125)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(129n);
    input.add8(125n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint8) => ebool test 1 (340282366920938463463366219489970616751, 237)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463366219489970616751n);
    input.add8(237n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint128, euint8) => ebool test 2 (233, 237)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(233n);
    input.add8(237n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint8) => ebool test 3 (237, 237)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(237n);
    input.add8(237n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint8) => ebool test 4 (237, 233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(237n);
    input.add8(233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint8) => ebool test 1 (340282366920938463463373208324647322337, 109)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463373208324647322337n);
    input.add8(109n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint8) => ebool test 2 (105, 109)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(105n);
    input.add8(109n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint128, euint8) => ebool test 3 (109, 109)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(109n);
    input.add8(109n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint8) => ebool test 4 (109, 105)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(109n);
    input.add8(105n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint128, euint8) => euint128 test 1 (340282366920938463463373568004603464131, 158)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463373568004603464131n);
    input.add8(158n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(158n);
  });

  it('test operator "min" overload (euint128, euint8) => euint128 test 2 (154, 158)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(154n);
    input.add8(158n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(154n);
  });

  it('test operator "min" overload (euint128, euint8) => euint128 test 3 (158, 158)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(158n);
    input.add8(158n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(158n);
  });

  it('test operator "min" overload (euint128, euint8) => euint128 test 4 (158, 154)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(158n);
    input.add8(154n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(154n);
  });

  it('test operator "max" overload (euint128, euint8) => euint128 test 1 (340282366920938463463371224543936087299, 71)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463371224543936087299n);
    input.add8(71n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(340282366920938463463371224543936087299n);
  });

  it('test operator "max" overload (euint128, euint8) => euint128 test 2 (67, 71)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(67n);
    input.add8(71n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(71n);
  });

  it('test operator "max" overload (euint128, euint8) => euint128 test 3 (71, 71)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(71n);
    input.add8(71n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(71n);
  });

  it('test operator "max" overload (euint128, euint8) => euint128 test 4 (71, 67)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(71n);
    input.add8(67n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(71n);
  });

  it('test operator "add" overload (euint128, euint16) => euint128 test 1 (32769, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(32769n);
    input.add16(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(32771n);
  });

  it('test operator "add" overload (euint128, euint16) => euint128 test 2 (19383, 19385)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(19383n);
    input.add16(19385n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(38768n);
  });

  it('test operator "add" overload (euint128, euint16) => euint128 test 3 (19385, 19385)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(19385n);
    input.add16(19385n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(38770n);
  });

  it('test operator "add" overload (euint128, euint16) => euint128 test 4 (19385, 19383)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(19385n);
    input.add16(19383n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(38768n);
  });

  it('test operator "sub" overload (euint128, euint16) => euint128 test 1 (51991, 51991)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(51991n);
    input.add16(51991n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint128, euint16) => euint128 test 2 (51991, 51987)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(51991n);
    input.add16(51987n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint128, euint16) => euint128 test 1 (16385, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(16385n);
    input.add16(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(32770n);
  });

  it('test operator "mul" overload (euint128, euint16) => euint128 test 2 (255, 255)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(255n);
    input.add16(255n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(65025n);
  });

  it('test operator "mul" overload (euint128, euint16) => euint128 test 3 (255, 255)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(255n);
    input.add16(255n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(65025n);
  });

  it('test operator "mul" overload (euint128, euint16) => euint128 test 4 (255, 255)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(255n);
    input.add16(255n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(65025n);
  });

  it('test operator "and" overload (euint128, euint16) => euint128 test 1 (340282366920938463463370544695774145829, 60335)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463370544695774145829n);
    input.add16(60335n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(59685n);
  });

  it('test operator "and" overload (euint128, euint16) => euint128 test 2 (60331, 60335)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(60331n);
    input.add16(60335n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(60331n);
  });

  it('test operator "and" overload (euint128, euint16) => euint128 test 3 (60335, 60335)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(60335n);
    input.add16(60335n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(60335n);
  });

  it('test operator "and" overload (euint128, euint16) => euint128 test 4 (60335, 60331)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(60335n);
    input.add16(60331n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(60331n);
  });

  it('test operator "or" overload (euint128, euint16) => euint128 test 1 (340282366920938463463370541502015291575, 45676)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463370541502015291575n);
    input.add16(45676n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(340282366920938463463370541502015337215n);
  });

  it('test operator "or" overload (euint128, euint16) => euint128 test 2 (45672, 45676)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(45672n);
    input.add16(45676n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(45676n);
  });

  it('test operator "or" overload (euint128, euint16) => euint128 test 3 (45676, 45676)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(45676n);
    input.add16(45676n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(45676n);
  });

  it('test operator "or" overload (euint128, euint16) => euint128 test 4 (45676, 45672)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(45676n);
    input.add16(45672n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(45676n);
  });

  it('test operator "xor" overload (euint128, euint16) => euint128 test 1 (340282366920938463463367500614507429201, 43805)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463367500614507429201n);
    input.add16(43805n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(340282366920938463463367500614507386444n);
  });

  it('test operator "xor" overload (euint128, euint16) => euint128 test 2 (43801, 43805)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(43801n);
    input.add16(43805n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint128, euint16) => euint128 test 3 (43805, 43805)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(43805n);
    input.add16(43805n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint128, euint16) => euint128 test 4 (43805, 43801)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(43805n);
    input.add16(43801n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint128, euint16) => ebool test 1 (340282366920938463463368403233692285691, 18974)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368403233692285691n);
    input.add16(18974n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint16) => ebool test 2 (18970, 18974)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(18970n);
    input.add16(18974n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint16) => ebool test 3 (18974, 18974)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(18974n);
    input.add16(18974n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint128, euint16) => ebool test 4 (18974, 18970)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(18974n);
    input.add16(18970n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint16) => ebool test 1 (340282366920938463463366982349118948841, 4600)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463366982349118948841n);
    input.add16(4600n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint16) => ebool test 2 (4596, 4600)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(4596n);
    input.add16(4600n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint16) => ebool test 3 (4600, 4600)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(4600n);
    input.add16(4600n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint16) => ebool test 4 (4600, 4596)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(4600n);
    input.add16(4596n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint16) => ebool test 1 (340282366920938463463365820015211535727, 26151)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463365820015211535727n);
    input.add16(26151n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint16) => ebool test 2 (26147, 26151)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(26147n);
    input.add16(26151n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint128, euint16) => ebool test 3 (26151, 26151)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(26151n);
    input.add16(26151n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint16) => ebool test 4 (26151, 26147)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(26151n);
    input.add16(26147n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint16) => ebool test 1 (340282366920938463463372971631158768815, 35817)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463372971631158768815n);
    input.add16(35817n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint16) => ebool test 2 (35813, 35817)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(35813n);
    input.add16(35817n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint16) => ebool test 3 (35817, 35817)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(35817n);
    input.add16(35817n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint16) => ebool test 4 (35817, 35813)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(35817n);
    input.add16(35813n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint16) => ebool test 1 (340282366920938463463370840244451276245, 25030)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463370840244451276245n);
    input.add16(25030n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint128, euint16) => ebool test 2 (25026, 25030)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(25026n);
    input.add16(25030n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint16) => ebool test 3 (25030, 25030)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(25030n);
    input.add16(25030n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint16) => ebool test 4 (25030, 25026)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(25030n);
    input.add16(25026n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint16) => ebool test 1 (340282366920938463463371382545981472713, 60562)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463371382545981472713n);
    input.add16(60562n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint16) => ebool test 2 (60558, 60562)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(60558n);
    input.add16(60562n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint128, euint16) => ebool test 3 (60562, 60562)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(60562n);
    input.add16(60562n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint16) => ebool test 4 (60562, 60558)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(60562n);
    input.add16(60558n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint128, euint16) => euint128 test 1 (340282366920938463463365863998782426575, 15013)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463365863998782426575n);
    input.add16(15013n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(15013n);
  });

  it('test operator "min" overload (euint128, euint16) => euint128 test 2 (15009, 15013)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(15009n);
    input.add16(15013n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(15009n);
  });

  it('test operator "min" overload (euint128, euint16) => euint128 test 3 (15013, 15013)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(15013n);
    input.add16(15013n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(15013n);
  });

  it('test operator "min" overload (euint128, euint16) => euint128 test 4 (15013, 15009)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(15013n);
    input.add16(15009n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(15009n);
  });

  it('test operator "max" overload (euint128, euint16) => euint128 test 1 (340282366920938463463369022435313353547, 55283)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463369022435313353547n);
    input.add16(55283n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(340282366920938463463369022435313353547n);
  });

  it('test operator "max" overload (euint128, euint16) => euint128 test 2 (55279, 55283)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(55279n);
    input.add16(55283n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(55283n);
  });

  it('test operator "max" overload (euint128, euint16) => euint128 test 3 (55283, 55283)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(55283n);
    input.add16(55283n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(55283n);
  });

  it('test operator "max" overload (euint128, euint16) => euint128 test 4 (55283, 55279)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(55283n);
    input.add16(55279n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(55283n);
  });

  it('test operator "add" overload (euint128, euint32) => euint128 test 1 (2147483649, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(2147483649n);
    input.add32(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(2147483651n);
  });

  it('test operator "add" overload (euint128, euint32) => euint128 test 2 (1130643039, 1130643043)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(1130643039n);
    input.add32(1130643043n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(2261286082n);
  });

  it('test operator "add" overload (euint128, euint32) => euint128 test 3 (1130643043, 1130643043)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(1130643043n);
    input.add32(1130643043n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(2261286086n);
  });

  it('test operator "add" overload (euint128, euint32) => euint128 test 4 (1130643043, 1130643039)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(1130643043n);
    input.add32(1130643039n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(2261286082n);
  });

  it('test operator "sub" overload (euint128, euint32) => euint128 test 1 (847394391, 847394391)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(847394391n);
    input.add32(847394391n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint128, euint32) => euint128 test 2 (847394391, 847394387)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(847394391n);
    input.add32(847394387n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint128, euint32) => euint128 test 1 (1073741825, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(1073741825n);
    input.add32(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(2147483650n);
  });

  it('test operator "mul" overload (euint128, euint32) => euint128 test 2 (50767, 50767)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(50767n);
    input.add32(50767n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(2577288289n);
  });

  it('test operator "mul" overload (euint128, euint32) => euint128 test 3 (50767, 50767)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(50767n);
    input.add32(50767n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(2577288289n);
  });

  it('test operator "mul" overload (euint128, euint32) => euint128 test 4 (50767, 50767)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(50767n);
    input.add32(50767n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(2577288289n);
  });

  it('test operator "and" overload (euint128, euint32) => euint128 test 1 (340282366920938463463368612677690430455, 155896687)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368612677690430455n);
    input.add32(155896687n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(134775655n);
  });

  it('test operator "and" overload (euint128, euint32) => euint128 test 2 (155896683, 155896687)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(155896683n);
    input.add32(155896687n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(155896683n);
  });

  it('test operator "and" overload (euint128, euint32) => euint128 test 3 (155896687, 155896687)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(155896687n);
    input.add32(155896687n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(155896687n);
  });

  it('test operator "and" overload (euint128, euint32) => euint128 test 4 (155896687, 155896683)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(155896687n);
    input.add32(155896683n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(155896683n);
  });

  it('test operator "or" overload (euint128, euint32) => euint128 test 1 (340282366920938463463373618516479316915, 3693968155)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463373618516479316915n);
    input.add32(3693968155n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(340282366920938463463373618519834852283n);
  });

  it('test operator "or" overload (euint128, euint32) => euint128 test 2 (3693968151, 3693968155)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(3693968151n);
    input.add32(3693968155n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(3693968159n);
  });

  it('test operator "or" overload (euint128, euint32) => euint128 test 3 (3693968155, 3693968155)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(3693968155n);
    input.add32(3693968155n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(3693968155n);
  });

  it('test operator "or" overload (euint128, euint32) => euint128 test 4 (3693968155, 3693968151)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(3693968155n);
    input.add32(3693968151n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(3693968159n);
  });

  it('test operator "xor" overload (euint128, euint32) => euint128 test 1 (340282366920938463463367646128001436567, 2260960816)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463367646128001436567n);
    input.add32(2260960816n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(340282366920938463463367646130236902823n);
  });

  it('test operator "xor" overload (euint128, euint32) => euint128 test 2 (2260960812, 2260960816)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(2260960812n);
    input.add32(2260960816n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(28n);
  });

  it('test operator "xor" overload (euint128, euint32) => euint128 test 3 (2260960816, 2260960816)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(2260960816n);
    input.add32(2260960816n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint128, euint32) => euint128 test 4 (2260960816, 2260960812)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(2260960816n);
    input.add32(2260960812n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(28n);
  });

  it('test operator "eq" overload (euint128, euint32) => ebool test 1 (340282366920938463463371782842944307973, 4292974644)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463371782842944307973n);
    input.add32(4292974644n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint32) => ebool test 2 (4292974640, 4292974644)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(4292974640n);
    input.add32(4292974644n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint32) => ebool test 3 (4292974644, 4292974644)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(4292974644n);
    input.add32(4292974644n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint128, euint32) => ebool test 4 (4292974644, 4292974640)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(4292974644n);
    input.add32(4292974640n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint32) => ebool test 1 (340282366920938463463367376505379286009, 3415207888)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463367376505379286009n);
    input.add32(3415207888n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint32) => ebool test 2 (3415207884, 3415207888)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(3415207884n);
    input.add32(3415207888n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint32) => ebool test 3 (3415207888, 3415207888)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(3415207888n);
    input.add32(3415207888n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint32) => ebool test 4 (3415207888, 3415207884)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(3415207888n);
    input.add32(3415207884n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint32) => ebool test 1 (340282366920938463463369756594865599661, 2001593551)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463369756594865599661n);
    input.add32(2001593551n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint32) => ebool test 2 (2001593547, 2001593551)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(2001593547n);
    input.add32(2001593551n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint128, euint32) => ebool test 3 (2001593551, 2001593551)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(2001593551n);
    input.add32(2001593551n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint32) => ebool test 4 (2001593551, 2001593547)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(2001593551n);
    input.add32(2001593547n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint32) => ebool test 1 (340282366920938463463368088305041666389, 3748575780)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463368088305041666389n);
    input.add32(3748575780n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint32) => ebool test 2 (3748575776, 3748575780)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(3748575776n);
    input.add32(3748575780n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint32) => ebool test 3 (3748575780, 3748575780)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(3748575780n);
    input.add32(3748575780n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint32) => ebool test 4 (3748575780, 3748575776)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(3748575780n);
    input.add32(3748575776n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint32) => ebool test 1 (340282366920938463463374500428068138387, 105148215)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463374500428068138387n);
    input.add32(105148215n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint128, euint32) => ebool test 2 (105148211, 105148215)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(105148211n);
    input.add32(105148215n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint32) => ebool test 3 (105148215, 105148215)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(105148215n);
    input.add32(105148215n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint32) => ebool test 4 (105148215, 105148211)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(105148215n);
    input.add32(105148211n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint32) => ebool test 1 (340282366920938463463371153619678451819, 35707623)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(340282366920938463463371153619678451819n);
    input.add32(35707623n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint32) => ebool test 2 (35707619, 35707623)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(35707619n);
    input.add32(35707623n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint128, euint32) => ebool test 3 (35707623, 35707623)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(35707623n);
    input.add32(35707623n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint32) => ebool test 4 (35707623, 35707619)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add128(35707623n);
    input.add32(35707619n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });
});
