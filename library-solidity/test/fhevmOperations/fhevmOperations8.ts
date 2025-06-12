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

describe('FHEVM operations 8', function () {
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

  it('test operator "ge" overload (euint128, euint128) => ebool test 1 (340282366920938463463366979368712504533, 340282366920938463463368248008261445385)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463366979368712504533n);
    input.add128(340282366920938463463368248008261445385n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint128, euint128) => ebool test 2 (340282366920938463463366979368712504529, 340282366920938463463366979368712504533)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463366979368712504529n);
    input.add128(340282366920938463463366979368712504533n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint128, euint128) => ebool test 3 (340282366920938463463366979368712504533, 340282366920938463463366979368712504533)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463366979368712504533n);
    input.add128(340282366920938463463366979368712504533n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint128) => ebool test 4 (340282366920938463463366979368712504533, 340282366920938463463366979368712504529)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463366979368712504533n);
    input.add128(340282366920938463463366979368712504529n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint128) => ebool test 1 (340282366920938463463370995022472569787, 340282366920938463463368896639485130255)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463370995022472569787n);
    input.add128(340282366920938463463368896639485130255n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint128) => ebool test 2 (340282366920938463463368896639485130251, 340282366920938463463368896639485130255)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463368896639485130251n);
    input.add128(340282366920938463463368896639485130255n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint128) => ebool test 3 (340282366920938463463368896639485130255, 340282366920938463463368896639485130255)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463368896639485130255n);
    input.add128(340282366920938463463368896639485130255n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint128) => ebool test 4 (340282366920938463463368896639485130255, 340282366920938463463368896639485130251)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463368896639485130255n);
    input.add128(340282366920938463463368896639485130251n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint128) => ebool test 1 (340282366920938463463371623229046350155, 340282366920938463463372721976260954101)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463371623229046350155n);
    input.add128(340282366920938463463372721976260954101n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint128) => ebool test 2 (340282366920938463463371623229046350151, 340282366920938463463371623229046350155)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463371623229046350151n);
    input.add128(340282366920938463463371623229046350155n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint128) => ebool test 3 (340282366920938463463371623229046350155, 340282366920938463463371623229046350155)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463371623229046350155n);
    input.add128(340282366920938463463371623229046350155n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint128) => ebool test 4 (340282366920938463463371623229046350155, 340282366920938463463371623229046350151)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463371623229046350155n);
    input.add128(340282366920938463463371623229046350151n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint128) => ebool test 1 (340282366920938463463371362312241676061, 340282366920938463463368735450827730505)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463371362312241676061n);
    input.add128(340282366920938463463368735450827730505n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint128) => ebool test 2 (340282366920938463463368735450827730501, 340282366920938463463368735450827730505)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463368735450827730501n);
    input.add128(340282366920938463463368735450827730505n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint128, euint128) => ebool test 3 (340282366920938463463368735450827730505, 340282366920938463463368735450827730505)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463368735450827730505n);
    input.add128(340282366920938463463368735450827730505n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint128) => ebool test 4 (340282366920938463463368735450827730505, 340282366920938463463368735450827730501)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463368735450827730505n);
    input.add128(340282366920938463463368735450827730501n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint128, euint128) => euint128 test 1 (340282366920938463463367333886528321945, 340282366920938463463373899630486691515)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463367333886528321945n);
    input.add128(340282366920938463463373899630486691515n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(340282366920938463463367333886528321945n);
  });

  it('test operator "min" overload (euint128, euint128) => euint128 test 2 (340282366920938463463367333886528321941, 340282366920938463463367333886528321945)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463367333886528321941n);
    input.add128(340282366920938463463367333886528321945n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(340282366920938463463367333886528321941n);
  });

  it('test operator "min" overload (euint128, euint128) => euint128 test 3 (340282366920938463463367333886528321945, 340282366920938463463367333886528321945)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463367333886528321945n);
    input.add128(340282366920938463463367333886528321945n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(340282366920938463463367333886528321945n);
  });

  it('test operator "min" overload (euint128, euint128) => euint128 test 4 (340282366920938463463367333886528321945, 340282366920938463463367333886528321941)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463367333886528321945n);
    input.add128(340282366920938463463367333886528321941n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(340282366920938463463367333886528321941n);
  });

  it('test operator "max" overload (euint128, euint128) => euint128 test 1 (340282366920938463463368599296840715501, 340282366920938463463374389163885189209)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463368599296840715501n);
    input.add128(340282366920938463463374389163885189209n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(340282366920938463463374389163885189209n);
  });

  it('test operator "max" overload (euint128, euint128) => euint128 test 2 (340282366920938463463368599296840715497, 340282366920938463463368599296840715501)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463368599296840715497n);
    input.add128(340282366920938463463368599296840715501n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(340282366920938463463368599296840715501n);
  });

  it('test operator "max" overload (euint128, euint128) => euint128 test 3 (340282366920938463463368599296840715501, 340282366920938463463368599296840715501)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463368599296840715501n);
    input.add128(340282366920938463463368599296840715501n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(340282366920938463463368599296840715501n);
  });

  it('test operator "max" overload (euint128, euint128) => euint128 test 4 (340282366920938463463368599296840715501, 340282366920938463463368599296840715497)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463368599296840715501n);
    input.add128(340282366920938463463368599296840715497n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(340282366920938463463368599296840715501n);
  });

  it('test operator "and" overload (euint128, euint256) => euint256 test 1 (340282366920938463463370024640501661081, 115792089237316195423570985008687907853269984665640564039457577868855061970127)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463370024640501661081n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577868855061970127n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463368468347250949257n);
  });

  it('test operator "and" overload (euint128, euint256) => euint256 test 2 (340282366920938463463370024640501661077, 340282366920938463463370024640501661081)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463370024640501661077n);
    input.add256(340282366920938463463370024640501661081n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463370024640501661073n);
  });

  it('test operator "and" overload (euint128, euint256) => euint256 test 3 (340282366920938463463370024640501661081, 340282366920938463463370024640501661081)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463370024640501661081n);
    input.add256(340282366920938463463370024640501661081n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463370024640501661081n);
  });

  it('test operator "and" overload (euint128, euint256) => euint256 test 4 (340282366920938463463370024640501661081, 340282366920938463463370024640501661077)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463370024640501661081n);
    input.add256(340282366920938463463370024640501661077n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463370024640501661073n);
  });

  it('test operator "or" overload (euint128, euint256) => euint256 test 1 (340282366920938463463368082156914000585, 115792089237316195423570985008687907853269984665640564039457582733899624575513)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463368082156914000585n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582733899624575513n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457582874654330581721n);
  });

  it('test operator "or" overload (euint128, euint256) => euint256 test 2 (340282366920938463463368082156914000581, 340282366920938463463368082156914000585)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463368082156914000581n);
    input.add256(340282366920938463463368082156914000585n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463368082156914000589n);
  });

  it('test operator "or" overload (euint128, euint256) => euint256 test 3 (340282366920938463463368082156914000585, 340282366920938463463368082156914000585)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463368082156914000585n);
    input.add256(340282366920938463463368082156914000585n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463368082156914000585n);
  });

  it('test operator "or" overload (euint128, euint256) => euint256 test 4 (340282366920938463463368082156914000585, 340282366920938463463368082156914000581)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463368082156914000585n);
    input.add256(340282366920938463463368082156914000581n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463368082156914000589n);
  });

  it('test operator "xor" overload (euint128, euint256) => euint256 test 1 (340282366920938463463369662172578650737, 115792089237316195423570985008687907853269984665640564039457576112646911470579)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463369662172578650737n);
    input.add256(115792089237316195423570985008687907853269984665640564039457576112646911470579n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907852929702298719625575994213232399828365698n);
  });

  it('test operator "xor" overload (euint128, euint256) => euint256 test 2 (340282366920938463463369662172578650733, 340282366920938463463369662172578650737)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463369662172578650733n);
    input.add256(340282366920938463463369662172578650737n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(28n);
  });

  it('test operator "xor" overload (euint128, euint256) => euint256 test 3 (340282366920938463463369662172578650737, 340282366920938463463369662172578650737)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463369662172578650737n);
    input.add256(340282366920938463463369662172578650737n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint128, euint256) => euint256 test 4 (340282366920938463463369662172578650737, 340282366920938463463369662172578650733)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463369662172578650737n);
    input.add256(340282366920938463463369662172578650733n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(28n);
  });

  it('test operator "eq" overload (euint128, euint256) => ebool test 1 (340282366920938463463374476336879084811, 115792089237316195423570985008687907853269984665640564039457577008691976424477)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463374476336879084811n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577008691976424477n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint256) => ebool test 2 (340282366920938463463374476336879084807, 340282366920938463463374476336879084811)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463374476336879084807n);
    input.add256(340282366920938463463374476336879084811n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint256) => ebool test 3 (340282366920938463463374476336879084811, 340282366920938463463374476336879084811)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463374476336879084811n);
    input.add256(340282366920938463463374476336879084811n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint128, euint256) => ebool test 4 (340282366920938463463374476336879084811, 340282366920938463463374476336879084807)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463374476336879084811n);
    input.add256(340282366920938463463374476336879084807n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint256) => ebool test 1 (340282366920938463463370130193905683605, 115792089237316195423570985008687907853269984665640564039457578602129839620975)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463370130193905683605n);
    input.add256(115792089237316195423570985008687907853269984665640564039457578602129839620975n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint256) => ebool test 2 (340282366920938463463370130193905683601, 340282366920938463463370130193905683605)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463370130193905683601n);
    input.add256(340282366920938463463370130193905683605n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint256) => ebool test 3 (340282366920938463463370130193905683605, 340282366920938463463370130193905683605)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463370130193905683605n);
    input.add256(340282366920938463463370130193905683605n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint256) => ebool test 4 (340282366920938463463370130193905683605, 340282366920938463463370130193905683601)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463370130193905683605n);
    input.add256(340282366920938463463370130193905683601n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "and" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457580171435466527881, 244)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580171435466527881n);
    input.add8(244n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(128n);
  });

  it('test operator "and" overload (euint256, euint8) => euint256 test 2 (240, 244)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(240n);
    input.add8(244n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(240n);
  });

  it('test operator "and" overload (euint256, euint8) => euint256 test 3 (244, 244)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(244n);
    input.add8(244n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(244n);
  });

  it('test operator "and" overload (euint256, euint8) => euint256 test 4 (244, 240)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(244n);
    input.add8(240n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(240n);
  });

  it('test operator "or" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457580467003603747089, 159)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580467003603747089n);
    input.add8(159n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457580467003603747231n);
  });

  it('test operator "or" overload (euint256, euint8) => euint256 test 2 (155, 159)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(155n);
    input.add8(159n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(159n);
  });

  it('test operator "or" overload (euint256, euint8) => euint256 test 3 (159, 159)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(159n);
    input.add8(159n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(159n);
  });

  it('test operator "or" overload (euint256, euint8) => euint256 test 4 (159, 155)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(159n);
    input.add8(155n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(159n);
  });

  it('test operator "xor" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457582739021644288993, 228)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582739021644288993n);
    input.add8(228n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457582739021644288773n);
  });

  it('test operator "xor" overload (euint256, euint8) => euint256 test 2 (224, 228)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(224n);
    input.add8(228n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint256, euint8) => euint256 test 3 (228, 228)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(228n);
    input.add8(228n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint256, euint8) => euint256 test 4 (228, 224)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(228n);
    input.add8(224n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint256, euint8) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457577397514474237541, 76)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577397514474237541n);
    input.add8(76n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint8) => ebool test 2 (72, 76)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(72n);
    input.add8(76n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint8) => ebool test 3 (76, 76)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(76n);
    input.add8(76n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint256, euint8) => ebool test 4 (76, 72)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(76n);
    input.add8(72n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint8) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457580258135508136247, 227)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580258135508136247n);
    input.add8(227n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint8) => ebool test 2 (223, 227)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(223n);
    input.add8(227n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint8) => ebool test 3 (227, 227)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(227n);
    input.add8(227n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint8) => ebool test 4 (227, 223)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(227n);
    input.add8(223n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "and" overload (euint256, euint16) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457581385482146273257, 29756)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581385482146273257n);
    input.add16(29756n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(21544n);
  });

  it('test operator "and" overload (euint256, euint16) => euint256 test 2 (29752, 29756)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(29752n);
    input.add16(29756n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(29752n);
  });

  it('test operator "and" overload (euint256, euint16) => euint256 test 3 (29756, 29756)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(29756n);
    input.add16(29756n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(29756n);
  });

  it('test operator "and" overload (euint256, euint16) => euint256 test 4 (29756, 29752)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(29756n);
    input.add16(29752n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(29752n);
  });

  it('test operator "or" overload (euint256, euint16) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457576699899793345395, 1195)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576699899793345395n);
    input.add16(1195n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576699899793346555n);
  });

  it('test operator "or" overload (euint256, euint16) => euint256 test 2 (1191, 1195)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(1191n);
    input.add16(1195n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(1199n);
  });

  it('test operator "or" overload (euint256, euint16) => euint256 test 3 (1195, 1195)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(1195n);
    input.add16(1195n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(1195n);
  });

  it('test operator "or" overload (euint256, euint16) => euint256 test 4 (1195, 1191)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(1195n);
    input.add16(1191n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(1199n);
  });

  it('test operator "xor" overload (euint256, euint16) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457580838795922646125, 11043)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580838795922646125n);
    input.add16(11043n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457580838795922640718n);
  });

  it('test operator "xor" overload (euint256, euint16) => euint256 test 2 (11039, 11043)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(11039n);
    input.add16(11043n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(60n);
  });

  it('test operator "xor" overload (euint256, euint16) => euint256 test 3 (11043, 11043)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(11043n);
    input.add16(11043n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint256, euint16) => euint256 test 4 (11043, 11039)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(11043n);
    input.add16(11039n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(60n);
  });

  it('test operator "eq" overload (euint256, euint16) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457583590859977133017, 27363)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457583590859977133017n);
    input.add16(27363n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint16) => ebool test 2 (27359, 27363)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(27359n);
    input.add16(27363n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint16) => ebool test 3 (27363, 27363)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(27363n);
    input.add16(27363n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint256, euint16) => ebool test 4 (27363, 27359)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(27363n);
    input.add16(27359n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint16) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457576885586324296393, 31463)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576885586324296393n);
    input.add16(31463n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint16) => ebool test 2 (31459, 31463)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(31459n);
    input.add16(31463n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint16) => ebool test 3 (31463, 31463)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(31463n);
    input.add16(31463n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint16) => ebool test 4 (31463, 31459)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(31463n);
    input.add16(31459n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "and" overload (euint256, euint32) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457581674976804078387, 3278454088)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581674976804078387n);
    input.add32(3278454088n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(3272081664n);
  });

  it('test operator "and" overload (euint256, euint32) => euint256 test 2 (3278454084, 3278454088)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(3278454084n);
    input.add32(3278454088n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(3278454080n);
  });

  it('test operator "and" overload (euint256, euint32) => euint256 test 3 (3278454088, 3278454088)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(3278454088n);
    input.add32(3278454088n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(3278454088n);
  });

  it('test operator "and" overload (euint256, euint32) => euint256 test 4 (3278454088, 3278454084)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(3278454088n);
    input.add32(3278454084n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(3278454080n);
  });

  it('test operator "or" overload (euint256, euint32) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457577906086807900785, 571800297)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577906086807900785n);
    input.add32(571800297n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577906087378352889n);
  });

  it('test operator "or" overload (euint256, euint32) => euint256 test 2 (571800293, 571800297)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(571800293n);
    input.add32(571800297n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(571800301n);
  });

  it('test operator "or" overload (euint256, euint32) => euint256 test 3 (571800297, 571800297)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(571800297n);
    input.add32(571800297n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(571800297n);
  });

  it('test operator "or" overload (euint256, euint32) => euint256 test 4 (571800297, 571800293)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(571800297n);
    input.add32(571800293n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(571800301n);
  });

  it('test operator "xor" overload (euint256, euint32) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457578678094830687969, 2765705167)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578678094830687969n);
    input.add32(2765705167n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457578678092208773422n);
  });

  it('test operator "xor" overload (euint256, euint32) => euint256 test 2 (2765705163, 2765705167)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(2765705163n);
    input.add32(2765705167n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint256, euint32) => euint256 test 3 (2765705167, 2765705167)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(2765705167n);
    input.add32(2765705167n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint256, euint32) => euint256 test 4 (2765705167, 2765705163)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(2765705167n);
    input.add32(2765705163n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint256, euint32) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457582844216705976809, 1527031774)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582844216705976809n);
    input.add32(1527031774n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint32) => ebool test 2 (1527031770, 1527031774)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(1527031770n);
    input.add32(1527031774n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint32) => ebool test 3 (1527031774, 1527031774)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(1527031774n);
    input.add32(1527031774n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint256, euint32) => ebool test 4 (1527031774, 1527031770)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(1527031774n);
    input.add32(1527031770n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint32) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457577491374040630423, 2694038329)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577491374040630423n);
    input.add32(2694038329n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint32) => ebool test 2 (2694038325, 2694038329)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(2694038325n);
    input.add32(2694038329n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint32) => ebool test 3 (2694038329, 2694038329)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(2694038329n);
    input.add32(2694038329n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint32) => ebool test 4 (2694038329, 2694038325)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(2694038329n);
    input.add32(2694038325n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "and" overload (euint256, euint64) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457577763310056966991, 18441143179652592781)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577763310056966991n);
    input.add64(18441143179652592781n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(18440006266589216781n);
  });

  it('test operator "and" overload (euint256, euint64) => euint256 test 2 (18441143179652592777, 18441143179652592781)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18441143179652592777n);
    input.add64(18441143179652592781n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(18441143179652592777n);
  });

  it('test operator "and" overload (euint256, euint64) => euint256 test 3 (18441143179652592781, 18441143179652592781)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18441143179652592781n);
    input.add64(18441143179652592781n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(18441143179652592781n);
  });

  it('test operator "and" overload (euint256, euint64) => euint256 test 4 (18441143179652592781, 18441143179652592777)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18441143179652592781n);
    input.add64(18441143179652592777n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(18441143179652592777n);
  });

  it('test operator "or" overload (euint256, euint64) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457575417313334930731, 18446340287582270945)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575417313334930731n);
    input.add64(18446340287582270945n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457584002309268274667n);
  });

  it('test operator "or" overload (euint256, euint64) => euint256 test 2 (18446340287582270941, 18446340287582270945)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18446340287582270941n);
    input.add64(18446340287582270945n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(18446340287582270973n);
  });

  it('test operator "or" overload (euint256, euint64) => euint256 test 3 (18446340287582270945, 18446340287582270945)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18446340287582270945n);
    input.add64(18446340287582270945n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(18446340287582270945n);
  });

  it('test operator "or" overload (euint256, euint64) => euint256 test 4 (18446340287582270945, 18446340287582270941)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18446340287582270945n);
    input.add64(18446340287582270941n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(18446340287582270973n);
  });

  it('test operator "xor" overload (euint256, euint64) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457577381012197796683, 18440395870105721993)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577381012197796683n);
    input.add64(18440395870105721993n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039439137552503245486018n);
  });

  it('test operator "xor" overload (euint256, euint64) => euint256 test 2 (18440395870105721989, 18440395870105721993)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18440395870105721989n);
    input.add64(18440395870105721993n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint256, euint64) => euint256 test 3 (18440395870105721993, 18440395870105721993)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18440395870105721993n);
    input.add64(18440395870105721993n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint256, euint64) => euint256 test 4 (18440395870105721993, 18440395870105721989)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18440395870105721993n);
    input.add64(18440395870105721989n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint256, euint64) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457583153535025406931, 18439347021796871195)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457583153535025406931n);
    input.add64(18439347021796871195n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint64) => ebool test 2 (18439347021796871191, 18439347021796871195)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18439347021796871191n);
    input.add64(18439347021796871195n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint64) => ebool test 3 (18439347021796871195, 18439347021796871195)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18439347021796871195n);
    input.add64(18439347021796871195n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint256, euint64) => ebool test 4 (18439347021796871195, 18439347021796871191)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18439347021796871195n);
    input.add64(18439347021796871191n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint64) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457582188327862575507, 18438219907286461813)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582188327862575507n);
    input.add64(18438219907286461813n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint64) => ebool test 2 (18438219907286461809, 18438219907286461813)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18438219907286461809n);
    input.add64(18438219907286461813n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint64) => ebool test 3 (18438219907286461813, 18438219907286461813)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18438219907286461813n);
    input.add64(18438219907286461813n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint64) => ebool test 4 (18438219907286461813, 18438219907286461809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18438219907286461813n);
    input.add64(18438219907286461809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "and" overload (euint256, euint128) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457575538259235497407, 340282366920938463463368384266959187597)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575538259235497407n);
    input.add128(340282366920938463463368384266959187597n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463366127998229341325n);
  });

  it('test operator "and" overload (euint256, euint128) => euint256 test 2 (340282366920938463463368384266959187593, 340282366920938463463368384266959187597)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463368384266959187593n);
    input.add128(340282366920938463463368384266959187597n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463368384266959187593n);
  });

  it('test operator "and" overload (euint256, euint128) => euint256 test 3 (340282366920938463463368384266959187597, 340282366920938463463368384266959187597)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463368384266959187597n);
    input.add128(340282366920938463463368384266959187597n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463368384266959187597n);
  });

  it('test operator "and" overload (euint256, euint128) => euint256 test 4 (340282366920938463463368384266959187597, 340282366920938463463368384266959187593)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463368384266959187597n);
    input.add128(340282366920938463463368384266959187593n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463368384266959187593n);
  });

  it('test operator "or" overload (euint256, euint128) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457581270489204586017, 340282366920938463463367448294069154213)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581270489204586017n);
    input.add128(340282366920938463463367448294069154213n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457581429420384384933n);
  });

  it('test operator "or" overload (euint256, euint128) => euint256 test 2 (340282366920938463463367448294069154209, 340282366920938463463367448294069154213)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463367448294069154209n);
    input.add128(340282366920938463463367448294069154213n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463367448294069154213n);
  });

  it('test operator "or" overload (euint256, euint128) => euint256 test 3 (340282366920938463463367448294069154213, 340282366920938463463367448294069154213)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463367448294069154213n);
    input.add128(340282366920938463463367448294069154213n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463367448294069154213n);
  });

  it('test operator "or" overload (euint256, euint128) => euint256 test 4 (340282366920938463463367448294069154213, 340282366920938463463367448294069154209)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463367448294069154213n);
    input.add128(340282366920938463463367448294069154209n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463367448294069154213n);
  });

  it('test operator "xor" overload (euint256, euint128) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457578157325127042359, 340282366920938463463372729994023374687)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578157325127042359n);
    input.add128(340282366920938463463372729994023374687n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907852929702298719625575994214575406426704488n);
  });

  it('test operator "xor" overload (euint256, euint128) => euint256 test 2 (340282366920938463463372729994023374683, 340282366920938463463372729994023374687)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463372729994023374683n);
    input.add128(340282366920938463463372729994023374687n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint256, euint128) => euint256 test 3 (340282366920938463463372729994023374687, 340282366920938463463372729994023374687)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463372729994023374687n);
    input.add128(340282366920938463463372729994023374687n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint256, euint128) => euint256 test 4 (340282366920938463463372729994023374687, 340282366920938463463372729994023374683)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463372729994023374687n);
    input.add128(340282366920938463463372729994023374683n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint256, euint128) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457583332264470522795, 340282366920938463463373866753031371121)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457583332264470522795n);
    input.add128(340282366920938463463373866753031371121n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint128) => ebool test 2 (340282366920938463463373866753031371117, 340282366920938463463373866753031371121)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463373866753031371117n);
    input.add128(340282366920938463463373866753031371121n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint128) => ebool test 3 (340282366920938463463373866753031371121, 340282366920938463463373866753031371121)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463373866753031371121n);
    input.add128(340282366920938463463373866753031371121n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint256, euint128) => ebool test 4 (340282366920938463463373866753031371121, 340282366920938463463373866753031371117)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463373866753031371121n);
    input.add128(340282366920938463463373866753031371117n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint128) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457576604433106185701, 340282366920938463463372962579491817401)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576604433106185701n);
    input.add128(340282366920938463463372962579491817401n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint128) => ebool test 2 (340282366920938463463372962579491817397, 340282366920938463463372962579491817401)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463372962579491817397n);
    input.add128(340282366920938463463372962579491817401n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint128) => ebool test 3 (340282366920938463463372962579491817401, 340282366920938463463372962579491817401)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463372962579491817401n);
    input.add128(340282366920938463463372962579491817401n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint128) => ebool test 4 (340282366920938463463372962579491817401, 340282366920938463463372962579491817397)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463372962579491817401n);
    input.add128(340282366920938463463372962579491817397n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "and" overload (euint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457583476569507143393, 115792089237316195423570985008687907853269984665640564039457576398624363873279)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457583476569507143393n);
    input.add256(115792089237316195423570985008687907853269984665640564039457576398624363873279n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576148760310874849n);
  });

  it('test operator "and" overload (euint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457576398624363873275, 115792089237316195423570985008687907853269984665640564039457576398624363873279)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576398624363873275n);
    input.add256(115792089237316195423570985008687907853269984665640564039457576398624363873279n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576398624363873275n);
  });

  it('test operator "and" overload (euint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457576398624363873279, 115792089237316195423570985008687907853269984665640564039457576398624363873279)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576398624363873279n);
    input.add256(115792089237316195423570985008687907853269984665640564039457576398624363873279n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576398624363873279n);
  });

  it('test operator "and" overload (euint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457576398624363873279, 115792089237316195423570985008687907853269984665640564039457576398624363873275)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576398624363873279n);
    input.add256(115792089237316195423570985008687907853269984665640564039457576398624363873275n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576398624363873275n);
  });

  it('test operator "or" overload (euint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457576120721729938827, 115792089237316195423570985008687907853269984665640564039457581172888450353493)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576120721729938827n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581172888450353493n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457581755767157943775n);
  });

  it('test operator "or" overload (euint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457576120721729938823, 115792089237316195423570985008687907853269984665640564039457576120721729938827)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576120721729938823n);
    input.add256(115792089237316195423570985008687907853269984665640564039457576120721729938827n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576120721729938831n);
  });

  it('test operator "or" overload (euint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457576120721729938827, 115792089237316195423570985008687907853269984665640564039457576120721729938827)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576120721729938827n);
    input.add256(115792089237316195423570985008687907853269984665640564039457576120721729938827n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576120721729938827n);
  });

  it('test operator "or" overload (euint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457576120721729938827, 115792089237316195423570985008687907853269984665640564039457576120721729938823)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576120721729938827n);
    input.add256(115792089237316195423570985008687907853269984665640564039457576120721729938823n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576120721729938831n);
  });

  it('test operator "xor" overload (euint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457582474364217937289, 115792089237316195423570985008687907853269984665640564039457583270149374380145)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582474364217937289n);
    input.add256(115792089237316195423570985008687907853269984665640564039457583270149374380145n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(2230080913786360n);
  });

  it('test operator "xor" overload (euint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457582474364217937285, 115792089237316195423570985008687907853269984665640564039457582474364217937289)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582474364217937285n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582474364217937289n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457582474364217937289, 115792089237316195423570985008687907853269984665640564039457582474364217937289)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582474364217937289n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582474364217937289n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457582474364217937289, 115792089237316195423570985008687907853269984665640564039457582474364217937285)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582474364217937289n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582474364217937285n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457581509794520384545, 115792089237316195423570985008687907853269984665640564039457581649355389960443)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581509794520384545n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581649355389960443n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457581509794520384541, 115792089237316195423570985008687907853269984665640564039457581509794520384545)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581509794520384541n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581509794520384545n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457581509794520384545, 115792089237316195423570985008687907853269984665640564039457581509794520384545)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581509794520384545n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581509794520384545n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457581509794520384545, 115792089237316195423570985008687907853269984665640564039457581509794520384541)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581509794520384545n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581509794520384541n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457584002695830945561, 115792089237316195423570985008687907853269984665640564039457580644856625898095)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457584002695830945561n);
    input.add256(115792089237316195423570985008687907853269984665640564039457580644856625898095n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457580644856625898091, 115792089237316195423570985008687907853269984665640564039457580644856625898095)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580644856625898091n);
    input.add256(115792089237316195423570985008687907853269984665640564039457580644856625898095n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457580644856625898095, 115792089237316195423570985008687907853269984665640564039457580644856625898095)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580644856625898095n);
    input.add256(115792089237316195423570985008687907853269984665640564039457580644856625898095n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457580644856625898095, 115792089237316195423570985008687907853269984665640564039457580644856625898091)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580644856625898095n);
    input.add256(115792089237316195423570985008687907853269984665640564039457580644856625898091n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 1 (125, 59)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(125n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint8_uint8(encryptedAmount.handles[0], 59n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(184n);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 2 (81, 85)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(81n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint8_uint8(encryptedAmount.handles[0], 85n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(166n);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 3 (85, 85)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(85n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint8_uint8(encryptedAmount.handles[0], 85n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(170n);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 4 (85, 81)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(85n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint8_uint8(encryptedAmount.handles[0], 81n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(166n);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 1 (78, 59)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(59n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_uint8_euint8(78n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(137n);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 2 (81, 85)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(85n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_uint8_euint8(81n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(166n);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 3 (85, 85)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(85n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_uint8_euint8(85n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(170n);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 4 (85, 81)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(81n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_uint8_euint8(85n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(166n);
  });

  it('test operator "sub" overload (euint8, uint8) => euint8 test 1 (188, 188)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(188n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint8_uint8(encryptedAmount.handles[0], 188n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, uint8) => euint8 test 2 (188, 184)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(188n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint8_uint8(encryptedAmount.handles[0], 184n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint8, euint8) => euint8 test 1 (188, 188)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(188n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_uint8_euint8(188n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint8, euint8) => euint8 test 2 (188, 184)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(184n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_uint8_euint8(188n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 1 (17, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(17n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint8_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(136n);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 2 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(10n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint8_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(100n);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(10n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint8_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(100n);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 4 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(10n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint8_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(100n);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 1 (11, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_uint8_euint8(11n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(88n);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 2 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_uint8_euint8(10n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(100n);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_uint8_euint8(10n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(100n);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 4 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_uint8_euint8(10n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(100n);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 1 (35, 206)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(35n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.div_euint8_uint8(encryptedAmount.handles[0], 206n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 2 (31, 35)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(31n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.div_euint8_uint8(encryptedAmount.handles[0], 35n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 3 (35, 35)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(35n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.div_euint8_uint8(encryptedAmount.handles[0], 35n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 4 (35, 31)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(35n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.div_euint8_uint8(encryptedAmount.handles[0], 31n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 1 (137, 139)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(137n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rem_euint8_uint8(encryptedAmount.handles[0], 139n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(137n);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 2 (133, 137)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(133n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rem_euint8_uint8(encryptedAmount.handles[0], 137n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(133n);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 3 (137, 137)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(137n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rem_euint8_uint8(encryptedAmount.handles[0], 137n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 4 (137, 133)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(137n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rem_euint8_uint8(encryptedAmount.handles[0], 133n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(4n);
  });

  it('test operator "and" overload (euint8, uint8) => euint8 test 1 (208, 146)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(208n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint8_uint8(encryptedAmount.handles[0], 146n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(144n);
  });

  it('test operator "and" overload (euint8, uint8) => euint8 test 2 (204, 208)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(204n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint8_uint8(encryptedAmount.handles[0], 208n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(192n);
  });

  it('test operator "and" overload (euint8, uint8) => euint8 test 3 (208, 208)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(208n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint8_uint8(encryptedAmount.handles[0], 208n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(208n);
  });

  it('test operator "and" overload (euint8, uint8) => euint8 test 4 (208, 204)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(208n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint8_uint8(encryptedAmount.handles[0], 204n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(192n);
  });

  it('test operator "and" overload (uint8, euint8) => euint8 test 1 (242, 146)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(146n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint8_euint8(242n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(146n);
  });

  it('test operator "and" overload (uint8, euint8) => euint8 test 2 (204, 208)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(208n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint8_euint8(204n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(192n);
  });

  it('test operator "and" overload (uint8, euint8) => euint8 test 3 (208, 208)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(208n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint8_euint8(208n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(208n);
  });

  it('test operator "and" overload (uint8, euint8) => euint8 test 4 (208, 204)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(204n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint8_euint8(208n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(192n);
  });

  it('test operator "or" overload (euint8, uint8) => euint8 test 1 (22, 162)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(22n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint8_uint8(encryptedAmount.handles[0], 162n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(182n);
  });

  it('test operator "or" overload (euint8, uint8) => euint8 test 2 (18, 22)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(18n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint8_uint8(encryptedAmount.handles[0], 22n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(22n);
  });

  it('test operator "or" overload (euint8, uint8) => euint8 test 3 (22, 22)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(22n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint8_uint8(encryptedAmount.handles[0], 22n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(22n);
  });

  it('test operator "or" overload (euint8, uint8) => euint8 test 4 (22, 18)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(22n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint8_uint8(encryptedAmount.handles[0], 18n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(22n);
  });
});
