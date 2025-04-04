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

  it('test operator "ge" overload (euint128, uint128) => ebool test 1 (340282366920938463463374062692812447739, 340282366920938463463368072997589206223)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463374062692812447739n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368072997589206223n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, uint128) => ebool test 2 (340282366920938463463368733357817288295, 340282366920938463463368733357817288299)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368733357817288295n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368733357817288299n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint128, uint128) => ebool test 3 (340282366920938463463368733357817288299, 340282366920938463463368733357817288299)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368733357817288299n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368733357817288299n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, uint128) => ebool test 4 (340282366920938463463368733357817288299, 340282366920938463463368733357817288295)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368733357817288299n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368733357817288295n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint128, euint128) => ebool test 1 (340282366920938463463373084475875340379, 340282366920938463463368072997589206223)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463368072997589206223n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_uint128_euint128(
      340282366920938463463373084475875340379n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint128, euint128) => ebool test 2 (340282366920938463463368733357817288295, 340282366920938463463368733357817288299)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463368733357817288299n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_uint128_euint128(
      340282366920938463463368733357817288295n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint128, euint128) => ebool test 3 (340282366920938463463368733357817288299, 340282366920938463463368733357817288299)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463368733357817288299n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_uint128_euint128(
      340282366920938463463368733357817288299n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint128, euint128) => ebool test 4 (340282366920938463463368733357817288299, 340282366920938463463368733357817288295)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463368733357817288295n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_uint128_euint128(
      340282366920938463463368733357817288299n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, uint128) => ebool test 1 (340282366920938463463368915607430090497, 340282366920938463463367863843086139561)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368915607430090497n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367863843086139561n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, uint128) => ebool test 2 (340282366920938463463368915607430090493, 340282366920938463463368915607430090497)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368915607430090493n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368915607430090497n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, uint128) => ebool test 3 (340282366920938463463368915607430090497, 340282366920938463463368915607430090497)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368915607430090497n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368915607430090497n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, uint128) => ebool test 4 (340282366920938463463368915607430090497, 340282366920938463463368915607430090493)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368915607430090497n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368915607430090493n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint128, euint128) => ebool test 1 (340282366920938463463367740945961536967, 340282366920938463463367863843086139561)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463367863843086139561n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_uint128_euint128(
      340282366920938463463367740945961536967n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint128, euint128) => ebool test 2 (340282366920938463463368915607430090493, 340282366920938463463368915607430090497)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463368915607430090497n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_uint128_euint128(
      340282366920938463463368915607430090493n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint128, euint128) => ebool test 3 (340282366920938463463368915607430090497, 340282366920938463463368915607430090497)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463368915607430090497n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_uint128_euint128(
      340282366920938463463368915607430090497n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint128, euint128) => ebool test 4 (340282366920938463463368915607430090497, 340282366920938463463368915607430090493)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463368915607430090493n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_uint128_euint128(
      340282366920938463463368915607430090497n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, uint128) => ebool test 1 (340282366920938463463373001702715130267, 340282366920938463463369515662004060953)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463373001702715130267n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369515662004060953n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint128, uint128) => ebool test 2 (340282366920938463463369072062439799753, 340282366920938463463369072062439799757)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463369072062439799753n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369072062439799757n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, uint128) => ebool test 3 (340282366920938463463369072062439799757, 340282366920938463463369072062439799757)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463369072062439799757n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369072062439799757n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, uint128) => ebool test 4 (340282366920938463463369072062439799757, 340282366920938463463369072062439799753)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463369072062439799757n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463369072062439799753n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint128, euint128) => ebool test 1 (340282366920938463463367546182583357199, 340282366920938463463369515662004060953)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463369515662004060953n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_uint128_euint128(
      340282366920938463463367546182583357199n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint128, euint128) => ebool test 2 (340282366920938463463369072062439799753, 340282366920938463463369072062439799757)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463369072062439799757n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_uint128_euint128(
      340282366920938463463369072062439799753n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint128, euint128) => ebool test 3 (340282366920938463463369072062439799757, 340282366920938463463369072062439799757)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463369072062439799757n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_uint128_euint128(
      340282366920938463463369072062439799757n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint128, euint128) => ebool test 4 (340282366920938463463369072062439799757, 340282366920938463463369072062439799753)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463369072062439799753n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_uint128_euint128(
      340282366920938463463369072062439799757n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, uint128) => ebool test 1 (340282366920938463463366980508623565339, 340282366920938463463366333361670400843)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366980508623565339n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366333361670400843n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, uint128) => ebool test 2 (340282366920938463463366980508623565335, 340282366920938463463366980508623565339)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366980508623565335n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366980508623565339n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint128, uint128) => ebool test 3 (340282366920938463463366980508623565339, 340282366920938463463366980508623565339)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366980508623565339n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366980508623565339n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, uint128) => ebool test 4 (340282366920938463463366980508623565339, 340282366920938463463366980508623565335)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366980508623565339n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366980508623565335n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint128, euint128) => ebool test 1 (340282366920938463463370680716095307935, 340282366920938463463366333361670400843)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463366333361670400843n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_uint128_euint128(
      340282366920938463463370680716095307935n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint128, euint128) => ebool test 2 (340282366920938463463366980508623565335, 340282366920938463463366980508623565339)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463366980508623565339n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_uint128_euint128(
      340282366920938463463366980508623565335n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint128, euint128) => ebool test 3 (340282366920938463463366980508623565339, 340282366920938463463366980508623565339)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463366980508623565339n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_uint128_euint128(
      340282366920938463463366980508623565339n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint128, euint128) => ebool test 4 (340282366920938463463366980508623565339, 340282366920938463463366980508623565335)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463366980508623565335n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_uint128_euint128(
      340282366920938463463366980508623565339n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint128, uint128) => euint128 test 1 (340282366920938463463368061225670626939, 340282366920938463463372525687250643717)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368061225670626939n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463372525687250643717n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463368061225670626939n);
  });

  it('test operator "min" overload (euint128, uint128) => euint128 test 2 (340282366920938463463368061225670626935, 340282366920938463463368061225670626939)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368061225670626935n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368061225670626939n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463368061225670626935n);
  });

  it('test operator "min" overload (euint128, uint128) => euint128 test 3 (340282366920938463463368061225670626939, 340282366920938463463368061225670626939)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368061225670626939n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368061225670626939n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463368061225670626939n);
  });

  it('test operator "min" overload (euint128, uint128) => euint128 test 4 (340282366920938463463368061225670626939, 340282366920938463463368061225670626935)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463368061225670626939n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463368061225670626935n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463368061225670626935n);
  });

  it('test operator "min" overload (uint128, euint128) => euint128 test 1 (340282366920938463463368210308138224953, 340282366920938463463372525687250643717)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463372525687250643717n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_uint128_euint128(
      340282366920938463463368210308138224953n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463368210308138224953n);
  });

  it('test operator "min" overload (uint128, euint128) => euint128 test 2 (340282366920938463463368061225670626935, 340282366920938463463368061225670626939)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463368061225670626939n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_uint128_euint128(
      340282366920938463463368061225670626935n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463368061225670626935n);
  });

  it('test operator "min" overload (uint128, euint128) => euint128 test 3 (340282366920938463463368061225670626939, 340282366920938463463368061225670626939)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463368061225670626939n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_uint128_euint128(
      340282366920938463463368061225670626939n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463368061225670626939n);
  });

  it('test operator "min" overload (uint128, euint128) => euint128 test 4 (340282366920938463463368061225670626939, 340282366920938463463368061225670626935)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463368061225670626935n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_uint128_euint128(
      340282366920938463463368061225670626939n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463368061225670626935n);
  });

  it('test operator "max" overload (euint128, uint128) => euint128 test 1 (340282366920938463463374443859252212225, 340282366920938463463373677123645667041)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463374443859252212225n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463373677123645667041n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463374443859252212225n);
  });

  it('test operator "max" overload (euint128, uint128) => euint128 test 2 (340282366920938463463366360738217278175, 340282366920938463463366360738217278179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366360738217278175n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366360738217278179n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463366360738217278179n);
  });

  it('test operator "max" overload (euint128, uint128) => euint128 test 3 (340282366920938463463366360738217278179, 340282366920938463463366360738217278179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366360738217278179n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366360738217278179n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463366360738217278179n);
  });

  it('test operator "max" overload (euint128, uint128) => euint128 test 4 (340282366920938463463366360738217278179, 340282366920938463463366360738217278175)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add128(340282366920938463463366360738217278179n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463366360738217278175n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463366360738217278179n);
  });

  it('test operator "max" overload (uint128, euint128) => euint128 test 1 (340282366920938463463367409485085639033, 340282366920938463463373677123645667041)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463373677123645667041n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_uint128_euint128(
      340282366920938463463367409485085639033n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463373677123645667041n);
  });

  it('test operator "max" overload (uint128, euint128) => euint128 test 2 (340282366920938463463366360738217278175, 340282366920938463463366360738217278179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463366360738217278179n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_uint128_euint128(
      340282366920938463463366360738217278175n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463366360738217278179n);
  });

  it('test operator "max" overload (uint128, euint128) => euint128 test 3 (340282366920938463463366360738217278179, 340282366920938463463366360738217278179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463366360738217278179n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_uint128_euint128(
      340282366920938463463366360738217278179n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463366360738217278179n);
  });

  it('test operator "max" overload (uint128, euint128) => euint128 test 4 (340282366920938463463366360738217278179, 340282366920938463463366360738217278175)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);

    input.add128(340282366920938463463366360738217278175n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_uint128_euint128(
      340282366920938463463366360738217278179n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract7.res128());
    expect(res).to.equal(340282366920938463463366360738217278179n);
  });

  it('test operator "add" overload (euint256, euint8) => euint256 test 1 (129, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(129n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(131n);
  });

  it('test operator "add" overload (euint256, euint8) => euint256 test 2 (74, 76)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(74n);
    input.add8(76n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(150n);
  });

  it('test operator "add" overload (euint256, euint8) => euint256 test 3 (76, 76)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(76n);
    input.add8(76n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(152n);
  });

  it('test operator "add" overload (euint256, euint8) => euint256 test 4 (76, 74)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(76n);
    input.add8(74n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(150n);
  });

  it('test operator "sub" overload (euint256, euint8) => euint256 test 1 (191, 191)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(191n);
    input.add8(191n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.sub_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint256, euint8) => euint256 test 2 (191, 187)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(191n);
    input.add8(187n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.sub_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint256, euint8) => euint256 test 1 (65, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(65n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(130n);
  });

  it('test operator "mul" overload (euint256, euint8) => euint256 test 2 (9, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(9n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(90n);
  });

  it('test operator "mul" overload (euint256, euint8) => euint256 test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(10n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(100n);
  });

  it('test operator "mul" overload (euint256, euint8) => euint256 test 4 (10, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(10n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(90n);
  });

  it('test operator "and" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457580234786554247531, 247)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580234786554247531n);
    input.add8(247n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(99n);
  });

  it('test operator "and" overload (euint256, euint8) => euint256 test 2 (243, 247)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(243n);
    input.add8(247n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(243n);
  });

  it('test operator "and" overload (euint256, euint8) => euint256 test 3 (247, 247)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(247n);
    input.add8(247n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(247n);
  });

  it('test operator "and" overload (euint256, euint8) => euint256 test 4 (247, 243)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(247n);
    input.add8(243n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(243n);
  });

  it('test operator "or" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457582264424867230739, 58)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582264424867230739n);
    input.add8(58n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457582264424867230779n);
  });

  it('test operator "or" overload (euint256, euint8) => euint256 test 2 (54, 58)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(54n);
    input.add8(58n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(62n);
  });

  it('test operator "or" overload (euint256, euint8) => euint256 test 3 (58, 58)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(58n);
    input.add8(58n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(58n);
  });

  it('test operator "or" overload (euint256, euint8) => euint256 test 4 (58, 54)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(58n);
    input.add8(54n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(62n);
  });

  it('test operator "xor" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457580128908241920303, 228)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580128908241920303n);
    input.add8(228n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457580128908241920459n);
  });

  it('test operator "xor" overload (euint256, euint8) => euint256 test 2 (224, 228)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(224n);
    input.add8(228n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint256, euint8) => euint256 test 3 (228, 228)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(228n);
    input.add8(228n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint256, euint8) => euint256 test 4 (228, 224)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(228n);
    input.add8(224n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint256, euint8) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457576606400121377767, 27)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576606400121377767n);
    input.add8(27n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint8) => ebool test 2 (23, 27)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(23n);
    input.add8(27n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint8) => ebool test 3 (27, 27)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(27n);
    input.add8(27n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint256, euint8) => ebool test 4 (27, 23)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(27n);
    input.add8(23n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint8) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457576131397466862727, 35)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576131397466862727n);
    input.add8(35n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint8) => ebool test 2 (31, 35)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(31n);
    input.add8(35n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint8) => ebool test 3 (35, 35)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(35n);
    input.add8(35n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint8) => ebool test 4 (35, 31)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(35n);
    input.add8(31n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint256, euint8) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457579158881510425159, 108)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579158881510425159n);
    input.add8(108n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint256, euint8) => ebool test 2 (104, 108)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(104n);
    input.add8(108n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint256, euint8) => ebool test 3 (108, 108)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(108n);
    input.add8(108n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint256, euint8) => ebool test 4 (108, 104)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(108n);
    input.add8(104n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint256, euint8) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457575983015400199233, 83)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575983015400199233n);
    input.add8(83n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint256, euint8) => ebool test 2 (79, 83)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(79n);
    input.add8(83n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint256, euint8) => ebool test 3 (83, 83)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(83n);
    input.add8(83n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint256, euint8) => ebool test 4 (83, 79)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(83n);
    input.add8(79n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, euint8) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457578800906159906655, 157)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578800906159906655n);
    input.add8(157n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint256, euint8) => ebool test 2 (153, 157)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(153n);
    input.add8(157n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, euint8) => ebool test 3 (157, 157)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(157n);
    input.add8(157n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, euint8) => ebool test 4 (157, 153)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(157n);
    input.add8(153n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, euint8) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457577510676868999329, 89)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577510676868999329n);
    input.add8(89n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, euint8) => ebool test 2 (85, 89)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(85n);
    input.add8(89n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint256, euint8) => ebool test 3 (89, 89)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(89n);
    input.add8(89n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, euint8) => ebool test 4 (89, 85)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(89n);
    input.add8(85n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457583615939684667571, 15)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457583615939684667571n);
    input.add8(15n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(15n);
  });

  it('test operator "min" overload (euint256, euint8) => euint256 test 2 (11, 15)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(11n);
    input.add8(15n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(11n);
  });

  it('test operator "min" overload (euint256, euint8) => euint256 test 3 (15, 15)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(15n);
    input.add8(15n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(15n);
  });

  it('test operator "min" overload (euint256, euint8) => euint256 test 4 (15, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(15n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(11n);
  });

  it('test operator "max" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457576505801778430433, 137)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576505801778430433n);
    input.add8(137n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576505801778430433n);
  });

  it('test operator "max" overload (euint256, euint8) => euint256 test 2 (133, 137)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(133n);
    input.add8(137n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(137n);
  });

  it('test operator "max" overload (euint256, euint8) => euint256 test 3 (137, 137)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(137n);
    input.add8(137n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(137n);
  });

  it('test operator "max" overload (euint256, euint8) => euint256 test 4 (137, 133)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(137n);
    input.add8(133n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(137n);
  });

  it('test operator "add" overload (euint256, euint16) => euint256 test 1 (32769, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(32769n);
    input.add16(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(32771n);
  });

  it('test operator "add" overload (euint256, euint16) => euint256 test 2 (28970, 28974)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(28970n);
    input.add16(28974n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(57944n);
  });

  it('test operator "add" overload (euint256, euint16) => euint256 test 3 (28974, 28974)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(28974n);
    input.add16(28974n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(57948n);
  });

  it('test operator "add" overload (euint256, euint16) => euint256 test 4 (28974, 28970)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(28974n);
    input.add16(28970n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(57944n);
  });

  it('test operator "sub" overload (euint256, euint16) => euint256 test 1 (55921, 55921)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(55921n);
    input.add16(55921n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.sub_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint256, euint16) => euint256 test 2 (55921, 55917)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(55921n);
    input.add16(55917n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.sub_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint256, euint16) => euint256 test 1 (16385, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(16385n);
    input.add16(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(32770n);
  });

  it('test operator "mul" overload (euint256, euint16) => euint256 test 2 (186, 186)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(186n);
    input.add16(186n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(34596n);
  });

  it('test operator "mul" overload (euint256, euint16) => euint256 test 3 (186, 186)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(186n);
    input.add16(186n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(34596n);
  });

  it('test operator "mul" overload (euint256, euint16) => euint256 test 4 (186, 186)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(186n);
    input.add16(186n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(34596n);
  });

  it('test operator "and" overload (euint256, euint16) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457578295533875260935, 42161)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578295533875260935n);
    input.add16(42161n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(9217n);
  });

  it('test operator "and" overload (euint256, euint16) => euint256 test 2 (42157, 42161)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(42157n);
    input.add16(42161n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(42145n);
  });

  it('test operator "and" overload (euint256, euint16) => euint256 test 3 (42161, 42161)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(42161n);
    input.add16(42161n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(42161n);
  });

  it('test operator "and" overload (euint256, euint16) => euint256 test 4 (42161, 42157)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(42161n);
    input.add16(42157n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(42145n);
  });

  it('test operator "or" overload (euint256, euint16) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457582763476603623741, 8686)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582763476603623741n);
    input.add16(8686n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457582763476603632127n);
  });

  it('test operator "or" overload (euint256, euint16) => euint256 test 2 (8682, 8686)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(8682n);
    input.add16(8686n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(8686n);
  });

  it('test operator "or" overload (euint256, euint16) => euint256 test 3 (8686, 8686)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(8686n);
    input.add16(8686n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(8686n);
  });

  it('test operator "or" overload (euint256, euint16) => euint256 test 4 (8686, 8682)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(8686n);
    input.add16(8682n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(8686n);
  });

  it('test operator "xor" overload (euint256, euint16) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457581042218742388331, 7753)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581042218742388331n);
    input.add16(7753n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457581042218742384674n);
  });

  it('test operator "xor" overload (euint256, euint16) => euint256 test 2 (7749, 7753)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(7749n);
    input.add16(7753n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint256, euint16) => euint256 test 3 (7753, 7753)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(7753n);
    input.add16(7753n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint256, euint16) => euint256 test 4 (7753, 7749)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(7753n);
    input.add16(7749n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint256, euint16) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457579475694290786671, 9045)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579475694290786671n);
    input.add16(9045n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint16) => ebool test 2 (9041, 9045)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(9041n);
    input.add16(9045n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint16) => ebool test 3 (9045, 9045)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(9045n);
    input.add16(9045n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint256, euint16) => ebool test 4 (9045, 9041)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(9045n);
    input.add16(9041n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint16) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457583562606565207771, 42296)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457583562606565207771n);
    input.add16(42296n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint16) => ebool test 2 (42292, 42296)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(42292n);
    input.add16(42296n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint16) => ebool test 3 (42296, 42296)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(42296n);
    input.add16(42296n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint16) => ebool test 4 (42296, 42292)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(42296n);
    input.add16(42292n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint256, euint16) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457577579809876720721, 57911)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577579809876720721n);
    input.add16(57911n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint256, euint16) => ebool test 2 (57907, 57911)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(57907n);
    input.add16(57911n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint256, euint16) => ebool test 3 (57911, 57911)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(57911n);
    input.add16(57911n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint256, euint16) => ebool test 4 (57911, 57907)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(57911n);
    input.add16(57907n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint256, euint16) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457577705704703020503, 5055)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577705704703020503n);
    input.add16(5055n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint256, euint16) => ebool test 2 (5051, 5055)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(5051n);
    input.add16(5055n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint256, euint16) => ebool test 3 (5055, 5055)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(5055n);
    input.add16(5055n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint256, euint16) => ebool test 4 (5055, 5051)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(5055n);
    input.add16(5051n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, euint16) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457579568952158519959, 10587)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579568952158519959n);
    input.add16(10587n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint256, euint16) => ebool test 2 (10583, 10587)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(10583n);
    input.add16(10587n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, euint16) => ebool test 3 (10587, 10587)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(10587n);
    input.add16(10587n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, euint16) => ebool test 4 (10587, 10583)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(10587n);
    input.add16(10583n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, euint16) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457581072310175668915, 55583)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581072310175668915n);
    input.add16(55583n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, euint16) => ebool test 2 (55579, 55583)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(55579n);
    input.add16(55583n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint256, euint16) => ebool test 3 (55583, 55583)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(55583n);
    input.add16(55583n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, euint16) => ebool test 4 (55583, 55579)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(55583n);
    input.add16(55579n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint256, euint16) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457579202529292671527, 20384)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579202529292671527n);
    input.add16(20384n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(20384n);
  });

  it('test operator "min" overload (euint256, euint16) => euint256 test 2 (20380, 20384)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(20380n);
    input.add16(20384n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(20380n);
  });

  it('test operator "min" overload (euint256, euint16) => euint256 test 3 (20384, 20384)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(20384n);
    input.add16(20384n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(20384n);
  });

  it('test operator "min" overload (euint256, euint16) => euint256 test 4 (20384, 20380)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(20384n);
    input.add16(20380n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(20380n);
  });

  it('test operator "max" overload (euint256, euint16) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457581527188452983821, 31214)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581527188452983821n);
    input.add16(31214n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457581527188452983821n);
  });

  it('test operator "max" overload (euint256, euint16) => euint256 test 2 (31210, 31214)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(31210n);
    input.add16(31214n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(31214n);
  });

  it('test operator "max" overload (euint256, euint16) => euint256 test 3 (31214, 31214)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(31214n);
    input.add16(31214n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(31214n);
  });

  it('test operator "max" overload (euint256, euint16) => euint256 test 4 (31214, 31210)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(31214n);
    input.add16(31210n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(31214n);
  });

  it('test operator "add" overload (euint256, euint32) => euint256 test 1 (2147483649, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(2147483649n);
    input.add32(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(2147483651n);
  });

  it('test operator "add" overload (euint256, euint32) => euint256 test 2 (1577684250, 1577684254)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(1577684250n);
    input.add32(1577684254n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(3155368504n);
  });

  it('test operator "add" overload (euint256, euint32) => euint256 test 3 (1577684254, 1577684254)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(1577684254n);
    input.add32(1577684254n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(3155368508n);
  });

  it('test operator "add" overload (euint256, euint32) => euint256 test 4 (1577684254, 1577684250)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(1577684254n);
    input.add32(1577684250n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(3155368504n);
  });

  it('test operator "sub" overload (euint256, euint32) => euint256 test 1 (3280888733, 3280888733)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(3280888733n);
    input.add32(3280888733n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.sub_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint256, euint32) => euint256 test 2 (3280888733, 3280888729)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(3280888733n);
    input.add32(3280888729n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.sub_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint256, euint32) => euint256 test 1 (1073741825, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(1073741825n);
    input.add32(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(2147483650n);
  });

  it('test operator "mul" overload (euint256, euint32) => euint256 test 2 (51434, 51434)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(51434n);
    input.add32(51434n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(2645456356n);
  });

  it('test operator "mul" overload (euint256, euint32) => euint256 test 3 (51434, 51434)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(51434n);
    input.add32(51434n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(2645456356n);
  });

  it('test operator "mul" overload (euint256, euint32) => euint256 test 4 (51434, 51434)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(51434n);
    input.add32(51434n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(2645456356n);
  });

  it('test operator "and" overload (euint256, euint32) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457580135894251844749, 1017551892)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580135894251844749n);
    input.add32(1017551892n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(404788228n);
  });

  it('test operator "and" overload (euint256, euint32) => euint256 test 2 (1017551888, 1017551892)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(1017551888n);
    input.add32(1017551892n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(1017551888n);
  });

  it('test operator "and" overload (euint256, euint32) => euint256 test 3 (1017551892, 1017551892)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(1017551892n);
    input.add32(1017551892n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(1017551892n);
  });

  it('test operator "and" overload (euint256, euint32) => euint256 test 4 (1017551892, 1017551888)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(1017551892n);
    input.add32(1017551888n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(1017551888n);
  });

  it('test operator "or" overload (euint256, euint32) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457580535228462546277, 2803108407)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580535228462546277n);
    input.add32(2803108407n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457580535229118168951n);
  });

  it('test operator "or" overload (euint256, euint32) => euint256 test 2 (2803108403, 2803108407)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(2803108403n);
    input.add32(2803108407n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(2803108407n);
  });

  it('test operator "or" overload (euint256, euint32) => euint256 test 3 (2803108407, 2803108407)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(2803108407n);
    input.add32(2803108407n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(2803108407n);
  });

  it('test operator "or" overload (euint256, euint32) => euint256 test 4 (2803108407, 2803108403)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(2803108407n);
    input.add32(2803108403n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(2803108407n);
  });

  it('test operator "xor" overload (euint256, euint32) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457575115179814773807, 4146332628)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575115179814773807n);
    input.add32(4146332628n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575115182714873851n);
  });

  it('test operator "xor" overload (euint256, euint32) => euint256 test 2 (4146332624, 4146332628)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(4146332624n);
    input.add32(4146332628n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint256, euint32) => euint256 test 3 (4146332628, 4146332628)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(4146332628n);
    input.add32(4146332628n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint256, euint32) => euint256 test 4 (4146332628, 4146332624)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(4146332628n);
    input.add32(4146332624n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint256, euint32) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457575208895610381995, 1151998660)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575208895610381995n);
    input.add32(1151998660n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint32) => ebool test 2 (1151998656, 1151998660)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(1151998656n);
    input.add32(1151998660n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint32) => ebool test 3 (1151998660, 1151998660)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(1151998660n);
    input.add32(1151998660n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint256, euint32) => ebool test 4 (1151998660, 1151998656)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(1151998660n);
    input.add32(1151998656n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint32) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457576836305999960381, 1504121746)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576836305999960381n);
    input.add32(1504121746n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint32) => ebool test 2 (1504121742, 1504121746)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(1504121742n);
    input.add32(1504121746n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint32) => ebool test 3 (1504121746, 1504121746)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(1504121746n);
    input.add32(1504121746n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint32) => ebool test 4 (1504121746, 1504121742)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(1504121746n);
    input.add32(1504121742n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint256, euint32) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457576741611868544875, 1979134286)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576741611868544875n);
    input.add32(1979134286n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint256, euint32) => ebool test 2 (1979134282, 1979134286)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(1979134282n);
    input.add32(1979134286n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint256, euint32) => ebool test 3 (1979134286, 1979134286)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(1979134286n);
    input.add32(1979134286n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint256, euint32) => ebool test 4 (1979134286, 1979134282)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(1979134286n);
    input.add32(1979134282n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint256, euint32) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457580020051908357643, 3579231376)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580020051908357643n);
    input.add32(3579231376n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint256, euint32) => ebool test 2 (3579231372, 3579231376)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(3579231372n);
    input.add32(3579231376n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint256, euint32) => ebool test 3 (3579231376, 3579231376)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(3579231376n);
    input.add32(3579231376n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint256, euint32) => ebool test 4 (3579231376, 3579231372)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(3579231376n);
    input.add32(3579231372n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, euint32) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457582220742230944655, 2415198935)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582220742230944655n);
    input.add32(2415198935n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint256, euint32) => ebool test 2 (2415198931, 2415198935)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(2415198931n);
    input.add32(2415198935n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, euint32) => ebool test 3 (2415198935, 2415198935)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(2415198935n);
    input.add32(2415198935n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, euint32) => ebool test 4 (2415198935, 2415198931)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(2415198935n);
    input.add32(2415198931n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.le_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, euint32) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457581572272405219229, 2562585079)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581572272405219229n);
    input.add32(2562585079n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, euint32) => ebool test 2 (2562585075, 2562585079)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(2562585075n);
    input.add32(2562585079n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint256, euint32) => ebool test 3 (2562585079, 2562585079)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(2562585079n);
    input.add32(2562585079n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, euint32) => ebool test 4 (2562585079, 2562585075)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(2562585079n);
    input.add32(2562585075n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.lt_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint256, euint32) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457577389015782082037, 525636135)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577389015782082037n);
    input.add32(525636135n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(525636135n);
  });

  it('test operator "min" overload (euint256, euint32) => euint256 test 2 (525636131, 525636135)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(525636131n);
    input.add32(525636135n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(525636131n);
  });

  it('test operator "min" overload (euint256, euint32) => euint256 test 3 (525636135, 525636135)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(525636135n);
    input.add32(525636135n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(525636135n);
  });

  it('test operator "min" overload (euint256, euint32) => euint256 test 4 (525636135, 525636131)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(525636135n);
    input.add32(525636131n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(525636131n);
  });

  it('test operator "max" overload (euint256, euint32) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457579248405090823281, 2211971845)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579248405090823281n);
    input.add32(2211971845n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579248405090823281n);
  });

  it('test operator "max" overload (euint256, euint32) => euint256 test 2 (2211971841, 2211971845)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(2211971841n);
    input.add32(2211971845n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(2211971845n);
  });

  it('test operator "max" overload (euint256, euint32) => euint256 test 3 (2211971845, 2211971845)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(2211971845n);
    input.add32(2211971845n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(2211971845n);
  });

  it('test operator "max" overload (euint256, euint32) => euint256 test 4 (2211971845, 2211971841)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(2211971845n);
    input.add32(2211971841n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.max_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(2211971845n);
  });

  it('test operator "add" overload (euint256, euint64) => euint256 test 1 (9223372036854775809, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(9223372036854775809n);
    input.add64(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(9223372036854775811n);
  });

  it('test operator "add" overload (euint256, euint64) => euint256 test 2 (9219406278164547430, 9219406278164547432)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(9219406278164547430n);
    input.add64(9219406278164547432n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(18438812556329094862n);
  });

  it('test operator "add" overload (euint256, euint64) => euint256 test 3 (9219406278164547432, 9219406278164547432)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(9219406278164547432n);
    input.add64(9219406278164547432n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(18438812556329094864n);
  });

  it('test operator "add" overload (euint256, euint64) => euint256 test 4 (9219406278164547432, 9219406278164547430)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(9219406278164547432n);
    input.add64(9219406278164547430n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.add_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(18438812556329094862n);
  });

  it('test operator "sub" overload (euint256, euint64) => euint256 test 1 (18439562371434642917, 18439562371434642917)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(18439562371434642917n);
    input.add64(18439562371434642917n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.sub_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint256, euint64) => euint256 test 2 (18439562371434642917, 18439562371434642913)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(18439562371434642917n);
    input.add64(18439562371434642913n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.sub_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint256, euint64) => euint256 test 1 (4611686018427387905, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(4611686018427387905n);
    input.add64(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(9223372036854775810n);
  });

  it('test operator "mul" overload (euint256, euint64) => euint256 test 2 (4293794135, 4293794135)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(4293794135n);
    input.add64(4293794135n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(18436668073760398225n);
  });

  it('test operator "mul" overload (euint256, euint64) => euint256 test 3 (4293794135, 4293794135)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(4293794135n);
    input.add64(4293794135n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(18436668073760398225n);
  });

  it('test operator "mul" overload (euint256, euint64) => euint256 test 4 (4293794135, 4293794135)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(4293794135n);
    input.add64(4293794135n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.mul_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(18436668073760398225n);
  });

  it('test operator "and" overload (euint256, euint64) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457578640934655549941, 18446092218699243979)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578640934655549941n);
    input.add64(18446092218699243979n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(18441305889744380353n);
  });

  it('test operator "and" overload (euint256, euint64) => euint256 test 2 (18446092218699243975, 18446092218699243979)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(18446092218699243975n);
    input.add64(18446092218699243979n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(18446092218699243971n);
  });

  it('test operator "and" overload (euint256, euint64) => euint256 test 3 (18446092218699243979, 18446092218699243979)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(18446092218699243979n);
    input.add64(18446092218699243979n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(18446092218699243979n);
  });

  it('test operator "and" overload (euint256, euint64) => euint256 test 4 (18446092218699243979, 18446092218699243975)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(18446092218699243979n);
    input.add64(18446092218699243975n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(18446092218699243971n);
  });

  it('test operator "or" overload (euint256, euint64) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457579730350904251439, 18442451967564225067)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579730350904251439n);
    input.add64(18442451967564225067n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579730668745629231n);
  });

  it('test operator "or" overload (euint256, euint64) => euint256 test 2 (18442451967564225063, 18442451967564225067)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(18442451967564225063n);
    input.add64(18442451967564225067n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(18442451967564225071n);
  });

  it('test operator "or" overload (euint256, euint64) => euint256 test 3 (18442451967564225067, 18442451967564225067)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(18442451967564225067n);
    input.add64(18442451967564225067n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(18442451967564225067n);
  });

  it('test operator "or" overload (euint256, euint64) => euint256 test 4 (18442451967564225067, 18442451967564225063)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(18442451967564225067n);
    input.add64(18442451967564225063n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(18442451967564225071n);
  });

  it('test operator "xor" overload (euint256, euint64) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457581185938538361745, 18441437709469751259)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581185938538361745n);
    input.add64(18441437709469751259n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039439144261741751034954n);
  });

  it('test operator "xor" overload (euint256, euint64) => euint256 test 2 (18441437709469751255, 18441437709469751259)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(18441437709469751255n);
    input.add64(18441437709469751259n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint256, euint64) => euint256 test 3 (18441437709469751259, 18441437709469751259)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(18441437709469751259n);
    input.add64(18441437709469751259n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint256, euint64) => euint256 test 4 (18441437709469751259, 18441437709469751255)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(18441437709469751259n);
    input.add64(18441437709469751255n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.xor_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract7.res256());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint256, euint64) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457577178449647855491, 18441163410202368937)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577178449647855491n);
    input.add64(18441163410202368937n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint64) => ebool test 2 (18441163410202368933, 18441163410202368937)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(18441163410202368933n);
    input.add64(18441163410202368937n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint64) => ebool test 3 (18441163410202368937, 18441163410202368937)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(18441163410202368937n);
    input.add64(18441163410202368937n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint256, euint64) => ebool test 4 (18441163410202368937, 18441163410202368933)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(18441163410202368937n);
    input.add64(18441163410202368933n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint64) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457577041801886536391, 18440568181538382129)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577041801886536391n);
    input.add64(18440568181538382129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint64) => ebool test 2 (18440568181538382125, 18440568181538382129)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(18440568181538382125n);
    input.add64(18440568181538382129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint64) => ebool test 3 (18440568181538382129, 18440568181538382129)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(18440568181538382129n);
    input.add64(18440568181538382129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint64) => ebool test 4 (18440568181538382129, 18440568181538382125)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract7Address, this.signers.alice.address);
    input.add256(18440568181538382129n);
    input.add64(18440568181538382125n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ne_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract7.resb());
    expect(res).to.equal(true);
  });
});
