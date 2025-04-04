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

describe('TFHE operations 12', function () {
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

  it('test operator "max" overload (euint256, uint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457577650256858684559, 115792089237316195423570985008687907853269984665640564039457580989807717533443)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577650256858684559n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457580989807717533443n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457580989807717533443n);
  });

  it('test operator "max" overload (euint256, uint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457577650256858684555, 115792089237316195423570985008687907853269984665640564039457577650256858684559)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577650256858684555n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457577650256858684559n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577650256858684559n);
  });

  it('test operator "max" overload (euint256, uint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457577650256858684559, 115792089237316195423570985008687907853269984665640564039457577650256858684559)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577650256858684559n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457577650256858684559n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577650256858684559n);
  });

  it('test operator "max" overload (euint256, uint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457577650256858684559, 115792089237316195423570985008687907853269984665640564039457577650256858684555)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577650256858684559n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457577650256858684555n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577650256858684559n);
  });

  it('test operator "max" overload (uint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457582656121426033755, 115792089237316195423570985008687907853269984665640564039457580989807717533443)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457580989807717533443n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457582656121426033755n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457582656121426033755n);
  });

  it('test operator "max" overload (uint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457577650256858684555, 115792089237316195423570985008687907853269984665640564039457577650256858684559)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457577650256858684559n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457577650256858684555n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577650256858684559n);
  });

  it('test operator "max" overload (uint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457577650256858684559, 115792089237316195423570985008687907853269984665640564039457577650256858684559)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457577650256858684559n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457577650256858684559n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577650256858684559n);
  });

  it('test operator "max" overload (uint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457577650256858684559, 115792089237316195423570985008687907853269984665640564039457577650256858684555)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457577650256858684555n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.max_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457577650256858684559n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract8.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577650256858684559n);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 1 (183, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add8(183n);
    input.add8(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract8.res8());
    expect(res).to.equal(184n);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add8(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract8.res8());
    expect(res).to.equal(4n);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add8(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract8.res8());
    expect(res).to.equal(8n);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add8(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract8.res8());
    expect(res).to.equal(128n);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 1 (183, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add8(183n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shl_euint8_uint8(encryptedAmount.handles[0], 3n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract8.res8());
    expect(res).to.equal(184n);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add8(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shl_euint8_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract8.res8());
    expect(res).to.equal(4n);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add8(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shl_euint8_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract8.res8());
    expect(res).to.equal(8n);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add8(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shl_euint8_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract8.res8());
    expect(res).to.equal(128n);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 1 (9, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add8(9n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract8.res8());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add8(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract8.res8());
    expect(res).to.equal(4n);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add8(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract8.res8());
    expect(res).to.equal(8n);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add8(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract8.res8());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 1 (9, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add8(9n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shr_euint8_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract8.res8());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add8(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shr_euint8_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract8.res8());
    expect(res).to.equal(4n);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add8(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shr_euint8_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract8.res8());
    expect(res).to.equal(8n);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add8(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shr_euint8_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract8.res8());
    expect(res).to.equal(0n);
  });

  it('test operator "rotl" overload (euint8, euint8) => euint8 test 1 (95, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add8(95n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract8.res8());
    expect(res).to.equal(215n);
  });

  it('test operator "rotl" overload (euint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add8(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract8.res8());
    expect(res).to.equal(4n);
  });

  it('test operator "rotl" overload (euint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add8(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract8.res8());
    expect(res).to.equal(8n);
  });

  it('test operator "rotl" overload (euint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add8(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract8.res8());
    expect(res).to.equal(128n);
  });

  it('test operator "rotl" overload (euint8, uint8) => euint8 test 1 (95, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add8(95n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotl_euint8_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract8.res8());
    expect(res).to.equal(215n);
  });

  it('test operator "rotl" overload (euint8, uint8) => euint8 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add8(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotl_euint8_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract8.res8());
    expect(res).to.equal(4n);
  });

  it('test operator "rotl" overload (euint8, uint8) => euint8 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add8(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotl_euint8_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract8.res8());
    expect(res).to.equal(8n);
  });

  it('test operator "rotl" overload (euint8, uint8) => euint8 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add8(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotl_euint8_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract8.res8());
    expect(res).to.equal(128n);
  });

  it('test operator "rotr" overload (euint8, euint8) => euint8 test 1 (222, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add8(222n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract8.res8());
    expect(res).to.equal(123n);
  });

  it('test operator "rotr" overload (euint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add8(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract8.res8());
    expect(res).to.equal(4n);
  });

  it('test operator "rotr" overload (euint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add8(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract8.res8());
    expect(res).to.equal(8n);
  });

  it('test operator "rotr" overload (euint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add8(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract8.res8());
    expect(res).to.equal(128n);
  });

  it('test operator "rotr" overload (euint8, uint8) => euint8 test 1 (222, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add8(222n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotr_euint8_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract8.res8());
    expect(res).to.equal(123n);
  });

  it('test operator "rotr" overload (euint8, uint8) => euint8 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add8(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotr_euint8_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract8.res8());
    expect(res).to.equal(4n);
  });

  it('test operator "rotr" overload (euint8, uint8) => euint8 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add8(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotr_euint8_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract8.res8());
    expect(res).to.equal(8n);
  });

  it('test operator "rotr" overload (euint8, uint8) => euint8 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add8(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotr_euint8_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract8.res8());
    expect(res).to.equal(128n);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 1 (52932, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add16(52932n);
    input.add8(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract8.res16());
    expect(res).to.equal(30240n);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add16(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract8.res16());
    expect(res).to.equal(1024n);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add16(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract8.res16());
    expect(res).to.equal(2048n);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add16(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract8.res16());
    expect(res).to.equal(128n);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 1 (52932, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add16(52932n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shl_euint16_uint8(encryptedAmount.handles[0], 3n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract8.res16());
    expect(res).to.equal(30240n);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add16(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shl_euint16_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract8.res16());
    expect(res).to.equal(1024n);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add16(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shl_euint16_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract8.res16());
    expect(res).to.equal(2048n);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add16(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shl_euint16_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract8.res16());
    expect(res).to.equal(128n);
  });

  it('test operator "shr" overload (euint16, euint8) => euint16 test 1 (55265, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add16(55265n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract8.res16());
    expect(res).to.equal(863n);
  });

  it('test operator "shr" overload (euint16, euint8) => euint16 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add16(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract8.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint16, euint8) => euint16 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add16(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract8.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint16, euint8) => euint16 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add16(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract8.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 1 (55265, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add16(55265n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shr_euint16_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract8.res16());
    expect(res).to.equal(863n);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add16(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shr_euint16_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract8.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add16(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shr_euint16_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract8.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add16(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shr_euint16_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract8.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "rotl" overload (euint16, euint8) => euint16 test 1 (6810, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add16(6810n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract8.res16());
    expect(res).to.equal(27240n);
  });

  it('test operator "rotl" overload (euint16, euint8) => euint16 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add16(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract8.res16());
    expect(res).to.equal(1024n);
  });

  it('test operator "rotl" overload (euint16, euint8) => euint16 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add16(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract8.res16());
    expect(res).to.equal(2048n);
  });

  it('test operator "rotl" overload (euint16, euint8) => euint16 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add16(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract8.res16());
    expect(res).to.equal(128n);
  });

  it('test operator "rotl" overload (euint16, uint8) => euint16 test 1 (6810, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add16(6810n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotl_euint16_uint8(encryptedAmount.handles[0], 2n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract8.res16());
    expect(res).to.equal(27240n);
  });

  it('test operator "rotl" overload (euint16, uint8) => euint16 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add16(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotl_euint16_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract8.res16());
    expect(res).to.equal(1024n);
  });

  it('test operator "rotl" overload (euint16, uint8) => euint16 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add16(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotl_euint16_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract8.res16());
    expect(res).to.equal(2048n);
  });

  it('test operator "rotl" overload (euint16, uint8) => euint16 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add16(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotl_euint16_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract8.res16());
    expect(res).to.equal(128n);
  });

  it('test operator "rotr" overload (euint16, euint8) => euint16 test 1 (63833, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add16(63833n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract8.res16());
    expect(res).to.equal(40853n);
  });

  it('test operator "rotr" overload (euint16, euint8) => euint16 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add16(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract8.res16());
    expect(res).to.equal(1024n);
  });

  it('test operator "rotr" overload (euint16, euint8) => euint16 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add16(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract8.res16());
    expect(res).to.equal(2048n);
  });

  it('test operator "rotr" overload (euint16, euint8) => euint16 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add16(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract8.res16());
    expect(res).to.equal(32768n);
  });

  it('test operator "rotr" overload (euint16, uint8) => euint16 test 1 (63833, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add16(63833n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotr_euint16_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract8.res16());
    expect(res).to.equal(40853n);
  });

  it('test operator "rotr" overload (euint16, uint8) => euint16 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add16(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotr_euint16_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract8.res16());
    expect(res).to.equal(1024n);
  });

  it('test operator "rotr" overload (euint16, uint8) => euint16 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add16(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotr_euint16_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract8.res16());
    expect(res).to.equal(2048n);
  });

  it('test operator "rotr" overload (euint16, uint8) => euint16 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add16(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotr_euint16_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract8.res16());
    expect(res).to.equal(32768n);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 1 (2506241727, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add32(2506241727n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract8.res32());
    expect(res).to.equal(2890323936n);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add32(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract8.res32());
    expect(res).to.equal(1024n);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add32(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract8.res32());
    expect(res).to.equal(2048n);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add32(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract8.res32());
    expect(res).to.equal(128n);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 1 (2506241727, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add32(2506241727n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shl_euint32_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract8.res32());
    expect(res).to.equal(2890323936n);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add32(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shl_euint32_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract8.res32());
    expect(res).to.equal(1024n);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add32(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shl_euint32_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract8.res32());
    expect(res).to.equal(2048n);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add32(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shl_euint32_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract8.res32());
    expect(res).to.equal(128n);
  });

  it('test operator "shr" overload (euint32, euint8) => euint32 test 1 (513565963, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add32(513565963n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract8.res32());
    expect(res).to.equal(128391490n);
  });

  it('test operator "shr" overload (euint32, euint8) => euint32 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add32(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract8.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint32, euint8) => euint32 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add32(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract8.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint32, euint8) => euint32 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add32(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract8.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint32, uint8) => euint32 test 1 (513565963, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add32(513565963n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shr_euint32_uint8(encryptedAmount.handles[0], 2n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract8.res32());
    expect(res).to.equal(128391490n);
  });

  it('test operator "shr" overload (euint32, uint8) => euint32 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add32(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shr_euint32_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract8.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint32, uint8) => euint32 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add32(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shr_euint32_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract8.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint32, uint8) => euint32 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add32(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shr_euint32_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract8.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "rotl" overload (euint32, euint8) => euint32 test 1 (1383877263, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add32(1383877263n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract8.res32());
    expect(res).to.equal(1334399466n);
  });

  it('test operator "rotl" overload (euint32, euint8) => euint32 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add32(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract8.res32());
    expect(res).to.equal(1024n);
  });

  it('test operator "rotl" overload (euint32, euint8) => euint32 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add32(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract8.res32());
    expect(res).to.equal(2048n);
  });

  it('test operator "rotl" overload (euint32, euint8) => euint32 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add32(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract8.res32());
    expect(res).to.equal(128n);
  });

  it('test operator "rotl" overload (euint32, uint8) => euint32 test 1 (1383877263, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add32(1383877263n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotl_euint32_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract8.res32());
    expect(res).to.equal(1334399466n);
  });

  it('test operator "rotl" overload (euint32, uint8) => euint32 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add32(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotl_euint32_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract8.res32());
    expect(res).to.equal(1024n);
  });

  it('test operator "rotl" overload (euint32, uint8) => euint32 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add32(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotl_euint32_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract8.res32());
    expect(res).to.equal(2048n);
  });

  it('test operator "rotl" overload (euint32, uint8) => euint32 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add32(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotl_euint32_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract8.res32());
    expect(res).to.equal(128n);
  });

  it('test operator "rotr" overload (euint32, euint8) => euint32 test 1 (242743044, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add32(242743044n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract8.res32());
    expect(res).to.equal(1088913264n);
  });

  it('test operator "rotr" overload (euint32, euint8) => euint32 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add32(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract8.res32());
    expect(res).to.equal(67108864n);
  });

  it('test operator "rotr" overload (euint32, euint8) => euint32 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add32(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract8.res32());
    expect(res).to.equal(134217728n);
  });

  it('test operator "rotr" overload (euint32, euint8) => euint32 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add32(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract8.res32());
    expect(res).to.equal(2147483648n);
  });

  it('test operator "rotr" overload (euint32, uint8) => euint32 test 1 (242743044, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add32(242743044n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotr_euint32_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract8.res32());
    expect(res).to.equal(1088913264n);
  });

  it('test operator "rotr" overload (euint32, uint8) => euint32 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add32(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotr_euint32_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract8.res32());
    expect(res).to.equal(67108864n);
  });

  it('test operator "rotr" overload (euint32, uint8) => euint32 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add32(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotr_euint32_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract8.res32());
    expect(res).to.equal(134217728n);
  });

  it('test operator "rotr" overload (euint32, uint8) => euint32 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add32(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotr_euint32_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract8.res32());
    expect(res).to.equal(2147483648n);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 1 (18446247156160430851, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(18446247156160430851n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(18414941350565822656n);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(1024n);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(2048n);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(128n);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 1 (18446247156160430851, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(18446247156160430851n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shl_euint64_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(18414941350565822656n);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shl_euint64_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(1024n);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shl_euint64_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(2048n);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shl_euint64_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(128n);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 1 (18445146601156108451, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(18445146601156108451n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(4611286650289027112n);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 1 (18445146601156108451, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(18445146601156108451n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shr_euint64_uint8(encryptedAmount.handles[0], 2n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(4611286650289027112n);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shr_euint64_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shr_euint64_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shr_euint64_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "rotl" overload (euint64, euint8) => euint64 test 1 (18438351242968701585, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(18438351242968701585n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(17909602906295149695n);
  });

  it('test operator "rotl" overload (euint64, euint8) => euint64 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(1024n);
  });

  it('test operator "rotl" overload (euint64, euint8) => euint64 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(2048n);
  });

  it('test operator "rotl" overload (euint64, euint8) => euint64 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(128n);
  });

  it('test operator "rotl" overload (euint64, uint8) => euint64 test 1 (18438351242968701585, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(18438351242968701585n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotl_euint64_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(17909602906295149695n);
  });

  it('test operator "rotl" overload (euint64, uint8) => euint64 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotl_euint64_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(1024n);
  });

  it('test operator "rotl" overload (euint64, uint8) => euint64 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotl_euint64_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(2048n);
  });

  it('test operator "rotl" overload (euint64, uint8) => euint64 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotl_euint64_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(128n);
  });

  it('test operator "rotr" overload (euint64, euint8) => euint64 test 1 (18445564081789762939, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(18445564081789762939n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(13834984305787176919n);
  });

  it('test operator "rotr" overload (euint64, euint8) => euint64 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(288230376151711744n);
  });

  it('test operator "rotr" overload (euint64, euint8) => euint64 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(576460752303423488n);
  });

  it('test operator "rotr" overload (euint64, euint8) => euint64 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(9223372036854775808n);
  });

  it('test operator "rotr" overload (euint64, uint8) => euint64 test 1 (18445564081789762939, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(18445564081789762939n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotr_euint64_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(13834984305787176919n);
  });

  it('test operator "rotr" overload (euint64, uint8) => euint64 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotr_euint64_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(288230376151711744n);
  });

  it('test operator "rotr" overload (euint64, uint8) => euint64 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotr_euint64_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(576460752303423488n);
  });

  it('test operator "rotr" overload (euint64, uint8) => euint64 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add64(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.rotr_euint64_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract8.res64());
    expect(res).to.equal(9223372036854775808n);
  });

  it('test operator "shl" overload (euint128, euint8) => euint128 test 1 (340282366920938463463371483598815178563, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463371483598815178563n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(340282366920938463462974756813780001152n);
  });

  it('test operator "shl" overload (euint128, euint8) => euint128 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(1024n);
  });

  it('test operator "shl" overload (euint128, euint8) => euint128 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(2048n);
  });

  it('test operator "shl" overload (euint128, euint8) => euint128 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(128n);
  });

  it('test operator "shl" overload (euint128, uint8) => euint128 test 1 (340282366920938463463371483598815178563, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463371483598815178563n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shl_euint128_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(340282366920938463462974756813780001152n);
  });

  it('test operator "shl" overload (euint128, uint8) => euint128 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shl_euint128_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(1024n);
  });

  it('test operator "shl" overload (euint128, uint8) => euint128 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shl_euint128_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(2048n);
  });

  it('test operator "shl" overload (euint128, uint8) => euint128 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shl_euint128_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(128n);
  });

  it('test operator "shr" overload (euint128, euint8) => euint128 test 1 (340282366920938463463371314732687630861, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463371314732687630861n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(85070591730234615865842828683171907715n);
  });

  it('test operator "shr" overload (euint128, euint8) => euint128 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint128, euint8) => euint128 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint128, euint8) => euint128 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint128, uint8) => euint128 test 1 (340282366920938463463371314732687630861, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(340282366920938463463371314732687630861n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shr_euint128_uint8(encryptedAmount.handles[0], 2n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(85070591730234615865842828683171907715n);
  });

  it('test operator "shr" overload (euint128, uint8) => euint128 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shr_euint128_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint128, uint8) => euint128 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shr_euint128_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint128, uint8) => euint128 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract8Address, this.signers.alice.address);
    input.add128(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract8.shr_euint128_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract8.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "rotl" overload (euint128, euint8) => euint128 test 1 (340282366920938463463367203689643093663, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463367203689643093663n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.rotl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463137687683764442111n);
  });

  it('test operator "rotl" overload (euint128, euint8) => euint128 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.rotl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(1024n);
  });

  it('test operator "rotl" overload (euint128, euint8) => euint128 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.rotl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(2048n);
  });

  it('test operator "rotl" overload (euint128, euint8) => euint128 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.rotl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(128n);
  });

  it('test operator "rotl" overload (euint128, uint8) => euint128 test 1 (340282366920938463463367203689643093663, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463367203689643093663n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.rotl_euint128_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463137687683764442111n);
  });

  it('test operator "rotl" overload (euint128, uint8) => euint128 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.rotl_euint128_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(1024n);
  });

  it('test operator "rotl" overload (euint128, uint8) => euint128 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.rotl_euint128_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(2048n);
  });

  it('test operator "rotl" overload (euint128, uint8) => euint128 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.rotl_euint128_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(128n);
  });

  it('test operator "rotr" overload (euint128, euint8) => euint128 test 1 (340282366920938463463367079532490566339, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463367079532490566339n);
    input.add8(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.rotr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463370843482129388897n);
  });

  it('test operator "rotr" overload (euint128, euint8) => euint128 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.rotr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(5316911983139663491615228241121378304n);
  });

  it('test operator "rotr" overload (euint128, euint8) => euint128 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.rotr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(10633823966279326983230456482242756608n);
  });

  it('test operator "rotr" overload (euint128, euint8) => euint128 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.rotr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(170141183460469231731687303715884105728n);
  });

  it('test operator "rotr" overload (euint128, uint8) => euint128 test 1 (340282366920938463463367079532490566339, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463367079532490566339n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.rotr_euint128_uint8(encryptedAmount.handles[0], 1n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(340282366920938463463370843482129388897n);
  });

  it('test operator "rotr" overload (euint128, uint8) => euint128 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.rotr_euint128_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(5316911983139663491615228241121378304n);
  });

  it('test operator "rotr" overload (euint128, uint8) => euint128 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.rotr_euint128_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(10633823966279326983230456482242756608n);
  });

  it('test operator "rotr" overload (euint128, uint8) => euint128 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.rotr_euint128_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(170141183460469231731687303715884105728n);
  });

  it('test operator "shl" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457576020806830007291, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576020806830007291n);
    input.add8(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.shl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457568033700530374646n);
  });

  it('test operator "shl" overload (euint256, euint8) => euint256 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.shl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(1024n);
  });

  it('test operator "shl" overload (euint256, euint8) => euint256 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.shl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(2048n);
  });

  it('test operator "shl" overload (euint256, euint8) => euint256 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.shl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(128n);
  });

  it('test operator "shl" overload (euint256, uint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457576020806830007291, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576020806830007291n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.shl_euint256_uint8(encryptedAmount.handles[0], 1n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457568033700530374646n);
  });

  it('test operator "shl" overload (euint256, uint8) => euint256 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.shl_euint256_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(1024n);
  });

  it('test operator "shl" overload (euint256, uint8) => euint256 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.shl_euint256_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(2048n);
  });

  it('test operator "shl" overload (euint256, uint8) => euint256 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.shl_euint256_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(128n);
  });

  it('test operator "shr" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457580007709015698127, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580007709015698127n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.shr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(904625697166532776746648320380374280103671755200316906558262343810226685141n);
  });

  it('test operator "shr" overload (euint256, euint8) => euint256 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.shr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint256, euint8) => euint256 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.shr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint256, euint8) => euint256 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.shr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint256, uint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457580007709015698127, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580007709015698127n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.shr_euint256_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(904625697166532776746648320380374280103671755200316906558262343810226685141n);
  });

  it('test operator "shr" overload (euint256, uint8) => euint256 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.shr_euint256_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint256, uint8) => euint256 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.shr_euint256_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint256, uint8) => euint256 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.shr_euint256_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "rotl" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457576706125599989537, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576706125599989537n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.rotl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039456649379109334388991n);
  });

  it('test operator "rotl" overload (euint256, euint8) => euint256 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.rotl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(1024n);
  });

  it('test operator "rotl" overload (euint256, euint8) => euint256 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.rotl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(2048n);
  });

  it('test operator "rotl" overload (euint256, euint8) => euint256 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.rotl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(128n);
  });

  it('test operator "rotl" overload (euint256, uint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457576706125599989537, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576706125599989537n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.rotl_euint256_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039456649379109334388991n);
  });

  it('test operator "rotl" overload (euint256, uint8) => euint256 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.rotl_euint256_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(1024n);
  });

  it('test operator "rotl" overload (euint256, uint8) => euint256 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.rotl_euint256_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(2048n);
  });

  it('test operator "rotl" overload (euint256, uint8) => euint256 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.rotl_euint256_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(128n);
  });

  it('test operator "rotr" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457580153722339598383, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580153722339598383n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.rotr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457583044365432129547n);
  });

  it('test operator "rotr" overload (euint256, euint8) => euint256 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.rotr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(1809251394333065553493296640760748560207343510400633813116524750123642650624n);
  });

  it('test operator "rotr" overload (euint256, euint8) => euint256 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.rotr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(3618502788666131106986593281521497120414687020801267626233049500247285301248n);
  });

  it('test operator "rotr" overload (euint256, euint8) => euint256 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.rotr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(57896044618658097711785492504343953926634992332820282019728792003956564819968n);
  });

  it('test operator "rotr" overload (euint256, uint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457580153722339598383, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580153722339598383n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.rotr_euint256_uint8(encryptedAmount.handles[0], 2n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457583044365432129547n);
  });

  it('test operator "rotr" overload (euint256, uint8) => euint256 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.rotr_euint256_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(1809251394333065553493296640760748560207343510400633813116524750123642650624n);
  });

  it('test operator "rotr" overload (euint256, uint8) => euint256 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.rotr_euint256_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(3618502788666131106986593281521497120414687020801267626233049500247285301248n);
  });

  it('test operator "rotr" overload (euint256, uint8) => euint256 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.rotr_euint256_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(57896044618658097711785492504343953926634992332820282019728792003956564819968n);
  });

  it('test operator "neg" overload (euint8) => euint8 test 1 (103)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add8(103n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.neg_euint8(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract9.res8());
    expect(res).to.equal(153n);
  });

  it('test operator "not" overload (euint8) => euint8 test 1 (145)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add8(145n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.not_euint8(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract9.res8());
    expect(res).to.equal(110n);
  });

  it('test operator "neg" overload (euint16) => euint16 test 1 (47105)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add16(47105n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.neg_euint16(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract9.res16());
    expect(res).to.equal(18431n);
  });

  it('test operator "not" overload (euint16) => euint16 test 1 (48288)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add16(48288n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.not_euint16(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract9.res16());
    expect(res).to.equal(17247n);
  });

  it('test operator "neg" overload (euint32) => euint32 test 1 (1493412109)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add32(1493412109n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.neg_euint32(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract9.res32());
    expect(res).to.equal(2801555187n);
  });

  it('test operator "not" overload (euint32) => euint32 test 1 (418173546)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add32(418173546n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.not_euint32(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract9.res32());
    expect(res).to.equal(3876793749n);
  });

  it('test operator "neg" overload (euint64) => euint64 test 1 (18443945525974525069)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add64(18443945525974525069n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.neg_euint64(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract9.res64());
    expect(res).to.equal(2798547735026547n);
  });

  it('test operator "not" overload (euint64) => euint64 test 1 (18443908623320570321)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add64(18443908623320570321n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.not_euint64(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract9.res64());
    expect(res).to.equal(2835450388981294n);
  });

  it('test operator "neg" overload (euint128) => euint128 test 1 (340282366920938463463372905669251632341)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463372905669251632341n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.neg_euint128(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(1701762516579115n);
  });

  it('test operator "not" overload (euint128) => euint128 test 1 (340282366920938463463371735027542284905)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add128(340282366920938463463371735027542284905n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.not_euint128(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract9.res128());
    expect(res).to.equal(2872404225926550n);
  });

  it('test operator "neg" overload (euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457580354813562910899)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580354813562910899n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.neg_euint256(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(3653099566729037n);
  });

  it('test operator "not" overload (euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457577596774950849809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577596774950849809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.not_euint256(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(6411138178790126n);
  });
});
